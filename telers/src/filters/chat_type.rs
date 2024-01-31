use super::base::Filter;

use crate::{client::Bot, context::Context, enums::ChatType as ChatTypeEnum, types::Update};

use async_trait::async_trait;

/// Filter for checking the type of chat
#[derive(Debug, Clone)]
pub struct ChatType {
    chat_types: Box<[ChatTypeEnum]>,
}

impl ChatType {
    /// Creates a new [`ChatType`] filter with one allowed chat type.
    /// # Notes
    /// You can use [`ChatTypeEnum`] or its string representation.
    pub fn one(chat_type: impl Into<ChatTypeEnum>) -> Self {
        Self {
            chat_types: [chat_type.into()].into(),
        }
    }

    /// Creates a new [`ChatType`] filter with many allowed chat types.
    /// # Notes
    /// You can use [`ChatTypeEnum`] or its string representation.
    pub fn many<T, I>(chat_types: I) -> Self
    where
        T: Into<ChatTypeEnum>,
        I: IntoIterator<Item = T>,
    {
        Self {
            chat_types: chat_types.into_iter().map(Into::into).collect(),
        }
    }
}

impl ChatType {
    #[must_use]
    pub fn validate_chat_type(&self, chat_type: ChatTypeEnum) -> bool {
        self.chat_types
            .iter()
            .any(|allowed_chat_type| allowed_chat_type == &chat_type)
    }
}

#[async_trait]
impl<Client> Filter<Client> for ChatType {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        match update.chat() {
            Some(chat) => self.validate_chat_type(ChatTypeEnum::from(chat)),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_type() {
        let filter = ChatType::many([ChatTypeEnum::Private, ChatTypeEnum::Supergroup]);

        assert!(filter.validate_chat_type(ChatTypeEnum::Private));
        assert!(filter.validate_chat_type(ChatTypeEnum::Supergroup));
        assert!(!filter.validate_chat_type(ChatTypeEnum::Channel));
    }
}
