#[cxx::bridge(namespace = "nuitrack_bridge::gesture_recognizer")]
pub mod ffi {
    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type GestureRecognizer;

        // Alias the data frame types from their own FFI modules
        type GestureData = crate::nuitrack_bridge::types::gesture_data::ffi::GestureData;
        type UserStateData = crate::nuitrack_bridge::types::gesture_data::ffi::UserStateData;
        type UserGesturesStateData = crate::nuitrack_bridge::types::gesture_data::ffi::UserGesturesStateData;
    }

    // The main FFI function definitions
    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/gesture_recognizer.h");

        pub type c_void;

        // --- Lifecycle ---
        #[cxx_name = "createGestureRecognizer"]
        pub fn create_gesture_recognizer() -> Result<SharedPtr<GestureRecognizer>>;

        // --- Callbacks ---
        #[cxx_name = "connectOnCompletedGesturesFrameForAsync"]
        pub unsafe fn connect_on_completed_gestures_frame_async(
            recognizer: &SharedPtr<GestureRecognizer>,
            sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnCompletedGesturesFrame"]
        pub fn disconnect_on_completed_gestures_frame(
            recognizer: &SharedPtr<GestureRecognizer>,
            handler_id: u64,
        ) -> Result<()>;

        #[cxx_name = "connectOnUserStateChangeForAsync"]
        pub unsafe fn connect_on_user_state_change_async(
            recognizer: &SharedPtr<GestureRecognizer>,
            sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnUserStateChange"]
        pub fn disconnect_on_user_state_change(
            recognizer: &SharedPtr<GestureRecognizer>,
            handler_id: u64,
        ) -> Result<()>;

        #[cxx_name = "connectOnUpdateForAsync"]
        pub unsafe fn connect_on_update_async(
            recognizer: &SharedPtr<GestureRecognizer>,
            sender: *mut c_void,
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnUpdate"]
        pub fn disconnect_on_update(
            recognizer: &SharedPtr<GestureRecognizer>,
            handler_id: u64,
        ) -> Result<()>;

        // --- Configuration & Control ---
        #[cxx_name = "setControlGesturesStatus"]
        pub fn set_control_gestures_status(recognizer: &SharedPtr<GestureRecognizer>, status: bool) -> Result<()>;
        
        // --- Module Information ---
        #[cxx_name = "getProcessingTime"]
        pub fn processing_time(recognizer: &SharedPtr<GestureRecognizer>) -> Result<f32>;

        #[cxx_name = "getRecognizerTimestamp"]
        pub fn recognizer_timestamp(recognizer: &SharedPtr<GestureRecognizer>) -> Result<u64>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(recognizer: &SharedPtr<GestureRecognizer>) -> Result<bool>;
    }
}

// Mark the opaque C++ type as safe to send across threads
unsafe impl Send for ffi::GestureRecognizer {}
unsafe impl Sync for ffi::GestureRecognizer {}