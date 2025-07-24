#[macro_export]
macro_rules! setup_nuitrack_streams {
    ( $( $module_token:ident ),+ $( ; $( $key:expr => $value:expr ),* )? $(,)? ) => { // Match one or more tokens, with optional trailing comma
        async {
            use $crate::nuitrack::{
                async_api::{session_builder::NuitrackSessionBuilder,
                session::NuitrackSession}, // For the return type hint
                shared_types::session_config::{ModuleType, DeviceConfig, DeviceSelector}, // Assuming this is the correct path
                shared_types::error::{NuitrackError, Result as NuitrackResult},
            };

            let mut builder = NuitrackSessionBuilder::new();

            $(
                $(
                    builder = builder.with_config_value($key, $value);
                )*
            )?

            let modules_to_create = vec![
                $( ModuleType::$module_token ),*
            ];

            let device_config = DeviceConfig {
                selector: DeviceSelector::ByIndex(0),
                modules_to_create,
            };

            let mut session: NuitrackSession = builder
                .with_device_config(device_config)
                .init_session()
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
                ($session_ref:expr, DepthSensor) => {
                    {
                        let sensor = $session_ref.active_devices
                            .get_mut(0)
                            .and_then(|device_context| device_context.depth_sensor.as_mut())
                            .ok_or_else(|| NuitrackError::ModuleCreationFailed("AsyncDepthSensor not found for the configured device.".to_string()))?;
                        sensor.depth_frames_stream() // Method name from generate_async_tracker!
                            .map_err(|e_text| NuitrackError::OperationFailed(format!("Failed to get DepthSensor stream: {}", e_text)))
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