use crate::{entities::Context, widgets::ButtonAction};
use std::sync::Arc;
use telers::types::Message;

pub trait Input: Send + Sync + 'static {
    fn handle_message(&self, ctx: &Context, message: &Message) -> Option<ButtonAction>;
}

type InputHandler = dyn Fn(&Context, &Message) -> Option<ButtonAction> + Send + Sync + 'static;
type TextParser<T> = dyn Fn(&str) -> Result<T, String> + Send + Sync + 'static;
type TextSuccess<T> = dyn Fn(T) -> ButtonAction + Send + Sync + 'static;
type TextError = dyn Fn(String) -> Option<ButtonAction> + Send + Sync + 'static;

pub(super) struct MultiInput {
    inputs: Vec<Box<dyn Input>>,
}

impl MultiInput {
    #[inline]
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self { inputs: Vec::new() }
    }

    #[inline]
    #[must_use]
    pub(crate) fn input_boxed(mut self, input: Box<dyn Input>) -> Self {
        self.inputs.push(input);
        self
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
}

impl Input for MessageInput {
    fn handle_message(&self, ctx: &Context, message: &Message) -> Option<ButtonAction> {
        (self.handler)(ctx, message)
    }
}

pub struct TextInput<T = String> {
    id: Box<str>,
    parser: Arc<TextParser<T>>,
    on_success: Arc<TextSuccess<T>>,
    on_error: Option<Arc<TextError>>,
}

impl TextInput<String> {
    #[must_use]
    pub fn new(
        id: impl Into<Box<str>>,
        on_success: impl Fn(String) -> ButtonAction + Send + Sync + 'static,
    ) -> Self {
        Self::with_parser(id, |text| Ok::<String, String>(text.to_owned()), on_success)
    }
}

impl<T> TextInput<T>
where
    T: Send + Sync + 'static,
{
    #[must_use]
    pub fn with_parser<E>(
        id: impl Into<Box<str>>,
        parser: impl Fn(&str) -> Result<T, E> + Send + Sync + 'static,
        on_success: impl Fn(T) -> ButtonAction + Send + Sync + 'static,
    ) -> Self
    where
        E: ToString,
    {
        Self {
            id: id.into(),
            parser: Arc::new(move |text| parser(text).map_err(|err| err.to_string())),
            on_success: Arc::new(on_success),
            on_error: None,
        }
    }

    #[must_use]
    pub fn on_error(
        mut self,
        handler: impl Fn(String) -> Option<ButtonAction> + Send + Sync + 'static,
    ) -> Self {
        self.on_error = Some(Arc::new(handler));
        self
    }

    #[inline]
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn value(&self, ctx: &Context) -> Option<T> {
        let raw = ctx.widget_value_as::<String>(&self.id)?;
        (self.parser)(&raw).ok()
    }
}

impl<T> Input for TextInput<T>
where
    T: Send + Sync + 'static,
{
    fn handle_message(&self, _ctx: &Context, message: &Message) -> Option<ButtonAction> {
        let text = message.text()?.to_owned();
        match (self.parser)(&text) {
            Ok(value) => Some(ButtonAction::chain([
                ButtonAction::set_widget_value(self.id.clone(), text),
                (self.on_success)(value),
            ])),
            Err(err) => self.on_error.as_ref().and_then(|handler| handler(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Input, MessageInput, TextInput};
    use crate::{entities::Context, widgets::ButtonAction};
    use serde_json::json;
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
    fn message_input_text_can_store_dialog_value() {
        let input = MessageInput::text(|value| ButtonAction::set_dialog_value("name", value));
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

    #[test]
    fn text_input_stores_raw_text_in_widget_data_and_runs_success_action() {
        let input = TextInput::new("name_input", |value| {
            ButtonAction::set_dialog_value("name", value)
        });
        let ctx = Context::new("", "state", serde_json::Value::Null);

        let action = input
            .handle_message(&ctx, &text_message("alice"))
            .expect("text action");

        let ButtonAction::Chain(actions) = action else {
            panic!("expected chained action");
        };
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "name_input" && value == "alice"
        ));
        assert!(matches!(
            actions[1],
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "name" && value == "alice"
        ));
    }

    #[test]
    fn text_input_reads_typed_value_from_widget_data() {
        let input = TextInput::with_parser(
            "age_input",
            |text: &str| text.parse::<u8>(),
            |age| ButtonAction::set_dialog_value("age", age),
        );
        let mut ctx = Context::new("", "state", serde_json::Value::Null);
        ctx.widget_data.insert("age_input".into(), json!("42"));

        assert_eq!(input.value(&ctx), Some(42));
    }

    #[test]
    fn text_input_can_map_parse_errors_to_action() {
        let input = TextInput::with_parser(
            "age_input",
            |text: &str| text.parse::<u8>(),
            |age| ButtonAction::set_dialog_value("age", age),
        )
        .on_error(|err| Some(ButtonAction::set_dialog_value("error", err)));
        let ctx = Context::new("", "state", serde_json::Value::Null);

        let action = input
            .handle_message(&ctx, &text_message("oops"))
            .expect("error action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "error" && value == "invalid digit found in string"
        ));
    }
}
