use crate::{client::Reqwest, errors::EventErrorKind, Request};

use std::sync::Arc;

/// The error that occurred while processing an update, together with
/// the request that was being processed when it happened.
///
/// It's propagated to the error observers registered with [`Router::on_error`].
///
/// [`Router::on_error`]: telers::Router#method.on_error
#[derive(Clone)]
pub struct ErrorEvent<Client = Reqwest> {
    pub request: Request<Client>,
    pub error: Arc<EventErrorKind>,
}

impl<Client> ErrorEvent<Client> {
    /// Create a new error event.
    ///
    /// # Arguments
    /// * `request` - The request that was being processed when the error occurred.
    /// * `error` - The error that occurred.
    #[must_use]
    pub fn new(request: Request<Client>, error: impl Into<Arc<EventErrorKind>>) -> Self {
        Self {
            request,
            error: error.into(),
        }
    }
}

impl<Client> std::fmt::Debug for ErrorEvent<Client> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorEvent")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}
