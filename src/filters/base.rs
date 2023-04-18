use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;
use std::{future::Future, sync::Arc};

/// This trait represents a filter
///
/// If the filter returns `true`, then the handler will be executed, otherwise it will not be executed. \
/// You can use this trait to create your own filters.
#[async_trait]
pub trait Filter<Client>: Send + Sync {
    /// Check if the filter pass
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool;
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
