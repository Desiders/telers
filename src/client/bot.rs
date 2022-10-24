use crate::types::User;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bot {}

impl Bot {
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }

    #[must_use]
    pub fn get_me(&self) -> User {
        todo!()
    }
}
