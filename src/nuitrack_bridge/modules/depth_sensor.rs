#[cxx::bridge(namespace = "nuitrack_bridge::depth_sensor")]
pub mod ffi {
    #[namespace = "nuitrack_bridge::vector3"]
    unsafe extern "C++" {
        type Vector3 = crate::nuitrack_bridge::types::vector3::ffi::Vector3;
    }

    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type DepthSensor;
        // DepthFrame is defined in its own FFI module, so we reference it
        type DepthFrame = crate::nuitrack_bridge::types::depth_frame::ffi::DepthFrame;
    }

    // Bridged C++ types from other modules
    #[namespace = "nuitrack_bridge::output_mode"]
    unsafe extern "C++" {
        type OutputMode = crate::nuitrack_bridge::types::output_mode::ffi::OutputMode;
    }

    // Functions exposed from the C++ bridge to Rust.
    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/depth_sensor.h");

        pub type c_void; // Alias for C void*

        #[cxx_name = "createDepthSensor"]
        pub fn create_depth_sensor() -> Result<SharedPtr<DepthSensor>>;

        #[cxx_name = "connectOnNewFrameForAsync"]
        pub unsafe fn connect_on_depth_frame_async(
            sensor: &SharedPtr<DepthSensor>,
            depth_frame_sender: *mut c_void, // Opaque pointer to Rust sender
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnNewFrame"]
        pub fn disconnect_on_depth_frame(
            sensor: &SharedPtr<DepthSensor>,
            handler_id: u64,
        ) -> Result<()>;

        #[cxx_name = "getDepthFrame"]
        pub fn depth_frame(sensor: &SharedPtr<DepthSensor>) -> Result<SharedPtr<DepthFrame>>;
        
        #[cxx_name = "getOutputMode"]
        pub fn output_mode(sensor: &SharedPtr<DepthSensor>) -> Result<OutputMode>;

        #[cxx_name = "isMirror"]
        pub fn is_mirror(sensor: &SharedPtr<DepthSensor>) -> Result<bool>;

        #[cxx_name = "setMirror"]
        pub fn set_mirror(sensor: &SharedPtr<DepthSensor>, mirror: bool) -> Result<()>;

        #[cxx_name = "convertProjToRealCoords"]
        pub fn convert_proj_to_real_coords(sensor: &SharedPtr<DepthSensor>, p: &Vector3) -> Result<Vector3>;

        #[cxx_name = "convertRealToProjCoords"]
        pub fn convert_real_to_proj_coords(sensor: &SharedPtr<DepthSensor>, p: &Vector3) -> Result<Vector3>;

        #[cxx_name = "getSensorTimestamp"]
        pub fn timestamp(sensor: &SharedPtr<DepthSensor>) -> Result<u64>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(sensor: &SharedPtr<DepthSensor>) -> Result<bool>;
    }
}

// Mark the C++ opaque type as Send + Sync
// This is an assertion that the Nuitrack DepthSensor can be safely used across threads.
unsafe impl Send for ffi::DepthSensor {}
unsafe impl Sync for ffi::DepthSensor {}