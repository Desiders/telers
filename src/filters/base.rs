use crate::{client::Bot, context::Context, types::Update};

use std::cell::RefCell;

/// A base filter trait for filters
pub trait Filter {
    /// Check if the filter pass
    fn check(&self, _: &Bot, _: &Update, _: &RefCell<Context>) -> bool;
}
