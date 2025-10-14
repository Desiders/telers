use super::base::Filter;
use crate::{enums::ContentType as ContentTypeEnum, Request};

use std::future::Future;

#[derive(Debug, Clone)]
pub struct ContentType<const N: usize> {
    content_types: [ContentTypeEnum; N],
}

impl ContentType<1> {
    pub fn one(content_type: impl Into<ContentTypeEnum>) -> Self {
        Self {
            content_types: [content_type.into(); 1],
        }
    }
}

impl<const N: usize> ContentType<N> {
    pub fn many(content_types: impl Into<[ContentTypeEnum; N]>) -> Self {
        Self {
            content_types: content_types.into(),
        }
    }
}

impl<const N: usize> ContentType<N> {
    #[must_use]
    pub fn validate(&self, content_type: ContentTypeEnum) -> bool {
        self.content_types
            .iter()
            .any(|allowed_content_type| *allowed_content_type == content_type)
    }
}

impl<Client, const N: usize> Filter<Client> for ContentType<N>
where
    Client: Send,
{
    fn check(&mut self, request: &mut Request<Client>) -> impl Future<Output = bool> {
        let res = match request.update.message() {
            Some(message) => self.validate(ContentTypeEnum::from(message)),
            None => false,
        };
        async move { res }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_type() {
        let filter = ContentType::many([ContentTypeEnum::Text, ContentTypeEnum::Photo]);

        assert!(filter.validate(ContentTypeEnum::Text));
        assert!(filter.validate(ContentTypeEnum::Photo));
        assert!(!filter.validate(ContentTypeEnum::Audio));
    }
}
