use super::base::{MiddlewareType, Middlewares};

use std::sync::Arc;

/// Manager for inner middlewares
#[derive(Default, Clone)]
pub struct Manager {
    middlewares: Middlewares,
}

impl Manager {
    /// Register a new middleware
    pub fn register(&mut self, middleware: MiddlewareType) {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Register a new middleware wrapper
    pub fn register_wrapper(&mut self, middleware: Arc<MiddlewareType>) {
        self.middlewares.push(middleware);
    }

    /// Get all middlewares
    #[must_use]
    pub fn middlewares(&self) -> &[Arc<MiddlewareType>] {
        &self.middlewares
    }
}
