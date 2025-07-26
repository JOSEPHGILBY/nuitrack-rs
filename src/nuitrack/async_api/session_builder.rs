use tracing::{info, info_span, instrument, trace_span, warn};
use std::collections::HashMap;
use std::path::Path;
use cxx::SharedPtr;

use crate::nuitrack::async_api::color_sensor::AsyncColorSensor;
use crate::nuitrack::async_api::depth_sensor::AsyncDepthSensor;
use crate::nuitrack::async_api::gesture_recognizer::AsyncGestureRecognizer;
use crate::nuitrack::async_api::user_tracker::AsyncUserTracker;
use crate::nuitrack_bridge::device::ffi as device_ffi;
use super::async_dispatch::run_blocking;
use super::skeleton_tracker::AsyncSkeletonTracker;
use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
use crate::nuitrack::shared_types::session_config::{
    DeviceConfig, DeviceSelector, DiscoveredDeviceInfo, ModuleType
};
use super::session::{
    ActiveDeviceContext, NuitrackRuntimeGuard, NuitrackSession, WaitableModuleFFIVariant, NUITRACK_GLOBAL_API_LOCK // Made pub(crate) in session.rs
};
// Import your async module wrappers
use super::hand_tracker::AsyncHandTracker;
// use super::skeleton_tracker::AsyncSkeletonTracker; // You'll need this

// Helper FFI type alias
type FFIDevice = crate::nuitrack_bridge::device::ffi::Device;


#[derive(Default)]
pub struct NuitrackSessionBuilder {
    global_config_path: Option<String>,
    device_configurations: Vec<DeviceConfig>,
    run_internal_update_loop: bool,
    config_values: HashMap<String, String>,
    // Add policy flags here if desired
    // policy_strict_device_match: bool, // e.g., error if a configured device selector finds no match
}

impl NuitrackSessionBuilder {
    pub fn new() -> Self {
        Self {
            run_internal_update_loop: cfg!(feature = "tokio_runtime"),
            ..Default::default()
        }
    }

    #[instrument]
    pub async fn create_session_from_single_default_device(modules_to_create: Vec<ModuleType>) -> NuitrackResult<NuitrackSession> {
        info!(?modules_to_create, "Creating session for single default device.");
        let device_config = DeviceConfig {
            selector: DeviceSelector::ByIndex(0), 
            modules_to_create,
        };
        Ok(Self::new()
            // .global_config_path("path/to/your/nuitrack.config") // Optional
            .with_device_config(device_config) // Add our device configuration
            // .manage_update_loop(true) // Default is true if 'tokio_runtime' feature is on
            .init_session() // Initialize the session
            .await?)
    }

    pub fn global_config_path(mut self, path: impl AsRef<Path>) -> Self {
        self.global_config_path = Some(path.as_ref().to_string_lossy().into_owned());
        self
    }

    pub fn with_config_value(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config_values.insert(key.into(), value.into());
        self
    }

    pub fn with_device_config(mut self, config: DeviceConfig) -> Self {
        self.device_configurations.push(config);
        self
    }
    
    pub fn manage_update_loop(mut self, enabled: bool) -> Self {
        self.run_internal_update_loop = enabled;
        self
    }

    /// Initializes the Nuitrack session based on the builder's configuration.
    /// This path is used when the user provides all configurations upfront.
    #[instrument(skip(self), name = "init_session")]
    pub async fn init_session(self) -> NuitrackResult<NuitrackSession> {
        let guard = NuitrackRuntimeGuard::acquire(
            &self.global_config_path.unwrap_or_default(),
            &self.config_values,
        ).await?;
        
        let available_devices_cache = Self::fetch_available_devices_info_internal().await.map_err(|e| {
            // Guard will drop and release if this errors
            e
        })?;
        
        let effective_configs = if self.device_configurations.is_empty() {
            if available_devices_cache.len() == 1 {
                info!("No device configurations provided; defaulting to the single available device.");
                vec![DeviceConfig { // Default to the single available device
                    selector: DeviceSelector::ByIndex(0), 
                    modules_to_create: vec![ModuleType::HandTracker, ModuleType::SkeletonTracker], // Sensible defaults
                }]
            } else if available_devices_cache.is_empty() {
                warn!("No devices found and no configurations provided. Session will have no active devices.");
                Vec::new() // No devices, no configs, session will have no active devices
            } else {
                // Multiple devices but no user config. Let configure_devices_and_modules handle or error based on policy.
                // Or error here directly if this is the "sensible default" path.
                return Err(NuitrackError::DeviceError("Multiple devices found; specific configuration or DefaultSingle selector required.".into()));
            }
        } else {
            self.device_configurations
        };

        let (active_device_contexts, modules_for_update_loop) = 
            Self::configure_devices_and_modules(
                available_devices_cache,
                effective_configs,
            ).await?;

        NuitrackSession::new(
            guard,
            active_device_contexts,
            modules_for_update_loop,
            self.run_internal_update_loop,
        )
    }

    /// Starts a phased initialization allowing device discovery first.
    #[instrument(skip(self))]
    pub async fn discover_devices_first(self) -> NuitrackResult<DeviceDiscoveryState> {
        let config_path_for_acquire = self.global_config_path.as_deref().unwrap_or_default();
        info!("Acquiring runtime guard and discovering devices...");
        let guard = NuitrackRuntimeGuard::acquire(
            config_path_for_acquire,
            &self.config_values
        ).await?;
        let available_devices = Self::fetch_available_devices_info_internal().await.map_err(|e| {
            // Guard will drop and release if this errors.
            e
        })?;
        info!(count = available_devices.len(), "Device discovery complete.");

        Ok(DeviceDiscoveryState {
            guard: Some(guard),
            available_devices,
            builder_settings: self, // Store the original builder settings (config_path, run_internal_update_loop)
        })
    }
    
    /// Internal helper to get device list and info. Assumes Nuitrack is globally initialized.
    #[instrument]
    async fn fetch_available_devices_info_internal() -> NuitrackResult<Vec<DiscoveredDeviceInfo>> {
        trace_span!("ffi", function = "Nuitrack::getDeviceList").in_scope(|| {
            run_blocking(move || {
                let _g_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| NuitrackError::OperationFailed("Global API lock for getDeviceList".into()))?;
                let devices = device_ffi::devices()
                    .map_err(|e| NuitrackError::DeviceError(format!("FFI GetDeviceList: {}", e)))?;
                let mut devices_info_vec = Vec::new();
                for i in 0..devices.len() {
                    let Some(wrapped_device) = devices.get(i) else { continue };
                    let device = device_ffi::unwrap_shared_ptr_device(wrapped_device);
                    let name = device_ffi::device_info(&device, device_ffi::DeviceInfoType::DEVICE_NAME).unwrap_or_else(|_| "N/A".to_string());
                    let serial = device_ffi::device_info(&device, device_ffi::DeviceInfoType::SERIAL_NUMBER).unwrap_or_else(|_| "N/A".to_string());
                    let provider = device_ffi::device_info(&device, device_ffi::DeviceInfoType::PROVIDER_NAME).unwrap_or_else(|_| "N/A".to_string());
                    devices_info_vec.push(DiscoveredDeviceInfo { 
                        name, 
                        serial_number: serial, 
                        provider_name: provider, 
                        original_index: i, 
                        ffi_device_ptr: device // Essential for selection
                    });
                    
                }
                Ok(devices_info_vec)
            })
        }).await
    }

    /// Common logic: takes discovered devices and user configs, sets devices, creates modules.
    #[instrument(skip(available_devices_cache, user_device_configs))]
    async fn configure_devices_and_modules(
        available_devices_cache: Vec<DiscoveredDeviceInfo>,
        user_device_configs: Vec<DeviceConfig>,
    ) -> NuitrackResult<(Vec<ActiveDeviceContext>, Vec<WaitableModuleFFIVariant>)> {
        let mut active_devices_built = Vec::new();
        let mut modules_for_update_loop: Vec<WaitableModuleFFIVariant> = Vec::new();

        if user_device_configs.is_empty() && !available_devices_cache.is_empty() {
            warn!("No device configurations provided, but devices are available. No modules will be activated.");
        }


        for (i, dev_config) in user_device_configs.into_iter().enumerate() {
            let config_span = info_span!("device_config", id = i, selector = ?dev_config.selector);
            let _enter = config_span.enter();
            let (selected_device_info_ref, target_ffi_device_ptr_clone) = 
                Self::find_target_device_from_cache(&available_devices_cache, &dev_config.selector)?;

            info!(device_serial = %selected_device_info_ref.serial_number, "Configuring device.");    

            // Set this device as active globally
            { // Scope for global lock
                let ptr_for_set = target_ffi_device_ptr_clone.clone();
                trace_span!("ffi", function="Nuitrack::setDevice").in_scope(|| {
                    run_blocking(move || {
                        let _g_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| NuitrackError::OperationFailed("Global API lock for set_device".into()))?;
                        device_ffi::set_device(&ptr_for_set)
                            .map_err(|cxx_e| NuitrackError::DeviceError(format!("FFI Nuitrack::setDevice failed: {}", cxx_e)))
                            // Alternatively, for a more generic FFI error:
                            // .map_err(NuitrackError::from)
                    })
                }).await?;
            }

            let mut ad_context = ActiveDeviceContext {
                info: selected_device_info_ref.clone(),
                color_sensor: None,
                hand_tracker: None, 
                skeleton_tracker: None,
                depth_sensor: None,
                user_tracker: None,
                gesture_recognizer: None,
            };
            
            let mut representative_module_for_device: Option<WaitableModuleFFIVariant> = None;

            for module_type in dev_config.modules_to_create {
                match module_type {
                    ModuleType::ColorSensor => {
                        let cs = AsyncColorSensor::new_async().await?; // Assumes device is set
                        if representative_module_for_device.is_none() { // Prefer ColorSensor if Skeleton not chosen
                            representative_module_for_device = Some(WaitableModuleFFIVariant::ColorSensor(cs.get_ffi_ptr_clone()));
                        }
                        ad_context.color_sensor = Some(cs);
                    }
                    ModuleType::HandTracker => {
                        let ht = AsyncHandTracker::new_async().await?; // Assumes device is set
                        if representative_module_for_device.is_none() { // Prefer HandTracker if Skeleton not chosen
                           representative_module_for_device = Some(WaitableModuleFFIVariant::Hand(ht.get_ffi_ptr_clone()));
                        }
                        ad_context.hand_tracker = Some(ht);
                    }
                    ModuleType::SkeletonTracker => {
                        let st = AsyncSkeletonTracker::new_async().await?;
                        if representative_module_for_device.is_none() {
                            representative_module_for_device = Some(WaitableModuleFFIVariant::Skeleton(st.get_ffi_ptr_clone()));
                        }
                        ad_context.skeleton_tracker = Some(st);
                        
                    }
                    ModuleType::DepthSensor => {
                        let ds = AsyncDepthSensor::new_async().await?;
                        if representative_module_for_device.is_none() {
                            representative_module_for_device = Some(WaitableModuleFFIVariant::DepthSensor(ds.get_ffi_ptr_clone()));
                        }
                        ad_context.depth_sensor = Some(ds);
                    }
                    ModuleType::UserTracker => { // ADD THIS BLOCK
                        let ut = AsyncUserTracker::new_async().await?;
                        if representative_module_for_device.is_none() {
                            representative_module_for_device = Some(WaitableModuleFFIVariant::UserTracker(ut.get_ffi_ptr_clone()));
                        }
                        ad_context.user_tracker = Some(ut);
                    }
                    ModuleType::GestureRecognizer => {
                        let gr = AsyncGestureRecognizer::new_async().await?;
                        if representative_module_for_device.is_none() {
                            representative_module_for_device = Some(WaitableModuleFFIVariant::GestureRecognizer(gr.get_ffi_ptr_clone()));
                        }
                        ad_context.gesture_recognizer = Some(gr);
                    }
                    _ => {}
                    // ... other module types like DepthSensor, ColorSensor, UserTracker ...
                }
            }
            if let Some(rep_module) = representative_module_for_device {
                modules_for_update_loop.push(rep_module);
            }
            active_devices_built.push(ad_context);
        }
        Ok((active_devices_built, modules_for_update_loop))
    }

    /// Helper to find a device in the cached list based on selector.
    #[instrument(skip(available_devices))]
    fn find_target_device_from_cache<'a>(
        available_devices: &'a [DiscoveredDeviceInfo],
        selector: &DeviceSelector,
    ) -> NuitrackResult<(&'a DiscoveredDeviceInfo, SharedPtr<FFIDevice>)> { // Returns refs/cloned SharedPtr
        match selector {
            DeviceSelector::DefaultSingle => {
                if available_devices.len() == 1 {
                    let info_ref = available_devices.get(0).unwrap(); // Safe due to check
                    Ok((info_ref, info_ref.ffi_device_ptr.clone()))
                } else {
                    Err(NuitrackError::DeviceError(format!(
                        "DefaultSingle policy: Expected 1 device, found {}.", available_devices.len()
                    )))
                }
            }
            DeviceSelector::ByIndex(idx) => available_devices
                .get(*idx)
                .map(|info_ref| (info_ref, info_ref.ffi_device_ptr.clone()))
                .ok_or_else(|| NuitrackError::DeviceError(format!("Device at index {} not found.", idx))),
            DeviceSelector::BySerialNumber(serial_to_find) => available_devices
                .iter()
                .find(|info_ref| info_ref.serial_number == *serial_to_find)
                .map(|info_ref| (info_ref, info_ref.ffi_device_ptr.clone()))
                .ok_or_else(|| NuitrackError::DeviceError(format!("Device with serial '{}' not found.", serial_to_find))),
        }
    }
}

// --- Typestate for Device Discovery ---
pub struct DeviceDiscoveryState {
    guard: Option<NuitrackRuntimeGuard>, // Option to allow taking it for finalization
    pub available_devices: Vec<DiscoveredDeviceInfo>, // User inspects this
    builder_settings: NuitrackSessionBuilder, // Carries over settings like config_path, run_internal_update_loop
}

impl DeviceDiscoveryState {
    pub fn list_devices(&self) -> &[DiscoveredDeviceInfo] {
        &self.available_devices
    }

    /// User calls this after inspecting devices and deciding on configurations.
    #[instrument(skip(self, user_selected_device_configs))]
    pub async fn finalize_session(
        mut self, // Takes ownership
        user_selected_device_configs: Vec<DeviceConfig>,
    ) -> NuitrackResult<NuitrackSession> {
        info!("Finalizing session from discovered devices.");
        let guard = self.guard.take().ok_or_else(|| NuitrackError::OperationFailed("NuitrackRuntimeGuard already taken/missing in DeviceDiscoveryState".into()))?;
        
        // Use the common configuration logic, passing the already discovered devices
        let (active_device_contexts, modules_for_update_loop) =
            NuitrackSessionBuilder::configure_devices_and_modules(
                self.available_devices.clone(), // These already have FFI ptrs
                user_selected_device_configs,
            ).await?;
        
        NuitrackSession::new(
            guard,
            active_device_contexts,
            modules_for_update_loop,
            self.builder_settings.run_internal_update_loop,
        )
    }
}

impl Drop for DeviceDiscoveryState {
    fn drop(&mut self) {
        if let Some(_guard_being_dropped) = self.guard.take() {
            warn!("DeviceDiscoveryState dropped without finalizing session. Nuitrack resources will be released by NuitrackRuntimeGuard's Drop.");
        }
    }
}