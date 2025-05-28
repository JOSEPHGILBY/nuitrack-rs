use cxx::{CxxString, CxxVector};

#[cxx::bridge(namespace = "nuitrack_bridge::device")]
pub mod ffi {

    #[repr(i32)]
    enum DeviceInfoType {
        PROVIDER_NAME = 0,
        DEVICE_NAME,
        SERIAL_NUMBER,
        Count
    }

    unsafe extern "C++" {
        include!("nuitrack_bridge/device.h");

        
        type Device;
        #[namespace = "tdv::nuitrack::device"]
        type DeviceInfoType;

        type DeviceList;

        fn get_nuitrack_device_list() -> Result<UniquePtr<DeviceList>>;

        #[cxx_name = "get_device_info_wrapper"] // Match C++ wrapper name
        // Pass the opaque type by reference. Result handles potential exceptions from wrapper.
        fn get_device_info(device: &Device, info_type: DeviceInfoType) -> Result<String>;

        #[cxx_name = "nuitrack_set_device_wrapper"]
        fn set_device(device: SharedPtr<Device>) -> Result<()>;

        #[cxx_name = "size"]
        fn device_list_len(self: &DeviceList) -> usize;
        #[cxx_name = "get"]
        fn device_list_get(self: &DeviceList, index: usize) -> SharedPtr<Device>;
       
    }
}

unsafe impl Send for ffi::Device {}
unsafe impl Sync for ffi::Device {}