//! Provides an asynchronous interface to Nuitrack's Hand Tracker module.

// Imports for the macro and the public API
use crate::nuitrack_bridge::modules::hand_tracker::ffi as ht_ffi;
use crate::nuitrack_bridge::types::hand_data::ffi as hand_data_ffi; // For FFI HandData type
use crate::nuitrack::shared_types::hand_frame::HandFrame; // Public Rust HandFrame type

// The generate_async_tracker! macro definition is assumed to be in scope
// (e.g., from File 1: /src/nuitrack/async_api/generate_tracker.rs)

generate_async_tracker! {
    tracker_name: AsyncHandTracker,
    ffi_tracker_type: crate::nuitrack_bridge::modules::hand_tracker::ffi::HandTracker,
    c_void_type: crate::nuitrack_bridge::modules::hand_tracker::ffi::c_void,
    ffi_create_function: crate::nuitrack_bridge::modules::hand_tracker::ffi::create_hand_tracker,
    module_creation_error_context: "HandTracker FFI create",

    streams: [
        { // Stream 1: Hand Frames
            stream_struct_name: HandFrameStream,
            stream_method_name: hand_frames_stream, // Public method to get this stream
            sender_type_alias: HandFrameMpscSender,  // Internal MPSC sender type alias
            handler_id_field: on_update_handler_id,  // Field in AsyncHandTracker for this callback's ID
            raw_sender_field: raw_hand_frame_sender, // Field for the raw pointer to the sender
            
            // The public Rust type that the stream will yield
            rust_item_type: crate::nuitrack::shared_types::hand_frame::HandFrame,
            
            // FFI functions for connecting/disconnecting the Nuitrack callback
            ffi_connect_stream_fn: crate::nuitrack_bridge::modules::hand_tracker::ffi::connect_on_update_for_async,
            ffi_disconnect_stream_fn: crate::nuitrack_bridge::modules::hand_tracker::ffi::disconnect_on_update,
            
            // The extern "C" Rust function that C++ will call
            // This is declared in /include/nuitrack_bridge/modules/hand_tracker.h
            // The macro will define this Rust function.
            dispatcher_name: rust_hand_tracker_on_update_dispatcher,
            
            dispatcher_kind: { FfiDataConversion {
                // Name of the FFI data argument in the dispatcher function
                ffi_arg_name: data,
                // Type of the FFI data argument
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::hand_data::ffi::HandData>,
                // Name of the user_data (sender pointer) argument
                user_data_arg_name: sender_ptr, 
                // Closure to convert FFI data to the public Rust item type
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::hand_data::ffi::HandData>| crate::nuitrack::shared_types::hand_frame::HandFrame::new(data_arg.clone()),
                // Error message if conversion_logic returns None
                conversion_error_msg: "FFI HandData was null or invalid for HandFrameStream",
            }}
        }
    ]
}

// Manual implementation block for additional synchronous FFI methods on AsyncHandTracker
impl AsyncHandTracker {
    /// Gets the last available hand data synchronously from the tracker.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing a `HandFrame` if successful, or an error.
    pub async fn get_data_sync(&self) -> NuitrackResult<HandFrame> {
        let tracker_ptr = self.get_ffi_ptr_clone(); // Clone the SharedPtr for the blocking task
        let ffi_hand_data_ptr = run_blocking(move || {
            ht_ffi::get_data(&tracker_ptr) // Call the FFI get_data function
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand data synchronously: {}", e)))
        }).await?;

        // Convert the FFI HandData pointer to the public Rust HandFrame type
        HandFrame::new(ffi_hand_data_ptr)
            .ok_or_else(|| NuitrackError::OperationFailed(
                "Received null HandData from get_data_sync FFI call.".to_string()
            ))
    }

    /// Gets the last hand data processing time in milliseconds.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the processing time as an `f32`, or an error.
    pub async fn get_processing_time(&self) -> NuitrackResult<f32> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            ht_ffi::get_processing_time(&tracker_ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand processing time: {}", e)))
        }).await
    }

    /// Gets the timestamp of the last processed data by the hand tracker in microseconds.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the timestamp as a `u64`, or an error.
    pub async fn get_tracker_timestamp(&self) -> NuitrackResult<u64> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            ht_ffi::get_tracker_timestamp(&tracker_ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand tracker timestamp: {}", e)))
        }).await
    }

    /// Checks if the Nuitrack Hand Tracker module can update.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing `true` if the module can update, `false` otherwise, or an error.
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            // Assuming the typo `SharedTracker` in hand_tracker.rs FFI was corrected to `SharedPtr<HandTracker>`
            ht_ffi::can_update(&tracker_ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check hand tracker can_update status: {}", e)))
        }).await
        
    }
}
