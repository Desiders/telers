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

    fn new_service(&self, _config: Cfg) -> Result<Self::Service, Self::InitError> {
        Ok(FnService::new(self.f.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tokio;

    #[derive(Debug, Eq, PartialEq)]
    struct TestRequest;

    #[derive(Debug, Eq, PartialEq)]
    struct TestConfig;

    #[tokio::test]
    async fn test_fn_service() {
        let request = TestRequest;
        let config = TestConfig;

        let service_factory =
            fn_service(|reques| async move { Ok::<_, TestConfig>(("test", reques)) });

        let service = service_factory.new_service(config).unwrap();
        let result = service.call(&request).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ("test", &request));
    }
}
