use tracing::instrument;
use crate::nuitrack_bridge::types::skeleton::ffi::{self as skeleton_ffi, Joint};

pub use crate::nuitrack_bridge::types::skeleton::ffi::JointType;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub user_id: i32,
    pub joints: Vec<Joint>,
}

impl Skeleton {
    /// Creates a Rust `Skeleton` from an FFI `skeleton_ffi::Skeleton` reference.
    ///
    /// This involves calling multiple FFI functions to populate the fields.
    #[instrument(level="trace", skip(ffi_skeleton))]
    pub(crate) fn from_ffi_skeleton(ffi_skeleton: &skeleton_ffi::Skeleton) -> Self {
        let user_id = skeleton_ffi::user_id(ffi_skeleton);

        let joints = skeleton_ffi::joints(ffi_skeleton).to_vec();

        Self {
            user_id,
            joints,
        }
    }
}