#[cxx::bridge(namespace = "nuitrack_bridge::core")]
pub mod ffi {


    #[namespace = "tdv::nuitrack"] // Ensure this is the correct C++ namespace for HandTracker
    unsafe extern "C++" {
        type HandTracker = crate::nuitrack_bridge::modules::hand_tracker::ffi::HandTracker; // This refers to ::tdv::nuitrack::HandTracker
        type SkeletonTracker = crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonTracker;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/core.h");
        
        fn init(config_path: &str) -> Result<()>;
        pub fn run() -> Result<()>;
        pub fn update() -> Result<()>;

        #[cxx_name = "waitUpdateHandTracker"]
        fn wait_update_hand_tracker(module: &SharedPtr<HandTracker>) -> Result<()>;

        #[cxx_name = "waitUpdateSkeletonTracker"]
        fn wait_update_skeleton_tracker(module: &SharedPtr<SkeletonTracker>) -> Result<()>;

        pub fn release() -> Result<()>;
    }
}