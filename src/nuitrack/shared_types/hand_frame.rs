use std::sync::OnceLock;
use tracing::{debug, instrument, trace, trace_span, warn};
use cxx::{SharedPtr, UniquePtr, CxxVector};
use crate::nuitrack_bridge::types::hand_data::ffi as hand_data_ffi;

// Import the public Rust UserHands type we defined earlier
use super::hand::UserHands; 
use super::error::{NuitrackError, Result as NuitrackResult};

pub struct HandFrame {
    /// Internal pointer to the FFI HandData object.
    internal_ptr: SharedPtr<hand_data_ffi::HandData>,
    users_hands_cache: OnceLock<NuitrackResult<Vec<UserHands>>>,
}

impl HandFrame {
    /// Creates a new `HandFrame` from a shared pointer to the FFI `HandData` object.
    ///
    /// Returns `None` if the provided FFI pointer is null.
    pub(crate) fn new(ffi_ptr: SharedPtr<hand_data_ffi::HandData>) -> Option<Self> {
        let is_null = ffi_ptr.is_null();
        trace!(is_null, "Attempting to create new HandFrame.");
        if is_null {
            None
        } else {
            Some(HandFrame { internal_ptr: ffi_ptr , users_hands_cache: OnceLock::new(),})
        }
    }

    /// Gets the number of users for whom hand data is available in this frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the number of users, or an error if the
    /// FFI call fails.
    #[instrument(skip(self))]
    pub fn num_users(&self) -> NuitrackResult<i32> {
        // Calls the FFI function on the internal HandData pointer.
        // The `?` operator propagates any error from the FFI call.
        trace_span!("ffi", function = "hand_data_ffi::num_users").in_scope( ||
            Ok(hand_data_ffi::num_users(&self.internal_ptr)?)
        )
    }

    /// Gets a list of `UserHands` objects, each representing the detected hands
    /// for a specific user in this frame.
    ///
    /// # Returns
    /// A `NuitrackResult` containing a vector of `UserHands`, or an error if
    /// the FFI call or data conversion fails.
    #[instrument(skip(self))]
    pub fn users_hands(&self) -> NuitrackResult<&[UserHands]> {
        // Use get_or_init to lazily populate our cache.
        // The closure will only ever be executed ONCE for this HandFrame instance.
        let cached_result = self.users_hands_cache.get_or_init(|| {
            debug!("Populating HandFrame cache: converting FFI UserHands to Rust.");
            
            // This internal closure helps with using the `?` operator.
            let conversion = || -> NuitrackResult<Vec<UserHands>> {
                // 1. Call FFI to get the owned vector of opaque UserHands objects
                let ffi_users_hands_ptr = trace_span!("ffi", function = "hand_data_ffi::users_hands").in_scope( ||
                    hand_data_ffi::users_hands(&self.internal_ptr)
                )?;
                let ffi_users_hands_vec_ref = ffi_users_hands_ptr
                    .as_ref()
                    .ok_or_else(|| {
                        warn!("FFI returned null UserHands vector pointer.");
                        NuitrackError::OperationFailed("FFI returned null UserHands vector".into())
                    })?;

                trace!(count = ffi_users_hands_vec_ref.len(), "Performing deep copy of UserHands.");
                let mut rust_user_hands_list = Vec::with_capacity(ffi_users_hands_vec_ref.len());
                for ffi_user_hands_ref in ffi_users_hands_vec_ref {
                    rust_user_hands_list.push(UserHands::from_ffi(ffi_user_hands_ref));
                }
                
                Ok(rust_user_hands_list)
            };

            // Execute the conversion and return the Result to be stored in the OnceLock
            conversion()
        });

        // "Unpack" the cached result for the caller
        match cached_result {
            Ok(user_hands_vec) => {
                trace!("Returning UserHands from cache.");
                Ok(user_hands_vec.as_slice())
            },
            Err(e) => {
                warn!(original_error = %e, "Returning cached error for users_hands access.");
                Err(NuitrackError::OperationFailed(format!("Cached FFI error: {}", e)))
            },
        }
    }
    
    /// Gets the timestamp of this hand data frame in microseconds.
    ///
    /// The timestamp indicates the time point to which the hand data corresponds.
    /// The exact meaning of this value can depend on the depth provider.
    ///
    /// # Returns
    /// A `NuitrackResult` containing the timestamp, or an error if the
    /// FFI call fails.
    #[instrument(skip(self))]
    pub fn timestamp(&self) -> NuitrackResult<u64> {
        // Calls the FFI function on the internal HandData pointer.
        trace_span!("ffi", function = "hand_data_ffi::timestamp").in_scope( ||
            Ok(hand_data_ffi::timestamp(&self.internal_ptr)?)
        )
    }
}
