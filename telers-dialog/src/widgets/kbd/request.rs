use async_trait::async_trait;
use bon::bon;
use std::borrow::Cow;

use telers::{
    enums::PollType,
    types::{
        ForceReply as TelegramForceReply, KeyboardButton, KeyboardButtonPollType,
        ReplyKeyboardMarkup, ReplyMarkup,
    },
};

use super::{when::is_allowed, ButtonAction, ClickContext, Keyboard, WhenCondition};
use crate::{
    entities::{Context, DataMap, RenderContext},
    widgets::Text,
};

#[derive(Default)]
pub struct ReplyKeyboardOptions {
    is_persistent: Option<bool>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    input_field_placeholder: Option<Cow<'static, str>>,
    selective: Option<bool>,
}

impl ReplyKeyboardOptions {
    fn apply(&self, markup: ReplyKeyboardMarkup) -> ReplyKeyboardMarkup {
        markup
            .is_persistent_option(self.is_persistent)
            .resize_keyboard_option(self.resize_keyboard)
            .one_time_keyboard_option(self.one_time_keyboard)
            .input_field_placeholder_option(self.input_field_placeholder.as_deref())
            .selective_option(self.selective)
    }
}

/// Reply keyboard button that requests the user's contact.
pub struct RequestContact<ButtonText> {
    text: ButtonText,
    options: ReplyKeyboardOptions,
    when: Option<WhenCondition>,
}

#[bon]
impl<ButtonText> RequestContact<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to share a contact.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] text: ButtonText,
        #[builder(field = ReplyKeyboardOptions::default())] options: ReplyKeyboardOptions,
        when: Option<WhenCondition>,
    ) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            options,
            when,
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

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

#[allow(clippy::wrong_self_convention)]
impl<ButtonText, S> RequestContactBuilder<ButtonText, S>
where
    S: request_contact_builder::State,
    ButtonText: Text,
{
    /// Set the persistent-keyboard flag.
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

#[async_trait]
impl<ButtonText> Keyboard for RequestContact<ButtonText>
where
    ButtonText: Text,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        let button = KeyboardButton::new(self.text.render_text_in_context(render_ctx).await)
            .request_contact(true);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

/// Reply keyboard button that requests the user's current location.
pub struct RequestLocation<ButtonText> {
    text: ButtonText,
    options: ReplyKeyboardOptions,
    when: Option<WhenCondition>,
}

#[bon]
impl<ButtonText> RequestLocation<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to share a location.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] text: ButtonText,
        #[builder(field = ReplyKeyboardOptions::default())] options: ReplyKeyboardOptions,
        when: Option<WhenCondition>,
    ) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            options,
            when,
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

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

#[allow(clippy::wrong_self_convention)]
impl<ButtonText, S> RequestLocationBuilder<ButtonText, S>
where
    S: request_location_builder::State,
    ButtonText: Text,
{
    /// Set the persistent-keyboard flag.
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

#[async_trait]
impl<ButtonText> Keyboard for RequestLocation<ButtonText>
where
    ButtonText: Text,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        let button = KeyboardButton::new(self.text.render_text_in_context(render_ctx).await)
            .request_location(true);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

/// Reply keyboard button that requests the user to create and send a poll.
pub struct RequestPoll<ButtonText> {
    text: ButtonText,
    options: ReplyKeyboardOptions,
    poll_type: Option<PollType>,
    when: Option<WhenCondition>,
}

#[bon]
impl<ButtonText> RequestPoll<ButtonText> {
    /// Create a one-button reply keyboard that asks the user to create a poll.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] text: ButtonText,
        #[builder(field = ReplyKeyboardOptions::default())] options: ReplyKeyboardOptions,
        poll_type: Option<PollType>,
        when: Option<WhenCondition>,
    ) -> Self
    where
        ButtonText: Text,
    {
        Self {
            text,
            options,
            poll_type,
            when,
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

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

#[allow(clippy::wrong_self_convention)]
impl<ButtonText, S> RequestPollBuilder<ButtonText, S>
where
    S: request_poll_builder::State,
    ButtonText: Text,
{
    /// Set the persistent-keyboard flag.
    pub fn is_persistent(mut self, value: bool) -> Self {
        self.options.is_persistent = Some(value);
        self
    }

    /// Set the resize-keyboard flag.
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.options.resize_keyboard = Some(value);
        self
    }

    /// Set the one-time-keyboard flag.
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.options.one_time_keyboard = Some(value);
        self
    }

    /// Set the input placeholder shown while the reply keyboard is active.
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether the reply keyboard is selective.
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

#[async_trait]
impl<ButtonText> Keyboard for RequestPoll<ButtonText>
where
    ButtonText: Text,
{
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        let request_poll =
            KeyboardButtonPollType::new().type_option(self.poll_type.map(Into::<Box<str>>::into));
        let button = KeyboardButton::new(self.text.render_text_in_context(render_ctx).await)
            .request_poll(request_poll);
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new([[button]]))
                .into(),
        )
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

/// Options for ForceReply widget.
#[derive(Default)]
pub struct ForceReplyOptions {
    input_field_placeholder: Option<Cow<'static, str>>,
    selective: Option<bool>,
}

impl ForceReplyOptions {
    fn apply(&self, reply: TelegramForceReply) -> TelegramForceReply {
        reply
            .input_field_placeholder_option(self.input_field_placeholder.as_deref())
            .selective_option(self.selective)
    }
}

/// Reply markup that forces the user to reply to the bot's message.
///
/// Upon receiving a message with this markup, Telegram clients will display
/// a reply interface to the user (as if the user has selected the bot's message
/// and tapped 'Reply').
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::ForceReply;
///
/// let reply = ForceReply::builder()
///     .input_field_placeholder("Enter your name")
///     .selective(true)
///     .build();
/// ```
pub struct ForceReply {
    options: ForceReplyOptions,
    when: Option<WhenCondition>,
}

#[bon]
impl ForceReply {
    /// Create a force reply widget.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field = ForceReplyOptions::default())] options: ForceReplyOptions,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            options,
            when,
        }
    }

    /// Set the input placeholder shown when the reply is active.
    #[must_use]
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether to force reply from specific users only.
    #[must_use]
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }
}

#[allow(clippy::wrong_self_convention)]
impl<S> ForceReplyBuilder<S>
where
    S: force_reply_builder::State,
{
    /// Set the input placeholder shown when the reply is active.
    pub fn input_field_placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.options.input_field_placeholder = Some(value.into());
        self
    }

    /// Set whether to force reply from specific users only.
    pub fn selective(mut self, value: bool) -> Self {
        self.options.selective = Some(value);
        self
    }
}

#[async_trait]
impl Keyboard for ForceReply {
    async fn is_visible(&self, ctx: &Context, data: &DataMap) -> bool {
        is_allowed(self.when.as_ref(), ctx, data).await
    }

    async fn render_keyboard(&self, render_ctx: &RenderContext) -> Option<ReplyMarkup> {
        if !self
            .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
            .await
        {
            return None;
        }
        let reply = self.options.apply(TelegramForceReply::new(true));
        Some(reply.into())
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use telers::enums::PollType;

    use telers::types::ReplyMarkup;

    use super::{ForceReply, RequestContact, RequestLocation, RequestPoll};
    use crate::{
        entities::{Context, DataMap},
        widgets::Keyboard,
    };

    #[tokio::test]
    async fn request_contact_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestContact::builder("Share phone")
            .resize_keyboard(true)
            .input_field_placeholder("Phone")
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let ReplyMarkup::ReplyKeyboardMarkup(markup) = markup else {
            panic!("reply keyboard");
        };

        assert_eq!(&*markup.keyboard[0][0].text, "Share phone");
        assert_eq!(markup.keyboard[0][0].request_contact, Some(true));
        assert_eq!(markup.resize_keyboard, Some(true));
        assert_eq!(markup.input_field_placeholder.as_deref(), Some("Phone"));
    }

    #[tokio::test]
    async fn request_location_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestLocation::builder("Share location")
            .one_time_keyboard(true)
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let ReplyMarkup::ReplyKeyboardMarkup(markup) = markup else {
            panic!("reply keyboard");
        };

        assert_eq!(&*markup.keyboard[0][0].text, "Share location");
        assert_eq!(markup.keyboard[0][0].request_location, Some(true));
        assert_eq!(markup.one_time_keyboard, Some(true));
    }

    #[tokio::test]
    async fn request_poll_renders_reply_keyboard_button() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = RequestPoll::builder("Create quiz")
            .poll_type(PollType::Quiz)
            .selective(true)
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
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

    #[tokio::test]
    async fn force_reply_renders_force_reply_markup() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = ForceReply::builder()
            .input_field_placeholder("Enter your name")
            .selective(true)
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .await
            .unwrap();
        let ReplyMarkup::ForceReply(reply) = markup else {
            panic!("force reply");
        };

        assert!(reply.force_reply);
        assert_eq!(
            reply.input_field_placeholder.as_deref(),
            Some("Enter your name")
        );
        assert_eq!(reply.selective, Some(true));
    }
}
