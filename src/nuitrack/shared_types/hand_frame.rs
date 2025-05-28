use std::{error::Error, fmt};

use cxx::SharedPtr;
use crate::nuitrack_bridge::hand_tracker::ffi::{self as ht_ffi, BridgedHandTrackerData};

use super::hand::Hand;
use tracing::{debug};

#[derive(Debug, Clone)]
pub struct UserHandsData {
    pub user_id: i32,
    pub left_hand: Option<Hand>,
    pub right_hand: Option<Hand>,
}

pub struct HandFrame {
    internal_ptr: SharedPtr<BridgedHandTrackerData>,
}

impl HandFrame {
    pub(crate) fn new(ffi_ptr: SharedPtr<BridgedHandTrackerData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            debug!(target: "nuitrack_rs::frame", "Attempted to create HandFrameData from null FFI pointer.");
            None
        } else {
            Some(HandFrame { internal_ptr: ffi_ptr })
        }
    }

    pub fn timestamp(&self) -> Result<u64, cxx::Exception> {
        ht_ffi::get_data_timestamp(&self.internal_ptr)
    }

    pub fn num_users(&self) -> Result<i32, cxx::Exception> {
        ht_ffi::get_data_num_users(&self.internal_ptr)
    }

    pub fn users_hands_vector_size(&self) -> Result<usize, cxx::Exception> {
        ht_ffi::get_users_hands_vector_size(&self.internal_ptr)
    }

    pub fn user_hands_at_index(&self, index: usize) -> Result<UserHandsData, Box<dyn Error>> {
        let user_id = ht_ffi::get_user_id_at(&self.internal_ptr, index)?;

        let left_ffi_ptr = ht_ffi::get_left_hand_at(&self.internal_ptr, index)?;
        let left_hand = Hand::new(left_ffi_ptr);

        let right_ffi_ptr = ht_ffi::get_right_hand_at(&self.internal_ptr, index)?;
        let right_hand = Hand::new(right_ffi_ptr);

        Ok(UserHandsData {
            user_id,
            left_hand,
            right_hand,
        })
    }

    // You could also provide an iterator over users
    // pub fn users_iter(&self) -> impl Iterator<Item = Result<UserHandsData, Box<dyn Error>>> + '_ { ... }
}


impl fmt::Debug for HandFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ds = f.debug_struct("HandFrameData");
        if self.internal_ptr.is_null() {
            ds.field("status", &"null_internal_pointer");
        } else {
            // Call getter methods and format their results.
            // Using .ok() on the NuitrackResult<T> will give Option<T>, which is Debug.
            // Errors from these getters are already logged by the methods themselves if instrumented.
            ds.field("timestamp", &self.timestamp().ok());
            ds.field("num_users", &self.num_users().ok());
            // For users_hands_vector_size, you might want to be more specific or show first user
            match self.users_hands_vector_size() {
                Ok(size) => {
                    ds.field("users_hands_vector_size", &size);
                    // Optionally, you could try to show the first user's hands if present,
                    // but be careful about making Debug too verbose or error-prone.
                    // if size > 0 {
                    //     ds.field("first_user_hands (details)", &self.user_hands_at_index(0).ok());
                    // }
                }
                Err(_) => {
                    ds.field("users_hands_vector_size", &"Error retrieving size");
                }
            }
        }
        ds.finish()
    }
}