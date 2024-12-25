use super::{And, Invert, Or};
use crate::{client::Reqwest, Request};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

/// Filters are used to filter updates before processing handlers and inner middlewares.
/// You can use filters to check if the update meets the necessary conditions,
/// and if it does, the update will be processed by the handler(s) and/or inner middleware(s).
/// # Notes
/// Check out the examples to see how to create your own filters and check ready-made implementations of filters
/// to avoid writing your own filters which are already implemented.
#[async_trait]
pub trait Filter<Client = Reqwest>: Send + Sync {
    /// Check if the filter passes
    /// # Returns
    /// `true` if the filter passes, otherwise `false`
    async fn check(&self, request: &mut Request<Client>) -> bool;

    /// Invert result of the filter
    /// # Notes
    /// This method is used to create [`Invert`] filter
    fn invert(self) -> Invert<Client>
    where
        Self: Sized + 'static,
    {
        Invert::new(self)
    }

    /// Combine two filters with logical `and`
    /// # Notes
    /// This method is used to create [`And`] filter
    fn and(self, filter: impl Filter<Client> + 'static) -> And<Client>
    where
        Self: Sized + 'static,
    {
        And::new(self).and(filter)
    }

    /// Combine two filters with logical `or`
    /// # Notes
    /// This method is used to create [`Or`] filter
    fn or(self, filter: impl Filter<Client> + 'static) -> Or<Client>
    where
        Self: Sized + 'static,
    {
        Or::new(self).or(filter)
    }
}

#[async_trait]
impl<T: ?Sized, Client> Filter<Client> for Arc<T>
where
    T: Filter<Client>,
    Client: Send + Sync,
{
    async fn check(&self, request: &mut Request<Client>) -> bool {
        T::check(self, request).await
    }
}

/// To possible use function-like as filters
#[async_trait]
impl<Client, Func, Fut> Filter<Client> for Func
where
    Client: Send + Sync,
    Func: Fn(&mut Request<Client>) -> Fut + Send + Sync,
    Fut: Future<Output = bool> + Send,
{
    async fn check(&self, request: &mut Request<Client>) -> bool {
        self(request).await
    }
}
