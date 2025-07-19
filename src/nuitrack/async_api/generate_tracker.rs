

macro_rules! generate_async_tracker {
    // ========================================================================================
    // Rule 1: PUBLIC ENTRY POINT - CORRECTED
    // ========================================================================================
    {
        base_module_name_snake: $base_module_name_snake:ident,
        module_ffi_path: $module_ffi_path:path,
        streams: [ $($stream_tokens:tt)* ]
    } => {
        use paste::paste;
        paste! {
            generate_async_tracker! {
                @process_streams
                base_module_name_snake: $base_module_name_snake,
                module_ffi_path: $module_ffi_path,
                tracker_name: [< Async $base_module_name_snake:camel >],
                ffi_tracker_type: $module_ffi_path::[< $base_module_name_snake:camel >],
                c_void_type: $module_ffi_path::c_void,
                ffi_create_function: $module_ffi_path::[< create_ $base_module_name_snake >],
                module_creation_error_context: concat!(stringify!([< $base_module_name_snake:camel >]), " FFI create"),
                processed_streams: [],
                remaining_streams: [ $($stream_tokens)* ]
            }
        }
    };

    // ========================================================================================
    // Rule 2: RECURSIVE STEP (OVERRIDE CASE) - MOST SPECIFIC, MUST BE FIRST
    // ========================================================================================
    {
        @process_streams
        base_module_name_snake: $base_module_name_snake:ident,
        module_ffi_path: $module_ffi_path:path,
        tracker_name: $tracker_name:ident,
        ffi_tracker_type: $ffi_tracker_type:path,
        c_void_type: $c_void_type:path,
        ffi_create_function: $ffi_create_function:path,
        module_creation_error_context: $module_creation_error_context:expr,
        processed_streams: [ $($processed:tt),* ],
        remaining_streams: [
            {
                item_base_name_snake: $item_base_name_snake:ident,
                item_base_name_pascal: $item_base_name_pascal:ident,
                rust_item_type: $rust_item_type:ty,
                dispatcher_kind: { $($dispatcher_kind_token:tt)+ }
            }
            $(, $($remaining_tokens:tt)+ )?
        ]
    } => {
        paste! {
            generate_async_tracker! {
                @process_streams
                base_module_name_snake: $base_module_name_snake,
                module_ffi_path: $module_ffi_path,
                tracker_name: $tracker_name,
                ffi_tracker_type: $ffi_tracker_type,
                c_void_type: $c_void_type,
                ffi_create_function: $ffi_create_function,
                module_creation_error_context: $module_creation_error_context,
                processed_streams: [
                    $($processed,)*
                    {
                        stream_struct_name: [< $item_base_name_pascal Stream >],
                        stream_method_name: [< $item_base_name_snake s_stream >],
                        sender_type_alias: [< $item_base_name_pascal Sender >],
                        handler_id_field: [< id_for_on_ $item_base_name_snake _handler >],
                        raw_sender_field: [< raw_ $item_base_name_snake _sender >],
                        rust_item_type: $rust_item_type,
                        ffi_connect_stream_fn: $module_ffi_path::[< connect_on_ $item_base_name_snake _async >],
                        ffi_disconnect_stream_fn: $module_ffi_path::[< disconnect_on_ $item_base_name_snake >],
                        dispatcher_name: [< rust_ $base_module_name_snake _ $item_base_name_snake _dispatcher_async >],
                        dispatcher_kind: { $($dispatcher_kind_token)+ }
                    }
                ],
                remaining_streams: [ $($($remaining_tokens)+)? ]
            }
        }
    };

    // ========================================================================================
    // Rule 3: RECURSIVE STEP (DEFAULT CASE) - MORE GENERAL, MUST BE SECOND
    // ========================================================================================
    {
        @process_streams
        base_module_name_snake: $base_module_name_snake:ident,
        module_ffi_path: $module_ffi_path:path,
        tracker_name: $tracker_name:ident,
        ffi_tracker_type: $ffi_tracker_type:path,
        c_void_type: $c_void_type:path,
        ffi_create_function: $ffi_create_function:path,
        module_creation_error_context: $module_creation_error_context:expr,
        processed_streams: [ $($processed:tt),* ],
        remaining_streams: [
            {
                item_base_name_snake: $item_base_name_snake:ident,
                rust_item_type: $rust_item_type:ty,
                dispatcher_kind: { $($dispatcher_kind_token:tt)+ }
            }
            $(, $($remaining_tokens:tt)+ )?
        ]
    } => {
        paste! {
            generate_async_tracker! {
                @process_streams
                base_module_name_snake: $base_module_name_snake,
                module_ffi_path: $module_ffi_path,
                tracker_name: $tracker_name,
                ffi_tracker_type: $ffi_tracker_type,
                c_void_type: $c_void_type,
                ffi_create_function: $ffi_create_function,
                module_creation_error_context: $module_creation_error_context,
                processed_streams: [ $($processed),* ],
                remaining_streams: [
                    {
                        item_base_name_snake: $item_base_name_snake,
                        item_base_name_pascal: [< $item_base_name_snake:camel >],
                        rust_item_type: $rust_item_type,
                        dispatcher_kind: { $($dispatcher_kind_token)+ }
                    }
                    $(, $($remaining_tokens)+ )?
                ]
            }
        }
    };

    // ========================================================================================
    // Rule 4: RECURSION BASE CASE & FINAL IMPLEMENTATION
    // ========================================================================================
    {
        @process_streams
        base_module_name_snake: $base_module_name_snake:ident,
        module_ffi_path: $module_ffi_path:path,
        tracker_name: $tracker_name:ident,
        ffi_tracker_type: $ffi_tracker_type:path,
        c_void_type: $c_void_type:path,
        ffi_create_function: $ffi_create_function:path,
        module_creation_error_context: $module_creation_error_context:expr,
        processed_streams: [
            $({
                stream_struct_name: $stream_struct_name:ident,
                stream_method_name: $stream_method_name:ident,
                sender_type_alias: $sender_type_alias:ident,
                handler_id_field: $handler_id_field:ident,
                raw_sender_field: $raw_sender_field:ident,
                rust_item_type: $rust_item_type:ty,
                ffi_connect_stream_fn: $ffi_connect_stream_fn:path,
                ffi_disconnect_stream_fn: $ffi_disconnect_stream_fn:path,
                dispatcher_name: $dispatcher_name:ident,
                dispatcher_kind: { $($dispatcher_kind_token:tt)+ }
            }),*
        ],
        // Match an empty list to terminate recursion
        remaining_streams: []
    } => {
        // ... The entire implementation block is unchanged ...
        use cxx::SharedPtr;
        use futures_core::Stream;
        use futures_channel::mpsc::{unbounded, UnboundedSender, UnboundedReceiver};
        use pin_project_lite::pin_project;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
        use super::async_dispatch::run_blocking;
        use tracing::{debug, error, instrument, trace_span};

        $(
            type $sender_type_alias = UnboundedSender<NuitrackResult<$rust_item_type>>;

            pin_project! {
                pub struct $stream_struct_name {
                    #[pin]
                    rx: UnboundedReceiver<NuitrackResult<$rust_item_type>>,
                }
            }

            impl Stream for $stream_struct_name {
                type Item = NuitrackResult<$rust_item_type>;
                fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                    self.project().rx.poll_next(cx)
                }
            }
            
            generate_async_tracker!(@generate_dispatcher
                $dispatcher_name,
                $sender_type_alias,
                $rust_item_type,
                $c_void_type,
                { $($dispatcher_kind_token)+ }
            );
        )*

        pub struct $tracker_name {
            ptr: SharedPtr<$ffi_tracker_type>,
            $(
                $handler_id_field: Option<u64>,
                $raw_sender_field: Option<*mut $c_void_type>,
            )*
        }

        impl Clone for $tracker_name {
            fn clone(&self) -> Self {
                Self {
                    ptr: self.ptr.clone(),
                    $(
                        $handler_id_field: None,
                        $raw_sender_field: None,
                    )*
                }
            }
        }

        unsafe impl Send for $tracker_name {}
        unsafe impl Sync for $tracker_name {}

        impl $tracker_name {

            #[instrument]
            pub(crate) async fn new_async() -> NuitrackResult<Self> {
                let tracker_ptr = trace_span!("ffi", function=stringify!($ffi_create_function)).in_scope(|| {
                    run_blocking( || {
                        $ffi_create_function()
                            .map_err(|e| NuitrackError::ModuleCreationFailed(
                                format!("{}: {}", $module_creation_error_context, e))
                            )
                    })
                }).await?;
                Ok(Self {
                    ptr: tracker_ptr,
                    $($handler_id_field: None, $raw_sender_field: None,)*
                })
            }

            pub(crate) fn get_ffi_ptr_clone(&self) -> SharedPtr<$ffi_tracker_type> {
                self.ptr.clone()
            }

            $(
                #[instrument(skip(self), name = "get_stream")]
                pub fn $stream_method_name(&mut self) -> NuitrackResult<$stream_struct_name> {
                    if self.$handler_id_field.is_some() {
                        return Err(NuitrackError::OperationFailed(
                            format!("Stream {} already initialized for {}.", stringify!($stream_struct_name), stringify!($tracker_name))
                        ));
                    }
                    let (tx, rx) = unbounded::<NuitrackResult<$rust_item_type>>();
                    
                    let sender_boxed = Box::new(tx);
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

        impl Drop for $tracker_name {
            fn drop(&mut self) {
                debug!(tracker = stringify!($tracker_name), "Dropping tracker and disconnecting streams.");
                $(
                    if let Some(handler_id) = self.$handler_id_field.take() {
                        if let Err(e) = $ffi_disconnect_stream_fn(&self.ptr, handler_id) {
                            error!(
                                tracker = stringify!([< Async $base_module_name_snake:camel >]),
                                ffi_fn = stringify!($ffi_disconnect_stream_fn),
                                error = %e,
                                "Error in FFI disconnect during Drop"
                            );
                        }
                    }
                    if let Some(raw_ptr) = self.$raw_sender_field.take() {
                        unsafe { let _ = Box::from_raw(raw_ptr as *mut $sender_type_alias); };
                    }
                )*
            }
        }
    };

    // ========================================================================================
    // Rule 5 & 6: DISPATCHER GENERATION (Unchanged)
    // ========================================================================================
    (@generate_dispatcher $dispatcher_name:ident, $sender_type_alias:ident, $rust_item_type:ty, $c_void_type:ty,
        { FFIDataConversion { ffi_arg_name: $ffi_arg_name:ident, ffi_arg_type: $ffi_arg_type:ty, conversion_logic: $conversion_logic:expr $(,)? }}
    ) => {
        #[instrument(name="ffi_callback", skip_all, fields(dispatcher.name = stringify!($dispatcher_name)))]
        #[unsafe(no_mangle)]
        pub extern "C" fn $dispatcher_name($ffi_arg_name: &$ffi_arg_type, raw_sender_ptr: *mut $c_void_type,) {
            if raw_sender_ptr.is_null() { 
                error!(dispatcher = stringify!($dispatcher_name), "raw_sender_ptr argument is null.");
                return; 
            }
            let tx = unsafe { &*(raw_sender_ptr as *const $sender_type_alias) };
            let conversion_closure = $conversion_logic;
            let result_to_send = match conversion_closure($ffi_arg_name) {
                Some(converted_item) => Ok(converted_item),
                None => Err(NuitrackError::OperationFailed(concat!("FFI data for ", stringify!($sender_type_alias), " was null or invalid").to_string())),
            };
            if tx.unbounded_send(result_to_send).is_err() { 
                debug!(dispatcher = stringify!($dispatcher_name), "Stream receiver dropped.");
            }
        }
    };
    (@generate_dispatcher $dispatcher_name:ident, $sender_type_alias:ident, $rust_item_type:ty, $c_void_type:ty,
        { DirectItem { ffi_item_arg_name: $ffi_item_arg_name:ident, ffi_item_arg_type: $ffi_item_arg_type:ty $(,)? }}
    ) => {
        #[instrument(name="ffi_callback", skip_all, fields(dispatcher.name = stringify!($dispatcher_name), item_id = ?$ffi_item_arg_name))]
        #[unsafe(no_mangle)]
        pub extern "C" fn $dispatcher_name($ffi_item_arg_name: $ffi_item_arg_type, raw_sender_ptr: *mut $c_void_type,) {
            if raw_sender_ptr.is_null() { 
                error!(dispatcher = stringify!($dispatcher_name), "raw_sender_ptr argument is null."); 
                return; 
            }
            let tx = unsafe { &*(raw_sender_ptr as *const $sender_type_alias) };
            if tx.unbounded_send(Ok($ffi_item_arg_name as $rust_item_type)).is_err() { 
                debug!(dispatcher = stringify!($dispatcher_name), "Stream receiver dropped.");
            }
        }
    };
}

