use anyhow::Result;
use futures_util::StreamExt;
use nuitrack_rs::nuitrack::{
    async_api::{session_builder::NuitrackSessionBuilder, session::NuitrackSession},
    shared_types::error::NuitrackError,
};
use std::time::Duration;
use tracing::{error, info, instrument, trace, warn, Level};

/// This test validates the RAII behavior of async streams.
/// 1. A stream's Drop implementation should unregister its FFI callback.
/// 2. A new stream can be created after the old one is dropped.
/// 3. A second stream cannot be created while the first is still active.
/// 4. The tracker's Drop implementation should clean up any active streams.
#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    // Setup tracing to show TRACE level logs, which are in the FFI callbacks.
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_thread_ids(true)
        .with_target(false)
        .init();

    info!("--- Test: Async Stream RAII and Cleanup ---");

    // --- Setup Nuitrack ---
    let mut session = NuitrackSessionBuilder::create_session_from_single_default_device(vec![
        nuitrack_rs::nuitrack::shared_types::session_config::ModuleType::ColorSensor,
    ])
    .await?;
    session.start_processing().await?;
    info!("Nuitrack session started and processing.");

    let color_sensor = session.active_devices[0].color_sensor.as_mut().unwrap();

    // --- Test 1: Create a stream and verify it works ---
    info!("[1] Creating first color stream. We expect to see FFI callback traces.");
    let mut color_stream_1 = color_sensor.rgb_frames_stream()?;
    
    // Let Nuitrack run for a bit to see the trace logs from the callback.
    tokio::time::sleep(Duration::from_secs(2)).await;

    // --- Test 2: Try to create a second stream while the first is active (should fail) ---
    info!("[2] Attempting to create a second stream. This should fail.");
    match color_sensor.rgb_frames_stream() {
        Ok(_) => {
            error!("FAIL: A second stream was created while the first was still active.");
            return Err(anyhow::anyhow!("Duplicate stream creation was allowed."));
        }
        Err(NuitrackError::OperationFailed(msg)) => {
            info!("SUCCESS: Correctly failed to create duplicate stream: {}", msg);
        }
        Err(e) => {
            error!("FAIL: Received an unexpected error: {:?}", e);
            return Err(e.into());
        }
    }
    
    // --- Test 3: Drop the first stream and verify callbacks stop ---
    info!("[3] Dropping the first stream. FFI callback traces should now STOP.");
    drop(color_stream_1);

    // Let Nuitrack run for a bit to confirm no more traces appear.
    tokio::time::sleep(Duration::from_secs(2)).await;

    // --- Test 4: Create a new stream in the same slot (should succeed) ---
    info!("[4] Creating a new stream in the now-empty slot. FFI traces should RESUME.");
    let mut color_stream_2 = color_sensor.rgb_frames_stream()?;
    
    // Let it run to see the new set of trace logs.
    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("[5] Dropping the tracker itself (via session). This should also stop FFI callbacks.");
    session.close().await?;
    
    // Let Nuitrack run for a bit to confirm no more traces appear.
    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("--- Test Complete ---");
    info!("Please check the logs to verify that trace messages appeared and disappeared as expected.");
    Ok(())
}