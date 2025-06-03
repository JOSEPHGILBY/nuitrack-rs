#[cxx::bridge(namespace = "nuitrack_bridge::color_sensor")]
pub mod ffi {
    // Opaque C++ types that Rust will interact with via pointers/references.
    #[namespace = "tdv::nuitrack"] // Map to the original Nuitrack namespace
    unsafe extern "C++" {
        type ColorSensor;
        // RGBFrame is defined in its own FFI module, so we reference it
        type RGBFrame = crate::nuitrack_bridge::types::rgb_frame::ffi::RGBFrame;
        // type OutputMode; // Declared as a bridged struct above.
    }

    #[namespace = "nuitrack_bridge::output_mode"]
    unsafe extern "C++" {
        type OutputMode = crate::nuitrack_bridge::types::output_mode::ffi::OutputMode;
    }

    // Functions exposed from C++ to Rust.
    unsafe extern "C++" {
        include!("nuitrack_bridge/modules/color_sensor.h");

        pub type c_void; // Alias for C void*

        #[cxx_name = "createColorSensor"]
        pub fn create_color_sensor() -> Result<SharedPtr<ColorSensor>>;

        #[cxx_name = "connectOnNewFrameForAsync"]
        pub unsafe fn connect_on_new_frame_for_async(
            sensor: &SharedPtr<ColorSensor>,
            rgb_frame_sender: *mut c_void, // Opaque pointer to Rust sender
        ) -> Result<u64>;

        #[cxx_name = "disconnectOnNewFrame"]
        pub fn disconnect_on_new_frame(
            sensor: &SharedPtr<ColorSensor>,
            handler_id: u64,
        ) -> Result<()>;

        #[cxx_name = "getOutputMode"]
        pub fn output_mode(sensor: &SharedPtr<ColorSensor>) -> Result<OutputMode>;

        #[cxx_name = "getColorFrame"]
        pub fn color_frame(sensor: &SharedPtr<ColorSensor>) -> Result<SharedPtr<RGBFrame>>;
        
        #[cxx_name = "getSensorTimestamp"]
        pub fn timestamp(sensor: &SharedPtr<ColorSensor>) -> Result<u64>;

        #[cxx_name = "canUpdate"]
        pub fn can_update(sensor: &SharedPtr<ColorSensor>) -> Result<bool>;
    }
}

// Mark the C++ opaque type as Send + Sync
// This is safe if Nuitrack's ColorSensor is thread-safe or if access is appropriately managed.
unsafe impl Send for ffi::ColorSensor {}
unsafe impl Sync for ffi::ColorSensor {}