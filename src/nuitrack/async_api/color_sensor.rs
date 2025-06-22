use tracing::warn;
use crate::nuitrack_bridge::modules::color_sensor::ffi as color_sensor_ffi;
use crate::nuitrack_bridge::types::output_mode::ffi::OutputMode as OutputModeFfi;
use crate::nuitrack::shared_types::rgb_frame::RGBFrame;


generate_async_tracker! {
    base_module_name_snake: color_sensor,
    module_ffi_path: crate::nuitrack_bridge::modules::color_sensor::ffi,
    streams: [
        {
            item_base_name_snake: rgb_frame,
            item_base_name_pascal: RGBFrame,
            rust_item_type: crate::nuitrack::shared_types::rgb_frame::RGBFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: frame,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::rgb_frame::ffi::RGBFrame>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::rgb_frame::ffi::RGBFrame>| {
                    crate::nuitrack::shared_types::rgb_frame::RGBFrame::new(data_arg.clone())
                },
            }}
        }
    ]
}


impl AsyncColorSensor {
    /// Gets the current output mode of the color sensor.
    ///
    /// The `OutputModeFfi` struct contains information like FPS, resolution, and field of view.
    #[instrument(skip(self))]
    pub async fn output_mode(&self) -> NuitrackResult<OutputModeFfi> {
        let ptr = self.get_ffi_ptr_clone(); // Assuming this method exists from the macro
        trace_span!("ffi", function="color_sensor_ffi::output_mode").in_scope(|| {
            run_blocking(move || { // Assuming run_blocking helper
                color_sensor_ffi::output_mode(&ptr) // Assuming correct FFI function name is output_mode
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get output mode: {}", e)))
            })
        }).await
    }

    /// Gets the last available color frame synchronously from the sensor.
    ///
    /// This provides direct access to an `RgbFrame` without using the asynchronous stream.
    #[instrument(skip(self))]
    pub async fn latest_color_frame_sync(&self) -> NuitrackResult<RGBFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_rgb_frame_ptr = trace_span!("ffi", function="color_sensor_ffi::color_frame").in_scope(|| {
            run_blocking(move || {
                color_sensor_ffi::color_frame(&ptr) // Assuming correct FFI function name is color_frame
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get color frame synchronously: {}", e)))
            })
        }).await?;

        RGBFrame::new(ffi_rgb_frame_ptr)
            .ok_or_else(|| {
                warn!("FFI call for latest color frame returned a null pointer.");
                NuitrackError::OperationFailed("Received null RGBFrame from get_color_frame_sync".to_string())
        })
    }

    /// Gets the timestamp of the last processed data by the color sensor in microseconds.
    #[instrument(skip(self))]
    pub async fn sensor_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="color_sensor_ffi::timestamp").in_scope(|| {
            run_blocking(move || {
                color_sensor_ffi::timestamp(&ptr) // Assuming correct FFI function name is timestamp
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get sensor timestamp: {}", e)))
            })
        }).await
    }

    /// Checks if the Nuitrack color sensor module can update (i.e., if it's running and providing data).
    #[instrument(skip(self))] 
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="color_sensor_ffi::can_update").in_scope(|| {
            run_blocking(move || {
                color_sensor_ffi::can_update(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
            })
        }).await
    }
}