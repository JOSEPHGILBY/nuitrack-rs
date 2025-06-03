use cxx::SharedPtr;

use crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonData as FFISkeletonData;
use crate::nuitrack_bridge::types::skeleton_data::ffi::{self as skeleton_data_ffi};

use super::error::{NuitrackError, Result as NuitrackResult};
use super::skeleton::Skeleton;


pub struct SkeletonFrame {
    internal_ptr: SharedPtr<FFISkeletonData>,
}

impl SkeletonFrame {
    pub(crate) fn new(ffi_ptr: SharedPtr<FFISkeletonData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            None
        } else {
            Some(SkeletonFrame { internal_ptr: ffi_ptr })
        }
    }

    pub fn get_num_skeletons(&self) -> NuitrackResult<i32> {
        Ok(skeleton_data_ffi::num_skeletons(&self.internal_ptr)?)
    }

    pub fn get_skeletons(&self) -> NuitrackResult<Vec<Skeleton>> {
        let skeletons = skeleton_data_ffi::skeletons(&self.internal_ptr)?;
        let ffi_skeletons_vec_ref = skeletons.as_ref()
            .ok_or_else(|| NuitrackError::OperationFailed("Failed to get skeletons vector".into()))?;
        let mut rust_skeletons = Vec::with_capacity(ffi_skeletons_vec_ref.len());
        for ffi_skeleton_ref in ffi_skeletons_vec_ref {
            rust_skeletons.push(Skeleton::from_ffi_skeleton(ffi_skeleton_ref)?);
        }

        Ok(rust_skeletons)

    }
    
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        Ok(skeleton_data_ffi::timestamp(&self.internal_ptr)?)
    }
}