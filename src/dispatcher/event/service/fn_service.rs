use super::base::{Service, ServiceFactory};

use futures::future::Future;
use std::marker::PhantomData;

/// Create [`ServiceFactory`] for function that can act as a [`Service`]
pub fn fn_service<F, Fut, Req, Res, Err, Cfg>(f: F) -> FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    FnServiceFactory::new(f)
}

/// Create [`ServiceFactory`] for function that can produce services
pub fn fn_factory_no_config<F, Cfg, Srv, Req, Err>(f: F) -> FnServiceNoConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn() -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    FnServiceNoConfig::new(f)
}

/// Create [`ServiceFactory`] for function that accepts config argument and can produce services
pub fn fn_factory_config<F, Cfg, Srv, Req, Err>(f: F) -> FnServiceConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    FnServiceConfig::new(f)
}

#[derive(Clone)]
pub struct FnService<F, Fut, Req, Res, Err>
where
    F: FnMut(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    f: F,
    phantom: PhantomData<Req>,
}

impl<F, Fut, Req, Res, Err> FnService<F, Fut, Req, Res, Err>
where
    F: FnMut(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    pub(crate) fn new(f: F) -> Self {
        Self {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, Fut, Req, Res, Err> Service<Req> for FnService<F, Fut, Req, Res, Err>
where
    F: Fn(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, req: Req) -> Self::Future {
        (self.f)(req)
    }
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    f: F,
    phantom: PhantomData<(Req, Cfg)>,
}

impl<F, Fut, Req, Res, Err, Cfg> FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    fn new(f: F) -> Self {
        FnServiceFactory {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, Fut, Req, Res, Err> Service<Req> for FnServiceFactory<F, Fut, Req, Res, Err, ()>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;
    type Future = Fut;

    fn call(&self, req: Req) -> Self::Future {
        (self.f)(req)
    }
}

impl<F, Fut, Req, Res, Err, Cfg> ServiceFactory<Req>
    for FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    type Response = Res;
    type Error = Err;

    type Config = Cfg;
    type Service = FnService<F, Fut, Req, Res, Err>;
    type InitError = ();

    fn new_service(&self, _: Cfg) -> Result<Self::Service, Self::InitError> {
        Ok(FnService::new(self.f.clone()))
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct FnServiceConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    f: F,
    phantom: PhantomData<(Cfg, Req)>,
}

impl<F, Cfg, Srv, Req, Err> FnServiceConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    fn new(f: F) -> Self {
        FnServiceConfig {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, Cfg, Srv, Req, Err> ServiceFactory<Req> for FnServiceConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    type Response = Srv::Response;
    type Error = Srv::Error;

    type Config = Cfg;
    type Service = Srv;
    type InitError = Err;

    fn new_service(&self, cfg: Cfg) -> Result<Self::Service, Self::InitError> {
        (self.f)(cfg)
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct FnServiceNoConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn() -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    f: F,
    phantom: PhantomData<(Cfg, Req)>,
}

impl<F, Cfg, Srv, Req, Err> FnServiceNoConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn() -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    fn new(f: F) -> Self {
        Self {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, Cfg, Srv, Req, Err> ServiceFactory<Req> for FnServiceNoConfig<F, Cfg, Srv, Req, Err>
where
    F: Fn() -> Result<Srv, Err>,
    Srv: Service<Req>,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Config = Cfg;
    type Service = Srv;
    type InitError = Err;

    fn new_service(&self, _: Cfg) -> Result<Self::Service, Self::InitError> {
        (self.f)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tokio;

    #[tokio::test]
    async fn test_fn_service() {
        let req = ();
        let config = ();

        let service_factory_or_service =
            fn_service(|req| async move { Ok::<_, ()>(("test", req)) });

        // Use as a service factory
        let factory = service_factory_or_service.new_service(config).unwrap();
        let result = factory.call(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", req));

        // Use as a service
        let service = service_factory_or_service;
        let result = service.call(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", req));
    }

    #[tokio::test]
    async fn test_fn_factory_no_config() {
        let req = ();
        let config = ();

        let service_factory = fn_factory_no_config(|| {
            let service_factory_or_service =
                fn_service(|req| async move { Ok::<_, ()>(("test", req)) });

            Ok::<_, ()>(service_factory_or_service)
        });

        let service = service_factory.new_service(config).unwrap();
        let result = service.call(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", req));
    }

    #[tokio::test]
    async fn test_fn_factory_config() {
        let req = ();
        let config = ();

        let service_factory = fn_factory_config(|config| {
            let service_factory_or_service =
                fn_service(move |req| async move { Ok::<_, ()>(("test", config, req)) });

            Ok::<_, ()>(service_factory_or_service)
        });

        let service = service_factory.new_service(config).unwrap();
        let result = service.call(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", config, req));
    }
}
