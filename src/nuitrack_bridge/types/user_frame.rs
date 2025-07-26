#[cxx::bridge(namespace = "nuitrack_bridge::user_frame")]
pub mod ffi {
    // --- Rust <-> C++ Type Declarations ---

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        /// An opaque type representing the Nuitrack UserFrame.
        type UserFrame;
    }

    #[namespace = "nuitrack_bridge::user"]
    unsafe extern "C++" {
        /// The User struct defined in user.rs
        type User = crate::nuitrack_bridge::types::user::ffi::User;
    }

    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
        /// The Vector3 struct defined in vector3.rs
        type Vector3 = crate::nuitrack_bridge::types::vector3::ffi::Vector3;
    }

    // Enable passing UserFrame in a smart pointer
    impl SharedPtr<UserFrame> {}
    // Enable receiving a vector of User structs
    impl CxxVector<User> {}

    // --- Function Signatures ---

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/user_frame.h");

        #[cxx_name = "getUsers"]
        pub fn users(frame: &UserFrame) -> Result<UniquePtr<CxxVector<User>>>;

        #[cxx_name = "getRows"]
        pub fn rows(frame: &UserFrame) -> Result<i32>;

        #[cxx_name = "getCols"]
        pub fn cols(frame: &UserFrame) -> Result<i32>;
        
        #[cxx_name = "getID"]
        pub fn id(frame: &UserFrame) -> Result<u64>;

        #[cxx_name = "getData"]
        pub fn data<'a>(frame: &'a UserFrame) -> Result<&'a [u16]>;

        #[cxx_name = "getTimestamp"]
        pub fn timestamp(frame: &UserFrame) -> Result<u64>;
        
        #[cxx_name = "getFloor"]
        pub fn floor(frame: &UserFrame) -> Result<Vector3>;

        #[cxx_name = "getFloorNormal"]
        pub fn floor_normal(frame: &UserFrame) -> Result<Vector3>;
    }
}

// Mark the opaque C++ type as safe to send across threads
unsafe impl Send for ffi::UserFrame {}
unsafe impl Sync for ffi::UserFrame {}