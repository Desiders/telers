use super::base::{Middleware, Middlewares};

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct Manager {
    pub middlewares: Middlewares,
}

impl Manager {
    pub fn register<T: Middleware + 'static>(&mut self, middleware: T) {
        self.middlewares.push(Arc::new(Box::new(middleware)));
    }

    pub fn register_wrapper(&mut self, middleware: Arc<Box<dyn Middleware>>) {
        self.middlewares.push(middleware);
    }

    pub fn register_at_position<T: Middleware + 'static>(&mut self, index: usize, middleware: T) {
        self.middlewares
            .insert(index, Arc::new(Box::new(middleware)));
    }

    pub fn register_wrapper_at_position(
        &mut self,
        index: usize,
        middleware: Arc<Box<dyn Middleware>>,
    ) {
        self.middlewares.insert(index, middleware);
    }
}
