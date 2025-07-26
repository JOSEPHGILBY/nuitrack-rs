use futures_util::StreamExt;
use nuitrack_rs::{
    nuitrack::shared_types::error::NuitrackError,
    setup_nuitrack_streams
};
use tracing::{error, info, warn, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO) // Set a default log level
        .with_target(true)           // Show the module path for context
        .init();
    info!("Attempting to initialize Nuitrack with Tokio runtime...");
    let (mut hand_stream, mut skeleton_stream, mut depth_stream, mut user_stream, session) = setup_nuitrack_streams!(HandTracker, SkeletonTracker, DepthSensor, UserTracker).await?;

    // 4. Start Nuitrack processing (this also starts the internal update loop if enabled)
    session.start_processing().await.map_err(|e: NuitrackError| {
        error!(error = %e, "Failed to start Nuitrack processing");
        anyhow::anyhow!("Nuitrack run failed: {}", e)
    })?;
    info!("Nuitrack processing and internal update loop started.");

    // 5. Process frames from the stream
    for i in 0..1000 { // Let's try to get a few frames
        info!("Waiting for synchronized frame set #{}...", i + 1);

        // 3. Use `tokio::join!` to await the next frame from each stream
        let (depth_result, user_result, skeleton_result, hand_result) = tokio::join!(
            depth_stream.next(),
            user_stream.next(),
            skeleton_stream.next(),
            hand_stream.next()
        );

        // 4. Match on the tuple of results to get the synchronized frames
        match (depth_result, user_result, skeleton_result, hand_result) {
            (
                Some(Ok(depth_frame)),
                Some(Ok(user_frame)),
                Some(Ok(skeleton_frame)),
                Some(Ok(hand_frame)),
            ) => {
                // All frames received successfully!
                let ts = depth_frame.timestamp()?;

                // Prove that all timestamps are identical
                assert_eq!(ts, user_frame.timestamp()?);
                assert_eq!(ts, skeleton_frame.timestamp()?);
                assert_eq!(ts, hand_frame.timestamp()?);

                info!(
                    "--> SYNCHRONIZED FRAME BUNDLE RECEIVED | Timestamp: {}",
                    ts
                );

                // Now you can process them together, e.g., find the skeleton for a specific user
                // let skeletons = skeleton_frame.skeletons()?;
                // let users = user_frame.users()?;
                // ... your application logic here ...
            }
            _ => {
                warn!("A stream ended or produced an error. Exiting loop.");
                break;
            }
        }
    }
    info!("Finished processing frames or example duration ended.");

    // 6. Close the Nuitrack session (this will stop the internal update loop and release resources)
    session.close().await.map_err(|e| {
        error!(error = ?e, "Error closing Nuitrack session");
        anyhow::anyhow!("Error closing Nuitrack session: {}", e)
    })?;
    info!("Nuitrack session closed successfully.");

    Ok(())
}
