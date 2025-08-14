#[cxx::bridge(namespace = "nuitrack_bridge::gesture_data")]
pub mod ffi {
    // Import the shared gesture types we defined previously.
    #[namespace = "nuitrack_bridge::gesture"]
    unsafe extern "C++" {
        type Gesture = crate::nuitrack_bridge::types::gesture::ffi::Gesture;
        type UserState = crate::nuitrack_bridge::types::gesture::ffi::UserState;
    }

    // Import the opaque C++ type from the previous step
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type UserGesturesState = crate::nuitrack_bridge::types::gesture::ffi::UserGesturesState;
    }

    impl SharedPtr<UserGesturesState> {}

    // Define the three data frame types from GestureData.h as opaque C++ types.
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type GestureData;
        type UserStateData;
        type UserGesturesStateData;
    }

    // Enable passing these types via smart pointers from C++ to Rust.
    impl SharedPtr<GestureData> {}
    impl SharedPtr<UserStateData> {}
    impl SharedPtr<UserGesturesStateData> {}

    // Define the accessor functions for each type.
    unsafe extern "C++" {
        include!("nuitrack_bridge/types/gesture_data.h");

        // --- GestureData Accessors ---
        #[cxx_name = "getGestureDataTimestamp"]
        pub fn gesture_data_timestamp(data: &GestureData) -> Result<u64>;
        #[cxx_name = "getGestureDataNumGestures"]
        pub fn gesture_data_num_gestures(data: &GestureData) -> Result<i32>;
        #[cxx_name = "getGestureDataGestures"]
        pub fn gesture_data_gestures(data: &GestureData) -> Result<Vec<Gesture>>;

        // --- UserStateData Accessors ---
        #[cxx_name = "getUserStateDataTimestamp"]
        pub fn user_state_data_timestamp(data: &UserStateData) -> Result<u64>;
        #[cxx_name = "getUserStateDataNumUserStates"]
        pub fn user_state_num_user_states(data: &UserStateData) -> Result<i32>;
        #[cxx_name = "getUserStateDataUserStates"]
        pub fn user_state_data_user_states(data: &UserStateData) -> Result<Vec<UserState>>;

        // --- UserGesturesStateData Accessors ---
        #[cxx_name = "getUserGesturesStateDataTimestamp"]
        pub fn user_gestures_state_data_timestamp(data: &UserGesturesStateData) -> Result<u64>;
        #[cxx_name = "getUserGesturesStateDataNumUsers"]
        pub fn user_gestures_state_data_num_users(data: &UserGesturesStateData) -> Result<i32>;
        #[cxx_name = "getUserGesturesStateDataUserGesturesStates"]
        pub fn user_gestures_state_data_user_gestures_states(data: &UserGesturesStateData) -> Result<UniquePtr<CxxVector<UserGesturesState>>>;
    }
}

// Mark the opaque C++ types as safe to send across threads
unsafe impl Send for ffi::GestureData {}
unsafe impl Sync for ffi::GestureData {}
unsafe impl Send for ffi::UserStateData {}
unsafe impl Sync for ffi::UserStateData {}
unsafe impl Send for ffi::UserGesturesStateData {}
unsafe impl Sync for ffi::UserGesturesStateData {}