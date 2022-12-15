use super::base::{MiddlewareType, MiddlewaresType};

use std::sync::Arc;

/// Manager for inner middlewares
#[derive(Default, Clone)]
pub struct Manager {
    middlewares: MiddlewaresType,
}

/// Inner middlewares manager
impl Manager {
    /// Register a new middleware
    pub fn register(&mut self, middleware: MiddlewareType) {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Register a new middleware wrapper
    pub fn register_wrapper(&mut self, middleware: Arc<MiddlewareType>) {
        self.middlewares.push(middleware);
    }

    /// Register a new middleware at position
    pub fn register_in_position(&mut self, index: usize, middleware: MiddlewareType) {
        self.middlewares.insert(index, Arc::new(middleware));
    }

    /// Register a new middleware wrapper at position
    pub fn register_wrapper_in_position(&mut self, index: usize, middleware: Arc<MiddlewareType>) {
        self.middlewares.insert(index, middleware);
    }

    /// Get all middlewares
    #[must_use]
    pub fn middlewares(&self) -> &[Arc<MiddlewareType>] {
        &self.middlewares
    }
}
