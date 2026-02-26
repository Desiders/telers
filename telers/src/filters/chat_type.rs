use super::base::Filter;
use crate::{enums, Request};

use std::future::Future;

#[derive(Debug, Clone)]
pub struct ChatType<const N: usize> {
    chat_types: [enums::ChatType; N],
}

impl ChatType<1> {
    pub fn one(chat_type: impl Into<enums::ChatType>) -> Self {
        Self {
            chat_types: [chat_type.into(); 1],
        }
    }
}

impl<const N: usize> ChatType<N> {
    pub fn many(chat_types: impl Into<[enums::ChatType; N]>) -> Self {
        Self {
            chat_types: chat_types.into(),
        }
    }
}

impl<const N: usize> ChatType<N> {
    #[must_use]
    pub fn validate(&self, chat_type: enums::ChatType) -> bool {
        self.chat_types.contains(&chat_type)
    }
}

impl<Client, const N: usize> Filter<Client> for ChatType<N>
where
    Client: Send,
{
    fn check(&mut self, request: &mut Request<Client>) -> impl Future<Output = bool> {
        let res = match request.update.chat() {
            Some(chat) => self.validate(enums::ChatType::from(chat)),
            None => false,
        };
        async move { res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_type() {
        let filter = ChatType::many([enums::ChatType::Private, enums::ChatType::Supergroup]);

        assert!(filter.validate(enums::ChatType::Private));
        assert!(filter.validate(enums::ChatType::Supergroup));
        assert!(!filter.validate(enums::ChatType::Channel));
    }
}
