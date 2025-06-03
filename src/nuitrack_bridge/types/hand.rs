#[cxx::bridge(namespace = "nuitrack_bridge::hand")]
pub mod ffi {
    
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type Hand;
        type UserHands;
    }

    impl SharedPtr<Hand> {}
    impl CxxVector<UserHands> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/hand.h");

        // --- NuitrackHand accessors ---
        #[cxx_name = "getHandX"] // Corrected
        pub fn hand_x(hand: &Hand) -> f32;
        #[cxx_name = "getHandY"] // Corrected
        pub fn hand_y(hand: &Hand) -> f32;
        #[cxx_name = "getHandClick"] // Corrected
        pub fn hand_click(hand: &Hand) -> bool;
        #[cxx_name = "getHandPressure"] // Corrected
        pub fn hand_pressure(hand: &Hand) -> i32;
        #[cxx_name = "getHandXReal"] // Corrected
        pub fn hand_x_real(hand: &Hand) -> f32;
        #[cxx_name = "getHandYReal"] // Corrected
        pub fn hand_y_real(hand: &Hand) -> f32;
        #[cxx_name = "getHandZReal"] // Corrected
        pub fn hand_z_real(hand: &Hand) -> f32;

        // --- NuitrackUserHands accessors ---
        #[cxx_name = "getUserHandsUserId"] // Corrected
        pub fn user_hands_user_id(user_hands: &UserHands) -> i32;
        
        #[cxx_name = "getUserHandsLeftHand"] // Corrected
        pub fn user_hands_left_hand(user_hands: &UserHands) -> SharedPtr<Hand>;
        #[cxx_name = "getUserHandsRightHand"] // Corrected
        pub fn user_hands_right_hand(user_hands: &UserHands) -> SharedPtr<Hand>;
    }
}