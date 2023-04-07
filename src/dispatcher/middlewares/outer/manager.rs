use super::base::{Middleware, Middlewares};

use std::sync::Arc;

#[derive(Clone)]
pub struct Manager<Client> {
    pub middlewares: Middlewares<Client>,
}

impl<Client> Manager<Client> {
    /// Register middleware in the end of the list
    pub fn register<T: Middleware<Client> + 'static>(&mut self, middleware: T) {
        self.middlewares.push(Arc::new(Box::new(middleware)));
    }

    pub fn register_wrapper(&mut self, middleware: Arc<Box<dyn Middleware<Client>>>) {
        self.middlewares.push(middleware);
    }

    /// Register middleware at the specified position
    /// # Warning
    /// Not recommended to use this method. Use it only if you know what you are doing. \
    /// You can break the order of middlewares, which can lead to unexpected behaviour for some middlewares,
    /// which depends on the order of middlewares.
    pub fn register_at_position<T: Middleware<Client> + 'static>(
        &mut self,
        index: usize,
        middleware: T,
    ) {
        self.middlewares
            .insert(index, Arc::new(Box::new(middleware)));
    }

    pub fn register_wrapper_at_position(
        &mut self,
        index: usize,
        middleware: Arc<Box<dyn Middleware<Client>>>,
    ) {
        self.middlewares.insert(index, middleware);
    }
}

impl<Client> Default for Manager<Client> {
    fn default() -> Self {
        Self {
            middlewares: Middlewares::new(),
        }
    }
}
