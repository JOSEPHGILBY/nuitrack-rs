#[cxx::bridge(namespace = "nuitrack_bridge::hand_tracker")]
pub mod ffi {

    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type HandTracker;
        //type HandTrackerData; // Corresponds to tdv::nuitrack::HandTrackerData
        #[cxx_name = "Hand"] // The actual C++ struct name is "Hand"
        type NuitrackHand;    // Alias in Rust to avoid conflict if Rust has its own "Hand" type
    }

    // Functions exposed from C++ to Rust.
    // The functions here default to the `nuitrack_bridge::hand_tracker` C++ namespace
    // unless overridden with `#[namespace = "..."]`.
    unsafe extern "C++" {
        include!("nuitrack_bridge/hand_tracker.h");

        pub type c_void;
        type BridgedHandTrackerData;

        // --- HandTracker Methods ---
        #[cxx_name = "create_hand_tracker"]
        pub fn hand_tracker_create() -> Result<SharedPtr<HandTracker>>;

        #[cxx_name = "connect_on_update_wrapper"]
        pub fn connect_on_update(
            tracker: &SharedPtr<HandTracker>, // Changed self to tracker for clarity as it's a free C++ function
            callback: fn(data: SharedPtr<BridgedHandTrackerData>),
        ) -> Result<u64>; // Returns handler_id

        pub unsafe fn connect_on_update_with_user_data(
            tracker: &SharedPtr<HandTracker>,
            user_data: *mut c_void, // Pass sender as raw pointer
        ) -> Result<u64>; // Returns handler_id

        pub unsafe fn connect_on_update_for_blocking( // Rust name for blocking API
            tracker: &SharedPtr<HandTracker>,
            user_data: *mut c_void,
        ) -> Result<u64>; 

        #[cxx_name = "disconnect_on_update_wrapper"]
        pub fn disconnect_on_update(
            tracker: &SharedPtr<HandTracker>,
            handler_id: u64,
        ) -> Result<()>;


        pub fn get_data_timestamp(bht_data: &BridgedHandTrackerData) -> Result<u64>;
        pub fn get_data_num_users(bht_data: &BridgedHandTrackerData) -> Result<i32>;
        pub fn get_users_hands_vector_size(bht_data: &BridgedHandTrackerData) -> Result<usize>;

        pub fn get_user_id_at(bht_data: &BridgedHandTrackerData, user_vec_idx: usize) -> Result<i32>;
        pub fn get_left_hand_at(bht_data: &BridgedHandTrackerData, user_vec_idx: usize) -> Result<SharedPtr<NuitrackHand>>;
        pub fn get_right_hand_at(bht_data: &BridgedHandTrackerData, user_vec_idx: usize) -> Result<SharedPtr<NuitrackHand>>;

        pub fn get_hand_x(hand: &NuitrackHand) -> f32; // No Result needed if C++ side doesn't throw
        pub fn get_hand_y(hand: &NuitrackHand) -> f32;
        #[cxx_name = "get_hand_is_click"]
        pub fn hand_is_click(hand: &NuitrackHand) -> bool;
        pub fn get_hand_pressure(hand: &NuitrackHand) -> i32;
        pub fn get_hand_x_real(hand: &NuitrackHand) -> f32;
        pub fn get_hand_y_real(hand: &NuitrackHand) -> f32;
        pub fn get_hand_z_real(hand: &NuitrackHand) -> f32;
    }
}

unsafe impl Send for ffi::HandTracker {}
unsafe impl Sync for ffi::HandTracker {}