use cxx::SharedPtr;
use crate::{
    nuitrack::shared_types::error::Result as NuitrackResult,
    nuitrack_bridge::types::depth_frame::ffi as depth_frame_ffi,
};
use tracing::{instrument, trace, trace_span};

pub struct DepthFrame {
    internal_ptr: SharedPtr<depth_frame_ffi::DepthFrame>,
}

impl DepthFrame {
    /// Creates a new `DepthFrame` from a CXX `SharedPtr<depth_frame_ffi::DepthFrame>`.
    ///
    /// Returns `None` if the provided FFI pointer is null, indicating an invalid frame.
    /// This method is `pub(crate)` as it's intended for internal use when wrapping
    /// frames received from the Nuitrack SDK via FFI.
    pub(crate) fn new(ffi_ptr: SharedPtr<depth_frame_ffi::DepthFrame>) -> Option<Self> {
        let is_null = ffi_ptr.is_null();
        trace!(is_null, "Attempting to create new DepthFrame.");
        if is_null {
            None
        } else {
            Some(DepthFrame {
                internal_ptr: ffi_ptr,
            })
        }
    }

    /// Gets the number of rows (height) of the depth frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of rows as an `i32`.
    #[instrument(skip(self))]
    pub fn rows(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "depth_frame_ffi::rows")
            .in_scope(|| Ok(depth_frame_ffi::rows(&self.internal_ptr)?))
    }

    /// Gets the number of columns (width) of the depth frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of columns as an `i32`.
    #[instrument(skip(self))]
    pub fn cols(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "depth_frame_ffi::cols")
            .in_scope(|| Ok(depth_frame_ffi::cols(&self.internal_ptr)?))
    }

    /// Gets the unique ID of the depth frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the frame ID as a `u64`.
    #[instrument(skip(self))]
    pub fn frame_id(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "depth_frame_ffi::id")
            .in_scope(|| Ok(depth_frame_ffi::id(&self.internal_ptr)?))
    }

    /// Gets the timestamp of the depth frame, typically in microseconds.
    ///
    /// The exact meaning of this value can depend on the depth provider.
    /// # Returns
    /// A `NuitrackResult` containing the timestamp as a `u64`.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "depth_frame_ffi::timestamp")
            .in_scope(|| Ok(depth_frame_ffi::timestamp(&self.internal_ptr)?))
    }

    /// Gets a slice representing the pixel data of the depth frame.
    ///
    /// Each element in the slice is a `u16` representing the depth value (distance) at that pixel.
    /// The slice provides a direct view into the frame's data buffer.
    /// The lifetime of the returned slice is tied to the lifetime of this `DepthFrame` object.
    ///
    /// # Returns
    /// A `NuitrackResult` containing a slice of `u16` depth values.
    #[instrument(skip(self))]
    pub fn data(&self) -> NuitrackResult<&[u16]> {
        trace_span!("ffi", function = "depth_frame_ffi::data")
            .in_scope(|| Ok(depth_frame_ffi::data(&self.internal_ptr)?))
    }

    /// Provides access to the internal `SharedPtr` to the FFI `DepthFrame` type.
    ///
    /// This is `pub(crate)` and primarily intended for internal use or advanced scenarios
    /// where direct access to the FFI pointer is necessary.
    #[allow(dead_code)]
    pub(crate) fn ffi_ptr(&self) -> &SharedPtr<depth_frame_ffi::DepthFrame> {
        &self.internal_ptr
    }
}