use core::future::Future;
use std::fmt::Debug;
use std::process::ExitCode;

use log::{info, LevelFilter, warn};

/// An implementation of the main() function of an example app.
///
/// Sets up logging and provides a reasonable process exit code.
///
/// # Example
///
/// ```rust
/// use std::process::ExitCode;
///
/// #[tokio::main]
/// async fn main() -> ExitCode {
///     passivized_test_support::cli::run(run).await
/// }
///
/// async fn run() -> Result<(), String> {
///     // Implementation of example goes here.
///
///     Ok(())
/// }
/// ```
pub async fn run<E, F, Fut>(implementation: F) -> ExitCode
where
    E: Debug,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), E>>
{
    super::logging::enable();

    run_impl(implementation).await
}

/// Same as run(), but with a custom log level
pub async fn run_with_level<E, F, Fut>(implementation: F, log_level: LevelFilter) -> ExitCode
where
    E: Debug,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), E>>
{
    super::logging::enable_with_level(log_level);

    run_impl(implementation).await
}

/// Same as run(), but does NOT configure a logger.
pub async fn run_impl<E, F, Fut>(implementation: F) -> ExitCode
where
    E: Debug,
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), E>>
{
    info!("Hello, world.");

    match implementation().await {
        Err(e) => {
            warn!("Failed: {:?}", e);
            ExitCode::FAILURE
        }
        Ok(_) => {
            info!("Done.");
            ExitCode::SUCCESS
        }
    }
}
