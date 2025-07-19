use cxx::SharedPtr;
use crate::{nuitrack::shared_types::error::Result as NuitrackResult, nuitrack_bridge::types::rgb_frame::ffi::{self as rgb_frame_ffi}};
use tracing::{instrument, trace, trace_span};

pub use crate::nuitrack_bridge::types::rgb_frame::ffi::Color3;

pub struct RGBFrame {
    internal_ptr: SharedPtr<rgb_frame_ffi::RGBFrame>,
}

impl RGBFrame {
    /// Creates a new `RgbFrame` from a CXX `SharedPtr<rgb_frame_ffi::RGBFrame>`.
    ///
    /// Returns `None` if the provided FFI pointer is null, indicating an invalid frame.
    /// This method is `pub(crate)` as it's intended for internal use when wrapping
    /// frames received from the Nuitrack SDK via FFI.
    pub(crate) fn new(ffi_ptr: SharedPtr<rgb_frame_ffi::RGBFrame>) -> Option<Self> {
        let is_null = ffi_ptr.is_null();
        trace!(is_null, "Attempting to create new RGBFrame.");
        if is_null {
            None
        } else {
            Some(RGBFrame {
                internal_ptr: ffi_ptr,
            })
        }
    }

    /// Gets the number of rows (height) of the color frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of rows as an `i32`.
    #[instrument(skip(self))]
    pub fn rows(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "rgb_frame_ffi::rows").in_scope(||
            Ok(rgb_frame_ffi::rows(&self.internal_ptr)?)
        )
    }

    /// Gets the number of columns (width) of the color frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of columns as an `i32`.
    #[instrument(skip(self))]
    pub fn cols(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "rgb_frame_ffi::cols").in_scope(||
            Ok(rgb_frame_ffi::cols(&self.internal_ptr)?)
        )
    }

    /// Gets the unique ID of the color frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the frame ID as a `u64`.
    #[instrument(skip(self))]
    pub fn frame_id(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "rgb_frame_ffi::id").in_scope(||
            Ok(rgb_frame_ffi::id(&self.internal_ptr)?)
        )
    }

    /// Gets the timestamp of the color frame, typically in microseconds.
    ///
    /// The exact meaning of this value can depend on the depth provider.
    /// # Returns
    /// A `NuitrackResult` containing the timestamp as a `u64`.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "rgb_frame_ffi::timestamp").in_scope(||
            Ok(rgb_frame_ffi::timestamp(&self.internal_ptr)?)
        )
    }

    /// Gets a slice representing the pixel data of the color frame.
    ///
    /// Each element in the slice is a `Color3` struct, containing red, green, and blue components.
    /// The slice provides a direct view into the frame's data buffer.
    /// The lifetime of the returned slice is tied to the lifetime of this `RgbFrame` object.
    ///
    /// # Returns
    /// A `NuitrackResult` containing a slice of `Color3` pixels.
    #[instrument(skip(self))]
    pub fn data(&self) -> NuitrackResult<&[Color3]> {
        trace_span!("ffi", function = "rgb_frame_ffi::data").in_scope(||
            Ok(rgb_frame_ffi::data(&self.internal_ptr)?)
        )
    }

    /// Provides access to the internal `SharedPtr` to the FFI `RGBFrame` type.
    ///
    /// This is `pub(crate)` and primarily intended for internal use or advanced scenarios
    /// where direct access to the FFI pointer is necessary (e.g., passing it back to
    /// another C++ function).
    #[allow(dead_code)] // Allow dead code if not immediately used elsewhere in crate
    pub(crate) fn ffi_ptr(&self) -> &SharedPtr<rgb_frame_ffi::RGBFrame> {
        &self.internal_ptr
    }
}