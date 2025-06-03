use futures_util::StreamExt;
use nuitrack_rs::{
    nuitrack::shared_types::error::NuitrackError,
    setup_nuitrack_streams
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Attempting to initialize Nuitrack with Tokio runtime using NuitrackSessionBuilder...");
    let (mut hand_stream, mut skeleton_stream, mut color_stream, session) = setup_nuitrack_streams!(HandTracker, SkeletonTracker, ColorSensor).await?;

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
            Some(hand_frame_result) = hand_stream.next() => { 
                let should_break_loop = 'process_frame: {
                    let hand_frame = match hand_frame_result {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("Error receiving frame data: {:?}", e);
                            // If the error is channel disconnected, it means the sender (AsyncHandTracker) was dropped or closed.
                            if matches!(e, NuitrackError::Wrapper(ref s) if s.contains("Channel disconnected")) {
                                println!("Hand tracker data channel disconnected, likely due to Nuitrack closing. Stopping frame processing.");
                                break 'process_frame true;
                            }
                            break 'process_frame false;
                        }
                    };
                    println!(
                        "Hand Frame {}:     Timestamp: {}, Users: {}",
                        i,
                        hand_frame.timestamp().unwrap_or(0), 
                        hand_frame.num_users().unwrap_or(0)  
                    );
                    let Ok(num_users) = hand_frame.num_users() else {
                        break 'process_frame false;
                    };
                    if num_users == 0 {
                        break 'process_frame false;
                    }

                    let Ok(users_hands) = hand_frame.users_hands() else {
                        break 'process_frame false;
                    };
                    let user_hands = &users_hands[0];
                    
                    if let Some(right_hand) = &user_hands.right_hand { // Borrow right_hand
                        println!("  User ID {} Right Hand: x_real: {:.3}", user_hands.user_id, right_hand.x_real);
                    }
                    if let Some(left_hand) = &user_hands.left_hand { // Borrow left_hand
                        println!("  User ID {} Left Hand: x_real: {:.3}", user_hands.user_id, left_hand.x_real);
                    }
                    
                    false 
                };
                if should_break_loop {
                    break;
                }
            }
            Some(skeleton_frame_result) = skeleton_stream.next() => {
                let should_break_loop = 'process_frame: {
                    let frame = match skeleton_frame_result {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("Error receiving skeleton frame data: {:?}", e);
                            // If the error is channel disconnected, it means the sender (AsyncHandTracker) was dropped or closed.
                            if matches!(e, NuitrackError::Wrapper(ref s) if s.contains("Channel disconnected")) {
                                println!("Skeleton tracker data channel disconnected, likely due to Nuitrack closing. Stopping frame processing.");
                                break 'process_frame true;
                            }
                            break 'process_frame false;
                        }
                    };

                    println!(
                        "Skeleton Frame {}: Timestamp: {}, Users: {}",
                        i,
                        frame.timestamp().unwrap_or(0), 
                        frame.get_num_skeletons().unwrap_or(0)  
                    );
                    false
                };

                if should_break_loop {
                    break;
                }
            }
            Some(color_frame_result) = color_stream.next() => {
                let should_break_loop = 'process_frame: {
                    let frame = match color_frame_result {
                        Ok(frame) => frame,
                        Err(e) => {
                            eprintln!("Error receiving color frame data: {:?}", e);
                            // If the error is channel disconnected, it means the sender (AsyncHandTracker) was dropped or closed.
                            if matches!(e, NuitrackError::Wrapper(ref s) if s.contains("Channel disconnected")) {
                                println!("Color sensor data channel disconnected, likely due to Nuitrack closing. Stopping frame processing.");
                                break 'process_frame true;
                            }
                            break 'process_frame false;
                        }
                    };
                    println!(
                        "Color Frame {}:    Timestamp: {}",
                        i,
                        frame.get_timestamp().unwrap_or(0), 
                    );

                    false
                };

                if should_break_loop {
                    break;
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
