use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InlineKeyboardMarkup, InputChecklist, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit a checklist on behalf of a connected business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagechecklist>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditMessageChecklist {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: String,
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Unique identifier for the target message
    pub message_id: i64,
    /// A JSON-serialized object for the new checklist
    pub checklist: InputChecklist,
    /// A JSON-serialized object for the new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) for the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageChecklist {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
        checklist: InputChecklist,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
            checklist,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: i64) -> Self {
        Self {
            chat_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn checklist(self, val: InputChecklist) -> Self {
        Self {
            checklist: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl EditMessageChecklist {
    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageChecklist {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&'_ self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("editMessageChecklist", self, None)
    }
}

impl AsRef<EditMessageChecklist> for EditMessageChecklist {
    fn as_ref(&self) -> &Self {
        self
    }
}
