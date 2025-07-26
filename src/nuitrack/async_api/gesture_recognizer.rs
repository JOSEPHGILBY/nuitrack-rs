use tracing::warn;
use crate::nuitrack_bridge::modules::gesture_recognizer::ffi as gr_ffi;
use crate::nuitrack::shared_types::{
    gesture_frame::{GestureFrame,
    UserGesturesFrame,
    UserStateFrame},
};

// Use the procedural macro to generate the async tracker boilerplate.
// This defines the AsyncGestureRecognizer struct, its `new` function, and the stream
// management logic for all three gesture-related streams.
generate_async_tracker! {
    base_module_name_snake: gesture_recognizer,
    module_ffi_path: crate::nuitrack_bridge::modules::gesture_recognizer::ffi,
    streams: [
        {
            // Stream for newly completed gestures.
            item_base_name_snake: completed_gestures_frame,
            rust_item_type: GestureFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::GestureData>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::GestureData>| GestureFrame::new(data_arg.clone()),
            }}
        },
        {
            // Stream for users changing state (e.g., appearing, becoming active).
            item_base_name_snake: user_state_change,
            rust_item_type: UserStateFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::UserStateData>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::UserStateData>| UserStateFrame::new(data_arg.clone()),
            }}
        },
        {
            // Stream for gesture progress updates for all users.
            // NOTE: The base name 'update' comes from the FFI function `connect_on_update_async`.
            item_base_name_snake: update,
            rust_item_type: UserGesturesFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::UserGesturesStateData>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::gesture_data::ffi::UserGesturesStateData>| UserGesturesFrame::new(data_arg.clone()),
            }}
        }
    ]
}

/// Provides an asynchronous, high-level interface to Nuitrack's `GestureRecognizer` module.
impl AsyncGestureRecognizer {
    /// Enables or disables the recognition of control gestures (e.g., 'Push', 'Swipe').
    #[instrument(skip(self))]
    pub async fn set_control_gestures_status(&self, status: bool) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="gr_ffi::set_control_gestures_status").in_scope(|| {
            run_blocking(move || {
                gr_ffi::set_control_gestures_status(&ptr, status)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to set control gesture status: {}", e)))
            })
        }).await
    }

    /// Gets the last gesture recognition time in milliseconds.
    #[instrument(skip(self))]
    pub async fn processing_time(&self) -> NuitrackResult<f32> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="gr_ffi::processing_time").in_scope(|| {
            run_blocking(move || {
                gr_ffi::processing_time(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get processing time: {}", e)))
            })
        }).await
    }
    
    /// Gets the timestamp of the last processed data by the recognizer in microseconds.
    #[instrument(skip(self))]
    pub async fn recognizer_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="gr_ffi::recognizer_timestamp").in_scope(|| {
            run_blocking(move || {
                gr_ffi::recognizer_timestamp(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get recognizer timestamp: {}", e)))
            })
        }).await
    }

    /// Checks if the Nuitrack module can update.
    #[instrument(skip(self))]
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="gr_ffi::can_update").in_scope(|| {
            run_blocking(move || {
                gr_ffi::can_update(&ptr)
                    .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
            })
        }).await
    }
}