use bon::bon;
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use telers::types::Message;

use crate::{entities::Context, widgets::ButtonAction};

pub trait Input: Send + Sync + 'static {
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;
}

pub(crate) struct MultiInput {
    inputs: Vec<Box<dyn Input>>,
}

impl MultiInput {
    #[inline]
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            inputs: Vec::new(),
        }
    }

    #[must_use]
    pub(crate) fn input_boxed(mut self, input: Box<dyn Input>) -> Self {
        self.inputs.push(input);
        self
    }
}

impl Input for MultiInput {
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        self.inputs
            .iter()
            .find_map(|input| input.handle_message(ctx, message.clone()))
    }
}

pub struct MessageInput<Handler, MessageType> {
    handler: Handler,
    marker: PhantomData<fn() -> MessageType>,
}

impl<Handler, MessageType> MessageInput<Handler, MessageType> {
    #[inline]
    #[must_use]
    pub const fn new(handler: Handler) -> Self
    where
        Handler: Fn(&Context, MessageType) -> ButtonAction,
        MessageType: TryFrom<Message>,
    {
        Self {
            handler,
            marker: PhantomData,
        }
    }
}

impl<Handler, MessageType> Input for MessageInput<Handler, MessageType>
where
    Handler: Fn(&Context, MessageType) -> ButtonAction + Send + Sync + 'static,
    MessageType: TryFrom<Message> + 'static,
{
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        Some((self.handler)(ctx, message.try_into().ok()?))
    }
}

pub struct TextInput<WidgetId, ParserOk, ParserErr, OnSuccess> {
    id: WidgetId,
    parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
    on_success: OnSuccess,
    on_error: Option<Box<dyn Fn(&Context, ParserErr) -> ButtonAction + Send + Sync>>,
    marker: PhantomData<fn() -> (ParserOk, ParserErr)>,
}

#[bon]
impl<WidgetId, ParserOk, ParserErr, OnSuccess> TextInput<WidgetId, ParserOk, ParserErr, OnSuccess>
where
    WidgetId: Display,
{
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        #[builder(
            default = Box::new(|text| text.parse()),
            with = |parser: impl Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync + 'static| Box::new(parser)
        )]
        parser: Box<dyn Fn(&str) -> Result<ParserOk, ParserErr> + Send + Sync>,
        on_success: OnSuccess,
        #[builder(with = |on_error: impl Fn(&Context, ParserErr) -> ButtonAction + Send + Sync + 'static| Box::new(on_error))]
        on_error: Option<Box<dyn Fn(&Context, ParserErr) -> ButtonAction + Send + Sync>>,
    ) -> Self
    where
        ParserOk: FromStr<Err = ParserErr> + Send + Sync + 'static,
        OnSuccess: Fn(&Context, ParserOk) -> ButtonAction,
    {
        Self {
            id,
            parser,
            on_success,
            on_error,
            marker: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn value(&self, ctx: &Context) -> Option<ParserOk> {
        let unparsed_value = ctx.widget_value_as::<String>(&self.id.to_string())?;
        (self.parser)(&unparsed_value).ok()
    }
}

impl<WidgetId, ParserOk, ParserErr, OnSuccess> Input
    for TextInput<WidgetId, ParserOk, ParserErr, OnSuccess>
where
    WidgetId: Display + Send + Sync + 'static,
    ParserOk: 'static,
    ParserErr: 'static,
    OnSuccess: Fn(&Context, ParserOk) -> ButtonAction + Send + Sync + 'static,
{
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        let text = message.text()?.to_owned();
        match (self.parser)(&text) {
            Ok(value) => Some(ButtonAction::chain([
                ButtonAction::set_widget_value(self.id.to_string(), text),
                (self.on_success)(ctx, value),
            ])),
            Err(err) => self
                .on_error
                .as_ref()
                .and_then(|on_error| Some(on_error(ctx, err))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Input, MessageInput, TextInput};
    use crate::{entities::Context, widgets::ButtonAction};
    use serde_json::{json, Value};
    use telers::types::{ChatPrivate, Message, MessageText, User};

    fn text_message(text: &str) -> Message {
        MessageText::new(1, 1, ChatPrivate::new(10), text)
            .from(User::new(10, false, "tester"))
            .into()
    }

    #[test]
    fn message_input_text_handles_text_messages() {
        let input = MessageInput::new(|_ctx, message: MessageText| {
            ButtonAction::set_dialog_value("name", message.text.to_string())
        });
        let ctx = Context::new("", "state", Value::Null);

        let action = input
            .handle_message(&ctx, text_message("alice"))
            .expect("text action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "name" && value == "alice"
        ));
    }

    #[test]
    fn message_input_text_can_store_dialog_value() {
        let input = MessageInput::new(|_ctx, message: MessageText| {
            ButtonAction::set_dialog_value("name", message.text.to_string())
        });
        let ctx = Context::new("", "state", Value::Null);

        let action = input
            .handle_message(&ctx, text_message("bob"))
            .expect("text action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "name" && value == "bob"
        ));
    }

    #[test]
    fn text_input_stores_raw_text_in_widget_data_and_runs_success_action() {
        let input = TextInput::builder("name_input")
            .on_success(|_ctx, value: String| ButtonAction::set_dialog_value("name", value))
            .build();

        let ctx = Context::new("", "state", Value::Null);

        let action = input
            .handle_message(&ctx, text_message("alice"))
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
        let input = TextInput::builder("age_input")
            .on_success(|_ctx, age: u8| ButtonAction::set_dialog_value("age", age))
            .build();
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("age_input".into(), json!("42"));

        assert_eq!(input.value(&ctx), Some(42));
    }

    #[test]
    fn text_input_can_map_parse_errors_to_action() {
        let input = TextInput::builder("age_input")
            .on_success(|_ctx, age: u8| ButtonAction::set_dialog_value("age", age))
            .on_error(|_ctx, err| ButtonAction::set_dialog_value("error", err.to_string()))
            .build();
        let ctx = Context::new("", "state", Value::Null);

        let action = input
            .handle_message(&ctx, text_message("oops"))
            .expect("error action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "error" && value == "invalid digit found in string"
        ));
    }
}
