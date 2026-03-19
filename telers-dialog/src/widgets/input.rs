use crate::{entities::Context, widgets::ButtonAction};
use std::sync::Arc;
use telers::types::Message;

pub trait Input: Send + Sync + 'static {
    fn handle_message(&self, ctx: &Context, message: &Message) -> Option<ButtonAction>;
}

type InputHandler = dyn Fn(&Context, &Message) -> Option<ButtonAction> + Send + Sync + 'static;

pub(super) struct MultiInput {
    inputs: Vec<Box<dyn Input>>,
}

impl MultiInput {
    pub(super) fn new(inputs: Vec<Box<dyn Input>>) -> Self {
        Self {
            inputs,
        }
    }
}

impl Input for MultiInput {
    fn handle_message(&self, ctx: &Context, message: &Message) -> Option<ButtonAction> {
        self.inputs
            .iter()
            .find_map(|input| input.handle_message(ctx, message))
    }
}

pub struct MessageInput {
    handler: Arc<InputHandler>,
}

impl MessageInput {
    #[must_use]
    pub fn new(
        handler: impl Fn(&Context, &Message) -> Option<ButtonAction> + Send + Sync + 'static,
    ) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }

    #[must_use]
    pub fn text(handler: impl Fn(String) -> ButtonAction + Send + Sync + 'static) -> Self {
        Self::new(move |_ctx, message| message.text().map(|text| handler(text.to_owned())))
    }

    #[must_use]
    pub fn store_text(key: impl Into<Box<str>>) -> Self {
        let key = key.into();
        Self::text(move |text| ButtonAction::set_dialog_value(key.clone(), text))
    }
}

impl Input for MessageInput {
    fn handle_message(&self, ctx: &Context, message: &Message) -> Option<ButtonAction> {
        (self.handler)(ctx, message)
    }
}

#[cfg(test)]
mod tests {
    use super::{Input, MessageInput};
    use crate::{entities::Context, widgets::ButtonAction};
    use telers::types::{ChatPrivate, Message, MessageText, User};

    fn text_message(text: &str) -> Message {
        MessageText::new(1, 1, ChatPrivate::new(10), text)
            .from(User::new(10, false, "tester"))
            .into()
    }

    #[test]
    fn message_input_text_handles_text_messages() {
        let input = MessageInput::text(|value| ButtonAction::set_dialog_value("name", value));
        let ctx = Context::new("", "state", serde_json::Value::Null);

        let action = input
            .handle_message(&ctx, &text_message("alice"))
            .expect("text action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "name" && value == "alice"
        ));
    }

    #[test]
    fn message_input_store_text_uses_dialog_value_action() {
        let input = MessageInput::store_text("name");
        let ctx = Context::new("", "state", serde_json::Value::Null);

        let action = input
            .handle_message(&ctx, &text_message("bob"))
            .expect("text action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "name" && value == "bob"
        ));
    }
}
