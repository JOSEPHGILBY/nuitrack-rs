use cxx::{CxxString, CxxVector, SharedPtr};

#[cxx::bridge(namespace = "nuitrack_bridge::device")]
pub mod ffi {

    #[repr(i32)]
    enum DeviceInfoType {
        #[cxx_name = "PROVIDER_NAME"]
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

        type SharedPtrDevice;

        #[cxx_name = "unwrapSharedPtrDevice"]
        fn unwrap_shared_ptr_device(spd: &SharedPtrDevice) -> SharedPtr<Device>;

        #[cxx_name = "getDevices"]
        fn get_devices() -> Result<UniquePtr<CxxVector<SharedPtrDevice>>>;

        #[cxx_name = "getDeviceInfo"]
        fn get_device_info(device: &SharedPtr<Device>, info_type: DeviceInfoType) -> Result<String>;

        #[cxx_name = "setDevice"]
        fn set_device(device: &SharedPtr<Device>) -> Result<()>;
       
    }
}

unsafe impl Send for ffi::Device {}
unsafe impl Sync for ffi::Device {}