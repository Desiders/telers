use super::{And, Invert, Or};
use crate::{
    client::Reqwest,
    event::service::{service_fn, BoxCloneService},
    Request,
};

use std::{convert::Infallible, future::Future};

pub type BoxedCloneFilterService<Client> =
    BoxCloneService<Request<Client>, (bool, Request<Client>), Infallible>;

/// Filters are used to filter updates before processing handlers and inner middlewares.
/// You can use filters to check if the update meets the necessary conditions,
/// and if it does, the update will be processed by the handler(s) and/or inner middleware(s).
/// # Notes
/// Check out the examples to see how to create your own filters and check ready-made implementations of filters
/// to avoid writing your own filters which are already implemented.
pub trait Filter<Client = Reqwest>: Clone + Send + Sync + 'static {
    /// Check if the filter passes
    /// # Returns
    /// `true` if the filter passes, otherwise `false`
    fn check(&mut self, request: &mut Request<Client>) -> impl Future<Output = bool> + Send;

    /// Invert result of the filter
    /// # Notes
    /// This method is used to create [`Invert`] filter
    fn invert(self) -> impl Filter<Client>
    where
        Client: Send,
    {
        Invert(self)
    }

    /// Combine two filters with logical `and`
    /// # Notes
    /// This method is used to create [`And`] filter
    fn and<F>(self, filter: F) -> impl Filter<Client>
    where
        F: Filter<Client>,
        Client: Send,
    {
        And(self, filter)
    }

    /// Combine two filters with logical `or`
    /// # Notes
    /// This method is used to create [`Or`] filter
    fn or<F>(self, filter: F) -> impl Filter<Client>
    where
        F: Filter<Client>,
        Client: Send,
    {
        Or(self, filter)
    }
}

impl<Client, F, Fut> Filter<Client> for F
where
    Client: Send + Sync + 'static,
    F: FnMut(&mut Request<Client>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = bool> + Send,
{
    fn check(&mut self, request: &mut Request<Client>) -> impl Future<Output = bool> + Send {
        self(request)
    }
}

pub fn boxed_filter_factory<Client, F>(filter: F) -> BoxedCloneFilterService<Client>
where
    Client: Send + Sync + 'static,
    F: Filter<Client>,
{
    BoxCloneService::new(service_fn(move |mut request| {
        let mut filter = filter.clone();

        async move {
            let result = filter.check(&mut request).await;
            Ok((result, request))
        }
    }))
}
