use crate::{client::Bot, types::Update};

pub type BoxFilter = Box<dyn Filter>;

pub trait Filter {
    /// Returns `true` if the update should be handled by the handler.
    /// Returns `false` if didn't pass all set filters.
    fn check(&self, _: &Bot, _: &Update) -> bool;
}
