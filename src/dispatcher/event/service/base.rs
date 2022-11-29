use std::future::Future;

pub trait Service<Req>: Send + Sync {
    /// Response given by the service
    type Response;

    /// Error produced by the service when executing call
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Process the event and return the asynchronous response
    #[must_use]
    fn call(&self, req: Req) -> Self::Future;
}

pub trait ServiceFactory<Req>: Send + Sync {
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

    /// The future response value
    type Future: Future<Output = Result<Self::Service, Self::InitError>> + Send + Sync;

    /// Create and return a new service asynchronously
    #[must_use]
    fn new_service(&self, cfg: Self::Config) -> Self::Future;
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
