#[cfg(feature = "tokio_runtime")]
use tokio::{task::JoinHandle, sync::Mutex as TokioMutex};
#[cfg(feature = "tokio_runtime")] // This import is only needed if tokio_runtime is active
use tokio_util::sync::CancellationToken;
#[cfg(feature = "tokio_runtime")] // Arc is from std, but used with Tokio types here for clarity
use std::sync::Arc;


use std::sync::atomic::{AtomicBool, Ordering};
use cxx::SharedPtr; // Used by WaitableModuleFfiVariant

use super::async_dispatch::run_blocking;

use crate::nuitrack::shared_types::{
    error::{NuitrackError, Result as NuitrackResult}, 
    session_config::DiscoveredDeviceInfo
};
use crate::nuitrack_bridge::core::ffi as core_ffi;

// --- FFI Type Aliases for WaitableModuleFfiVariant ---
// Ensure these paths are correct and these FFI types are Send+Sync marked in their bridge files
type FfiHandTracker = crate::nuitrack_bridge::hand_tracker::ffi::HandTracker;
type FfiSkeletonTracker = crate::nuitrack_bridge::skeleton_tracker::ffi::SkeletonTracker; // Ensure this bridge exists

// --- Make these crate-visible ---
pub(crate) static IS_NUITRACK_RUNTIME_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static NUITRACK_GLOBAL_API_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());


pub struct ActiveDeviceContext {
    pub info: DiscoveredDeviceInfo, // Resolved info of the active device
    //pub depth_sensor: Option<crate::nuitrack::depth::AsyncDepthSensor>, // Placeholder
    //pub color_sensor: Option<crate::nuitrack::color::AsyncColorSensor>, // Placeholder
    //pub user_tracker: Option<crate::nuitrack::user::AsyncUserTracker>, // Placeholder
    pub skeleton_tracker: Option<crate::nuitrack::async_api::hand_tracker::AsyncHandTracker>, // Placeholder
    pub hand_tracker: Option<crate::nuitrack::async_api::hand_tracker::AsyncHandTracker>,
    // pub other_tracker: Option<AsyncOtherTracker>, // Example for your second tracker
}

#[derive(Debug)]
pub(crate) struct NuitrackRuntimeGuard(());

impl NuitrackRuntimeGuard {
    pub(crate) async fn acquire(config_path_str: &str) -> NuitrackResult<Self> {
        if IS_NUITRACK_RUNTIME_INITIALIZED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(NuitrackError::AlreadyInitialized);
        }

        let config_path_owned = config_path_str.to_string();
        if let Err(e) = run_blocking(move || {
            let _global_lock_guard_inner = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|e| {
                NuitrackError::OperationFailed("NUITRACK_GLOBAL_API_LOCK poisoned during init attempt".into())
            })?;
            core_ffi::init(&config_path_owned)
                .map_err(|cxx_e| NuitrackError::InitFailed(format!("FFI init_nuitrack: {}", cxx_e))) // Corrected
        }).await {
            IS_NUITRACK_RUNTIME_INITIALIZED.store(false, Ordering::SeqCst);
            return Err(e); // Pass through the already correctly mapped error
        }
        Ok(Self(()))
    }

    pub(crate) async fn release_async(&self) -> NuitrackResult<()> {
        if IS_NUITRACK_RUNTIME_INITIALIZED.swap(false, Ordering::SeqCst) {
            run_blocking(|| {
                let _global_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| {
                    NuitrackError::OperationFailed("NUITRACK_GLOBAL_API_LOCK poisoned during release".into())
                })?;
                core_ffi::release()
                    .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::release: {}", cxx_e))) // Corrected
            }).await?;
            println!("[NuitrackRuntimeGuard] Nuitrack released via async release.");
        }
        Ok(())
    }
}

impl Drop for NuitrackRuntimeGuard {
    fn drop(&mut self) {
        if let Ok(_global_lock) = NUITRACK_GLOBAL_API_LOCK.try_lock() {
            if IS_NUITRACK_RUNTIME_INITIALIZED.swap(false, Ordering::SeqCst) {
                println!("[NuitrackRuntimeGuard] Dropping... Releasing Nuitrack resources (blocking in drop).");
                if let Err(e) = core_ffi::release() {
                    eprintln!("[NuitrackRuntimeGuard] Error releasing Nuitrack in Drop: {}", e);
                } else {
                    println!("[NuitrackRuntimeGuard] Nuitrack released via Drop.");
                }
            }
        } else {
            if IS_NUITRACK_RUNTIME_INITIALIZED.load(Ordering::SeqCst) {
                eprintln!("[NuitrackRuntimeGuard] Could not acquire global lock in Drop. Nuitrack might not be released if init flag was true.");
            }
        }
    }
}

/// Enum to hold different types of FFI module pointers for the update loop.
/// Make this pub(crate) so session_builder.rs can construct it.
#[derive(Clone)]
pub(crate) enum WaitableModuleFfiVariant {
    Hand(SharedPtr<FfiHandTracker>),
    Skeleton(SharedPtr<FfiSkeletonTracker>),
    // Add other waitable module FFI types here
}


pub struct NuitrackSession {
    pub(crate) guard: NuitrackRuntimeGuard, // Make pub(crate) if builder is in different file but same module tree
    pub active_devices: Vec<ActiveDeviceContext>,
    run_internal_update_loop: bool,
    
    // Store the FFI pointers for the internal loop directly
    #[cfg(feature = "tokio_runtime")]
    modules_for_internal_loop: Vec<WaitableModuleFfiVariant>, // New field

    #[cfg(feature = "tokio_runtime")]
    cancellation_token: Option<Arc<CancellationToken>>,
    #[cfg(feature = "tokio_runtime")]
    update_task_handle: Option<Arc<TokioMutex<Option<JoinHandle<()>>>>>,
}

impl NuitrackSession {
    pub(crate) fn new(
        guard: NuitrackRuntimeGuard,
        active_devices: Vec<ActiveDeviceContext>,
        modules_for_update_loop: Vec<WaitableModuleFfiVariant>,
        run_internal_update_loop: bool,
    ) -> NuitrackResult<Self> {
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

    pub async fn start_processing(&self) -> NuitrackResult<()> {
        {
            
            run_blocking(|| {
                let _g_lock = NUITRACK_GLOBAL_API_LOCK.lock().map_err(|_| NuitrackError::OperationFailed("Global API Lock poisoned for Nuitrack::run".into()))?;
                core_ffi::run()
                .map_err(|e| NuitrackError::OperationFailed(format!("FFI Nuitrack::run: {}", e)))
            }).await?;
        }
        println!("[NuitrackSession] Nuitrack processing started (Nuitrack::run() called).");

        #[cfg(feature = "tokio_runtime")]
        {
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
                        println!("[NuitrackSession] Warning: Internal update loop started but no specific modules collected for waitUpdate. Loop will use global Nuitrack::update().");
                    }

                    let update_task = tokio::spawn(async move {
                        println!("[NuitrackSession InternalLoop] Started.");
                        'update_loop: loop {
                            tokio::select! {
                                biased;
                                _ = token.cancelled() => {
                                    println!("[NuitrackSession InternalLoop] Cancellation received.");
                                    break 'update_loop;
                                }
                                _ = tokio::time::sleep(std::time::Duration::from_millis(1)) => { // Paces the loop
                                    if !modules_to_wait_on.is_empty() {
                                        for module_variant in &modules_to_wait_on {
                                            if token.is_cancelled() { break 'update_loop; }
                                            let wait_result = match module_variant {
                                                WaitableModuleFfiVariant::Hand(ptr) => {
                                                    let ptr_clone = ptr.clone();
                                                    run_blocking(move || {
                                                        core_ffi::wait_update_hand_tracker(&ptr_clone)
                                                            .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e_inner)))
                                                    }).await
                                                }
                                                WaitableModuleFfiVariant::Skeleton(ptr) => {
                                                    let ptr_clone = ptr.clone();
                                                    run_blocking(move || {
                                                        // Ensure you have this FFI function bridged:
                                                        core_ffi::wait_update_skeleton_tracker(&ptr_clone)
                                                            .map_err(|e_inner| NuitrackError::OperationFailed(format!("FFI wait_update_skeleton_tracker: {}", e_inner)))
                                                    }).await
                                                }
                                            };
                                            if let Err(e) = wait_result {
                                                eprintln!("[NuitrackSession InternalLoop] Error in module waitUpdate: {:?}", e);
                                                if NuitrackSession::is_fatal_error(&e) { token.cancel(); break 'update_loop; }
                                            }
                                        }
                                    } else if !token.is_cancelled() && active_devices_are_present { 
                                        // Fallback to global update if no specific modules, but devices are active
                                        if let Err(e) = run_blocking(|| {
                                            core_ffi::update()
                                                .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::update in internal loop: {}", cxx_e)))
                                        }).await { // Assuming nuitrack_update takes no args
                                            eprintln!("[NuitrackSession InternalLoop] Error in global Nuitrack::update: {:?}", e);
                                            if NuitrackSession::is_fatal_error(&e) { token.cancel(); break 'update_loop; }
                                        }
                                    } else if token.is_cancelled() { // Ensure break if cancelled after module loop
                                        break 'update_loop;
                                    }
                                }
                            }
                        }
                        println!("[NuitrackSession InternalLoop] Stopped.");
                    });
                    let mut handle_guard = task_handle_mutex.lock().await;
                    *handle_guard = Some(update_task);

                } else {
                    eprintln!("[NuitrackSession] Internal error: update loop components not initialized despite run_internal_update_loop flag being true.");
                }
            }
        }
        #[cfg(not(feature = "tokio_runtime"))]
        {
            if self.run_internal_update_loop {
                eprintln!(
                    "[NuitrackSession] Warning: Internal update loop was requested, but 'tokio_runtime' feature is not enabled. Manual updates via drive_update_cycle() are required."
                );
            }
        }
        Ok(())
    }

    pub async fn drive_update_cycle(&self) -> NuitrackResult<()> {
        if self.active_devices.is_empty() {
            // No active devices, perhaps a global update is sufficient or do nothing
            return run_blocking(|| {
                core_ffi::update()
                    .map_err(|cxx_e| NuitrackError::OperationFailed(format!("FFI Nuitrack::update in drive_update_cycle (no active devices): {}", cxx_e)))
            }).await; // Or return Ok(())
        }

        for device_ctx in &self.active_devices {
            // Determine which module to wait on for this device
            // This logic should align with how modules_for_internal_loop is populated
            let mut waited = false;
            if let Some(st_wrapper) = &device_ctx.skeleton_tracker { // Prioritize skeleton
                let ptr_clone = st_wrapper.get_ffi_ptr_clone();
                // run_blocking(move || core_ffi::nuitrack_wait_update_skeleton_tracker(&ptr_clone)
                //     .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_skeleton_tracker: {}", e)))).await?;
                waited = true;
            } else if let Some(ht_wrapper) = &device_ctx.hand_tracker {
                let ptr_clone = ht_wrapper.get_ffi_ptr_clone();
                run_blocking(move || core_ffi::wait_update_hand_tracker(&ptr_clone)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e)))).await?;
                waited = true;
            }
            // Add other representative modules here...

            if !waited {
                // If no representative module was found for this device to call specific waitUpdate on,
                // you might call the global Nuitrack::update() once per drive_update_cycle call
                // However, the loop structure here implies one wait per active device.
                // If a device has no "waitable" module, this might be an issue or global update is needed.
                // For now, this device's update might be missed by this specific module iteration.
                // Consider a global update if this loop doesn't do anything specific.
                 println!("[NuitrackSession] drive_update_cycle: Device {:?} has no representative module for specific waitUpdate.", device_ctx.info.serial_number);
            }
        }
        // If no specific waitUpdate was called at all (e.g., no active devices had representative modules),
        // then consider calling the global update once.
        // if self.active_devices.iter().all(|ad| ad.skeleton_tracker.is_none() && ad.hand_tracker.is_none()) {
        //    return run_blocking(core_ffi::nuitrack_update).await;
        // }
        Ok(())
    }
    
    fn is_fatal_error(e: &NuitrackError) -> bool {
        if let NuitrackError::Ffi(cxx_e) = e {
            if cxx_e.what().contains("LicenseNotAcquired") {
                return true;
            }
        }
        false
    }

    pub async fn close(self) -> NuitrackResult<()> {
        #[cfg(feature = "tokio_runtime")]
        {
            if self.run_internal_update_loop {
                if let Some(token) = &self.cancellation_token {
                    token.cancel();
                }
                if let Some(handle_arc) = &self.update_task_handle {
                    let mut handle_guard = handle_arc.lock().await;
                    if let Some(handle) = handle_guard.take() {
                        println!("[NuitrackSession] Awaiting internal update task termination...");
                        if let Err(e) = handle.await {
                            eprintln!("[NuitrackSession] Internal update task panicked or was cancelled with error: {:?}", e);
                        } else {
                            println!("[NuitrackSession] Internal update task joined cleanly.");
                        }
                    }
                }
            }
        }
        
        self.guard.release_async().await?;
        std::mem::forget(self.guard); 
        println!("[NuitrackSession] Nuitrack resources explicitly released.");
        Ok(())
    }
}