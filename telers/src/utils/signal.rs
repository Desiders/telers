//! Waiting for the signals that ask the bot to stop.
//!
//! [`shutdown_signal`] resolves when the process is interrupted, so it can be passed to the
//! graceful shutdown of the dispatcher.

use tokio::signal;

/// Waits for `Ctrl+C`, or for `SIGTERM` on unix, and returns when one of them arrives
///
/// # Panics
/// Failed to install signal handler
pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
