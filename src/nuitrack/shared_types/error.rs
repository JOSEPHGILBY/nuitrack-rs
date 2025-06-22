use thiserror::Error;

#[derive(Debug, Error)]
pub enum NuitrackError {
    #[error("Nuitrack FFI call failed: {0}")]
    FFI(#[from] cxx::Exception), // Can convert from cxx::Exception

    #[error("Nuitrack C++ wrapper error: {0}")]
    Wrapper(String),

    #[error("Nuitrack already initialized by this wrapper.")]
    AlreadyInitialized,

    #[error("Nuitrack initialization failed: {0}")]
    InitFailed(String),

    #[error("Nuitrack device error: {0}")]
    DeviceError(String),

    #[error("Nuitrack module creation failed: {0}")] // Specifically for creating modules like HandTracker
    ModuleCreationFailed(String),

    #[error("No Nuitrack device found")]
    NoDeviceFound,

    #[error("Nuitrack operation failed: {0}")]
    OperationFailed(String),
    // Add more specific Nuitrack errors as you identify them
}

pub type Result<T, E = NuitrackError> = std::result::Result<T, E>;