use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;

/// This trait represents a filter
///
/// If the filter returns `true`, then the handler will be executed, otherwise it will not be executed. \
/// You can use this trait to create your own filters.
#[async_trait]
pub trait Filter<Client>: Send + Sync {
    /// Check if the filter pass
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool;
}
