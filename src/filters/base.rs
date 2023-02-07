use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;

/// A base filter trait for filters
#[async_trait]
pub trait Filter: Send + Sync {
    /// Check if the filter pass
    async fn check(&self, _bot: &Bot, _update: &Update, _context: &Context) -> bool;
}
