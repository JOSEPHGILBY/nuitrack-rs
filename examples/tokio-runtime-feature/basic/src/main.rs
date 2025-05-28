use std::sync::{atomic::{AtomicBool, Ordering}, Arc}; // AtomicBool and Arc might not be needed if session handles updates

use futures_util::StreamExt;
use nuitrack_rs::nuitrack::{
    shared_types::{
        error::NuitrackError,
        session_config::{DeviceConfig, DeviceSelector, ModuleType}
    },
    async_api::{session::NuitrackSession, session_builder::NuitrackSessionBuilder}
    // Potentially frame::HandFrameData if you want to inspect it (already in use via stream)
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Attempting to initialize Nuitrack with Tokio runtime using NuitrackSessionBuilder...");

    let mut session = NuitrackSessionBuilder::create_session_from_single_default_device(
        vec![ModuleType::HandTracker]
    ).await?;
    println!("NuitrackSession initialized.");

    // 2. Retrieve the AsyncHandTracker from the session
    // Assuming the first configured device (index 0) and HandTracker was created.
    let hand_tracker = session.active_devices
        .get_mut(0) // Get the context for the first (and only, in this config) device
        .and_then(|device_context| device_context.hand_tracker.as_mut())
        .ok_or_else(|| {
            anyhow::anyhow!("AsyncHandTracker not found for the configured device.")
        })?;
    println!("AsyncHandTracker retrieved from session.");

    // 3. Get the hand frames stream
    let mut stream = hand_tracker.hand_frames_stream().map_err(|e| {
        eprintln!("Failed to get hand frames stream: {:?}", e);
        anyhow::anyhow!("Failed to get hand frames stream: {}", e)
    })?;
    println!("Got hand frames stream. Waiting for up to 100 frames or stream end...");

    // 4. Start Nuitrack processing (this also starts the internal update loop if enabled)
    session.start_processing().await.map_err(|e: NuitrackError| {
        eprintln!("Failed to start Nuitrack processing: {:?}", e);
        anyhow::anyhow!("Nuitrack run failed: {}", e)
    })?;
    println!("Nuitrack processing and internal update loop started.");

    // 5. Process frames from the stream
    for i in 0..100 { // Let's try to get a few frames
        tokio::select! {
            biased; // Ensure cancellation/timeout is checked preferentially if multiple branches are ready
            Some(frame_result) = stream.next() => {
                match frame_result {
                    Ok(frame_data) => {
                        println!(
                            "Frame {}: Timestamp: {}, Users: {}",
                            i,
                            frame_data.timestamp().unwrap_or(0), 
                            frame_data.num_users().unwrap_or(0)  
                        );
                        if let Ok(num_users) = frame_data.num_users() {
                            if num_users > 0 {
                                // Attempt to get hands for the first user (index 0)
                                if let Ok(user_hands) = frame_data.user_hands_at_index(0) { // Get the whole vector
                                    if let Some(right_hand) = &user_hands.right_hand { // Borrow right_hand
                                        println!("  User ID {} Right Hand: x_real: {:.3}", user_hands.user_id, right_hand.x_real());
                                    }
                                    if let Some(left_hand) = &user_hands.left_hand { // Borrow left_hand
                                        println!("  User ID {} Left Hand: x_real: {:.3}", user_hands.user_id, left_hand.x_real());
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving frame data: {:?}", e);
                        // If the error is channel disconnected, it means the sender (AsyncHandTracker) was dropped or closed.
                        if matches!(e, NuitrackError::Wrapper(ref s) if s.contains("Channel disconnected")) {
                            println!("Hand tracker data channel disconnected, likely due to Nuitrack closing. Stopping frame processing.");
                            break;
                        }
                    }
                }
            }
            // Consider a global timeout or cancellation mechanism if needed for the main example loop.
            // The tokio::select with stream.next() already handles the stream ending.
            // If you want an overall timeout for the example:
            _ = tokio::time::sleep(std::time::Duration::from_secs(60)) => { 
                println!("Example timeout after 60 seconds.");
                break;
            }
        }
    }
    println!("Finished processing frames or example duration ended.");

    // 6. Close the Nuitrack session (this will stop the internal update loop and release resources)
    session.close().await.map_err(|e| {
        eprintln!("Error closing Nuitrack session: {:?}", e);
        anyhow::anyhow!("Error closing Nuitrack session: {}", e)
    })?;
    println!("Nuitrack session closed successfully.");

    Ok(())
}
