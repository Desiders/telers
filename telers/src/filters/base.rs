use super::{And, Invert, Or};

use crate::{
    client::{Bot, Reqwest},
    context::Context,
    types::Update,
};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

#[async_trait]
pub trait Filter<Client = Reqwest>: Send + Sync {
    /// Check if the filter passes
    /// # Returns
    /// `true` if the filter passes, otherwise `false`
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool;

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
    Client: Sync,
{
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        T::check(self, bot, update, context).await
    }
}

/// To possible use function-like as filters
#[async_trait]
impl<Client, Func, Fut> Filter<Client> for Func
where
    Client: Sync,
    Func: Fn(&Bot<Client>, &Update, &Context) -> Fut + Send + Sync,
    Fut: Future<Output = bool> + Send,
{
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        self(bot, update, context).await
    }
}
