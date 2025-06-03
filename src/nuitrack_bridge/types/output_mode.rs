#[cxx::bridge(namespace = "nuitrack_bridge::output_mode")]
pub mod ffi {

    #[derive(Debug, Clone, Copy, PartialEq)] // Add PartialEq for easier comparison if needed
    pub struct Intrinsics {
        pub fx: f32,
        pub fy: f32,
        pub cx: f32,
        pub cy: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)] // Intrinsics must also be Copy for OutputMode to derive Copy
    pub struct OutputMode {
        pub fps: i32,
        pub xres: i32,
        pub yres: i32,
        pub hfov: f32,
        pub intrinsics: Intrinsics, // Embed the Rust Intrinsics struct
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/output_mode.h");
    }
}