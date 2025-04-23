use super::{And, Invert, Or};
use crate::{
    client::Reqwest,
    event::service::{service_fn, BoxCloneService},
    Request,
};

use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use std::{convert::Infallible, future::Future};

pub type BoxedCloneFilterService<Client> =
    BoxCloneService<Request<Client>, (bool, Request<Client>), Infallible>;

/// Filters are used to filter updates before processing handlers and inner middlewares.
/// You can use filters to check if the update meets the necessary conditions,
/// and if it does, the update will be processed by the handler(s) and/or inner middleware(s).
/// # Notes
/// Check out the examples to see how to create your own filters and check ready-made implementations of filters
/// to avoid writing your own filters which are already implemented.
#[async_trait]
pub trait Filter<Client = Reqwest>: DynClone + Send + Sync + 'static {
    /// Check if the filter passes
    /// # Returns
    /// `true` if the filter passes, otherwise `false`
    async fn check(&mut self, request: &mut Request<Client>) -> bool;

    /// Invert result of the filter
    /// # Notes
    /// This method is used to create [`Invert`] filter
    fn invert(self) -> Invert<Client>
    where
        Self: Sized,
        Client: Send + Sync + 'static,
    {
        Invert::new(self)
    }

    /// Combine two filters with logical `and`
    /// # Notes
    /// This method is used to create [`And`] filter
    fn and(self, filter: impl Filter<Client>) -> And<Client>
    where
        Self: Sized,
        Client: Send + Sync + 'static,
    {
        And::new(self).and(filter)
    }

    /// Combine two filters with logical `or`
    /// # Notes
    /// This method is used to create [`Or`] filter
    fn or(self, filter: impl Filter<Client>) -> Or<Client>
    where
        Self: Sized,
        Client: Send + Sync + 'static,
    {
        Or::new(self).or(filter)
    }
}

clone_trait_object!(<Client> Filter<Client>);

#[async_trait]
impl<Client, F, Fut> Filter<Client> for F
where
    Client: Send + Sync + 'static,
    F: FnMut(&mut Request<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send,
{
    async fn check(&mut self, request: &mut Request<Client>) -> bool {
        self(request).await
    }
}

pub fn boxed_filter_factory<Client, F>(filter: F) -> BoxedCloneFilterService<Client>
where
    Client: Send + Sync + 'static,
    F: Filter<Client>,
{
    let filter = Box::new(filter) as Box<dyn Filter<Client>>;

    BoxCloneService::new(service_fn(move |mut request| {
        let mut filter = filter.clone();

        async move {
            let result = filter.check(&mut request).await;
            Ok((result, request))
        }
    }))
}
