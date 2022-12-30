use std::{future::Future, sync::Arc};

pub trait Service<Req> {
    /// Response given by the service
    type Response;

    /// Error produced by the service when executing call
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Process the event and return the asynchronous response
    #[must_use]
    fn call(&self, req: Req) -> Self::Future;
}

impl<'a, S, Req> Service<Req> for &'a S
where
    S: Service<Req> + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, req: Req) -> S::Future {
        (**self).call(req)
    }
}

impl<S, Req> Service<Req> for Box<S>
where
    S: Service<Req> + ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn call(&self, req: Req) -> S::Future {
        (**self).call(req)
    }
}

pub trait ServiceFactory<Req> {
    /// Response given by the service
    type Response;

    /// Error produced by the service when executing call
    type Error;

    /// Service factory configuration
    type Config;

    /// The kind of `Service` created by this factory
    type Service: Service<Req, Response = Self::Response, Error = Self::Error>;

    /// Errors potentially raised while building a service
    type InitError;

    /// Create and return a new service
    /// # Errors
    /// If the service cannot be created by the factory
    fn new_service(&self, cfg: Self::Config) -> Result<Self::Service, Self::InitError>;
}

/// A marker trait, which means that it doesn't have any methods.
/// It is used to mark service containers, which will be used to provide [`Service`]s.
pub trait ServiceProvider {}

impl<S> ServiceProvider for Arc<S> where S: ServiceProvider {}

/// A trait that allows to convert structures, that contains [`ServiceFactory`]s, to [`ServiceProvider`],
/// which will contains [`Service`]s instead of [`ServiceFactory`]s.
/// In other words, it allows to build services from factories.
pub trait ToServiceProvider {
    /// Service factory configuration
    type Config;

    /// The provider to which the service will be converted
    type ServiceProvider: ServiceProvider;

    /// Errors potentially raised while building a service
    type InitError;

    /// Convert the service factory to the service.
    /// Call [`ServiceFactory::new_service`] for each factory and return the [`ServiceProvider`].
    /// # Errors
    /// If the service cannot be created by the factory
    fn to_service_provider(
        self,
        cfg: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError>;
}
