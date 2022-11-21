use super::base::Middleware;

use std::rc::Rc;

/// Manager for inner middlewares
#[derive(Default, Clone)]
pub struct Manager {
    middlewares: Vec<Rc<Box<dyn Middleware>>>,
}

impl Manager {
    /// Register a new middleware
    pub fn register(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(Rc::new(middleware));
    }

    /// Register a new middleware wrapper
    pub fn register_wrapper(&mut self, middleware: Rc<Box<dyn Middleware>>) {
        self.middlewares.push(middleware);
    }

    /// Get all middlewares
    #[must_use]
    pub fn middlewares(&self) -> &[Rc<Box<dyn Middleware>>] {
        &self.middlewares
    }
}
