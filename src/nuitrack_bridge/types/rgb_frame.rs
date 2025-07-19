#[cxx::bridge(namespace = "nuitrack_bridge::rgb_frame")]
pub mod ffi {
    
    #[derive{Debug, Clone, Copy, PartialEq}]
    pub struct Color3 {
        pub blue: u8,
        pub green: u8,
        pub red: u8,
    }

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type RGBFrame;
    }

    impl SharedPtr<RGBFrame> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/rgb_frame.h");

        #[cxx_name = "getRows"]
        pub fn rows(frame: &RGBFrame) -> Result<i32>;

        #[cxx_name = "getCols"]
        pub fn cols(frame: &RGBFrame) -> Result<i32>;

        #[cxx_name = "getID"]
        pub fn id(frame: &RGBFrame) -> Result<u64>;

        #[cxx_name = "getTimestamp"]
        pub fn timestamp(frame: &RGBFrame) -> Result<u64>;

        #[cxx_name = "getData"]
        pub fn data(frame: &RGBFrame) -> Result<&[Color3]>;
    }
}

unsafe impl Send for ffi::RGBFrame {}
unsafe impl Sync for ffi::RGBFrame {}