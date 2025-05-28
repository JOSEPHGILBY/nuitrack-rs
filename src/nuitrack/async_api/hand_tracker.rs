use cxx::SharedPtr;
use futures_core::Stream;
use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::nuitrack_bridge::hand_tracker::ffi::{self as ht_ffi, c_void};
use crate::nuitrack_bridge::hand_tracker::ffi::BridgedHandTrackerData; // For the C++ struct
use crate::nuitrack_bridge::hand_tracker::ffi::HandTracker as FFIHandTracker; // Alias for the C++ type
use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
use crate::nuitrack::shared_types::hand_frame::HandFrame; // Your existing Rust representation
use super::async_dispatch::run_blocking;


// Type alias for the MPSC sender we'll use
type FrameSender = UnboundedSender<NuitrackResult<HandFrame>>;

// Wrapper for the sender to be passed as user_data
struct SenderUserData {
    tx: FrameSender,
}

pub struct AsyncHandTracker {
    ptr: SharedPtr<FFIHandTracker>,
    handler_id: Option<u64>,
    // We need to keep the sender alive for the C++ callback to use.
    // Box it to get a stable pointer, then cast to void* for C++.
    // When AsyncHandTracker is dropped, we need to reclaim this Box.
    // The Option is so we can take it in drop.
    cb_user_data_ptr: Option<*mut c_void>, // Stores the raw pointer to Box<SenderUserData>
    // The receiver is moved into the HandFrameStream
}

unsafe impl Send for AsyncHandTracker {}
unsafe impl Sync for AsyncHandTracker {}

impl AsyncHandTracker {
    pub(crate) async fn new_async() -> NuitrackResult<Self> {
        let tracker_ptr = run_blocking(|| {
            ht_ffi::hand_tracker_create()
                .map_err(|e| NuitrackError::ModuleCreationFailed(format!("HandTracker FFI create: {}", e)))
        }).await?;
        Ok(AsyncHandTracker {
            ptr: tracker_ptr,
            handler_id: None,
            cb_user_data_ptr: None,
        })
    }

    pub async fn wait_for_its_update(&self) -> NuitrackResult<()> {
        let ptr_clone = self.ptr.clone(); // self.ptr is SharedPtr<FFIHandTracker>
        run_blocking(move || {
            // Ensure this FFI function is correctly exposed in your core.rs bridge
            // It takes &SharedPtr<tdv::nuitrack::HandTracker>
            crate::nuitrack_bridge::core::ffi::wait_update_hand_tracker(&ptr_clone)
                .map_err(|e| NuitrackError::OperationFailed(format!("FFI wait_update_hand_tracker: {}", e)))
        }).await
    }

    pub fn hand_frames_stream(&mut self) -> NuitrackResult<HandFrameStream> {
        if self.handler_id.is_some() {
            return Err(NuitrackError::OperationFailed("Stream already initialized for this HandTracker".into()));
        }

        let (tx, rx) = unbounded::<NuitrackResult<HandFrame>>();
        
        // Box the sender wrapper to get a stable memory address.
        let sender_user_data = Box::new(SenderUserData { tx });
        // Cast the Box to a raw pointer to pass to C++.
        let user_data_raw_ptr = Box::into_raw(sender_user_data) as *mut c_void;

        self.cb_user_data_ptr = Some(user_data_raw_ptr);

        // This FFI call is blocking and registers the callback.
        // While the callback registration itself might be quick, consistency suggests wrapping.
        // However, connectOnUpdate is often not long-blocking. If it's known to be quick,
        // direct call might be acceptable. Let's be safe.
        // For now, assuming connect_on_update_with_user_data is not heavily blocking and can be called directly.
        // If it can block significantly, it should also be wrapped with `run_blocking`.
        // Let's call it directly here as it primarily sets up a callback.
        let handler_id = unsafe {
            ht_ffi::connect_on_update_with_user_data(&self.ptr, user_data_raw_ptr)
        }.map_err(|e| NuitrackError::OperationFailed(format!("FFI connect_on_update_with_user_data: {}", e)))?;
        
        self.handler_id = Some(handler_id);

        Ok(HandFrameStream { rx })
    }

    // Async method to disconnect, preferred if disconnect can block.
    pub async fn disconnect_async(mut self) -> NuitrackResult<()> {
        if let Some(handler_id) = self.handler_id.take() {
            let ptr_clone = self.ptr.clone(); // Clone for the blocking call
            run_blocking(move || {
                ht_ffi::disconnect_on_update(&ptr_clone, handler_id)
                    .map_err(|e| NuitrackError::OperationFailed(format!("FFI disconnect_on_update: {}", e)))
            }).await?;
        }
        // Reclaim the Box<SenderUserData> if it exists
        if let Some(raw_ptr) = self.cb_user_data_ptr.take() {
            unsafe { Box::from_raw(raw_ptr as *mut SenderUserData) };
        }
        Ok(())
    }

    pub(crate) fn get_ffi_ptr_clone(&self) -> SharedPtr<FFIHandTracker> {
        self.ptr.clone()
    }
}

// Implement Drop to ensure disconnection and reclamation of sender data.
// This Drop will be synchronou s.
impl Drop for AsyncHandTracker {
    fn drop(&mut self) {
        if let Some(handler_id) = self.handler_id.take() {
            // This is a blocking call in a synchronous Drop.
            // For truly non-blocking drop from async, this would need an async_close or spawn_blocking.
            // println!("AsyncHandTracker: Disconnecting (can block in drop)...");
            if let Err(e) = ht_ffi::disconnect_on_update(&self.ptr, handler_id) {
                eprintln!("AsyncHandTracker: Error in disconnect_on_update during Drop: {}", e);
            }
        }
        // Reclaim the Box<SenderUserData> if it exists
        if let Some(raw_ptr) = self.cb_user_data_ptr.take() {
            unsafe { Box::from_raw(raw_ptr as *mut SenderUserData) };
            // println!("AsyncHandTracker: Reclaimed SenderUserData in Drop.");
        }
    }
}


// The Stream implementation
pin_project! {
    pub struct HandFrameStream {
        #[pin]
        rx: UnboundedReceiver<NuitrackResult<HandFrame>>,
    }
}

impl Stream for HandFrameStream {
    type Item = NuitrackResult<HandFrame>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().rx.poll_next(cx)
    }
}

// This is your FFI callback dispatcher, ensure it's correctly defined and exported.
// Make sure it's in a place where cxx build system picks it up as an extern "C" function.
// e.g., in src/nuitrack_bridge/hand_tracker.rs or a dedicated ffi_glue.rs
#[unsafe(no_mangle)]
pub extern "C" fn rust_hand_tracker_callback_dispatcher(
    data: SharedPtr<BridgedHandTrackerData>,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        eprintln!("Nuitrack HandTracker Callback: user_data is null.");
        return;
    }

    // Reconstruct the reference to SenderUserData. This is safe if user_data is indeed
    // the Box::into_raw pointer we created and it hasn't been dropped.
    let sender_user_data = unsafe { &*(user_data as *const SenderUserData) };
    let tx = &sender_user_data.tx;

    // Attempt to convert FFI data to Rust HandFrameData
    // Your HandFrameData::new might need to handle potential errors from C++ getters
    match HandFrame::new(data) { // Assuming HandFrameData::new returns Option or Result
        Some(frame_data) => {
            if let Err(e) = tx.unbounded_send(Ok(frame_data)) {
                // eprintln!("Nuitrack HandTracker Callback: Failed to send frame data: {}", e);
                // This error typically means the receiver has been dropped.
            }
        }
        None => { // Or if HandFrameData::new returns Result, handle Err case
            if let Err(e) = tx.unbounded_send(Err(NuitrackError::OperationFailed("Failed to process HandTrackerData from FFI".into()))) {
                 // eprintln!("Nuitrack HandTracker Callback: Failed to send error data: {}", e);
            }
        }
    }
}