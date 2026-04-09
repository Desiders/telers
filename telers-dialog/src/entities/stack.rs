use serde::{Deserialize, Serialize};
use telers::enums::MessageType;

use crate::entities::{AccessSettings, Context, Data};

/// Identifier of the default dialog stack.
pub const DEFAULT_STACK_ID: &str = "";

/// Persisted dialog stack and message snapshot state.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Stack {
    /// Stack id.
    pub id: String,
    /// Context ids in stack order.
    pub intents: Vec<String>,
    /// Last dialog message id.
    pub last_message_id: Option<i64>,
    /// Last dialog text snapshot.
    pub last_text: Option<Box<str>>,
    /// Whether the last reply markup was a reply keyboard.
    pub last_reply_keyboard: bool,
    /// Serialized reply markup snapshot.
    pub last_reply_markup: Option<serde_json::Value>,
    /// Serialized link preview options snapshot.
    pub last_link_preview_options: Option<serde_json::Value>,
    /// Last known media file id, when tracked.
    pub last_media_id: Option<String>,
    /// Last known media unique id, when tracked.
    pub last_media_unique_id: Option<String>,
    /// Last inbound media group id seen in this chat.
    pub last_income_media_group_id: Option<String>,
    /// Last telegram message type.
    pub message_type: Option<MessageType>,
    /// Stack-wide access settings fallback.
    pub access_settings: Option<AccessSettings>,
    /// Protected-content flag of the last dialog message.
    pub has_protected_content: Option<bool>,
}

impl Stack {
    /// Create an empty default stack.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: DEFAULT_STACK_ID.to_string(),
            ..Self::default()
        }
    }

    /// Push a new dialog context onto the stack and return it.
    #[must_use]
    pub fn push(&mut self, state: impl Into<String>, data: Data) -> Context {
        let ctx = Context::new(self.id.clone(), state, data);
        self.intents.push(ctx.id.clone());
        ctx
    }

    /// Pop the top context id from the stack.
    #[inline]
    #[must_use]
    pub fn pop(&mut self) -> Option<String> {
        self.intents.pop()
    }

    /// Return the current top context id.
    #[inline]
    #[must_use]
    pub fn last_intent_id(&self) -> Option<&str> {
        self.intents.last().map(String::as_str)
    }

    /// Return `true` when the stack has no active contexts.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.intents.is_empty()
    }

    /// Clear the cached snapshot of the last shown dialog message.
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
