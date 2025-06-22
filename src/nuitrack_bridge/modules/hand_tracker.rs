#[cxx::bridge(namespace = "nuitrack_bridge::hand_tracker")]
pub mod ffi {

    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type HandTracker;
        type HandData = crate::nuitrack_bridge::types::hand_data::ffi::HandData;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/hand_tracker.h");

        // Alias for *mut c_void from Rust's perspective
        pub type c_void;

        // --- Module Creation ---
        #[cxx_name = "createHandTracker"]
        pub fn create_hand_tracker() -> Result<SharedPtr<HandTracker>>;

        // --- Callback Management ---
        #[cxx_name = "connectOnUpdateForAsync"]
        pub unsafe fn connect_on_hand_frame_async(
            tracker: &SharedPtr<HandTracker>,
            hand_frame_sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnUpdate"]
        pub fn disconnect_on_hand_frame(
            tracker: &SharedPtr<HandTracker>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Synchronous Data Access ---
        #[cxx_name = "getData"]
        pub fn data(tracker: &SharedPtr<HandTracker>) -> Result<SharedPtr<HandData>>;

        // --- Module Information ---
        #[cxx_name = "getProcessingTime"]
        pub fn processing_time(tracker: &SharedPtr<HandTracker>) -> Result<f32>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(tracker: &SharedPtr<HandTracker>) -> Result<bool>;

        #[cxx_name = "getTrackerTimestamp"]
        pub fn tracker_timestamp(tracker: &SharedPtr<HandTracker>) -> Result<u64>;
    }
}

// Mark the C++ HandTracker opaque type as Send + Sync for Rust.
// This is an assertion that it's safe to send across threads or access globally.
// Nuitrack module pointers obtained via create() are generally safe this way.
unsafe impl Send for ffi::HandTracker {}
unsafe impl Sync for ffi::HandTracker {}