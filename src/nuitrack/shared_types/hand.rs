use std::fmt;

use cxx::SharedPtr;

use crate::nuitrack_bridge::hand_tracker::ffi::{self as ht_ffi, NuitrackHand as FFINuitrackHand};

#[derive(Clone)]
pub struct Hand {
    internal_ptr: SharedPtr<FFINuitrackHand>,
}

impl Hand {
    pub(crate) fn new(ffi_ptr: SharedPtr<FFINuitrackHand>) -> Option<Self> {
        if ffi_ptr.is_null() {
            None
        } else {
            Some(Hand { internal_ptr: ffi_ptr })
        }
    }

    pub fn x(&self) -> f32 {
        ht_ffi::get_hand_x(&self.internal_ptr)
    }

    pub fn y(&self) -> f32 {
        ht_ffi::get_hand_y(&self.internal_ptr)
    }

    pub fn is_click(&self) -> bool {
        ht_ffi::hand_is_click(&self.internal_ptr)
    }

    pub fn pressure(&self) -> i32 {
        ht_ffi::get_hand_pressure(&self.internal_ptr)
    }

    pub fn x_real(&self) -> f32 {
        ht_ffi::get_hand_x_real(&self.internal_ptr)
    }

    pub fn y_real(&self) -> f32 {
        ht_ffi::get_hand_y_real(&self.internal_ptr)
    }

    pub fn z_real(&self) -> f32 {
        ht_ffi::get_hand_z_real(&self.internal_ptr)
    }

}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("Hand");
        if self.internal_ptr.is_null() {
            // Clearly indicate that the internal pointer is null.
            ds.field("status", &"null_internal_pointer");
        } else {
            // Since we cannot directly format self.internal_ptr due to the
            // NuitrackHand: Debug constraint apparently imposed by SharedPtr<NuitrackHand>: Debug,
            // we will format the data fields that we can access through its public API.
            ds.field("x", &self.x());
            ds.field("y", &self.y());
            ds.field("is_click", &self.is_click());
            ds.field("pressure", &self.pressure());
            ds.field("x_real", &self.x_real());
            ds.field("y_real", &self.y_real());
            ds.field("z_real", &self.z_real());
            // We don't try to print self.internal_ptr directly using the field macro,
            // as that seems to be what triggers the transitive Debug requirement for NuitrackHand.
            // If you absolutely must include a representation of the pointer itself (like its address),
            // you would need a way to get that address from SharedPtr<T> as a usize or raw pointer
            // *without* invoking its Debug trait, and then format that usize/pointer.
            // e.g., ds.field("ptr_address", &format_args!("{:p}", self.internal_ptr.as_ptr_for_debug_if_available()));
            // However, cxx::SharedPtr doesn't directly expose a safe way to get the raw address just for Debug
            // without potentially hitting similar issues or using unsafe means.
            // Focusing on the accessible data is the safest and most informative approach here.
        }
        ds.finish()
    }
}