#[macro_export]
macro_rules! setup_nuitrack_streams {
    ( $( $module_token:ident ),+ $(,)? ) => { // Match one or more tokens, with optional trailing comma
        async {
            // Use fully qualified paths for types from the crate where this macro is defined.
            // If this macro is in `nuitrack_rs::nuitrack::macros`, $crate would be `nuitrack_rs`.
            // Adjust these paths based on where NuitrackSessionBuilder, ModuleType, etc., actually live
            // relative to where this macro is exported from.
            use $crate::nuitrack::{
                async_api::{session_builder::NuitrackSessionBuilder,
                session::NuitrackSession}, // For the return type hint
                shared_types::session_config::ModuleType, // Assuming this is the correct path
                shared_types::error::{NuitrackError, Result as NuitrackResult},
            };

            // 1. Create the Vec<ModuleType> for session initialization
            let modules_to_create = vec![
                $( ModuleType::$module_token ),*
            ];

            // 2. Create the NuitrackSession
            // NuitrackSessionBuilder::create_session_from_single_default_device already returns NuitrackResult
            let mut session: NuitrackSession = NuitrackSessionBuilder::create_session_from_single_default_device(
                modules_to_create
            )
            .await?; // Use `?` directly as it returns NuitrackResult

            // 3. Helper macro to extract a specific stream
            macro_rules! get_stream_for_module {
                ($session_ref:expr, HandTracker) => {
                    {
                        let tracker = $session_ref.active_devices
                            .get_mut(0)
                            .and_then(|device_context| device_context.hand_tracker.as_mut())
                            .ok_or_else(|| NuitrackError::ModuleCreationFailed("AsyncHandTracker not found for the configured device.".to_string()))?;
                        tracker.hand_frames_stream()
                            .map_err(|e_text| NuitrackError::OperationFailed(format!("Failed to get HandTracker stream: {}", e_text)))
                    }
                };
                ($session_ref:expr, SkeletonTracker) => {
                    {
                        let tracker = $session_ref.active_devices
                            .get_mut(0)
                            .and_then(|device_context| device_context.skeleton_tracker.as_mut())
                            .ok_or_else(|| NuitrackError::ModuleCreationFailed("AsyncSkeletonTracker not found for the configured device.".to_string()))?;
                        tracker.skeleton_frames_stream()
                            .map_err(|e_text| NuitrackError::OperationFailed(format!("Failed to get SkeletonTracker stream: {}", e_text)))
                    }
                };
                ($session_ref:expr, ColorSensor) => {
                    {
                        let tracker = $session_ref.active_devices
                            .get_mut(0)
                            .and_then(|device_context| device_context.color_sensor.as_mut())
                            .ok_or_else(|| NuitrackError::ModuleCreationFailed("AsyncColorSensor not found for the configured device.".to_string()))?;
                        tracker.rgb_frames_stream() // Method name is rgb_frames_stream
                            .map_err(|e_text| NuitrackError::OperationFailed(format!("Failed to get ColorSensor stream: {}", e_text)))
                    }
                };
                // To satisfy type checking for the Result in the compile_error! case,
                // we need a concrete error type. Since the macro is generic over stream types,
                // this fallback error type doesn't perfectly align with the stream's Ok type.
                // However, it won't compile anyway due to compile_error!.
                ($session_ref:expr, $unsupported_token:ident) => { {
                    compile_error!(concat!("Unsupported ModuleType token in setup_nuitrack_streams macro: ", stringify!($unsupported_token)));
                    // Provide a valid expression for type inference that results in NuitrackResult<_, NuitrackError>
                    // This part will not actually be executed due to compile_error!
                    Err(NuitrackError::OperationFailed(format!("Unsupported module: {}", stringify!($unsupported_token))))
                } };
            }

            // 4. Get each stream and package into a Result<Tuple, NuitrackError>
            // The `?` operator is used after each stream retrieval.
            let result: NuitrackResult<_> = Ok((
                $(
                    get_stream_for_module!(&mut session, $module_token)?
                ),*
                ,
                session
            ));
            result
        }
    };
}