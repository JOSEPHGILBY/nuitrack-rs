
use crate::nuitrack_bridge::modules::skeleton_tracker::ffi as st_ffi;
use crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame;

generate_async_tracker! {
    tracker_name: AsyncSkeletonTracker,
    ffi_tracker_type: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonTracker,
    c_void_type: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::c_void,
    ffi_create_function: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::create_skeleton_tracker,
    module_creation_error_context: "SkeletonTracker FFI create",

    streams: [
        { // Stream 1: Skeleton Frames
            stream_struct_name: SkeletonFrameStream,
            stream_method_name: skeleton_frames_stream,
            sender_type_alias: SkeletonFrameMpscSender, // Renamed for clarity from just SkeletonFrameSender
            handler_id_field: on_update_handler_id,
            raw_sender_field: raw_skeleton_frame_sender,
            rust_item_type: crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame,
            ffi_connect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::connect_on_update_for_async,
            ffi_disconnect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::disconnect_on_update,
            dispatcher_name: rust_skeleton_tracker_callback_which_sends_for_async,
            dispatcher_kind: { FfiDataConversion {
                ffi_arg_name: data,
                ffi_arg_type: cxx::SharedPtr<crate::nuitrack_bridge::types::skeleton_data::ffi::SkeletonData>,
                user_data_arg_name: sender_ptr, // Changed name for clarity in dispatcher
                conversion_logic: |data_arg: &cxx::SharedPtr<crate::nuitrack_bridge::types::skeleton_data::ffi::SkeletonData>| crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame::new(data_arg.clone()),
                conversion_error_msg: "FFI SkeletonData was null or invalid",
            }}
        },
        { // Stream 2: New User Events
            stream_struct_name: NewUserEventStream,
            stream_method_name: new_user_events_stream,
            sender_type_alias: NewUserEventMpscSender,
            handler_id_field: on_new_user_handler_id,
            raw_sender_field: raw_new_user_event_sender,
            rust_item_type: i32, // UserID
            ffi_connect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::connect_on_new_user_for_async,
            ffi_disconnect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::disconnect_on_new_user,
            dispatcher_name: rust_skeleton_tracker_new_user_dispatcher,
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id, // Name of the argument in the C-style dispatcher
                ffi_item_arg_type: i32,     // Type of that argument
                user_data_arg_name: sender_ptr, // Name for the user_data void* pointer
            }}
        },
        { // Stream 3: Lost User Events
            stream_struct_name: LostUserEventStream,
            stream_method_name: lost_user_events_stream,
            sender_type_alias: LostUserEventMpscSender,
            handler_id_field: on_lost_user_handler_id,
            raw_sender_field: raw_lost_user_event_sender,
            rust_item_type: i32, // UserID
            ffi_connect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::connect_on_lost_user_for_async,
            ffi_disconnect_stream_fn: crate::nuitrack_bridge::modules::skeleton_tracker::ffi::disconnect_on_lost_user,
            dispatcher_name: rust_skeleton_tracker_lost_user_dispatcher,
            dispatcher_kind: { DirectItem {
                ffi_item_arg_name: user_id,
                ffi_item_arg_type: i32,
                user_data_arg_name: sender_ptr,
            }}
        }
    ]
}

impl AsyncSkeletonTracker {
    /// Sets the maximum number of users for tracking.
    /// Supports tracking from 0 to 6 users. By default, 2 users are tracked.
    /// Tracking >2 users may impact performance.
    pub async fn set_num_active_users(&self, num_users: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::set_num_active_users(&ptr, num_users)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to set num active users: {}", e)))
        }).await
    }

    /// Checks if auto-tracking of skeletons is enabled.
    pub async fn is_auto_tracking(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::is_auto_tracking(&ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check auto tracking status: {}", e)))
        }).await
    }

    /// Enables or disables automatic skeleton tracking.
    /// If true, tracking starts when a user appears. Otherwise, manual start is needed.
    pub async fn set_auto_tracking(&self, tracking: bool) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::set_auto_tracking(&ptr, tracking)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to set auto tracking: {}", e)))
            
        }).await
    }

    /// Starts tracking the skeleton of a specific user.
    pub async fn start_tracking(&self, user_id: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::start_tracking(&ptr, user_id)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to start tracking user {}: {}", user_id, e)))
        }).await
        
    }

    /// Stops tracking the skeleton of a specific user.
    pub async fn stop_tracking(&self, user_id: i32) -> NuitrackResult<()> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::stop_tracking(&ptr, user_id)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to stop tracking user {}: {}", user_id, e)))
        }).await
    }

    /// Checks if a specific user's skeleton is currently being tracked.
    pub async fn is_tracking(&self, user_id: i32) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::is_tracking(&ptr, user_id)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check tracking status for user {}: {}", user_id, e)))
        }).await
    }
    
    /// Gets the last available skeleton data synchronously from the tracker.
    pub async fn get_skeletons_sync(&self) -> NuitrackResult<SkeletonFrame> {
        let ptr = self.get_ffi_ptr_clone();
        let ffi_skeleton_data_ptr = run_blocking(move || {
            // This calls the getSkeletons method on the SkeletonTracker FFI,
            // which returns a SharedPtr<SkeletonData>
            st_ffi::get_skeletons(&ptr) 
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get skeletons synchronously: {}", e)))
        }).await?;

        SkeletonFrame::new(ffi_skeleton_data_ptr) // SkeletonFrame::new expects SharedPtr<FFISkeletonData>
            .ok_or_else(|| NuitrackError::OperationFailed("Received null SkeletonData from get_skeletons_sync".to_string()))
    }

    /// Gets the last skeleton data processing time in milliseconds.
    pub async fn get_processing_time(&self) -> NuitrackResult<f32> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::get_processing_time(&ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get processing time: {}", e)))
        }).await
        
    }
    /// Gets the timestamp of the last processed data by the tracker in microseconds.
    pub async fn get_tracker_timestamp(&self) -> NuitrackResult<u64> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::get_tracker_timestamp(&ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to get tracker timestamp: {}", e)))
        }).await
    }

    /// Checks if the Nuitrack module can update.
    pub async fn can_update(&self) -> NuitrackResult<bool> {
        let ptr = self.get_ffi_ptr_clone();
        run_blocking(move || {
            st_ffi::can_update(&ptr)
            .map_err(|e| NuitrackError::OperationFailed(format!("Failed to check can_update status: {}", e)))
        }).await
    }
}
