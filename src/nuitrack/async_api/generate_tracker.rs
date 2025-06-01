
macro_rules! generate_async_tracker {
    (
        // Meta-info for the main tracker struct
        tracker_name: $tracker_name:ident,
        ffi_tracker_type: $ffi_tracker_type:path, // e.g., crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonTracker
        c_void_type: $c_void_type:path,         // e.g., crate::nuitrack_bridge::modules::skeleton_tracker::ffi::c_void
        
        // Info for the new_async constructor
        ffi_create_function: $ffi_create_function:path,
        module_creation_error_context: $module_creation_error_context:expr,

        // Array of stream configurations
        streams: [
            $( // Repeat for each stream configuration provided
                {
                    // --- Names specific to this stream ---
                    // Rust struct name for the stream itself (e.g., SkeletonFrameStream)
                    stream_struct_name: $stream_struct_name:ident, 
                    // Public Rust method name on the tracker to get this stream (e.g., skeleton_frames_stream)
                    stream_method_name: $stream_method_name:ident, 
                    // Internal type alias name for this stream's MPSC sender (e.g., SkeletonFrameSender)
                    sender_type_alias: $sender_type_alias:ident,     
                    // Field name in tracker struct for the callback handler ID (e.g., on_update_handler_id)
                    handler_id_field: $handler_id_field:ident,     
                    // Field name in tracker struct for the raw sender pointer (e.g., raw_skeleton_frame_sender)
                    raw_sender_field: $raw_sender_field:ident,     

                    // --- Types for this stream ---
                    // The final Rust item type yielded by the stream (e.g., crate::nuitrack::shared_types::skeleton_frame::SkeletonFrame)
                    rust_item_type: $rust_item_type:ty,            

                    // --- FFI connect/disconnect functions for this stream's callback ---
                    ffi_connect_stream_fn: $ffi_connect_stream_fn:path,
                    ffi_disconnect_stream_fn: $ffi_disconnect_stream_fn:path,

                    // --- Dispatcher function (extern "C" Rust function called by C++) ---
                    dispatcher_name: $dispatcher_name:ident,
                    // Dispatcher kind and its specific arguments
                    // This uses a tt muncher or specific @rule to parse one of two structures:
                    // dispatcher_kind: FfiData { ffi_arg_name: data, ffi_arg_type: cxx::SharedPtr<...>, user_data_arg: sender, conversion: |d| Struct::new(d), err_msg: "..." }
                    // dispatcher_kind: DirectItem { item_arg_name: id, item_arg_type: i32, user_data_arg: sender }
                    dispatcher_kind: { $($dispatcher_kind_token:tt)+ }
                }
            ),* $(,)? // Allow trailing comma
        ]
    ) => {
        // Use fully qualified paths for external crates inside the macro expansion
        use cxx::SharedPtr; // Assuming this is generally needed for ffi_tracker_type
        use futures_core::Stream;
        use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
        use pin_project_lite::pin_project;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
        use super::async_dispatch::run_blocking;

        // Part 1: Generate Sender Aliases, Stream Structs, Impl Stream, Dispatchers for EACH stream
        $(
            // Type alias for this specific stream's sender
            type $sender_type_alias = UnboundedSender<NuitrackResult<$rust_item_type>>;

            //The Stream struct definition
            pin_project! {
                pub struct $stream_struct_name {
                    #[pin]
                    rx: UnboundedReceiver<NuitrackResult<$rust_item_type>>,
                }
            }

            //The impl Stream for the struct
            impl Stream for $stream_struct_name {
                type Item = NuitrackResult<$rust_item_type>;

                fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                    self.project().rx.poll_next(cx)
                }
            }
            
            // Call internal macro rule to generate the specific dispatcher based on dispatcher_kind
            generate_async_tracker!(@generate_dispatcher
                $dispatcher_name,
                $sender_type_alias,
                $rust_item_type,
                $c_void_type, // Path to the FFI c_void (e.g., some_ffi_module::c_void or std::ffi::c_void if using that)
                { $($dispatcher_kind_token)+ } // Pass the dispatcher_kind block
            );
        )*

        // Part 2: Generate the main Async Tracker Struct
        pub struct $tracker_name {
            ptr: SharedPtr<$ffi_tracker_type>,
            $(
                $handler_id_field: Option<u64>,
                $raw_sender_field: Option<*mut $c_void_type>,
            )*
        }

        unsafe impl Send for $tracker_name {}
        unsafe impl Sync for $tracker_name {}

        // Part 3: Implement core methods for the Tracker Struct
        impl $tracker_name {
            pub(crate) async fn new_async() -> NuitrackResult<Self> {
                let tracker_ptr = run_blocking( || {
                    $ffi_create_function()
                        .map_err(|e| NuitrackError::ModuleCreationFailed(
                            format!("{}: {}", $module_creation_error_context, e))
                        )
                }).await?;

                Ok(Self {
                    ptr: tracker_ptr,
                    $(
                        $handler_id_field: None,
                        $raw_sender_field: None,
                    )*
                })
            }

            pub(crate) fn get_ffi_ptr_clone(&self) -> SharedPtr<$ffi_tracker_type> {
                self.ptr.clone()
            }

            // Generate stream-creating methods
            $(
                pub fn $stream_method_name(&mut self) -> NuitrackResult<$stream_struct_name> {
                    if self.$handler_id_field.is_some() {
                        return Err(NuitrackError::OperationFailed(
                            format!("Stream {} already initialized for {}.", stringify!($stream_struct_name), stringify!($tracker_name))
                        ));
                    }
                    let (tx, rx) = unbounded::<NuitrackResult<$rust_item_type>>();
                    
                    let sender_boxed = Box::new(tx); // tx is of type $sender_type_alias
                    let sender_raw_ptr = Box::into_raw(sender_boxed) as *mut $c_void_type;
                    self.$raw_sender_field = Some(sender_raw_ptr);

                    let handler_id = unsafe {
                        $ffi_connect_stream_fn(&self.ptr, sender_raw_ptr)
                    }.map_err(|e| NuitrackError::OperationFailed(
                        format!("FFI connect call {} failed: {}", stringify!($ffi_connect_stream_fn), e)
                    ))?;
                    self.$handler_id_field = Some(handler_id);
                    Ok($stream_struct_name { rx })
                }
            )*
        }

        // // Part 4: Implement Drop for the Tracker Struct
        impl Drop for $tracker_name {
            fn drop(&mut self) {
                $(
                    if let Some(handler_id) = self.$handler_id_field.take() {
                        // Note: FFI disconnect functions might not be fallible in the Result<...> sense from Rust,
                        // but C++ exceptions could occur. If they return Result<(), cxx::Exception>, this is fine.
                        // If they are void and can't fail FFI-wise, the unwrap_or_else is for logging.
                        if let Err(e) = $ffi_disconnect_stream_fn(&self.ptr, handler_id) {
                            eprintln!(
                                "[{}] Error in FFI disconnect {} during Drop: {}",
                                stringify!($tracker_name),
                                stringify!($ffi_disconnect_stream_fn),
                                e
                            );
                        }
                    }
                    if let Some(raw_ptr) = self.$raw_sender_field.take() {
                        unsafe { Box::from_raw(raw_ptr as *mut $sender_type_alias) };
                    }
                )*
            }
        }
    };

    // --- Internal Rule for FFIDataConversion Dispatcher ---
    (@generate_dispatcher
        $dispatcher_name:ident,
        $sender_type_alias:ident,
        $rust_item_type:ty,
        $c_void_type:ty,
        { FfiDataConversion { // Matcher for this dispatcher kind
            ffi_arg_name: $ffi_arg_name:ident,
            ffi_arg_type: $ffi_arg_type:ty,
            user_data_arg_name: $user_data_arg_name:ident,
            conversion_logic: $conversion_logic:expr, // Expects a closure: |T| -> Option<U> or Result<U,E>
            conversion_error_msg: $conversion_error_msg:expr $(,)?
        }}
    ) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $dispatcher_name(
            $ffi_arg_name: &$ffi_arg_type,
            $user_data_arg_name: *mut $c_void_type,
        ) {
            if $user_data_arg_name.is_null() {
                eprintln!(concat!(stringify!($dispatcher_name), ": user_data argument '", stringify!($user_data_arg_name), "' is null."));
                return;
            }
            let tx = unsafe { &*($user_data_arg_name as *const $sender_type_alias) };
            
            let conversion_closure = $conversion_logic;
            // Assuming the closure returns Option<ItemType> as per prior examples
            let result_to_send = match conversion_closure($ffi_arg_name) {
                Some(converted_item) => Ok(converted_item),
                None => Err(NuitrackError::OperationFailed(
                    $conversion_error_msg.to_string()
                )),
            };
            // If your conversion_logic returns Result<ItemType, YourErrorType>, you'd adjust:
            // let result_to_send = conversion_closure(data_arg).map_err(|conv_err| ... map to NuitrackError ...);

            if tx.unbounded_send(result_to_send).is_err() {
                // Optional: eprintln for receiver dropped
            }
        }
    };

    // --- Internal Rule for DirectItem Dispatcher ---
    (@generate_dispatcher
        $dispatcher_name:ident,
        $sender_type_alias:ident,
        $rust_item_type:ty,
        $c_void_type:ty,
        { DirectItem { // Matcher for this dispatcher kind
            ffi_item_arg_name: $ffi_item_arg_name:ident,
            ffi_item_arg_type: $ffi_item_arg_type:ty,
            user_data_arg_name: $user_data_arg_name:ident $(,)?
        }}
    ) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $dispatcher_name(
            $ffi_item_arg_name: $ffi_item_arg_type,
            $user_data_arg_name: *mut $c_void_type,
        ) {
            if $user_data_arg_name.is_null() {
                eprintln!(concat!(stringify!($dispatcher_name), ": user_data argument '", stringify!($user_data_arg_name), "' is null."));
                return;
            }
            let tx = unsafe { &*($user_data_arg_name as *const $sender_type_alias) };
            // Assuming $ffi_item_arg_type can be cast or is identical to $rust_item_type.
            // For i32 -> i32, this is fine.
            if tx.unbounded_send(Ok($ffi_item_arg_name as $rust_item_type)).is_err() {
                // Optional: eprintln for receiver dropped
            }
        }
    };
}
