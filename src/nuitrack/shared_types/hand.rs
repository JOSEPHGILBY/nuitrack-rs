use cxx::SharedPtr; // For interacting with SharedPtr<Hand> from FFI
use crate::nuitrack_bridge::types::hand::ffi as hand_ffi; // Corrected path
use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};

/// Represents detailed information about a single tracked hand.
#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    /// The normalized projective x-coordinate of the hand (in range [0, 1]).
    pub x: f32,
    /// The normalized projective y-coordinate of the hand (in range [0, 1]).
    pub y: f32,
    /// True if the hand is making a click gesture, false otherwise.
    pub click: bool,
    /// Rate of hand clenching (interpretation might depend on Nuitrack version/config).
    pub pressure: i32,
    /// The x-coordinate of the hand in the world system (meters).
    pub x_real: f32,
    /// The y-coordinate of the hand in the world system (meters).
    pub y_real: f32,
    /// The z-coordinate of the hand in the world system (meters).
    pub z_real: f32,
}

impl Hand {
    /// Creates a Rust `Hand` from an FFI `hand_ffi::Hand` reference.
    ///
    /// This involves calling multiple FFI functions to populate the fields.
    /// It assumes the `ffi_hand` reference is valid and the underlying FFI calls
    /// are simple getters that do not return `Result` themselves.
    pub(crate) fn from_ffi_hand(ffi_hand: &hand_ffi::Hand) -> Self {
        Self {
            x: hand_ffi::hand_x(ffi_hand),
            y: hand_ffi::hand_y(ffi_hand),
            click: hand_ffi::hand_click(ffi_hand),
            pressure: hand_ffi::hand_pressure(ffi_hand),
            x_real: hand_ffi::hand_x_real(ffi_hand),
            y_real: hand_ffi::hand_y_real(ffi_hand),
            z_real: hand_ffi::hand_z_real(ffi_hand),
        }
    }
}

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
    pub(crate) fn from_ffi_user_hands(ffi_user_hands: &hand_ffi::UserHands) -> NuitrackResult<Self> {
        let user_id = hand_ffi::user_hands_user_id(ffi_user_hands);

        let ffi_left_hand_ptr: SharedPtr<hand_ffi::Hand> = hand_ffi::user_hands_left_hand(ffi_user_hands);
        let left_hand = if !ffi_left_hand_ptr.is_null() {
            // If SharedPtr is not null, as_ref() should provide a valid reference.
            // If as_ref() returns None on a non-null SharedPtr, it indicates an issue
            // with the CXX layer or an unexpected state.
            let ffi_hand_ref = ffi_left_hand_ptr.as_ref()
                .ok_or_else(|| NuitrackError::OperationFailed(
                    "FFI SharedPtr<Hand> for left hand was non-null but as_ref() failed.".to_string()
                ))?;
            Some(Hand::from_ffi_hand(ffi_hand_ref))
        } else {
            None
        };

        let ffi_right_hand_ptr: SharedPtr<hand_ffi::Hand> = hand_ffi::user_hands_right_hand(ffi_user_hands);
        let right_hand = if !ffi_right_hand_ptr.is_null() {
            let ffi_hand_ref = ffi_right_hand_ptr.as_ref()
                .ok_or_else(|| NuitrackError::OperationFailed(
                    "FFI SharedPtr<Hand> for right hand was non-null but as_ref() failed.".to_string()
                ))?;
            Some(Hand::from_ffi_hand(ffi_hand_ref))
        } else {
            None
        };
        
        Ok(Self {
            user_id,
            left_hand,
            right_hand,
        })
    }
}