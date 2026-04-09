use std::borrow::Cow;

use telers::{
    enums::PollType,
    types::{KeyboardButton, KeyboardButtonPollType, ReplyKeyboardMarkup, ReplyMarkup},
};

use super::{ButtonAction, Keyboard};
use crate::{
    entities::{Context, DataMap},
    widgets::Text,
};

#[derive(Default)]
struct ReplyKeyboardOptions {
    is_persistent: Option<bool>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    input_field_placeholder: Option<Cow<'static, str>>,
    selective: Option<bool>,
}

impl ReplyKeyboardOptions {
    fn apply(&self, markup: ReplyKeyboardMarkup) -> ReplyKeyboardMarkup {
        let markup = match self.is_persistent {
            Some(value) => markup.is_persistent(value),
            None => markup,
        };
        let markup = match self.resize_keyboard {
            Some(value) => markup.resize_keyboard(value),
            None => markup,
        };
        let markup = match self.one_time_keyboard {
            Some(value) => markup.one_time_keyboard(value),
            None => markup,
        };
        let markup = match self.input_field_placeholder.as_ref() {
            Some(value) => markup.input_field_placeholder(value.clone()),
            None => markup,
        };
        match self.selective {
            Some(value) => markup.selective(value),
            None => markup,
        }
    }
}

/// Reply keyboard button that requests the user's contact.
pub struct RequestContact<ButtonText> {
    text: ButtonText,
    options: ReplyKeyboardOptions,
}

impl<ButtonText> RequestContact<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to share a contact.
    #[must_use]
    pub fn new(text: ButtonText) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            options: ReplyKeyboardOptions::default(),
        }
    }

    /// Set the persistent-keyboard flag.
    #[must_use]
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    #[must_use]
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    #[must_use]
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    #[must_use]
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    #[must_use]
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

impl<ButtonText> Keyboard for RequestContact<ButtonText>
where
    ButtonText: Text,
{
    fn render_keyboard(&self, _ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let button =
            KeyboardButton::new(self.text.render_text_in_context(_ctx, data)).request_contact(true);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    fn handle_callback(&self, _ctx: &Context, _callback_data: &str) -> Option<ButtonAction> {
        None
    }
}

/// Reply keyboard button that requests the user's current location.
pub struct RequestLocation<ButtonText> {
    text: ButtonText,
    options: ReplyKeyboardOptions,
}

impl<ButtonText> RequestLocation<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to share a location.
    #[must_use]
    pub fn new(text: ButtonText) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            options: ReplyKeyboardOptions::default(),
        }
    }

    /// Set the persistent-keyboard flag.
    #[must_use]
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    #[must_use]
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    #[must_use]
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    #[must_use]
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    #[must_use]
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

impl<ButtonText> Keyboard for RequestLocation<ButtonText>
where
    ButtonText: Text,
{
    fn render_keyboard(&self, _ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let button = KeyboardButton::new(self.text.render_text_in_context(_ctx, data))
            .request_location(true);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    fn handle_callback(&self, _ctx: &Context, _callback_data: &str) -> Option<ButtonAction> {
        None
    }
}

/// Reply keyboard button that requests the user to create and send a poll.
pub struct RequestPoll<ButtonText> {
    text: ButtonText,
    poll_type: Option<PollType>,
    options: ReplyKeyboardOptions,
}

impl<ButtonText> RequestPoll<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to create a poll.
    #[must_use]
    pub fn new(text: ButtonText) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            poll_type: None,
            options: ReplyKeyboardOptions::default(),
        }
    }

    /// Restrict which poll type the user may create.
    #[must_use]
    pub fn poll_type(mut self, value: PollType) -> Self {
        self.poll_type = Some(value);
        self
    }

    /// Set the persistent-keyboard flag.
    #[must_use]
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    #[must_use]
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    #[must_use]
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    #[must_use]
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    #[must_use]
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

impl<ButtonText> Keyboard for RequestPoll<ButtonText>
where
    ButtonText: Text,
{
    fn render_keyboard(&self, _ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let request_poll =
            KeyboardButtonPollType::new().type_option(self.poll_type.map(Into::<Box<str>>::into));
        let button = KeyboardButton::new(self.text.render_text_in_context(_ctx, data))
            .request_poll(request_poll);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    fn handle_callback(&self, _ctx: &Context, _callback_data: &str) -> Option<ButtonAction> {
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use telers::enums::PollType;

    use telers::types::ReplyMarkup;

    use super::{RequestContact, RequestLocation, RequestPoll};
    use crate::{
        entities::{Context, DataMap},
        widgets::Keyboard,
    };

    #[test]
    fn request_contact_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestContact::new("Share phone")
            .resize_keyboard(true)
            .input_field_placeholder("Phone");

        let markup = keyboard.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let ReplyMarkup::ReplyKeyboardMarkup(markup) = markup else {
            panic!("reply keyboard");
        };

        assert_eq!(&*markup.keyboard[0][0].text, "Share phone");
        assert_eq!(markup.keyboard[0][0].request_contact, Some(true));
        assert_eq!(markup.resize_keyboard, Some(true));
        assert_eq!(markup.input_field_placeholder.as_deref(), Some("Phone"));
    }

    #[test]
    fn request_location_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestLocation::new("Share location").one_time_keyboard(true);

        let markup = keyboard.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let ReplyMarkup::ReplyKeyboardMarkup(markup) = markup else {
            panic!("reply keyboard");
        };

        assert_eq!(&*markup.keyboard[0][0].text, "Share location");
        assert_eq!(markup.keyboard[0][0].request_location, Some(true));
        assert_eq!(markup.one_time_keyboard, Some(true));
    }

    #[test]
    fn request_poll_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestPoll::new("Create quiz")
            .poll_type(PollType::Quiz)
            .selective(true);

        let markup = keyboard.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let ReplyMarkup::ReplyKeyboardMarkup(markup) = markup else {
            panic!("reply keyboard");
        };

        assert_eq!(&*markup.keyboard[0][0].text, "Create quiz");
        assert_eq!(
            markup.keyboard[0][0]
                .request_poll
                .as_ref()
                .and_then(|value| value.r#type.as_deref()),
            Some("quiz")
        );
        assert_eq!(markup.selective, Some(true));
    }
}
