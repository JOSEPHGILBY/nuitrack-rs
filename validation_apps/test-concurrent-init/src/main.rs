use nuitrack_rs::nuitrack::{
    async_api::{
        session_builder::NuitrackSessionBuilder,
        session::NuitrackSession
    },
    shared_types::{
        session_config::ModuleType,
        error::NuitrackError
    }
};
use tokio::task::JoinHandle;
use tracing::{error, info, instrument, span, warn, Instrument, Level};

// This function will be spawned by multiple tasks.
#[instrument(skip_all, fields(task_id))]
async fn attempt_session_initialization(task_id: usize) -> Result<NuitrackSession, NuitrackError> {
    info!("Attempting Nuitrack session initialization...");
    NuitrackSessionBuilder::create_session_from_single_default_device(
        vec![] // No specific modules needed for this init test, an empty session is fine.
    ).await // This internally calls NuitrackRuntimeGuard::acquire
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_thread_ids(true) // Show thread IDs for better async flow understanding
        .with_target(false) // Don't show module path, keep logs cleaner
        .init();

    info!("--- Test: Concurrent Nuitrack Session Initialization Attempts ---");

    let num_concurrent_tasks = 5; // Number of tasks trying to initialize
    let mut task_handles: Vec<JoinHandle<Result<NuitrackSession, NuitrackError>>> = Vec::new();

    info!("Spawning {} tasks to attempt session initialization concurrently.", num_concurrent_tasks);
    for i in 0..num_concurrent_tasks {
        // Each task gets its own span for easier log tracing
        let task_span = span!(Level::INFO, "concurrent_init_task", id = i);
        task_handles.push(tokio::spawn(attempt_session_initialization(i).instrument(task_span)));
    }

    let mut success_count = 0;
    let mut already_initialized_count = 0;
    let mut other_errors_count = 0;
    let mut successful_session_for_cleanup: Option<NuitrackSession> = None;

    for (i, handle) in task_handles.into_iter().enumerate() {
        let task_id = i; // For logging
        match handle.await { // Wait for each task to complete
            Ok(Ok(session)) => {
                // This block should ideally only be entered once.
                info!(task = task_id, "Successfully initialized Nuitrack session.");
                success_count += 1;
                if successful_session_for_cleanup.is_none() {
                    successful_session_for_cleanup = Some(session);
                } else {
                    // This case indicates a problem with the IS_NUITRACK_RUNTIME_INITIALIZED logic
                    // or multiple guards were created.
                    warn!(task = task_id, "UNEXPECTED: Another task also successfully initialized a session. This indicates a potential issue with the singleton guard logic.");
                    // Clean up the extra session immediately
                    if let Err(e) = session.close().await {
                       error!(task = task_id, "Error closing unexpected extra session: {:?}", e);
                    }
                }
            }
            Ok(Err(NuitrackError::AlreadyInitialized)) => {
                info!(task = task_id, "Correctly failed with NuitrackError::AlreadyInitialized.");
                already_initialized_count += 1;
            }
            Ok(Err(e)) => {
                error!(task = task_id, "Failed with an unexpected NuitrackError: {:?}", e);
                other_errors_count += 1;
            }
            Err(join_err) => { // Task panicked
                error!(task = task_id, "Task panicked or was cancelled: {:?}", join_err);
                other_errors_count += 1;
            }
        }
    }

    info!("--- Initialization Test Results ---");
    info!("Successful initializations: {}", success_count);
    info!("'AlreadyInitialized' errors: {}", already_initialized_count);
    info!("Other errors/panics: {}", other_errors_count);

    // Assertions for the test
    assert_eq!(success_count, 1, "Exactly one task should have successfully initialized Nuitrack.");
    assert_eq!(already_initialized_count, num_concurrent_tasks - 1, "The other tasks should have failed with AlreadyInitialized.");
    assert_eq!(other_errors_count, 0, "There should have been no other errors or task panics.");

    // Clean up the one successful session
    if let Some(session_to_close) = successful_session_for_cleanup {
        info!("Closing the successfully initialized Nuitrack session...");
        if let Err(e) = session_to_close.close().await {
            error!("Error during final close of Nuitrack session: {:?}", e);
            // You might choose to fail the test here if cleanup is critical.
        } else {
            info!("Nuitrack session closed successfully.");
        }
    } else if num_concurrent_tasks > 0 { // Only an error if we expected a success
         error!("No session was successfully initialized, which is unexpected.");
         return Err(anyhow::anyhow!("No session was successfully initialized in the test."));
    }

    info!("Concurrent initialization test completed.");
    Ok(())
}