use std::panic::{catch_unwind, AssertUnwindSafe};

use cxx::SharedPtr;
use crate::nuitrack_bridge::hand_tracker::ffi::{self as ht_ffi, c_void, BridgedHandTrackerData, HandTracker as FFIHandTracker};
use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
use crate::nuitrack::shared_types::hand_frame::HandFrame;

pub type BlockingHandFrameCallback = Box<dyn FnMut(NuitrackResult<HandFrame>) + Send + 'static>;


struct BlockingCallbackHolder {
    callback: BlockingHandFrameCallback,
}
pub struct BlockingHandTracker {
    ptr: SharedPtr<FFIHandTracker>,
    handler_id: Option<u64>,
    // Stores the Boxed callback data to keep it alive and manage its lifetime.
    callback_data_ptr: Option<*mut BlockingCallbackHolder>, // Pointer to the Box's content
}

impl BlockingHandTracker {
    pub(crate) fn new_blocking() -> NuitrackResult<Self> {
        let tracker_ptr = ht_ffi::hand_tracker_create().map_err(|e| {
            NuitrackError::ModuleCreationFailed(format!("BlockingHandTracker FFI create: {}", e))
        })?;
        Ok(Self {
            ptr: tracker_ptr,
            handler_id: None,
            callback_data_ptr: None,
        })
    }

    pub fn connect_on_update(
        &mut self,
        user_callback: BlockingHandFrameCallback,
    ) -> NuitrackResult<u64> {
        if self.handler_id.is_some() {
            return Err(NuitrackError::OperationFailed(
                "Callback already registered for this BlockingHandTracker".into(),
            ));
        }

        // Box the holder for the user's callback to get a stable heap pointer.
        let callback_holder = Box::new(BlockingCallbackHolder { callback: user_callback });
        // Convert the Box into a raw pointer to pass as `user_data`.
        let user_data_raw_ptr = Box::into_raw(callback_holder);

        self.callback_data_ptr = Some(user_data_raw_ptr); // Store for later cleanup

        let handler_id = unsafe {
            // Call the FFI function dedicated to blocking callbacks that uses the correct C++ wrapper
            // which in turn calls `rust_blocking_hand_tracker_trampoline`.
            ht_ffi::connect_on_update_for_blocking(&self.ptr, user_data_raw_ptr as *mut c_void)
        }
        .map_err(|e| {
            // If FFI call fails, reclaim the Box immediately.
            if let Some(raw_ptr) = self.callback_data_ptr.take() {
                unsafe { drop(Box::from_raw(raw_ptr)); }
            }
            NuitrackError::OperationFailed(format!("FFI connect_blocking_callback: {}", e))
        })?;
        
        self.handler_id = Some(handler_id);
        Ok(handler_id)
    }

    pub fn disconnect_on_update(&mut self) -> NuitrackResult<()> {
        if let Some(handler_id) = self.handler_id.take() {
            
            let disconnect_result = ht_ffi::disconnect_on_update(&self.ptr, handler_id)
                .map_err(|e| {
                    NuitrackError::OperationFailed(format!("FFI disconnect_on_update: {}", e))
                });

            // Reclaim the Boxed callback data *after* attempting disconnect.
            if let Some(raw_ptr) = self.callback_data_ptr.take() {
                unsafe { drop(Box::from_raw(raw_ptr)); } // This drops the Box<BlockingCallbackHolder>
            }
            disconnect_result
        } else {
            Ok(())
        }
    }
}

impl Drop for BlockingHandTracker {
    fn drop(&mut self) {
        if self.handler_id.is_some() { 
            if let Err(e) = self.disconnect_on_update() {
                
            }
        } else if let Some(raw_ptr) = self.callback_data_ptr.take() {
            unsafe { drop(Box::from_raw(raw_ptr)); }
        }
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_blocking_hand_tracker_trampoline(
    raw_ffi_data: SharedPtr<BridgedHandTrackerData>,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }

    // Reinterpret user_data as a mutable reference to our CallbackHolder.
    // This is safe as long as user_data actually points to a valid Box<CallbackHolder>
    // and BlockingHandTracker manages its lifetime correctly.
    let callback_holder = &mut *(user_data as *mut BlockingCallbackHolder);

    // Attempt to convert FFI data to Rust HandFrame.
    let frame_result = HandFrame::new(raw_ffi_data)
        .ok_or_else(|| NuitrackError::OperationFailed(
            "BlockingCallback: Failed to create HandFrame from FFI (BridgedHandTrackerData was null or invalid)".into()
        ));
    
    if frame_result.is_err() {
    
    }

    // Call the user's closure, catching potential panics.
    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        (callback_holder.callback)(frame_result);
    }));

    if panic_result.is_err() {
        //error!(target: "nuitrack_rs::blocking_callback", "User-provided blocking HandTracker callback panicked!");
        // Depending on library policy, you might want to consider automatically disconnecting
        // a panicking callback, but that adds complexity. For now, logging is essential.
    }
}