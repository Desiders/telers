use crate::{client::Bot, context::Context, types::Update};

use std::cell::RefCell;

pub type BoxFilter = Box<dyn Filter>;

/// A base filter trait for filters
pub trait Filter {
    /// Check if the filter pass.
    /// # Arguments
    /// * `bot` - [Bot] instance
    /// * `update` - [Update] instance
    /// * `context` - [Context] instance
    /// # Returns
    /// `true` if the filter pass, otherwise `false`.
    fn check(&self, _: &Bot, _: &Update, _: &RefCell<Context>) -> bool;
}
