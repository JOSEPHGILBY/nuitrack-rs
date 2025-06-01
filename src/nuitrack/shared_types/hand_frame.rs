use cxx::{SharedPtr, UniquePtr, CxxVector};
use crate::nuitrack_bridge::types::hand_data::ffi as hand_data_ffi;

// Import the public Rust UserHands type we defined earlier
use super::hand::UserHands; 
use super::error::{NuitrackError, Result as NuitrackResult};

pub struct HandFrame {
    /// Internal pointer to the FFI HandData object.
    internal_ptr: SharedPtr<hand_data_ffi::HandData>,
}

impl HandFrame {
    /// Creates a new `HandFrame` from a shared pointer to the FFI `HandData` object.
    ///
    /// Returns `None` if the provided FFI pointer is null.
    pub(crate) fn new(ffi_ptr: SharedPtr<hand_data_ffi::HandData>) -> Option<Self> {
        if ffi_ptr.is_null() {
            None
        } else {
            Some(HandFrame { internal_ptr: ffi_ptr })
        }
    }

    /// Gets the number of users for whom hand data is available in this frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of users, or an error if the
    /// FFI call fails.
    pub fn num_users(&self) -> NuitrackResult<i32> {
        // Calls the FFI function on the internal HandData pointer.
        // The `?` operator propagates any error from the FFI call.
        Ok(hand_data_ffi::num_users(&self.internal_ptr)?)
    }

    /// Gets a list of `UserHands` objects, each representing the detected hands
    /// for a specific user in this frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing a vector of `UserHands`, or an error if
    /// the FFI call or data conversion fails.
    pub fn users_hands(&self) -> NuitrackResult<Vec<UserHands>> {
        // 1. Call the FFI function to get a UniquePtr to a CxxVector of FFI UserHands.
        let ffi_users_hands_ptr: UniquePtr<CxxVector<hand_data_ffi::UserHands>> = 
            hand_data_ffi::users_hands(&self.internal_ptr)?;

        // 2. Get a safe reference to the CxxVector.
        // If the UniquePtr is null (shouldn't happen if FFI call succeeded without error
        // and Nuitrack returned valid data), or if as_ref() fails, return an error.
        let ffi_users_hands_vec_ref: &CxxVector<hand_data_ffi::UserHands> = ffi_users_hands_ptr.as_ref()
            .ok_or_else(|| NuitrackError::OperationFailed(
                "Failed to get CxxVector<UserHands> reference from UniquePtr.".into()
            ))?;

        // 3. Prepare a Rust Vec to store the converted UserHands.
        let mut rust_user_hands_list = Vec::with_capacity(ffi_users_hands_vec_ref.len());

        // 4. Iterate over the FFI UserHands objects in the CxxVector.
        for ffi_user_hands_ref in ffi_users_hands_vec_ref {
            // Convert each FFI UserHands object to the public Rust UserHands type.
            // The `from_ffi_user_hands` function is defined in `super::hand::UserHands`.
            // The `?` operator propagates any error from this conversion.
            rust_user_hands_list.push(UserHands::from_ffi_user_hands(ffi_user_hands_ref)?);
        }

        // 5. Return the list of converted Rust UserHands objects.
        Ok(rust_user_hands_list)
    }
    
    /// Gets the timestamp of this hand data frame in microseconds.
    ///
    /// The timestamp indicates the time point to which the hand data corresponds.
    /// The exact meaning of this value can depend on the depth provider.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the timestamp, or an error if the
    /// FFI call fails.
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        // Calls the FFI function on the internal HandData pointer.
        Ok(hand_data_ffi::timestamp(&self.internal_ptr)?)
    }
}
