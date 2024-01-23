use super::base::{Service, ServiceFactory};

use std::{future::Future, pin::Pin};

pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub type BoxService<Req, Res, Err> = Box<
    dyn Service<Req, Response = Res, Error = Err, Future = BoxFuture<Result<Res, Err>>>
        + Send
        + Sync,
>;

pub fn service<S, Req>(service: S) -> BoxService<Req, S::Response, S::Error>
where
    S: Service<Req> + Send + Sync + 'static,
    S::Future: Send + 'static,
{
    Box::new(ServiceWrapper::new(service))
}

struct ServiceWrapper<S> {
    inner: S,
}

impl<S> ServiceWrapper<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, Req, Res, Err> Service<Req> for ServiceWrapper<S>
where
    S: Service<Req, Response = Res, Error = Err> + Send + Sync,
    S::Future: Send + 'static,
{
    type Response = Res;
    type Error = Err;
    type Future = BoxFuture<Result<Res, Err>>;

    fn call(&self, request: Req) -> Self::Future {
        Box::pin(self.inner.call(request))
    }
}

/// Wrapper for a service factory that will map it's services to boxed trait object services
pub struct BoxServiceFactory<Cfg, Req, Res, Err, InitErr>(Inner<Cfg, Req, Res, Err, InitErr>);

type Inner<Cfg, Req, Res, Err, InitErr> = Box<
    dyn ServiceFactory<
            Req,
            Config = Cfg,
            Response = Res,
            Error = Err,
            InitError = InitErr,
            Service = BoxService<Req, Res, Err>,
        > + Send
        + Sync,
>;

impl<C, Req, Res, Err, InitErr> ServiceFactory<Req>
    for BoxServiceFactory<C, Req, Res, Err, InitErr>
{
    type Response = Res;
    type Error = Err;
    type Config = C;
    type Service = BoxService<Req, Res, Err>;
    type InitError = InitErr;

    fn new_service(&self, config: Self::Config) -> Result<Self::Service, Self::InitError> {
        self.0.new_service(config)
    }
}

/// Wraps a service factory that returns service trait objects
pub fn factory<SF, Req>(
    factory: SF,
) -> BoxServiceFactory<SF::Config, Req, SF::Response, SF::Error, SF::InitError>
where
    Req: 'static,
    SF: ServiceFactory<Req> + Send + Sync + 'static,
    SF::Service: Send + Sync,
    <SF::Service as Service<Req>>::Future: Send + 'static,
{
    BoxServiceFactory(Box::new(FactoryWrapper::new(factory)))
}

struct FactoryWrapper<SF> {
    inner: SF,
}

impl<SF> FactoryWrapper<SF> {
    fn new(inner: SF) -> Self {
        Self { inner }
    }
}

impl<SF, Req, Cfg, Res, Err, InitErr> ServiceFactory<Req> for FactoryWrapper<SF>
where
    SF: ServiceFactory<Req, Config = Cfg, Response = Res, Error = Err, InitError = InitErr>,
    SF::Service: Send + Sync + 'static,
    <SF::Service as Service<Req>>::Future: Send + 'static,
{
    type Response = Res;
    type Error = Err;
    type Config = Cfg;
    type Service = BoxService<Req, Res, Err>;
    type InitError = InitErr;

    fn new_service(&self, config: Self::Config) -> Result<Self::Service, Self::InitError> {
        self.inner
            .new_service(config)
            .map(|service| Box::new(ServiceWrapper::new(service)) as _)
    }
}
