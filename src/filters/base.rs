use crate::{client::Bot, context::Context, types::Update};

use std::{cell::RefCell, fmt::Debug};

/// A base filter trait for filters
pub trait Filter: Debug {
    /// Check if the filter pass
    fn check(&self, _: &Bot, _: &Update, _: &RefCell<Context>) -> bool;
}
