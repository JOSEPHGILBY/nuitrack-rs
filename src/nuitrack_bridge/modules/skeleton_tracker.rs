#[cxx::bridge(namespace = "nuitrack_bridge::skeleton_tracker")]
pub mod ffi {

    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type SkeletonTracker;
        type SkeletonData = crate::nuitrack_bridge::types::skeleton_data::ffi::SkeletonData;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/skeleton_tracker.h");

        pub type c_void;

        #[cxx_name = "createSkeletonTracker"]
        pub fn create_skeleton_tracker() -> Result<SharedPtr<SkeletonTracker>>;

        #[cxx_name = "connectOnUpdateForAsync"]
        pub unsafe fn connect_on_update_for_async(
            tracker: &SharedPtr<SkeletonTracker>,
            skeleton_frame_sender: *mut c_void, 
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnUpdate"]
        pub fn disconnect_on_update(
            tracker: &SharedPtr<SkeletonTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- New User Callbacks ---
        #[cxx_name = "connectOnNewUserForAsync"]
        pub unsafe fn connect_on_new_user_for_async(
            tracker: &SharedPtr<SkeletonTracker>,
            new_user_frame_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnNewUser"]
        pub fn disconnect_on_new_user(
            tracker: &SharedPtr<SkeletonTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Lost User Callbacks ---
        #[cxx_name = "connectOnLostUserForAsync"]
        pub unsafe fn connect_on_lost_user_for_async(
            tracker: &SharedPtr<SkeletonTracker>,
            lost_user_frame_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnLostUser"]
        pub fn disconnect_on_lost_user(
            tracker: &SharedPtr<SkeletonTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Configuration & Control ---
        #[cxx_name = "setNumActiveUsers"]
        pub fn set_num_active_users(tracker: &SharedPtr<SkeletonTracker>, num_users: i32) -> Result<()>;

        #[cxx_name = "isAutoTracking"]
        pub fn is_auto_tracking(tracker: &SharedPtr<SkeletonTracker>) -> Result<bool>;

        #[cxx_name = "setAutoTracking"]
        pub fn set_auto_tracking(tracker: &SharedPtr<SkeletonTracker>, tracking: bool) -> Result<()>;

        #[cxx_name = "startTracking"]
        pub fn start_tracking(tracker: &SharedPtr<SkeletonTracker>, user_id: i32) -> Result<()>;

        #[cxx_name = "stopTracking"]
        pub fn stop_tracking(tracker: &SharedPtr<SkeletonTracker>, user_id: i32) -> Result<()>;

        #[cxx_name = "isTracking"]
        pub fn is_tracking(tracker: &SharedPtr<SkeletonTracker>, user_id: i32) -> Result<bool>;
        
        // --- Synchronous Data Access ---
        #[cxx_name = "getSkeletons"]
        pub fn get_skeletons(tracker: &SharedPtr<SkeletonTracker>) -> Result<SharedPtr<SkeletonData>>;

        // --- Module Information ---
        #[cxx_name = "getProcessingTime"]
        pub fn get_processing_time(tracker: &SharedPtr<SkeletonTracker>) -> Result<f32>;

        #[cxx_name = "getTrackerTimestamp"]
        pub fn get_tracker_timestamp(tracker: &SharedPtr<SkeletonTracker>) -> Result<u64>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(tracker: &SharedPtr<SkeletonTracker>) -> Result<bool>;
    }
}

unsafe impl Send for ffi::SkeletonTracker {}
unsafe impl Sync for ffi::SkeletonTracker {}