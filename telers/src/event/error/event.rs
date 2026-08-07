use crate::Request;
use std::sync::Arc;

/// The error that occurred while processing an update, together with the
/// request that was being processed when it happened.
pub struct ErrorEvent<Client> {
    pub request: Request<Client>,
    pub error: Arc<crate::errors::EventErrorKind>,
}

impl<Client> Clone for ErrorEvent<Client> {
    fn clone(&self) -> Self {
        Self {
            request: self.request.clone(),
            error: self.error.clone(),
        }
    }
}