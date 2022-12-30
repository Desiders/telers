#![allow(clippy::module_name_repetitions)]

mod base;
mod boxed;
mod fn_service;

pub use base::{Service, ServiceFactory, ServiceProvider, ToServiceProvider};
pub use boxed::{factory, service, BoxFuture, BoxService, BoxServiceFactory};
pub use fn_service::{
    fn_factory_config, fn_factory_no_config, fn_service, FnService, FnServiceConfig,
    FnServiceFactory, FnServiceNoConfig,
};
