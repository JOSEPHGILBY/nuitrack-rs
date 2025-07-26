use std::sync::OnceLock;
use tracing::{debug, instrument, trace, trace_span, warn};
use cxx::SharedPtr;

use crate::nuitrack_bridge::modules::user_tracker::ffi::UserFrame as FFIUserFrame;
use crate::nuitrack_bridge::types::user_frame::ffi::{self as user_frame_ffi};
use crate::nuitrack_bridge::types::vector3::ffi::Vector3;

use super::error::{NuitrackError, Result as NuitrackResult};
use super::user::User;

/// A high-level wrapper for a Nuitrack user frame.
///
/// A `UserFrame` provides a snapshot of all detected users at a specific moment.
/// It contains a user segmentation map, a list of detailed user objects,
/// and information about the detected floor plane.
pub struct UserFrame {
    internal_ptr: SharedPtr<FFIUserFrame>,
    users_cache: OnceLock<NuitrackResult<Vec<User>>>,
}

impl UserFrame {
    /// Creates a new `UserFrame` from a CXX shared pointer. Returns `None` if the pointer is null.
    pub(crate) fn new(ffi_ptr: SharedPtr<FFIUserFrame>) -> Option<Self> {
        if ffi_ptr.is_null() {
            trace!("Attempted to create UserFrame from a null pointer.");
            None
        } else {
            Some(UserFrame {
                internal_ptr: ffi_ptr,
                users_cache: OnceLock::new(),
            })
        }
    }

    /// Gets a list of all detected users in the frame.
    ///
    /// This operation is cached after the first call for this `UserFrame` instance
    /// to improve performance.
    #[instrument(skip(self))]
    pub fn users(&self) -> NuitrackResult<&[User]> {
        let cached_result = self.users_cache.get_or_init(|| {
            debug!("Populating UserFrame cache: fetching users from FFI.");
            let conversion = || -> NuitrackResult<Vec<User>> {
                let ffi_users_ptr = trace_span!("ffi", function = "user_frame_ffi::users")
                    .in_scope(|| user_frame_ffi::users(&self.internal_ptr))?;
                
                let ffi_vec = ffi_users_ptr.as_ref().ok_or_else(|| {
                    warn!("FFI returned null user vector pointer.");
                    NuitrackError::OperationFailed("FFI returned null user vector".into())
                })?;

                trace!(count = ffi_vec.len(), "Performing deep copy of Users.");
                let mut rust_users = Vec::with_capacity(ffi_vec.len());
                for ffi_s in ffi_vec {
                    rust_users.push(ffi_s.clone());
                }
                Ok(rust_users)
            };
            conversion()
        });

        match cached_result {
            Ok(users_vec) => {
                trace!("Returning Users from cache.");
                Ok(users_vec.as_slice())
            }
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for users access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            }
        }
    }

    /// Gets the user segmentation map as a flat slice of user IDs.
    ///
    /// The map's dimensions are `rows` x `cols`. A pixel value of 0 is the background.
    #[instrument(skip(self))]
    pub fn data(&self) -> NuitrackResult<&[u16]> {
        trace_span!("ffi", function = "user_frame_ffi::data").in_scope(||
            Ok(user_frame_ffi::data(&self.internal_ptr)?)
        )
    }

    /// Gets the number of rows in the user map.
    #[instrument(skip(self))]
    pub fn rows(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "user_frame_ffi::rows").in_scope(||
            Ok(user_frame_ffi::rows(&self.internal_ptr)?)
        )
    }

    /// Gets the number of columns in the user map.
    #[instrument(skip(self))]
    pub fn cols(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "user_frame_ffi::cols").in_scope(||
            Ok(user_frame_ffi::cols(&self.internal_ptr)?)
        )
    }

    /// Gets the timestamp of the frame in microseconds.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "user_frame_ffi::timestamp").in_scope(||
            Ok(user_frame_ffi::timestamp(&self.internal_ptr)?)
        )
    }

    /// Gets a point on the detected floor plane.
    #[instrument(skip(self))]
    pub fn floor(&self) -> NuitrackResult<Vector3> {
        trace_span!("ffi", function = "user_frame_ffi::floor").in_scope(||
            Ok(user_frame_ffi::floor(&self.internal_ptr)?)
        )
    }

    /// Gets the normal vector of the detected floor plane.
    #[instrument(skip(self))]
    pub fn floor_normal(&self) -> NuitrackResult<Vector3> {
        trace_span!("ffi", function = "user_frame_ffi::floor_normal").in_scope(||
            Ok(user_frame_ffi::floor_normal(&self.internal_ptr)?)
        )
    }
}