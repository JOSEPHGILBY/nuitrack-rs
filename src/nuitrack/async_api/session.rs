#[cfg(feature = "tokio_runtime")]
use tokio::{task::JoinHandle, sync::Mutex as TokioMutex};
#[cfg(feature = "tokio_runtime")] // This import is only needed if tokio_runtime is active
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tokio_runtime")] // Arc is from std, but used with Tokio types here for clarity
use std::sync::Arc;

use tracing::{debug, error, info, trace, info_span, instrument, trace_span, warn, Instrument};
use std::{collections::HashMap, sync::atomic::{AtomicBool, Ordering}};
use cxx::SharedPtr; // Used by WaitableModuleFfiVariant

use super::{async_dispatch::run_blocking, hand_tracker::AsyncHandTracker, skeleton_tracker::AsyncSkeletonTracker, color_sensor::AsyncColorSensor, depth_sensor::AsyncDepthSensor};

use crate::nuitrack::shared_types::{
    error::{NuitrackError, Result as NuitrackResult}, 
    session_config::DiscoveredDeviceInfo
};
use crate::nuitrack_bridge::core::ffi as core_ffi;

// --- FFI Type Aliases for WaitableModuleFfiVariant ---
// Ensure these paths are correct and these FFI types are Send+Sync marked in their bridge files
type FFIColorSensor = crate::nuitrack_bridge::modules::color_sensor::ffi::ColorSensor;
type FFIHandTracker = crate::nuitrack_bridge::modules::hand_tracker::ffi::HandTracker;
type FFISkeletonTracker = crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonTracker; // Ensure this bridge exists
type FFIDepthSensor = crate::nuitrack_bridge::modules::depth_sensor::ffi::DepthSensor;

// --- Make these crate-visible ---
pub(crate) static IS_NUITRACK_RUNTIME_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static NUITRACK_GLOBAL_API_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());


pub struct ActiveDeviceContext {
    pub info: DiscoveredDeviceInfo, // Resolved info of the active device
    //pub depth_sensor: Option<crate::nuitrack::depth::AsyncDepthSensor>, // Placeholder
    pub color_sensor: Option<AsyncColorSensor>, // Placeholder
    //pub user_tracker: Option<crate::nuitrack::user::AsyncUserTracker>, // Placeholder
    pub skeleton_tracker: Option<AsyncSkeletonTracker>, // Placeholder
    pub hand_tracker: Option<AsyncHandTracker>,
    pub depth_sensor: Option<AsyncDepthSensor>,
    // pub other_tracker: Option<AsyncOtherTracker>, // Example for your second tracker
}

#[derive(Debug)]
pub(crate) struct NuitrackRuntimeGuard(());

impl NuitrackRuntimeGuard {

    #[instrument]
    pub(crate) async fn acquire(
        config_path_str: &str,
        config_values: &HashMap<String, String>,
    ) -> NuitrackResult<Self> {
        if IS_NUITRACK_RUNTIME_INITIALIZED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            debug!("Initialization skipped: Nuitrack runtime is already initialized.");
            return Err(NuitrackError::AlreadyInitialized);
        }

        let config_path_owned = config_path_str.to_string();
        let config_values_owned = config_values.clone();
        if let Err(e) = trace_span!("ffi", function = "Nuitrack::init").in_scope(|| {
            run_blocking(move || {
                let _global_lock_guard_inner = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| {
                    NuitrackError::OperationFailed("NUITRACK_GLOBAL_API_LOCK poisoned during init attempt".into())
                })?;
                
                core_ffi::init(&config_path_owned)
                    .map_err(|cxx_e| NuitrackError::InitFailed(format!("FFI init_nuitrack: {}", cxx_e)))?; // Corrected

                for (key, value) in &config_values_owned {
                    core_ffi::set_config_value(key, value)
                        .map_err(|e| NuitrackError::InitFailed(format!("FFI set_config_value for key '{}': {}", key, e)))?;
                }

                Ok(())
            })
        }).await {
            IS_NUITRACK_RUNTIME_INITIALIZED.store(false, Ordering::SeqCst);
            return Err(e); // Pass through the already correctly mapped error
        }
        info!("Nuitrack runtime initialized.");
        Ok(Self(()))
    }

    #[instrument(skip(self))]
    pub(crate) async fn release_async(&self) -> NuitrackResult<()> {
        if IS_NUITRACK_RUNTIME_INITIALIZED.swap(false, Ordering::SeqCst) {
            trace_span!("ffi", function = "Nuitrack::release").in_scope(|| {
                run_blocking(|| {
                    let _global_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| {
                        NuitrackError::OperationFailed("NUITRACK_GLOBAL_API_LOCK poisoned during release".into())
                    })?;
                    core_ffi::release()
                        .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::release: {}", cxx_e))) // Corrected
                })
            }).await?;
            info!("Nuitrack runtime released via async call.");
        }
        Ok(())
    }
}

impl Drop for NuitrackRuntimeGuard {
    fn drop(&mut self) {
        if let Ok(_global_lock) = NUITRACK_GLOBAL_API_LOCK.try_lock() {
            if IS_NUITRACK_RUNTIME_INITIALIZED.swap(false, Ordering::SeqCst) {
                info!("Dropping NuitrackRuntimeGuard, releasing resources (blocking).");
                if let Err(e) = core_ffi::release() {
                    error!(error = %e, "Failed to release Nuitrack in Drop.");
                }
            } else {
                trace!("Guard dropped, but runtime was not marked as initialized. No-op.");
            }
        } else {
            if IS_NUITRACK_RUNTIME_INITIALIZED.load(Ordering::SeqCst) {
                warn!("Could not acquire global lock in Drop. Nuitrack might not be released if init flag was true.");
            }
        }
    }
}

/// Enum to hold different types of FFI module pointers for the update loop.
/// Make this pub(crate) so session_builder.rs can construct it.
#[derive(Clone)]
pub(crate) enum WaitableModuleFFIVariant {
    ColorSensor(SharedPtr<FFIColorSensor>),
    Hand(SharedPtr<FFIHandTracker>),
    Skeleton(SharedPtr<FFISkeletonTracker>),
    DepthSensor(SharedPtr<FFIDepthSensor>)
    // Add other waitable module FFI types here
}


pub struct NuitrackSession {
    pub(crate) guard: NuitrackRuntimeGuard, // Make pub(crate) if builder is in different file but same module tree
    pub active_devices: Vec<ActiveDeviceContext>,
    run_internal_update_loop: bool,
    
    // Store the FFI pointers for the internal loop directly
    #[cfg(feature = "tokio_runtime")]
    modules_for_internal_loop: Vec<WaitableModuleFFIVariant>, // New field

    #[cfg(feature = "tokio_runtime")]
    cancellation_token: Option<Arc<CancellationToken>>,
    #[cfg(feature = "tokio_runtime")]
    update_task_handle: Option<Arc<TokioMutex<Option<JoinHandle<()>>>>>,
}

impl NuitrackSession {
    #[instrument(skip(guard, active_devices, modules_for_update_loop))]
    pub(crate) fn new(
        guard: NuitrackRuntimeGuard,
        active_devices: Vec<ActiveDeviceContext>,
        modules_for_update_loop: Vec<WaitableModuleFFIVariant>,
        run_internal_update_loop: bool,
    ) -> NuitrackResult<Self> {
        debug!(
            num_devices = active_devices.len(),
            num_update_modules = modules_for_update_loop.len(),
            internal_loop_enabled = run_internal_update_loop,
            "Creating new NuitrackSession."
        );
        Ok(Self {
            guard,
            active_devices,
            run_internal_update_loop,
            #[cfg(feature = "tokio_runtime")]
            modules_for_internal_loop: if run_internal_update_loop { modules_for_update_loop } else { Vec::new() },
            #[cfg(feature = "tokio_runtime")]
            cancellation_token: if run_internal_update_loop { Some(Arc::new(CancellationToken::new())) } else { None },
            #[cfg(feature = "tokio_runtime")]
            update_task_handle: if run_internal_update_loop { Some(Arc::new(TokioMutex::new(None))) } else { None },
        })
    }

    #[instrument(skip(self), name = "nuitrack_start_processing")]
    pub async fn start_processing(&self) -> NuitrackResult<()> {
        {
            trace_span!("ffi", function = "Nuitrack::run").in_scope(|| {
                    run_blocking(|| {
                    let _g_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| NuitrackError::OperationFailed("Global API Lock poisoned for Nuitrack::run".into()))?;
                    core_ffi::run()
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI Nuitrack::run: {}", e)))
                })
            }).await?;
        }
        info!("Nuitrack background processing thread started.");

        #[cfg(feature = "tokio_runtime")]
        {
            debug!("Tokio runtime detected.");
            if self.run_internal_update_loop {
                if let (Some(token_arc), Some(task_handle_mutex_arc)) =
                    (&self.cancellation_token, &self.update_task_handle)
                {

                    let token = Arc::clone(token_arc);
                    let task_handle_mutex = Arc::clone(task_handle_mutex_arc);
                    
                    let active_devices_are_present = !self.active_devices.is_empty();
                    // modules_for_internal_loop is moved into the spawned task
                    let modules_to_wait_on = self.modules_for_internal_loop.clone(); 

                    if modules_to_wait_on.is_empty() && !self.active_devices.is_empty() {
                        warn!("Internal update loop started but no specific modules collected for waitUpdate. Loop will use global Nuitrack::update().");
                    }

                    let update_task = tokio::spawn(
                        async move {
                            debug!("Task started.");
                            'update_loop: loop {
                                tokio::select! {
                                    biased;
                                    _ = token.cancelled() => {
                                        info!("Cancellation received.");
                                        break 'update_loop;
                                    }
                                    _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => { // Paces the loop
                                        if !modules_to_wait_on.is_empty() {
                                            for module_variant in &modules_to_wait_on {
                                                if token.is_cancelled() { break 'update_loop; }

                                                // TODO: trace!(module = ?module_variant, "Calling module-specific waitUpdate.");

                                                let wait_result = match module_variant {
                                                    WaitableModuleFFIVariant::ColorSensor(ptr) => {
                                                        let ptr_clone = ptr.clone();
                                                        trace_span!("ffi", function="wait_update_color_sensor").in_scope(|| {
                                                            run_blocking(move || {
                                                                core_ffi::wait_update_color_sensor(&ptr_clone)
                                                                    .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e_inner)))
                                                            })
                                                        }).await
                                                    }
                                                    WaitableModuleFFIVariant::Hand(ptr) => {
                                                        let ptr_clone = ptr.clone();
                                                        trace_span!("ffi", function="wait_update_hand_tracker").in_scope(|| {
                                                            run_blocking(move || {
                                                                core_ffi::wait_update_hand_tracker(&ptr_clone)
                                                                    .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e_inner)))
                                                            })
                                                        }).await
                                                    }
                                                    WaitableModuleFFIVariant::Skeleton(ptr) => {
                                                        let ptr_clone = ptr.clone();
                                                        trace_span!("ffi", function="wait_update_skeleton_tracker").in_scope(|| {
                                                            run_blocking(move || {
                                                                // Ensure you have this FFI function bridged:
                                                                core_ffi::wait_update_skeleton_tracker(&ptr_clone)
                                                                    .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_skeleton_tracker: {}", e_inner)))
                                                            })
                                                        }).await
                                                    }
                                                    WaitableModuleFFIVariant::DepthSensor(ptr) => {
                                                        let ptr_clone = ptr.clone();
                                                        trace_span!("ffi", function="wait_update_depth_sensor").in_scope(|| {
                                                            run_blocking(move || {
                                                                // Assuming you have this FFI function bridged:
                                                                core_ffi::wait_update_depth_sensor(&ptr_clone)
                                                                    .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_depth_sensor: {}", e_inner)))
                                                            })
                                                        }).await
                                                    }
                                                };
                                                if let Err(e) = wait_result {
                                                    error!(error = %e, "Error in module waitUpdate");
                                                    if NuitrackSession::is_fatal_error(&e) { token.cancel(); break 'update_loop; }
                                                }
                                            }
                                        } else if !token.is_cancelled() && active_devices_are_present { 
                                            debug!("No specific modules to wait on; falling back to global Nuitrack::update().");
                                            // Fallback to global update if no specific modules, but devices are active
                                            if let Err(e) = trace_span!("ffi", function="Nuitrack::update").in_scope(|| {
                                                run_blocking(|| {
                                                    core_ffi::update()
                                                        .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::update in internal loop: {}", cxx_e)))
                                                })
                                            }).await { // Assuming nuitrack_update takes no args
                                                error!(error = %e, "Error in global Nuitrack::update");
                                                if NuitrackSession::is_fatal_error(&e) { token.cancel(); break 'update_loop; }
                                            }
                                        } else if token.is_cancelled() { // Ensure break if cancelled after module loop
                                            break 'update_loop;
                                        }
                                    }
                                }
                            }
                            debug!("Task stopped.");
                        }
                        .instrument(info_span!("nuitrack_internal_update_loop")),
                    );
                    let mut handle_guard = task_handle_mutex.lock().await;
                    *handle_guard = Some(update_task);

                } else {
                    error!("Internal logic error: update loop components were missing when run_internal_update_loop was true.");
                }
            }
        }
        #[cfg(not(feature = "tokio_runtime"))]
        {
            if self.run_internal_update_loop {
                warn!("Internal update loop was requested, but 'tokio_runtime' feature is not enabled. Manual updates via drive_update_cycle() are required.");
            }
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn drive_update_cycle(&self) -> NuitrackResult<()> {
        if self.active_devices.is_empty() {
            // No active devices, perhaps a global update is sufficient or do nothing
            return trace_span!("ffi", function="Nuitrack::update").in_scope(|| {
                run_blocking(|| {
                core_ffi::update()
                    .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::update in drive_update_cycle (no active devices): {}", cxx_e)))
                })
            }).await; // Or return Ok(())
        }

        for device_ctx in &self.active_devices {
            // Determine which module to wait on for this device
            // This logic should align with how modules_for_internal_loop is populated
            let mut waited = false;
            let device_span = info_span!("device_update", serial = %device_ctx.info.serial_number);
            let _enter = device_span.enter();

            if let Some(cs_wrapper) = &device_ctx.color_sensor { // Prioritize skeleton
                let ptr_clone = cs_wrapper.get_ffi_ptr_clone();
                trace_span!("ffi", function="wait_update_skeleton_tracker").in_scope(|| {
                    run_blocking(move || core_ffi::wait_update_color_sensor(&ptr_clone)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_color_sensor: {}", e))))
                }).await?;
                waited = true;
            } else if let Some(st_wrapper) = &device_ctx.skeleton_tracker { // Prioritize skeleton
                let ptr_clone = st_wrapper.get_ffi_ptr_clone();
                trace_span!("ffi", function="wait_update_hand_tracker").in_scope(|| {
                    run_blocking(move || core_ffi::wait_update_skeleton_tracker(&ptr_clone)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_skeleton_tracker: {}", e))))
                }).await?;
                waited = true;
            } else if let Some(ht_wrapper) = &device_ctx.hand_tracker {
                let ptr_clone = ht_wrapper.get_ffi_ptr_clone();
                trace_span!("ffi", function="wait_update_color_sensor").in_scope(|| {
                    run_blocking(move || core_ffi::wait_update_hand_tracker(&ptr_clone)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e))))
                }).await?;
                waited = true;
            } else if let Some(ds_wrapper) = &device_ctx.depth_sensor { // Add this block
                let ptr_clone = ds_wrapper.get_ffi_ptr_clone();
                trace_span!("ffi", function="wait_update_depth_sensor").in_scope(|| {
                    run_blocking(move || core_ffi::wait_update_depth_sensor(&ptr_clone)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_depth_sensor: {}", e))))
                }).await?;
                waited = true;
            }

            if !waited {
                debug!("Device has no representative module for a specific waitUpdate call.");
            }
        }
        Ok(())
    }
    
    fn is_fatal_error(e: &NuitrackError) -> bool {
        if let NuitrackError::FFI(cxx_e) = e {
            let what = cxx_e.what();
            if what.contains("LicenseNotAcquired") {
                // Add this line
                warn!(error_msg = %what, "Fatal Nuitrack error detected.");
                return true;
            }
        }
        false
    }

    #[instrument(skip(self))]
    pub async fn close(self) -> NuitrackResult<()> {
        #[cfg(feature = "tokio_runtime")]
        {
            if self.run_internal_update_loop {
                if let Some(token) = &self.cancellation_token {
                    debug!("Requesting cancellation of internal update loop.");
                    token.cancel();
                }
                if let Some(handle_arc) = &self.update_task_handle {
                    let mut handle_guard = handle_arc.lock().await;
                    if let Some(handle) = handle_guard.take() {
                        info!("Awaiting internal update task termination...");
                        if let Err(e) = handle.await {
                            error!(join_error = ?e, "Internal update task panicked or was cancelled.");
                        } else {
                            info!("Internal update task joined cleanly.");
                        }
                    }
                }
            }
        }
        
        self.guard.release_async().await?;
        debug!("Explicitly forgetting NuitrackRuntimeGuard to prevent double-release in Drop.");
        std::mem::forget(self.guard); 
        info!("Nuitrack session closed successfully.");
        Ok(())
    }
}