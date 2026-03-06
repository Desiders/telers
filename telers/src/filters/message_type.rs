use super::base::Filter;
use crate::{enums, Request};

use std::{convert::Infallible, future::Future};

#[derive(Debug, Clone)]
pub struct MessageType<const N: usize> {
    types: [enums::MessageType; N],
}

impl MessageType<1> {
    pub fn one(content_type: impl Into<enums::MessageType>) -> Self {
        Self {
            types: [content_type.into(); 1],
        }
    }
}

impl<const N: usize> MessageType<N> {
    pub fn many(types: impl Into<[enums::MessageType; N]>) -> Self {
        Self {
            types: types.into(),
        }
    }
}

impl<const N: usize> MessageType<N> {
    #[must_use]
    pub fn validate(&self, content_type: enums::MessageType) -> bool {
        self.types.contains(&content_type)
    }
}

impl<Client, const N: usize> Filter<Client> for MessageType<N>
where
    Client: Send,
{
    type Error = Infallible;

    fn check(
        &mut self,
        request: &mut Request<Client>,
    ) -> impl Future<Output = Result<bool, Self::Error>> {
        let res = match request.update.message() {
            Some(message) => self.validate(enums::MessageType::from(message)),
            None => false,
        };
        async move { Ok(res) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_type() {
        let filter = MessageType::many([enums::MessageType::Text, enums::MessageType::Photo]);

        assert!(filter.validate(enums::MessageType::Text));
        assert!(filter.validate(enums::MessageType::Photo));
        assert!(!filter.validate(enums::MessageType::Audio));
    }
}
