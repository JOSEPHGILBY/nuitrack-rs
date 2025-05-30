use crate::nuitrack_bridge::types::skeleton::ffi as skeleton_ffi;
use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};
use cxx::{UniquePtr, CxxVector};

#[derive(Debug, Clone)]
pub struct Joint {
    pub joint_type: skeleton_ffi::JointType,
    pub confidence: f32,
    pub real_x: f32,
    pub real_y: f32,
    pub real_z: f32,
    pub proj_x: f32,
    pub proj_y: f32,
    pub proj_z: f32,
    /// Orientation as a flat 3x3 matrix (row-major: [m00, m01, m02, m10, m11, m12, m20, m21, m22])
    pub orientation_matrix: Vec<f32>,
}

impl Joint {
    /// Creates a Rust `Joint` from an FFI `skeleton_ffi::Joint` reference.
    ///
    /// This involves calling multiple FFI functions to populate the fields.
    pub(crate) fn from_ffi_joint(ffi_joint: &skeleton_ffi::Joint) -> NuitrackResult<Self> {
        let orientation_matrix_ptr: UniquePtr<CxxVector<f32>> = skeleton_ffi::joint_orientation_matrix(ffi_joint)?;
        
        let orientation_slice = orientation_matrix_ptr.as_ref()
            .ok_or_else(|| NuitrackError::OperationFailed("Joint orientation matrix FFI call returned null UniquePtr.".to_string()))?;

        // Assuming these FFI calls don't return Result as they are simple getters
        // from a valid reference. If they could fail, they'd need to return Result
        // and be handled with `?` and map_err.
        Ok(Self {
            joint_type: skeleton_ffi::joint_type(ffi_joint),
            confidence: skeleton_ffi::joint_confidence(ffi_joint),
            real_x: skeleton_ffi::joint_real_x(ffi_joint),
            real_y: skeleton_ffi::joint_real_y(ffi_joint),
            real_z: skeleton_ffi::joint_real_z(ffi_joint),
            proj_x: skeleton_ffi::joint_proj_x(ffi_joint),
            proj_y: skeleton_ffi::joint_proj_y(ffi_joint),
            proj_z: skeleton_ffi::joint_proj_z(ffi_joint),
            orientation_matrix: orientation_slice.iter().cloned().collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub user_id: i32,
    pub joints: Vec<Joint>,
}

impl Skeleton {
    /// Creates a Rust `Skeleton` from an FFI `skeleton_ffi::Skeleton` reference.
    ///
    /// This involves calling multiple FFI functions to populate the fields.
    pub(crate) fn from_ffi_skeleton(ffi_skeleton: &skeleton_ffi::Skeleton) -> NuitrackResult<Self> {
        // Assuming these FFI calls don't return Result from CXX
        // as they are simple getters on a valid &Skeleton reference.
        // If they could throw (e.g. if skeleton was null, but CXX usually ensures &T is not null),
        // then they would return Result and need `?` and `map_err`.
        let user_id = skeleton_ffi::user_id(ffi_skeleton);

        let ffi_joints_ptr: UniquePtr<CxxVector<skeleton_ffi::Joint>> = skeleton_ffi::joints(ffi_skeleton)?;
        
        let ffi_joints_vec_ref = ffi_joints_ptr.as_ref()
            .ok_or_else(|| NuitrackError::OperationFailed("Skeleton joints FFI call returned null UniquePtr.".to_string()))?;

        let mut joints = Vec::with_capacity(ffi_joints_vec_ref.len());
        for ffi_joint_ref in ffi_joints_vec_ref {
            joints.push(Joint::from_ffi_joint(ffi_joint_ref)?);
        }

        Ok(Self {
            user_id,
            joints,
        })
    }
}