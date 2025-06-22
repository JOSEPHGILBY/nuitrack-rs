use std::sync::OnceLock;
use tracing::{debug, instrument, trace, trace_span, warn};
use cxx::SharedPtr;

use crate::nuitrack_bridge::modules::skeleton_tracker::ffi::SkeletonData as FFISkeletonData;
use crate::nuitrack_bridge::types::skeleton_data::ffi::{self as skeleton_data_ffi};

use super::error::{NuitrackError, Result as NuitrackResult};
use super::skeleton::Skeleton;


pub struct SkeletonFrame {
    internal_ptr: SharedPtr<FFISkeletonData>,
    skeletons_cache: OnceLock<NuitrackResult<Vec<Skeleton>>>,
}

impl SkeletonFrame {
    pub(crate) fn new(ffi_ptr: SharedPtr<FFISkeletonData>) -> Option<Self> {
        let is_null = ffi_ptr.is_null();
        trace!(is_null, "Attempting to create new SkeletonFrame.");
        if is_null {
            None
        } else {
            Some(SkeletonFrame { internal_ptr: ffi_ptr, skeletons_cache: OnceLock::new() })
        }
    }

    #[instrument(skip(self))]
    pub fn num_skeletons(&self) -> NuitrackResult<i32> {
        trace_span!("ffi", function = "skeleton_data_ffi::num_skeletons").in_scope(||
            Ok(skeleton_data_ffi::num_skeletons(&self.internal_ptr)?)
        )
    }

    #[instrument(skip(self))]
    pub fn skeletons(&self) -> NuitrackResult<&[Skeleton]> {
        let cached_result = self.skeletons_cache.get_or_init(|| {
            debug!("Populating SkeletonFrame cache: converting FFI Skeletons to Rust.");
            let conversion = || -> NuitrackResult<Vec<Skeleton>> {
                let ffi_skeletons = trace_span!("ffi", function = "skeleton_data_ffi::skeletons").in_scope(||
                    skeleton_data_ffi::skeletons(&self.internal_ptr)
                )?;
                let ffi_vec = ffi_skeletons
                    .as_ref()
                    .ok_or_else(|| {
                         warn!("FFI returned null skeleton vector pointer.");
                        NuitrackError::OperationFailed("FFI returned null skeleton vector".into())
                    })?;

                trace!(count = ffi_vec.len(), "Performing deep copy of Skeletons.");
                let mut rust_skeletons = Vec::with_capacity(ffi_vec.len());
                for ffi_s in ffi_vec {
                    rust_skeletons.push(Skeleton::from_ffi_skeleton(ffi_s));
                }
                Ok(rust_skeletons)
            };

            conversion()
        });

        match cached_result {
            Ok(skeletons_vec) => {
                trace!("Returning Skeletons from cache.");
                Ok(skeletons_vec.as_slice())
            },
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for skeletons access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            },
        }
    }
    
    #[instrument(skip(self))] 
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        trace_span!("ffi", function = "skeleton_data_ffi::timestamp").in_scope(||
            Ok(skeleton_data_ffi::timestamp(&self.internal_ptr)?)
        )
    }
}