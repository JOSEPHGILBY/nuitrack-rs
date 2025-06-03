use crate::nuitrack::shared_types::error::{NuitrackError, Result as NuitrackResult};

#[cfg(not(feature = "tokio_runtime"))]
pub(crate) async fn run_blocking<F, T>(func: F) -> NuitrackResult<T>
where
    F: FnOnce() -> NuitrackResult<T> + Send + 'static,
    T: Send + 'static,
{
    blocking::unblock(func).await
}

#[cfg(feature = "tokio_runtime")]
pub(crate) async fn run_blocking<F, T>(func: F) -> NuitrackResult<T>
where
    F: FnOnce() -> NuitrackResult<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(func).await {
        Ok(res) => res,
        Err(join_error) => Err(NuitrackError::OperationFailed(format!(
            "Tokio spawn_blocking task failed: {}",
            join_error
        ))),
    }
}
