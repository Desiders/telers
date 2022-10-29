use crate::types::User;

/// Represents a bot
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bot {}

impl Bot {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    #[must_use]
    pub fn get_me(&self) -> User {
        todo!()
    }
}
