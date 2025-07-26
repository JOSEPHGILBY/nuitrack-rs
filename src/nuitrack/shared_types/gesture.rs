use tracing::instrument;
use crate::nuitrack_bridge::types::gesture::ffi as gesture_ffi;

pub use crate::nuitrack_bridge::types::gesture::ffi::{
    Gesture, GestureState, GestureType, UserState, UserStateType,
};

#[derive(Debug, Clone)]
pub struct UserGestures {
    pub user_id: i32,
    pub user_state: UserStateType,
    pub gestures: Vec<GestureState>,
}

impl UserGestures {
    /// Creates a Rust `UserGestures` from an FFI reference.
    #[instrument(level="trace", skip(ffi_user_gestures))]
    pub(crate) fn from_ffi(ffi_user_gestures: &gesture_ffi::UserGesturesState) -> Self {
        let user_id = gesture_ffi::user_id(ffi_user_gestures);
        let user_state = gesture_ffi::user_state(ffi_user_gestures);
        let gestures = gesture_ffi::gestures(ffi_user_gestures).to_vec();

        Self { user_id, user_state, gestures }
    }
}