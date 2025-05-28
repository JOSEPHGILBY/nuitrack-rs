// In src/nuitrack/mod.rs or a new session_config.rs

use cxx::SharedPtr;
use std::path::Path;
use super::error::NuitrackError; // Assuming error module is at super::error

// --- For Device Discovery ---
#[derive(Clone)]
pub struct DiscoveredDeviceInfo {
    pub name: String,
    pub serial_number: String,
    pub provider_name: String, // Add if you can get it
    pub original_index: usize,
    // Keep the FFI Ptr to allow selection based on this discovered object later
    // This means FfiDevice needs to be Send+Sync for SharedPtr to be Send+Sync for Vec to be Send
    pub(crate) ffi_device_ptr: SharedPtr<crate::nuitrack_bridge::device::ffi::Device>,
}

// --- For Builder Configuration ---
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModuleType {
    DepthSensor,
    ColorSensor,
    UserTracker,
    SkeletonTracker,
    HandTracker,
    // Example: Add another for testing your "second tracker type" idea
    // OtherTracker, // You'd need an AsyncOtherTracker and its FFI
}

#[derive(Clone, Debug)]
pub enum DeviceSelector {
    /// Use the device at this specific index from the getDeviceList()
    ByIndex(usize),
    /// Use the device matching this serial number.
    BySerialNumber(String),
    /// Policy: Expects exactly one device. Uses it. Errors if 0 or >1.
    DefaultSingle,
    // /// Policy: Use all available devices with a common module configuration.
    // AllAvailable,
}

#[derive(Clone, Debug)]
pub struct DeviceConfig {
    pub selector: DeviceSelector,
    pub modules_to_create: Vec<ModuleType>,
}
