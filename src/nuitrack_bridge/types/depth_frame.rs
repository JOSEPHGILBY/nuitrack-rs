#[cxx::bridge(namespace = "nuitrack_bridge::depth_frame")]
pub mod ffi {

    #[namespace = "tdv::nuitrack"]
    unsafe extern "C++" {
        type DepthFrame;
    }

    impl SharedPtr<DepthFrame> {}

    unsafe extern "C++" {
        include!("nuitrack_bridge/types/depth_frame.h");

        #[cxx_name = "getRows"]
        pub fn rows(frame: &DepthFrame) -> Result<i32>;

        #[cxx_name = "getCols"]
        pub fn cols(frame: &DepthFrame) -> Result<i32>;

        #[cxx_name = "getID"]
        pub fn id(frame: &DepthFrame) -> Result<u64>;

        #[cxx_name = "getTimestamp"]
        pub fn timestamp(frame: &DepthFrame) -> Result<u64>;

        #[cxx_name = "getData"]
        pub fn data(frame: &DepthFrame) -> Result<&[u16]>;
    }
}

unsafe impl Send for ffi::DepthFrame {}
unsafe impl Sync for ffi::DepthFrame {}






