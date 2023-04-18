use super::base::{Middleware, Middlewares};

use std::sync::Arc;

#[derive(Clone)]
pub struct Manager<Client> {
    pub middlewares: Middlewares<Client>,
}

impl<Client> Manager<Client> {
    /// Register middleware in the end of the list
    pub fn register<T>(&mut self, middleware: T)
    where
        T: Middleware<Client> + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Register middleware at the specified position
    /// # Warning
    /// Not recommended to use this method. Use it only if you know what you are doing. \
    /// You can break the order of middlewares, which can lead to unexpected behaviour for some middlewares,
    /// which depends on the order of middlewares.
    pub fn register_at_position<T>(&mut self, index: usize, middleware: T)
    where
        T: Middleware<Client> + 'static,
    {
        self.middlewares.insert(index, Arc::new(middleware));
    }
}

impl<Client> Default for Manager<Client> {
    fn default() -> Self {
        Self {
            middlewares: Middlewares::new(),
        }
    }
}
