use crate::{client::Bot, context::Context, types::Update};

/// A base filter trait for filters
pub trait Filter: Send + Sync {
    /// Check if the filter pass
    fn check(&self, _: &Bot, _: &Update, _: &Context) -> bool;
}
