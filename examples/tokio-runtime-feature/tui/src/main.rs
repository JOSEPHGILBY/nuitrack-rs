use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures_util::StreamExt;
use nuitrack_rs::{
    nuitrack::shared_types::{
        hand_frame::HandFrame, rgb_frame::Color3, skeleton_frame::SkeletonFrame,
    },
    setup_nuitrack_streams,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use tokio::{sync::{mpsc, watch}};
use image::{DynamicImage, ImageBuffer, Rgb};
use std::{io::stdout, sync::{Arc, Mutex}, time::Duration};
use tracing::{debug, info, warn, Level};
use fast_image_resize as fr;

pub const GRADIENT: &str = r#" ░▒▓█"#;

/// AppState holds all data needed for rendering.
struct AppState {
    latest_skeleton: Option<SkeletonFrame>,
    latest_hands: Option<HandFrame>,
    ascii_art: (String, Vec<u8>),
    frame_dims: (u32, u32),
}

impl AppState {
    fn new() -> Self {
        Self {
            latest_skeleton: None,
            latest_hands: None,
            ascii_art: ("Waiting for camera feed...".to_string(), Vec::new()),
            frame_dims: (0,0),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup
    let file_appender = tracing_appender::rolling::daily("examples/tokio-runtime-feature/tui", "tui-debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG) // Use DEBUG to see our new timing logs
        .with_writer(non_blocking_appender)
        .init();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;


    // 3. Initialize Nuitrack
    let (mut hand_stream, mut skeleton_stream, mut color_stream, session) =
        setup_nuitrack_streams!(HandTracker, SkeletonTracker, ColorSensor).await?;
    session.start_processing().await?;
    info!("Nuitrack session started.");

    // 4. Setup App State and Processing Channels
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let mut app_running = true;
    let (image_tx, mut image_rx) = mpsc::channel::<(DynamicImage, Rect)>(2);
    let (ascii_tx, mut ascii_rx) = watch::channel(app_state.lock().unwrap().ascii_art.clone());
    let char_map: Vec<char> = GRADIENT.chars().collect();

    tokio::spawn(async move {
        while let Some((image, area)) = image_rx.recv().await {
            let start = tokio::time::Instant::now();
            let src_image = fr::images::Image::from_vec_u8(
                image.width(),
                image.height(),
                image.to_owned().into_rgb8().to_vec(),
                fr::PixelType::U8x3
            ).unwrap();
            let mut dst_image = fr::images::Image::new(
                area.width.into(), 
                area.height.into(),
                fr::PixelType::U8x3
            );

            fr::Resizer::new()
                .resize(
                    &src_image, 
                    &mut dst_image, 
                    &fr::ResizeOptions::new().resize_alg(fr::ResizeAlg::Nearest)
                ).unwrap();

            let dst_image = dst_image.into_vec();
            let img_buff = image::ImageBuffer::<image::Rgb<u8>, _>::from_vec(
                area.width.into(),
                area.height.into(),
                dst_image
            ).unwrap();
            let processed_image = DynamicImage::ImageRgb8(img_buff);
            let gray_image = processed_image.clone().into_luma8();
            let rgb_info = processed_image.into_rgb8().to_vec();

            let (width, height) = (gray_image.width(), gray_image.height());
            let capacity = (width + 1) * height + 1;
            let mut ascii = String::with_capacity(capacity as usize);

            let char_map_len = char_map.len();
            for y in 0..height {
                ascii.extend((0..width).map(|x| {
                    let lum = gray_image.get_pixel(x, y)[0] as u32;
                    let lookup_idx = char_map_len * lum as usize / (u8::MAX as usize + 1);
                    char_map[lookup_idx]
                }));
            }


            debug!("Image processing task took {:?}", start.elapsed());
            if ascii_tx.send((ascii, rgb_info)).is_err() {
                info!("Main loop ascii_rx channel closed.");
                break; // Exit task if the receiver is gone
            }
        }
    });

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // Eagerly pull skeleton data and update state
                Some(Ok(skeletons)) = skeleton_stream.next() => {
                    app_state_clone.lock().unwrap().latest_skeleton = Some(skeletons);
                }
                // Eagerly pull hand data and update state
                Some(Ok(hands)) = hand_stream.next() => {
                    app_state_clone.lock().unwrap().latest_hands = Some(hands);
                }
                // Eagerly pull color data
                Some(Ok(rgb)) = color_stream.next() => {
                    let area = app_state_clone.lock().unwrap().frame_dims;
                    if area.0 > 0 && area.1 > 0 {
                        if let (Ok(cols), Ok(rows), Ok(data)) = (rgb.cols(), rgb.rows(), rgb.data()) {
                            let mut rgb_bytes = Vec::with_capacity(data.len() * 3);
                            for pixel in data {
                                rgb_bytes.push(pixel.red);
                                rgb_bytes.push(pixel.green);
                                rgb_bytes.push(pixel.blue);
                            }
                            if let Some(img_buf) = ImageBuffer::<Rgb<u8>, _>::from_raw(cols as u32, rows as u32, rgb_bytes) {
                                let dynamic_image = DynamicImage::ImageRgb8(img_buf);
                                let rec = Rect::new(0, 0, area.0 as u16, area.1 as u16);
                                // Try to send to the processor. If the processor is busy,
                                // this fails immediately and we drop the frame, ready to
                                // process the next one. THIS is the eager dropping logic.
                                match image_tx.try_send((dynamic_image, rec)) {
                                    Ok(_) => {}, // Sent successfully
                                    Err(mpsc::error::TrySendError::Full(_)) => {
                                        debug!("Image processor is busy; dropping raw Nuitrack frame.");
                                    }
                                    Err(mpsc::error::TrySendError::Closed(_)) => {
                                        info!("Image processor channel closed.");
                                        break; // Exit if the receiver is gone
                                    }
                                }
                            }
                        }
                    }
                }
                else => {
                    // All streams have closed
                    break;
                }
            }
        }
    });

    let mut app_running = true;
    while app_running {
        // --- Drawing ---
        {
            let mut state = app_state.lock().unwrap();
            let start = std::time::Instant::now();
            terminal.draw(|frame| ui(frame, &mut state))?;
            debug!("UI draw call took {:?}", start.elapsed());
        }

        // --- Input Handling ---
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    app_running = false;
                }
            }
        }

        // --- State Update ---
        // Check if the watch channel has new ASCII art. This is non-blocking.
        if ascii_rx.has_changed()? {
            let latest_art = ascii_rx.borrow_and_update().clone();
            app_state.lock().unwrap().ascii_art = latest_art;
        }

        // --- Tick Rate ---
        // Sleep to maintain a consistent frame rate and prevent 100% CPU usage.
        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

    // 6. Cleanup
    info!("Closing Nuitrack session...");
    session.close().await?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame, state: &mut AppState) {
    // Define the layout areas
    let main_layout = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)]);
    let main_chunks = main_layout.split(frame.area());
    let rgb_area = main_chunks[0];
    let side_area = main_chunks[1];

    let side_layout = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let side_chunks = side_layout.split(side_area);
    let skeleton_area = side_chunks[0];
    let hands_area = side_chunks[1];

    // --- ASCII Art Rendering ---

    // 1. Update the app state with the current viewport dimensions for the image processor.
    // This ensures the next frame is resized to the correct size.
    state.frame_dims = (rgb_area.width as u32, rgb_area.height as u32);

    let (ascii_str, color_data) = &state.ascii_art;
    let (width, height) = state.frame_dims;

    let art_block = Block::default().title("ASCII Camera Feed").borders(Borders::ALL);
    let inner_area = art_block.inner(rgb_area);

    // 2. Check if we have valid data to render.
    if !color_data.is_empty() && width > 0 && height > 0 && !ascii_str.starts_with("Waiting") {
        let chars: Vec<char> = ascii_str.chars().collect();
        let mut lines = Vec::with_capacity(height as usize);

        // 3. Reconstruct the image from the flat character and color vectors.
        for y in 0..height {
            // Ensure we don't read past the end of the line
            if y >= inner_area.height as u32 { break; }

            let mut spans = Vec::with_capacity(width as usize);
            let line_start_idx = (y * width) as usize;

            for x in 0..width {
                 // Ensure we don't read past the end of the line
                if x >= inner_area.width as u32 { break; }

                let char_idx = line_start_idx + x as usize;
                let color_idx = char_idx * 3;

                // Bounds check to prevent panics if data is malformed
                if char_idx < chars.len() && (color_idx + 2) < color_data.len() {
                    let char = chars[char_idx];
                    let r = color_data[color_idx];
                    let g = color_data[color_idx + 1];
                    let b = color_data[color_idx + 2];

                    // 4. Create a styled Span for each character with its unique color.
                    let style = Style::default().fg(Color::Rgb(r, g, b));
                    spans.push(Span::styled(char.to_string(), style));
                }
            }
            lines.push(Line::from(spans));
        }

        // 5. Render the collection of styled lines as a Paragraph.
        let art_widget = Paragraph::new(Text::from(lines));
        frame.render_widget(art_widget, inner_area);

    } else {
        // Display a placeholder message if no data is available.
        let placeholder = Paragraph::new(state.ascii_art.0.as_str()).alignment(Alignment::Center);
        frame.render_widget(placeholder, inner_area);
    }
    frame.render_widget(art_block, rgb_area);

    let skeleton_text = if let Some(skeleton_data) = &state.latest_skeleton {
        let skeletons = skeleton_data.skeletons().unwrap_or_default();
        if skeletons.is_empty() {
            "No skeletons detected.".to_string()
        } else {
            skeletons.iter().map(|s| {
                // Find the Head joint for its real-world position
                let head = s.joints.iter().find(|j| j.joint_type == nuitrack_rs::nuitrack::shared_types::skeleton::JointType::Head);
                
                // Find the Right Hand joint for its projected position
                let right_hand = s.joints.iter().find(|j| j.joint_type == nuitrack_rs::nuitrack::shared_types::skeleton::JointType::RightHand);

                let head_info = format!("Head @ ({:.0}, {:.0}, {:.0})",
                    head.map_or(0.0, |j| j.real.x),
                    head.map_or(0.0, |j| j.real.y),
                    head.map_or(0.0, |j| j.real.z)
                );

                let right_hand_proj = right_hand.map_or_else(
                    || "---".to_string(),
                    |j| format!("({:.2}, {:.2})", j.proj.x, j.proj.y)
                );

                format!("User {}: {}\n  └ R.Hand Proj: {}", s.user_id, head_info, right_hand_proj)
            }).collect::<Vec<_>>().join("\n\n")
        }
    } else { "Waiting for skeleton data...".to_string() };
    frame.render_widget(Paragraph::new(skeleton_text).block(Block::default().title("Skeletons").borders(Borders::ALL)), skeleton_area);
    
    let hands_text = if let Some(hands_data) = &state.latest_hands {
        let users_hands = hands_data.users_hands().unwrap_or_default();

        // A small closure to format a single hand neatly.
        let format_hand = |hand: &nuitrack_rs::nuitrack::shared_types::hand::Hand| -> String {
            format!(
                "    Proj: ({:.2}, {:.2})\n    Real: ({:.0}, {:.0}, {:.0}) mm\n    Click: {} (Pressure: {})",
                hand.x, hand.y,
                hand.x_real, hand.y_real, hand.z_real,
                hand.click, hand.pressure
            )
        };

        if users_hands.is_empty() {
            "No users detected.".to_string()
        } else {
            users_hands.iter().map(|h| {
                let user_id = format!("User {}:", h.user_id);

                let left_details = h.left_hand.as_ref()
                    .map(|hand| format!("  Left Hand:\n{}", format_hand(hand)))
                    .unwrap_or_else(|| "  Left Hand: ---".to_string());

                let right_details = h.right_hand.as_ref()
                    .map(|hand| format!("  Right Hand:\n{}", format_hand(hand)))
                    .unwrap_or_else(|| "  Right Hand: ---".to_string());

                format!("{}\n{}\n{}", user_id, left_details, right_details)
            }).collect::<Vec<_>>().join("\n\n") // Add blank line between users
        }
    } else {
        "Waiting for hand data...".to_string()
    };
    frame.render_widget(Paragraph::new(hands_text).block(Block::default().title("Hands").borders(Borders::ALL)), hands_area);
}