use crate::{client::Bot, types::Update};

pub trait Filter {
    fn check(&self, bot: &Bot, update: &Update) -> bool;
}
