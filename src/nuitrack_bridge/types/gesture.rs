#[cxx::bridge(namespace = "nuitrack_bridge::gesture")]
pub mod ffi {
    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum GestureType {
        #[cxx_name = "GESTURE_WAVING"]
        Waving = 0,
        #[cxx_name = "GESTURE_SWIPE_LEFT"]
        SwipeLeft = 1,
        #[cxx_name = "GESTURE_SWIPE_RIGHT"]
        SwipeRight = 2,
        #[cxx_name = "GESTURE_SWIPE_UP"]
        SwipeUp = 3,
        #[cxx_name = "GESTURE_SWIPE_DOWN"]
        SwipeDown = 4,
        #[cxx_name = "GESTURE_PUSH"]
        Push = 5,
    }

    #[repr(i32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum UserStateType {
        #[cxx_name = "USER_IS_ABSENT"]
        IsAbsent = 0,
        #[cxx_name = "USER_IN_SCENE"]
        InScene = 1,
        #[cxx_name = "USER_ACTIVE"]
        Active = 2,
    }

    // --- Structs ---

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Gesture {
        #[cxx_name = "userId"]
        pub user_id: i32,
        #[cxx_name = "type"]
        pub gesture_type: GestureType,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UserState {
        #[cxx_name = "userId"]
        pub user_id: i32,
        pub state: UserStateType,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GestureState {
        #[cxx_name = "type"]
        pub gesture_type: GestureType,
        pub progress: i32,
    }

    // `UserGesturesState` contains a C++ vector, which we must treat
    // as an opaque type. We'll define accessors for it later when we
    // bind the GestureRecognizer module.
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        // Opaque C++ type for std::vector<GestureState>
        type UserGesturesState;

    }

    impl CxxVector<UserGesturesState> {}

    // We can't put `std::vector` inside a shared Rust struct.
    // Instead, we create accessor functions.
    unsafe extern "C++" {
        include!("nuitrack_bridge/types/gesture.h");

        #[cxx_name = "getUserID"]
        pub fn user_id(state: &UserGesturesState) -> i32;
        #[cxx_name = "getUserState"]
        pub fn user_state(state: &UserGesturesState) -> UserStateType;
        #[cxx_name = "getGestures"]
        pub fn gestures<'a>(user_gestures: &UserGesturesState) -> &'a [GestureState];

    }

}