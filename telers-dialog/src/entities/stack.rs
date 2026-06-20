use serde::{Deserialize, Serialize};
use telers::enums::{MessageType, ReplyMarkupType};

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
    /// Type of the last reply markup, when one was shown.
    ///
    /// Persisted so a transition can tell a `ForceReply`/`ReplyKeyboardRemove`
    /// message apart from an inline-keyboard one (the raw `last_reply_markup`
    /// value alone cannot distinguish them).
    #[serde(default)]
    pub last_reply_markup_type: Option<ReplyMarkupType>,
    /// Serialized reply markup snapshot.
    pub last_reply_markup: Option<serde_json::Value>,
    /// Serialized link preview options snapshot.
    pub last_link_preview_options: Option<serde_json::Value>,
    /// Last known media file id, when tracked.
    pub last_media_id: Option<String>,
    /// Last known media unique id, when tracked.
    pub last_media_unique_id: Option<String>,
    /// Last dialog media content type, when tracked.
    pub last_media_content_type: Option<MessageType>,
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
        self.last_reply_markup_type = None;
        self.last_reply_markup = None;
        self.last_link_preview_options = None;
        self.last_media_id = None;
        self.last_media_unique_id = None;
        self.last_media_content_type = None;
        self.last_income_media_group_id = None;
        self.message_type = None;
        self.has_protected_content = None;
    }
}

#[cfg(test)]
mod tests {
    use super::{Stack, DEFAULT_STACK_ID};
    use serde_json::Value;

    #[test]
    fn new_is_empty_with_default_id() {
        let stack = Stack::new();

        assert_eq!(stack.id, DEFAULT_STACK_ID);
        assert!(stack.is_empty());
        assert_eq!(stack.last_intent_id(), None);
        assert_eq!(stack.last_message_id, None);
        assert_eq!(stack.last_text, None);
        assert_eq!(stack.last_reply_markup_type, None);
        assert_eq!(stack.last_reply_markup, None);
        assert!(stack.access_settings.is_none());
    }

    #[test]
    fn push_returns_context_and_tracks_intent() {
        let mut stack = Stack::new();

        let ctx = stack.push("a", Value::Null);

        assert_eq!(ctx.state, "a");
        assert!(!stack.is_empty());
        assert_eq!(stack.last_intent_id(), Some(ctx.id.as_str()));
    }

    #[test]
    fn push_propagates_stack_id_to_context() {
        let mut stack = Stack::new();

        let ctx = stack.push("a", Value::Null);

        assert_eq!(ctx.stack_id, DEFAULT_STACK_ID);
    }

    #[test]
    fn pop_returns_most_recent_intent() {
        let mut stack = Stack::new();

        let first = stack.push("a", Value::Null);
        let second = stack.push("b", Value::Null);

        assert_eq!(stack.pop(), Some(second.id.clone()));
        assert_eq!(stack.last_intent_id(), Some(first.id.as_str()));
    }

    #[test]
    fn pop_on_fresh_stack_is_none() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_pop_behaves_as_lifo() {
        let mut stack = Stack::new();

        let a = stack.push("a", Value::Null);
        let b = stack.push("b", Value::Null);
        let c = stack.push("c", Value::Null);

        assert_eq!(stack.pop(), Some(c.id));
        assert_eq!(stack.pop(), Some(b.id));
        assert_eq!(stack.last_intent_id(), Some(a.id.as_str()));
        assert_eq!(stack.pop(), Some(a.id));
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn clear_last_message_resets_all_fields() {
        use telers::enums::ReplyMarkupType;

        let mut stack = Stack::new();
        stack.last_message_id = Some(5);
        stack.last_text = Some("t".into());
        stack.last_reply_markup_type = Some(ReplyMarkupType::InlineKeyboardMarkup);
        stack.last_reply_markup = Some(serde_json::json!({}));
        stack.last_media_id = Some("file".into());
        stack.last_income_media_group_id = Some("group".into());
        stack.has_protected_content = Some(true);

        stack.clear_last_message();

        assert_eq!(stack.last_message_id, None);
        assert_eq!(stack.last_text, None);
        assert_eq!(stack.last_reply_markup_type, None);
        assert_eq!(stack.last_reply_markup, None);
        assert_eq!(stack.last_media_id, None);
        assert_eq!(stack.last_media_unique_id, None);
        assert_eq!(stack.last_media_content_type, None);
        assert_eq!(stack.last_income_media_group_id, None);
        assert_eq!(stack.message_type, None);
        assert_eq!(stack.has_protected_content, None);
    }
}
