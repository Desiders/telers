use crate::{client::Bot, types::Update};

pub type BoxFilter<E> = Box<dyn Filter<Event = E>>;

pub trait Filter {
    type Event: From<Update>;

    /// Returns `true` if the update should be handled by the handler.
    /// Returns `false` if didn't pass all set filters.
    fn check(&self, _: &Bot, _: &Self::Event) -> bool;
}
