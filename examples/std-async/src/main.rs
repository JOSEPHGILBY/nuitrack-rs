use std::{sync::Arc, time::Duration};

use async_std::task;
use futures::{future::FutureExt, select, StreamExt};
use nuitrack_rs::{nuitrack::shared_types::error::NuitrackError, setup_nuitrack_streams};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    println!("Attempting to initialize Nuitrack with async-std runtime...");
    let (mut hand_stream, mut skeleton_stream, mut color_stream, session) =
        setup_nuitrack_streams!(HandTracker, SkeletonTracker, ColorSensor).await?;

    let session = Arc::new(session);
    let session_for_update_task = Arc::clone(&session);
    let update_task_handle = task::spawn(async move {
        println!("[Update Task] Started.");
        // Loop until the main task signals for shutdown.
        loop {
            // drive_update_cycle() polls Nuitrack for new data, feeding the streams.
            if let Err(e) = session_for_update_task.drive_update_cycle().await {
                eprintln!("[Update Task] Error driving update cycle: {:?}", e);
                break;
            }
            // Yield control and prevent a tight loop that would burn 100% CPU.
            task::sleep(Duration::from_millis(1)).await;
        }
        println!("[Update Task] Stopped.");
    });


    session.start_processing().await.map_err(|e: NuitrackError| {
        eprintln!("Failed to start Nuitrack processing: {:?}", e);
        anyhow::anyhow!("Nuitrack run failed: {}", e)
    })?;
    println!("Nuitrack processing started.");

    // Process frames from the streams
    for i in 0..1000 {
        // Use the runtime-agnostic select! from the `futures` crate
        select! {
            hand_frame_result = hand_stream.next().fuse() => {
                if let Some(Ok(hand_frame)) = hand_frame_result {
                    println!(
                        "Hand Frame {}:     Timestamp: {}, Users: {}",
                        i,
                        hand_frame.timestamp().unwrap_or(0),
                        hand_frame.num_users().unwrap_or(0)
                    );
                }
            },
            skeleton_frame_result = skeleton_stream.next().fuse() => {
                if let Some(Ok(frame)) = skeleton_frame_result {
                    println!(
                        "Skeleton Frame {}: Timestamp: {}",
                        i,
                        frame.timestamp().unwrap_or(0),
                    );
                }
            },
            color_frame_result = color_stream.next().fuse() => {
                if let Some(Ok(frame)) = color_frame_result {
                    println!(
                        "Color Frame {}:    Timestamp: {}",
                        i,
                        frame.timestamp().unwrap_or(0),
                    );
                }
            },
            // The `complete` branch ensures the select macro doesn't panic
            // if all streams have ended.
            complete => break,
        }
    }
    println!("Finished processing frames.");
    update_task_handle.cancel().await;
    if let Ok(session) = Arc::try_unwrap(session) {
        session.close().await.map_err(|e| {
            eprintln!("Error closing Nuitrack session: {:?}", e);
            anyhow::anyhow!("Error closing Nuitrack session: {}", e)
        })?;
        println!("Nuitrack session closed successfully.");
    } else {
        eprintln!("Could not get exclusive ownership of session to close. This is unexpected.");
    }
    println!("Nuitrack session closed successfully.");

    Ok(())
}