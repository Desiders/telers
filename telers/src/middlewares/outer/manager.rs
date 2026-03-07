use super::base::{boxed_middleware_factory, BoxedCloneMiddlewareService, Middleware};

pub struct Manager<Client> {
    pub middlewares: Vec<BoxedCloneMiddlewareService<Client>>,
}

impl<Client> Manager<Client> {
    /// Register middleware in the end of the list
    pub fn register(&mut self, middleware: impl Middleware<Client>) -> &mut Self
    where
        Client: Send + Sync + 'static,
    {
        self.middlewares.push(boxed_middleware_factory(middleware));
        self
    }

    /// Register boxed middleware in the end of the list
    pub fn register_boxed(&mut self, middleware: BoxedCloneMiddlewareService<Client>) -> &mut Self {
        self.middlewares.push(middleware);
        self
    }

    /// Register middleware at the specified position
    /// # Warning
    /// Not recommended to use this method. Use it only if you know what you are doing. \
    /// You can break the order of middlewares, which can lead to unexpected behaviour for some middlewares,
    /// which depends on the order of middlewares.
    pub fn register_at_position(
        &mut self,
        index: usize,
        middleware: impl Middleware<Client>,
    ) -> &mut Self
    where
        Client: Send + Sync + 'static,
    {
        self.middlewares
            .insert(index, boxed_middleware_factory(middleware));
        self
    }

    /// Register boxed middleware at the specified position
    /// # Warning
    /// Not recommended to use this method. Use it only if you know what you are doing. \
    /// You can break the order of middlewares, which can lead to unexpected behaviour for some middlewares,
    /// which depends on the order of middlewares.
    pub fn register_boxed_at_position(
        &mut self,
        index: usize,
        middleware: BoxedCloneMiddlewareService<Client>,
    ) -> &mut Self {
        self.middlewares.insert(index, middleware);
        self
    }
}

impl<Client> Default for Manager<Client> {
    fn default() -> Self {
        Self {
            middlewares: vec![],
        }
    }
}

impl<Client> Clone for Manager<Client> {
    fn clone(&self) -> Self {
        Self {
            middlewares: self.middlewares.clone(),
        }
    }
}
