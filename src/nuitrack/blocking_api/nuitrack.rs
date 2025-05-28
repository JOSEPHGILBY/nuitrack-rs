use std::{path::Path, sync::atomic::{AtomicBool, Ordering}};
use crate::nuitrack_bridge::{core::ffi::{init_nuitrack, nuitrack_release, nuitrack_run}, device::ffi::{get_device_info, get_nuitrack_device_list, set_device, Device, DeviceInfoType}};
use crate::nuitrack_bridge::{
    core::ffi as core_ffi,
    device::ffi as device_ffi,
};

use super::{async_dispatch::run_blocking, error::{NuitrackError, Result}, hand_tracker::AsyncHandTracker};


static IS_NUITRACK_INITIALIZED_BY_WRAPPER: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct Nuitrack {
    // This field makes the struct non-ZST (Zero-Sized Type) and ensures
    // its lifetime is tied to the Nuitrack initialized state via this wrapper.
    _private_guard: (),
}

impl Nuitrack {
    pub fn init(config_path: Option<&str>) -> Result<Self> {
        if IS_NUITRACK_INITIALIZED_BY_WRAPPER.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(NuitrackError::AlreadyInitialized);
        }

        let path_str = config_path.unwrap_or("");
        init_nuitrack(path_str)
            .map_err(|e| NuitrackError::InitFailed(format!("FFI init_nuitrack: {}", e)))?;
        println!("NuitrackSystem: Nuitrack initialized.");

        // Auto-select and set the first available device as a default behavior
        let ffi_device_list = get_nuitrack_device_list()
            .map_err(|e| NuitrackError::DeviceError(format!("FFI get_nuitrack_device_list: {}", e)))?;
        
        if (&ffi_device_list).device_list_len() == 0 {
            // Attempt to release before bailing if init succeeded but no devices found.
            let _ = nuitrack_release(); // Best effort
            IS_NUITRACK_INITIALIZED_BY_WRAPPER.store(false, Ordering::SeqCst);
            return Err(NuitrackError::NoDeviceFound);
        }
        let first_device_ffi_ptr = (&ffi_device_list).device_list_get(0);
        set_device(first_device_ffi_ptr)
            .map_err(|e| NuitrackError::DeviceError(format!("FFI set_device: {}", e)))?;
        println!("NuitrackSystem: Default device set.");

        Ok(Nuitrack { _private_guard: () })
    }

    pub fn run(&self) -> Result<()> {
        nuitrack_run()?;
        println!("NuitrackSystem: Processing started (Nuitrack::run() called).");
        Ok(())
    }
}

impl Drop for Nuitrack {
    fn drop(&mut self) {
        println!("NuitrackSystem: Dropping... Releasing Nuitrack resources.");
        if let Err(e) = nuitrack_release() {
            eprintln!("NuitrackSystem: Error releasing Nuitrack in Drop: {}", e);
        } else {
            println!("NuitrackSystem: Nuitrack released automatically.");
        }
        IS_NUITRACK_INITIALIZED_BY_WRAPPER.store(false, Ordering::SeqCst);
    }
}