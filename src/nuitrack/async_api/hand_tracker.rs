use tracing::warn;
use crate::nuitrack_bridge::modules::hand_tracker::ffi as ht_ffi;
use crate::nuitrack::shared_types::hand_frame::HandFrame;


generate_async_tracker! {
    base_module_name_snake: hand_tracker,
    module_ffi_path: crate::nuitrack_bridge::modules::hand_tracker::ffi,
    streams: [
        {
            item_base_name_snake: hand_frame,
            rust_item_type: crate::nuitrack::shared_types::hand_frame::HandFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::hand_data::ffi::HandData>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::hand_data::ffi::HandData>| crate::nuitrack::shared_types::hand_frame::HandFrame::new(data_arg.clone()),
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
    #[instrument(skip(self))]
    pub async fn latest_hand_frame_sync(&self) -> NuitrackResult<HandFrame> {
        let tracker_ptr = self.get_ffi_ptr_clone(); // Clone the SharedPtr for the blocking task
        let ffi_hand_data_ptr = trace_span!("ffi", function="ht_ffi::data").in_scope(|| {
            run_blocking(move || {
                ht_ffi::data(&tracker_ptr) // Call the FFI get_data function
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand data synchronously: {}", e)))
            })
        }).await?;  

        // Convert the FFI HandData pointer to the public Rust HandFrame type
        HandFrame::new(ffi_hand_data_ptr)
            .ok_or_else(|| {
                warn!("FFI call for latest hand frame returned a null pointer.");
                NuitrackError::OperationFailed("Received null HandData from get_data_sync FFI call.".to_string())
        })
    }

    /// Gets the last hand data processing time in milliseconds.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the processing time as an `f32`, or an error.
    #[instrument(skip(self))]
    pub async fn processing_time(&self) -> NuitrackResult<f32> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="ht_ffi::processing_time").in_scope(|| {
            run_blocking(move || {
                ht_ffi::processing_time(&tracker_ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand processing time: {}", e)))
            })
        }).await
    }

    /// Gets the timestamp of the last processed data by the hand tracker in microseconds.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the timestamp as a `u64`, or an error.
    #[instrument(skip(self))]
    pub async fn tracker_timestamp(&self) -> NuitrackResult<u64> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="ht_ffi::tracker_timestamp").in_scope(|| {
            run_blocking(move || {
                ht_ffi::tracker_timestamp(&tracker_ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get hand tracker timestamp: {}", e)))
            })
        }).await
    }

    /// Checks if the Nuitrack Hand Tracker module can update.
    /// This method blocks the current thread until the FFI call completes.
    ///
    /// # Returns
    /// A `NuitrackResult` containing `true` if the module can update, `false` otherwise, or an error.
    #[instrument(skip(self))] 
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let tracker_ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="ht_ffi::can_update").in_scope(|| {
            run_blocking(move || {
            // Assuming the typo `SharedTracker` in hand_tracker.rs FFI was corrected to `SharedPtr<HandTracker>`
                ht_ffi::can_update(&tracker_ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check hand tracker can_update status: {}", e)))
            })
        }).await
    }
}
