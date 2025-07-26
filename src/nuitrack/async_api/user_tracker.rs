use tracing::{warn};
use crate::nuitrack_bridge::modules::user_tracker::ffi as ut_ffi;
use crate::nuitrack::shared_types::user_frame::UserFrame;

// Use the existing procedural macro to generate the async tracker boilerplate.
// This defines the AsyncUserTracker struct, its `new` function, and the stream
// management logic for the specified streams.
generate_async_tracker! {
    base_module_name_snake: user_tracker,
    module_ffi_path: crate::nuitrack_bridge::modules::user_tracker::ffi,
    streams: [
        {
            item_base_name_snake: user_frame,
            rust_item_type: crate::nuitrack::shared_types::user_frame::UserFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::user_frame::ffi::UserFrame>,
                // The conversion logic takes the raw FFI pointer and wraps it in our safe Rust type.
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::user_frame::ffi::UserFrame>| crate::nuitrack::shared_types::user_frame::UserFrame::new(data_arg.clone()),
            }}
        },
        {
            item_base_name_snake: new_user_event,
            rust_item_type: i32, // The event just yields the UserID
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id,
                ffi_item_arg_type: i32,
            }}
        },
        {
            item_base_name_snake: lost_user_event,
            rust_item_type: i32, // The event just yields the UserID
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id,
                ffi_item_arg_type: i32,
            }}
        }
    ]
}

/// Provides an asynchronous, high-level interface to Nuitrack's `UserTracker` module.
impl AsyncUserTracker {
    /// Gets the last available user frame synchronously from the tracker.
    ///
    /// This method blocks the calling thread within a dedicated thread pool
    /// until the data is retrieved from the Nuitrack SDK.
    #[instrument(skip(self))]
    pub async fn latest_user_frame_sync(&self) -> NuitrackResult<UserFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_user_frame_ptr = trace_span!("ffi", function = "ut_ffi::user_frame").in_scope(
            || {
                run_blocking(move || {
                    ut_ffi::user_frame(&ptr)
                        .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get user frame synchronously: {}", e)))
                })
            },
        )
        .await?;

        UserFrame::new(ffi_user_frame_ptr).ok_or_else(|| {
            warn!("FFI call for latest user frame returned a null pointer.");
            NuitrackError::OperationFailed(
                "Received null UserFrame from get_user_frame_sync".to_string(),
            )
        })
    }

    /// Gets the last frame processing time in milliseconds.
    #[instrument(skip(self))]
    pub async fn processing_time(&self) -> NuitrackResult<f32> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "ut_ffi::processing_time").in_scope(|| {
            run_blocking(move || {
                ut_ffi::processing_time(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get processing time: {}", e)))
            })
        })
        .await
    }

    /// Gets the timestamp of the last processed data by the tracker in microseconds.
    #[instrument(skip(self))]
    pub async fn tracker_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "ut_ffi::tracker_timestamp").in_scope(|| {
            run_blocking(move || {
                ut_ffi::tracker_timestamp(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get tracker timestamp: {}", e)))
            })
        })
        .await
    }

    /// Checks if the Nuitrack module can update.
    #[instrument(skip(self))]
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function = "ut_ffi::can_update").in_scope(|| {
            run_blocking(move || {
                ut_ffi::can_update(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
            })
        })
        .await
    }
}