use tracing::warn;
use crate::nuitrack_bridge::modules::skeleton_tracker::ffi as st_ffi;
use crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame;

generate_async_tracker! {
    base_module_name_snake: skeleton_tracker,
    module_ffi_path: crate::nuitrack_bridge::modules::skeleton_tracker::ffi,
    streams: [
        {
            item_base_name_snake: skeleton_frame,
            rust_item_type: crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame,
            dispatcher_kind: { FFIDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::skeleton_data::ffi::SkeletonData>,
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::skeleton_data::ffi::SkeletonData>| crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame::new(data_arg.clone()),
            }}
        },
        {
            item_base_name_snake: new_user_event,
            rust_item_type: i32, // UserID
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id, // Name of the argument in the C-style dispatcher
                ffi_item_arg_type: i32,     // Type of that argument
            }}
        },
        {
            item_base_name_snake: lost_user_event,
            rust_item_type: i32, // UserID
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id,
                ffi_item_arg_type: i32,
            }}
        }
    ]
}

impl AsyncSkeletonTracker {
    /// Sets the maximum number of users for tracking.
    /// Supports tracking from 0 to 6 users. By default, 2 users are tracked.
    /// Tracking >2 users may impact performance.
    #[instrument(skip(self))]
    pub async fn set_num_active_users(&self, num_users: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::set_num_active_users").in_scope(|| {
            run_blocking(move || {
                st_ffi::set_num_active_users(&ptr, num_users)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to set num active users: {}", e)))
            })
        }).await
    }

    /// Checks if auto-tracking of skeletons is enabled.
    #[instrument(skip(self))]
    pub async fn is_auto_tracking(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::is_auto_tracking").in_scope(|| {
            run_blocking(move || {
                st_ffi::is_auto_tracking(&ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check auto tracking status: {}", e)))
            })
        }).await
    }

    /// Enables or disables automatic skeleton tracking.
    /// If true, tracking starts when a user appears. Otherwise, manual start is needed.
    #[instrument(skip(self))]
    pub async fn set_auto_tracking(&self, tracking: bool) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::set_auto_tracking").in_scope(|| {
            run_blocking(move || {
                st_ffi::set_auto_tracking(&ptr, tracking)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to set auto tracking: {}", e)))
            })
        }).await
    }

    /// Starts tracking the skeleton of a specific user.
    #[instrument(skip(self))]
    pub async fn start_tracking(&self, user_id: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::start_tracking").in_scope(|| {
            run_blocking(move || {
                st_ffi::start_tracking(&ptr, user_id)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to start tracking user {}: {}", user_id, e)))
            })
        }).await
        
    }

    /// Stops tracking the skeleton of a specific user.
    #[instrument(skip(self))]
    pub async fn stop_tracking(&self, user_id: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::stop_tracking").in_scope(|| {
            run_blocking(move || {
                st_ffi::stop_tracking(&ptr, user_id)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to stop tracking user {}: {}", user_id, e)))
            })
        }).await
    }

    /// Checks if a specific user's skeleton is currently being tracked.
    #[instrument(skip(self))]
    pub async fn is_tracking(&self, user_id: i32) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::is_tracking").in_scope(|| {
            run_blocking(move || {
                st_ffi::is_tracking(&ptr, user_id)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check tracking status for user {}: {}", user_id, e)))
            })
        }).await
    }
    
    /// Gets the last available skeleton data synchronously from the tracker.
    #[instrument(skip(self))] 
    pub async fn latest_skeletons_frame_sync(&self) -> NuitrackResult<SkeletonFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_skeleton_data_ptr = trace_span!("ffi", function="st_ffi::skeletons").in_scope(|| {
            run_blocking(move || {
                // This calls the getSkeletons method on the SkeletonTracker FFI,
                // which returns a SharedPtr<SkeletonData>
                st_ffi::skeletons(&ptr) 
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get skeletons synchronously: {}", e)))
            })
        }).await?;

        SkeletonFrame::new(ffi_skeleton_data_ptr) // SkeletonFrame::new expects SharedPtr<FFISkeletonData>
            .ok_or_else(|| {
                warn!("FFI call for latest skeletons frame returned a null pointer.");
                NuitrackError::OperationFailed("Received null SkeletonData from get_skeletons_sync".to_string())
        })
    }

    /// Gets the last skeleton data processing time in milliseconds.
    #[instrument(skip(self))]
    pub async fn processing_time(&self) -> NuitrackResult<f32> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::processing_time").in_scope(|| {
            run_blocking(move || {
                st_ffi::processing_time(&ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get processing time: {}", e)))
            })
        }).await
        
    }
    
    /// Gets the timestamp of the last processed data by the tracker in microseconds.
    #[instrument(skip(self))]
    pub async fn tracker_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::tracker_timestamp").in_scope(|| {
            run_blocking(move || {
                st_ffi::tracker_timestamp(&ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get tracker timestamp: {}", e)))
            })
        }).await
    }

    /// Checks if the Nuitrack module can update.
    #[instrument(skip(self))]
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        trace_span!("ffi", function="st_ffi::can_update").in_scope(|| {
            run_blocking(move || {
                st_ffi::can_update(&ptr)
                .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
            })
        }).await
    }
}
