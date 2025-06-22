#[cxx::bridge(namespace = "nuitrack_bridge::hand")]
pub mod ffi {

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Hand {
        pub x: f32,
        pub y: f32,
        pub click: bool,
        pub pressure: i32,
        pub x_real: f32,
        pub y_real: f32,
        pub z_real: f32,
    }
    
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type UserHands;
    }

    impl CxxVector<UserHands> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/hand.h");

        // --- NuitrackUserHands accessors ---
        #[cxx_name = "getUserHandsUserId"] // Corrected
        pub fn user_hands_user_id(user_hands: &UserHands) -> i32;
        
        #[cxx_name = "getUserHandsLeftHand"] // Corrected
        pub fn user_hands_left_hand(user_hands: &UserHands) -> SharedPtr<Hand>;
        #[cxx_name = "getUserHandsRightHand"] // Corrected
        pub fn user_hands_right_hand(user_hands: &UserHands) -> SharedPtr<Hand>;
    }
}