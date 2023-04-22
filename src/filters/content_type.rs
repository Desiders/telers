use super::base::Filter;

use crate::{client::Bot, context::Context, enums::ContentType as ContentTypeEnum, types::Update};

use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ContentType {
    content_types: Vec<ContentTypeEnum>,
}

impl ContentType {
    #[must_use]
    pub fn one(content_type: impl Into<ContentTypeEnum>) -> Self {
        Self {
            content_types: vec![content_type.into()],
        }
    }

    #[must_use]
    pub fn many<T, I>(content_types: I) -> Self
    where
        T: Into<ContentTypeEnum>,
        I: IntoIterator<Item = T>,
    {
        Self {
            content_types: content_types.into_iter().map(Into::into).collect(),
        }
    }
}

impl ContentType {
    #[must_use]
    pub fn validate_content_type(&self, content_type: ContentTypeEnum) -> bool {
        self.content_types
            .iter()
            .any(|allowed_content_type| allowed_content_type == &content_type)
    }
}

#[async_trait]
impl<Client> Filter<Client> for ContentType {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        update.message.as_ref().map_or(false, |message| {
            self.validate_content_type(message.content_type())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_type() {
        let filter = ContentType::many([ContentTypeEnum::Text, ContentTypeEnum::Photo]);

        assert!(filter.validate_content_type(ContentTypeEnum::Text));
        assert!(filter.validate_content_type(ContentTypeEnum::Photo));
        assert!(!filter.validate_content_type(ContentTypeEnum::Audio));
    }
}
