use super::base::{Service, ServiceFactory};

use futures::future::{ok, Future, Ready};
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
pub fn fn_factory_no_config<F, Cfg, Srv, Req, Fut, Err>(
    f: F,
) -> FnServiceNoConfig<F, Cfg, Srv, Req, Fut, Err>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    FnServiceNoConfig::new(f)
}

/// Create [`ServiceFactory`] for function that accepts config argument and can produce services
pub fn fn_factory_config<F, Fut, Cfg, Srv, Req, Err>(
    f: F,
) -> FnServiceConfig<F, Fut, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
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
    _t: PhantomData<fn(Req)>,
}

impl<F, Fut, Req, Res, Err> FnService<F, Fut, Req, Res, Err>
where
    F: FnMut(Req) -> Fut,
    Fut: Future<Output = Result<Res, Err>>,
{
    pub(crate) fn new(f: F) -> Self {
        Self { f, _t: PhantomData }
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
    _t: PhantomData<fn(Req, Cfg)>,
}

impl<F, Fut, Req, Res, Err, Cfg> FnServiceFactory<F, Fut, Req, Res, Err, Cfg>
where
    F: Fn(Req) -> Fut + Clone,
    Fut: Future<Output = Result<Res, Err>>,
{
    fn new(f: F) -> Self {
        FnServiceFactory { f, _t: PhantomData }
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
    type Future = Ready<Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Cfg) -> Self::Future {
        ok(FnService::new(self.f.clone()))
    }
}

/// Convert `Fn(&Config) -> Future<Service>` fn to new service
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FnServiceConfig<F, Fut, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    f: F,
    #[allow(clippy::type_complexity)]
    _t: PhantomData<fn(Cfg, Req) -> (Fut, Srv, Err)>,
}

impl<F, Fut, Cfg, Srv, Req, Err> FnServiceConfig<F, Fut, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    fn new(f: F) -> Self {
        FnServiceConfig { f, _t: PhantomData }
    }
}

impl<F, Fut, Cfg, Srv, Req, Err> ServiceFactory<Req> for FnServiceConfig<F, Fut, Cfg, Srv, Req, Err>
where
    F: Fn(Cfg) -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    type Response = Srv::Response;
    type Error = Srv::Error;

    type Config = Cfg;
    type Service = Srv;
    type InitError = Err;
    type Future = Fut;

    fn new_service(&self, cfg: Cfg) -> Self::Future {
        (self.f)(cfg)
    }
}

/// Converter for `Fn() -> Future<Service>` fn
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FnServiceNoConfig<F, Cfg, Srv, Req, Fut, Err>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    f: F,
    _t: PhantomData<fn(Cfg, Req)>,
}

impl<F, Cfg, Srv, Req, Fut, Err> FnServiceNoConfig<F, Cfg, Srv, Req, Fut, Err>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    fn new(f: F) -> Self {
        Self { f, _t: PhantomData }
    }
}

impl<F, Cfg, Srv, Req, Fut, Err> ServiceFactory<Req>
    for FnServiceNoConfig<F, Cfg, Srv, Req, Fut, Err>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<Srv, Err>>,
    Srv: Service<Req>,
{
    type Response = Srv::Response;
    type Error = Srv::Error;
    type Config = Cfg;
    type Service = Srv;
    type InitError = Err;
    type Future = Fut;

    fn new_service(&self, _: Cfg) -> Self::Future {
        (self.f)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use futures::future::ok;
    use tokio;

    #[tokio::test]
    async fn test_fn_service() {
        let service_factory_or_service = fn_service(|()| ok::<_, ()>("test"));

        let result = service_factory_or_service.call(()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");

        let service = service_factory_or_service.new_service(()).await.unwrap();
        let result = service.call(()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[tokio::test]
    async fn test_fn_service_factory_no_config() {
        let service_factory =
            fn_factory_no_config(|| ok::<_, ()>(fn_service(|()| ok::<_, ()>("test"))));

        let service = service_factory.new_service(()).await.unwrap();
        let result = service.call(()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[tokio::test]
    async fn test_fn_service_factory_config() {
        let service_factory = fn_factory_config(|config: ()| {
            ok::<_, ()>(fn_service(move |()| ok::<_, ()>(("test", config))))
        });

        let service = service_factory.new_service(()).await.unwrap();
        let result = service.call(()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", ()));
    }

    #[test]
    fn test_auto_impl_send_and_sync() {
        fn is_send_and_sync<T: Send + Sync + Clone>(_: &T) {}

        let service = FnService::new(|()| {
            type Error = ();

            ok::<_, Error>(())
        });

        is_send_and_sync(&service);
        is_send_and_sync(&service.clone());
    }
}
