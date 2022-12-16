use super::base::{Middleware, Middlewares};

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Manager {
    middlewares: Middlewares,
}

impl Manager {
    pub fn register<T, M>(&mut self, middleware: T)
    where
        T: Into<Box<M>>,
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.push(Arc::new(middleware.into()));
    }

    pub fn register_wrapper(
        &mut self,
        middleware: Arc<Box<dyn Middleware + Send + Sync + 'static>>,
    ) {
        self.middlewares.push(middleware);
    }

    pub fn register_at_position<T, M>(&mut self, index: usize, middleware: T)
    where
        T: Into<Box<M>>,
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.insert(index, Arc::new(middleware.into()));
    }

    pub fn register_wrapper_at_position(
        &mut self,
        index: usize,
        middleware: Arc<Box<dyn Middleware + Send + Sync + 'static>>,
    ) {
        self.middlewares.insert(index, middleware);
    }

    #[must_use]
    pub fn middlewares(&self) -> &Middlewares {
        &self.middlewares
    }
}
