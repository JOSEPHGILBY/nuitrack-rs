use crate::nuitrack_bridge::modules::color_sensor::ffi as color_sensor_ffi;
use crate::nuitrack_bridge::types::output_mode::ffi::OutputMode as OutputModeFfi;
use crate::nuitrack::shared_types::rgb_frame::RGBFrame;


generate_async_tracker! {
    tracker_name: AsyncColorSensor,
    ffi_tracker_type: crate::nuitrack_bridge::modules::color_sensor::ffi::ColorSensor,
    c_void_type: crate::nuitrack_bridge::modules::color_sensor::ffi::c_void,
    ffi_create_function: crate::nuitrack_bridge::modules::color_sensor::ffi::create_color_sensor,
    module_creation_error_context: "ColorSensor FFI create",

    streams: [
        { // Stream 1: RGB Frames
            stream_struct_name: RgbFrameStream,
            stream_method_name: rgb_frames_stream,
            sender_type_alias: RgbFrameMpscSender,
            handler_id_field: on_new_frame_handler_id,
            raw_sender_field: raw_rgb_frame_sender,
            rust_item_type: crate::nuitrack::shared_types::rgb_frame::RGBFrame,
            ffi_connect_stream_fn: crate::nuitrack_bridge::modules::color_sensor::ffi::connect_on_new_frame_for_async,
            ffi_disconnect_stream_fn: crate::nuitrack_bridge::modules::color_sensor::ffi::disconnect_on_new_frame,
            dispatcher_name: rust_color_sensor_callback_which_sends_for_async, // Matches extern "C" in color_sensor.h
            dispatcher_kind: { FfiDataConversion {
                ffi_arg_name: frame, // Name of the argument in the extern "C" C++ dispatcher signature
                // Type of the argument received by the Rust extern "C" dispatcher from C++
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::rgb_frame::ffi::RGBFrame>,
                user_data_arg_name: sender_ptr,
                // Logic to convert the FFI C++ SharedPtr to the public Rust RgbFrame
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::rgb_frame::ffi::RGBFrame>| {
                    crate::nuitrack::shared_types::rgb_frame::RGBFrame::new(data_arg.clone())
                },
                conversion_error_msg: "FFI RGBFrame was null or invalid",
            }}
        }
    ]
}


impl AsyncColorSensor {
    /// Gets the current output mode of the color sensor.
    ///
    /// The `OutputModeFfi` struct contains information like FPS, resolution, and field of view.
    pub async fn get_output_mode(&self) -> NuitrackResult<OutputModeFfi> {
        let ptr = self.get_ffi_ptr_clone(); // Assuming this method exists from the macro
        run_blocking(move || { // Assuming run_blocking helper
            color_sensor_ffi::output_mode(&ptr) // Assuming correct FFI function name is output_mode
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get output mode: {}", e)))
        }).await
    }

    /// Gets the last available color frame synchronously from the sensor.
    ///
    /// This provides direct access to an `RgbFrame` without using the asynchronous stream.
    pub async fn get_color_frame_sync(&self) -> NuitrackResult<RGBFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_rgb_frame_ptr = run_blocking(move || {
            color_sensor_ffi::color_frame(&ptr) // Assuming correct FFI function name is color_frame
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get color frame synchronously: {}", e)))
        }).await?;

        RGBFrame::new(ffi_rgb_frame_ptr)
            .ok_or_else(|| NuitrackError::OperationFailed("Received null RGBFrame from get_color_frame_sync".to_string()))
    }

    /// Gets the timestamp of the last processed data by the color sensor in microseconds.
    pub async fn get_sensor_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            color_sensor_ffi::timestamp(&ptr) // Assuming correct FFI function name is timestamp
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get sensor timestamp: {}", e)))
        }).await
    }

    /// Checks if the Nuitrack color sensor module can update (i.e., if it's running and providing data).
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            color_sensor_ffi::can_update(&ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
        }).await
    }
}