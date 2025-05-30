#[cxx::bridge(namespace = "nuitrack_bridge::hand_data")]
pub mod ffi {

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type HandData;
        type UserHands = crate::nuitrack_bridge::types::hand::ffi::UserHands;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/hand_data.h");

        #[cxx_name = "getTimestamp"] // Corrected
        pub fn timestamp(data: &HandData) -> Result<u64>;
        #[cxx_name = "getNumUsers"] // Corrected
        pub fn num_users(data: &HandData) -> Result<i32>;
        
        #[cxx_name = "getUsersHands"] // Corrected
        pub fn users_hands(data: &HandData) -> Result<UniquePtr<CxxVector<UserHands>>>;

        #[cxx_name = "doNotUseMakeHandTrackerDataSharedPtrAware"] // Corrected
        fn do_not_use_make_hand_tracker_data_shared_ptr_aware(data: &SharedPtr<HandData>);
    }
}

unsafe impl Send for ffi::HandData {}
unsafe impl Sync for ffi::HandData {}