#[cxx::bridge(namespace = "nuitrack_bridge::user_tracker")]
pub mod ffi {

    // Opaque C++ types that Rust will interact with via smart pointers.
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type UserTracker;
        type UserFrame = crate::nuitrack_bridge::types::user_frame::ffi::UserFrame;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/user_tracker.h");

        // We need a C++ type for the void* pointer.
        // This is aliased in `user_tracker.h` but must be declared here for cxx.
        pub type c_void;

        // --- Module Lifecycle ---
        #[cxx_name = "createUserTracker"]
        pub fn create_user_tracker() -> Result<SharedPtr<UserTracker>>;

        // --- On-Update Callbacks ---
        #[cxx_name = "connectOnUpdateForAsync"]
        pub unsafe fn connect_on_user_frame_async(
            tracker: &SharedPtr<UserTracker>,
            user_frame_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnUpdate"]
        pub fn disconnect_on_user_frame(
            tracker: &SharedPtr<UserTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- New User Callbacks ---
        #[cxx_name = "connectOnNewUserForAsync"]
        pub unsafe fn connect_on_new_user_event_async(
            tracker: &SharedPtr<UserTracker>,
            new_user_event_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnNewUser"]
        pub fn disconnect_on_new_user_event(
            tracker: &SharedPtr<UserTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Lost User Callbacks ---
        #[cxx_name = "connectOnLostUserForAsync"]
        pub unsafe fn connect_on_lost_user_event_async(
            tracker: &SharedPtr<UserTracker>,
            lost_user_event_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnLostUser"]
        pub fn disconnect_on_lost_user_event(
            tracker: &SharedPtr<UserTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Synchronous Data Access ---
        #[cxx_name = "getUserFrame"]
        pub fn user_frame(tracker: &SharedPtr<UserTracker>) -> Result<SharedPtr<UserFrame>>;

        // --- Module Information ---
        #[cxx_name = "getProcessingTime"]
        pub fn processing_time(tracker: &SharedPtr<UserTracker>) -> Result<f32>;

        #[cxx_name = "getTrackerTimestamp"]
        pub fn tracker_timestamp(tracker: &SharedPtr<UserTracker>) -> Result<u64>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(tracker: &SharedPtr<UserTracker>) -> Result<bool>;
    }
}

// Mark the opaque C++ type as safe to send across threads.
// This is an assertion that std::shared_ptr<UserTracker> is thread-safe.
unsafe impl Send for ffi::UserTracker {}
unsafe impl Sync for ffi::UserTracker {}