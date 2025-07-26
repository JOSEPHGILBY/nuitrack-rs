use std::sync::OnceLock;
use tracing::{debug, instrument, trace, trace_span, warn};
use cxx::SharedPtr;

use crate::nuitrack::shared_types::error::NuitrackError;
use crate::nuitrack::shared_types::gesture::{UserGestures, UserState};
use crate::nuitrack_bridge::types::gesture_data::ffi::{self as gesture_data_ffi, GestureData as FFIGestureData, UserStateData as FFIUserStateData, UserGesturesStateData as FFIUserGesturesStateData};
use super::error::Result as NuitrackResult;
use super::gesture::Gesture;

/// A high-level wrapper for a Nuitrack gesture frame.
///
/// This frame contains a list of gestures that have been completed at a specific moment.
pub struct GestureFrame {
    internal_ptr: SharedPtr<FFIGestureData>,
    gestures_cache: OnceLock<NuitrackResult<Vec<Gesture>>>,
}

impl GestureFrame {
    /// Creates a new `GestureFrame` from a CXX shared pointer. Returns `None` if the pointer is null.
    pub(crate) fn new(ffi_ptr: SharedPtr<FFIGestureData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            trace!("Attempted to create GestureFrame from a null pointer.");
            None
        } else {
            Some(GestureFrame {
                internal_ptr: ffi_ptr,
                gestures_cache: OnceLock::new(),
            })
        }
    }

    /// Gets the list of completed gestures in this frame.
    ///
    /// This operation is cached after the first call.
    #[instrument(skip(self))]
    pub fn gestures(&self) -> NuitrackResult<&[Gesture]> {
        let cached_result = self.gestures_cache.get_or_init(|| {
            debug!("Populating GestureFrame cache: converting FFI Gestures to Rust.");
            Ok(trace_span!("ffi", function = "gesture_data_ffi::gesture_data_gestures")
                .in_scope(|| gesture_data_ffi::gesture_data_gestures(&self.internal_ptr))?
                .to_vec())
        });

        match cached_result {
            Ok(gestures_vec) => {
                trace!("Returning Gestures from cache.");
                Ok(gestures_vec.as_slice())
            },
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for gestures access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            },
        }
    }

    /// Gets the number of completed gestures in this frame.
    #[instrument(skip(self))]
    pub fn num_gestures(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "gesture_data_ffi::gesture_data_num_gestures").in_scope(|| 
            Ok(gesture_data_ffi::gesture_data_num_gestures(&self.internal_ptr)?)
        )
    }
    
    /// Gets the timestamp of the frame in microseconds.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "gesture_data_ffi::gesture_data_timestamp")
            .in_scope(|| Ok(gesture_data_ffi::gesture_data_timestamp(&self.internal_ptr)?))
    }
}


/// A high-level wrapper for a Nuitrack user state frame.
///
/// This frame contains updates about users changing their state (e.g., appearing, becoming active).
pub struct UserStateFrame {
    internal_ptr: SharedPtr<FFIUserStateData>,
    user_states_cache: OnceLock<NuitrackResult<Vec<UserState>>>,
}

impl UserStateFrame {
    pub(crate) fn new(ffi_ptr: SharedPtr<FFIUserStateData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            trace!("Attempted to create UserStateFrame from a null pointer.");
            None
        } else {
            Some(UserStateFrame {
                internal_ptr: ffi_ptr,
                user_states_cache: OnceLock::new(),
            })
        }
    }

    /// Gets the list of user state changes in this frame.
    #[instrument(skip(self))]
    pub fn user_states(&self) -> NuitrackResult<&[UserState]> {
        let cached_result = self.user_states_cache.get_or_init(|| {
            debug!("Populating UserStateFrame cache: converting FFI UserStates to Rust.");
            Ok(trace_span!("ffi", function = "gesture_data_ffi::user_state_data_user_states")
                .in_scope(|| gesture_data_ffi::user_state_data_user_states(&self.internal_ptr))?
                .to_vec())
        });

        match cached_result {
            Ok(user_state_vec) => {
                trace!("Returning UserState from cache.");
                Ok(user_state_vec.as_slice())
            },
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for user states access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            },
        }
    }

    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "gesture_data_ffi::user_state_data_timestamp")
            .in_scope(|| Ok(gesture_data_ffi::user_state_data_timestamp(&self.internal_ptr)?))
    }
}


/// A frame containing the gesture progress state for all tracked users.
pub struct UserGesturesFrame {
    internal_ptr: SharedPtr<FFIUserGesturesStateData>,
    users_cache: OnceLock<NuitrackResult<Vec<UserGestures>>>,
}

impl UserGesturesFrame {
    pub(crate) fn new(ffi_ptr: SharedPtr<FFIUserGesturesStateData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            trace!("Attempted to create UserGesturesFrame from a null pointer.");
            None
        } else {
            Some(UserGesturesFrame {
                internal_ptr: ffi_ptr,
                users_cache: OnceLock::new(),
            })
        }
    }

    /// Gets the list of user gesture states in this frame.
    #[instrument(skip(self))]
    pub fn users(&self) -> NuitrackResult<&[UserGestures]> {
        let cached_result = self.users_cache.get_or_init(|| {
            debug!("Populating UserGesturesFrame cache: fetching user gestures from FFI.");
            let conversion = || -> NuitrackResult<Vec<UserGestures>> {
                let ffi_users_ptr = trace_span!("ffi", function = "gesture_data_ffi::user_gestures_state_data_user_gestures_states")
                    .in_scope(|| gesture_data_ffi::user_gestures_state_data_user_gestures_states(&self.internal_ptr))?;
                
                let ffi_vec = ffi_users_ptr.as_ref().ok_or_else(|| {
                    warn!("FFI returned null user gestures vector pointer.");
                    NuitrackError::OperationFailed("FFI returned null user gestures vector".into())
                })?;

                trace!(count = ffi_vec.len(), "Performing deep copy of UserGestures.");
                let mut rust_users = Vec::with_capacity(ffi_vec.len());
                for ffi_s in ffi_vec {
                    rust_users.push(UserGestures::from_ffi(ffi_s));
                }
                Ok(rust_users)
            };
            conversion()
        });

        match cached_result {
            Ok(user_gestures_vec) => {
                trace!("Returning UserGestures from cache.");
                Ok(user_gestures_vec.as_slice())
            },
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for user gestures access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            },
        }
    }

    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "gesture_data_ffi::user_gestures_state_data_timestamp")
            .in_scope(|| Ok(gesture_data_ffi::user_gestures_state_data_timestamp(&self.internal_ptr)?))
    }
}