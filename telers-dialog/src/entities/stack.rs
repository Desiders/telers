use serde::{Deserialize, Serialize};
use telers::enums::MessageType;

use crate::entities::{AccessSettings, Context, Data};

pub const DEFAULT_STACK_ID: &str = "";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stack {
    pub id: String,
    pub intents: Vec<String>,
    pub last_message_id: Option<i64>,
    pub last_text: Option<Box<str>>,
    pub last_reply_keyboard: bool,
    pub last_reply_markup: Option<serde_json::Value>,
    pub last_link_preview_options: Option<serde_json::Value>,
    pub last_media_id: Option<String>,
    pub last_media_unique_id: Option<String>,
    pub last_income_media_group_id: Option<String>,
    pub message_type: Option<MessageType>,
    pub access_settings: Option<AccessSettings>,
    pub has_protected_content: Option<bool>,
}

impl Stack {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: DEFAULT_STACK_ID.to_string(),
            ..Self::default()
        }
    }

    #[must_use]
    pub fn push(&mut self, state: impl Into<String>, data: Data) -> Context {
        let ctx = Context::new(self.id.clone(), state, data);
        self.intents.push(ctx.id.clone());
        ctx
    }

    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<String> {
        self.intents.pop()
    }

    #[inline]
    #[must_use]
    pub fn last_intent_id(&self) -> Option<&str> {
        self.intents.last().map(String::as_str)
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
    }

    pub fn clear_last_message(&mut self) {
        self.last_message_id = None;
        self.last_text = None;
        self.last_reply_keyboard = false;
        self.last_reply_markup = None;
        self.last_link_preview_options = None;
        self.last_media_id = None;
        self.last_media_unique_id = None;
        self.last_income_media_group_id = None;
        self.message_type = None;
        self.has_protected_content = None;
    }
}
