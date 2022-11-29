use crate::{client::Bot, context::Context, types::Update};

use std::sync::RwLock;

/// A base filter trait for filters
pub trait Filter: Send + Sync {
    /// Check if the filter pass
    fn check(&self, _: &Bot, _: &Update, _: &RwLock<Context>) -> bool;
}
