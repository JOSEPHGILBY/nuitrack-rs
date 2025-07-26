#[cxx::bridge(namespace = "nuitrack_bridge::core")]
pub mod ffi {


    #[namespace = "tdv::nuitrack"] // Ensure this is the correct C++ namespace for HandTracker
    unsafe extern "C++" {
        type ColorSensor = crate::nuitrack_bridge::modules::color_sensor::ffi::ColorSensor;
        type HandTracker = crate::nuitrack_bridge::modules::hand_tracker::ffi::HandTracker; // This refers to ::tdv::nuitrack::HandTracker
        type SkeletonTracker = crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonTracker;
        type DepthSensor = crate::nuitrack_bridge::modules::depth_sensor::ffi::DepthSensor;
        type UserTracker = crate::nuitrack_bridge::modules::user_tracker::ffi::UserTracker;
        type GestureRecognizer = crate::nuitrack_bridge::modules::gesture_recognizer::ffi::GestureRecognizer;
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/core.h");
        
        fn init(config_path: &str) -> Result<()>;
        pub fn run() -> Result<()>;
        pub fn update() -> Result<()>;

        #[cxx_name = "waitUpdateColorSensor"]
        fn wait_update_color_sensor(module: &SharedPtr<ColorSensor>) -> Result<()>;

        #[cxx_name = "waitUpdateHandTracker"]
        fn wait_update_hand_tracker(module: &SharedPtr<HandTracker>) -> Result<()>;

        #[cxx_name = "waitUpdateSkeletonTracker"]
        fn wait_update_skeleton_tracker(module: &SharedPtr<SkeletonTracker>) -> Result<()>;

        #[cxx_name = "waitUpdateDepthSensor"]
        fn wait_update_depth_sensor(module: &SharedPtr<DepthSensor>) -> Result<()>;

        #[cxx_name = "waitUpdateUserTracker"]
        fn wait_update_user_tracker(module: &SharedPtr<UserTracker>) -> Result<()>;

        #[cxx_name = "waitUpdateGestureRecognizer"]
        fn wait_update_gesture_recognizer(module: &SharedPtr<GestureRecognizer>) -> Result<()>;

        pub fn release() -> Result<()>;

        #[cxx_name = "setConfigValue"]
        pub fn set_config_value(key: &str, value: &str) -> Result<()>;

        #[cxx_name = "getConfigValue"]
        pub fn get_config_value(key: &str) -> Result<String>;
    }
}