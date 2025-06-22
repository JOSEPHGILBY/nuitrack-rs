 // For interacting with SharedPtr<Hand> from FFI
use tracing::instrument;
use crate::nuitrack_bridge::types::hand::ffi::{self as hand_ffi}; // Corrected path

pub use crate::nuitrack_bridge::types::hand::ffi::Hand;

/// Represents the hands (left and/or right) detected for a single user.
#[derive(Debug, Clone, PartialEq)]
pub struct UserHands {
    /// The ID of the user to whom this hand information applies.
    pub user_id: i32,
    /// Information about the user's left hand, if detected.
    pub left_hand: Option<Hand>,
    /// Information about the user's right hand, if detected.
    pub right_hand: Option<Hand>,
}

impl UserHands {
    /// Creates a Rust `UserHands` from an FFI `hand_ffi::UserHands` reference.
    ///
    /// This involves retrieving shared pointers to FFI `Hand` objects for left and
    /// right hands and converting them if they are not null.
    #[instrument(level="trace", skip(ffi_user_hands))]
    pub(crate) fn from_ffi(ffi_user_hands: &hand_ffi::UserHands) -> Self {
        // --- This is the simplified logic ---

        // 1. Get the smart pointer from the FFI.
        let ffi_left_hand_ptr = hand_ffi::user_hands_left_hand(ffi_user_hands);
        // 2. Check if the pointer is null. If not, dereference it to create a cheap copy.
        let left_hand = if ffi_left_hand_ptr.is_null() {
            None
        } else {
            // The `*` performs the fast, bit-for-bit copy because Hand is `Copy`.
            Some(*ffi_left_hand_ptr)
        };

        let ffi_right_hand_ptr = hand_ffi::user_hands_right_hand(ffi_user_hands);
        let right_hand = if ffi_right_hand_ptr.is_null() {
            None
        } else {
            Some(*ffi_right_hand_ptr)
        };
        
        Self {
            user_id: hand_ffi::user_hands_user_id(ffi_user_hands),
            left_hand,
            right_hand,
        }
    }
}