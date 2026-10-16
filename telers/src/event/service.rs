//! Service abstraction shared by handlers, filters and middlewares.
//!
//! It re-exports the `Service` trait of `tower` together with its boxed cloneable adapter and
//! `service_fn`, so a processing unit can be stored and cloned behind one erased type.

mod base;
mod boxed_clone;
mod service_fn;

pub(crate) use base::Service;
pub(crate) use boxed_clone::BoxCloneService;
pub(crate) use service_fn::service_fn;
