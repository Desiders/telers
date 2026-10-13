use std::{borrow::Cow, sync::Arc};

use async_fn_traits::AsyncFn1;
use async_trait::async_trait;
use bon::bon;
use telers::{
    enums::PollType,
    types::{
        KeyboardButton, KeyboardButtonPollType, KeyboardButtonRequestChat,
        KeyboardButtonRequestManagedBot, KeyboardButtonRequestUsers, ReplyKeyboardMarkup,
        ReplyKeyboardRemove as TelegramReplyKeyboardRemove, ReplyMarkup, WebAppInfo,
    },
};

use super::{
    macros::impl_reply_keyboard_options_setters, request::ReplyKeyboardOptions, when::is_allowed,
    ButtonAction, ButtonAppearance, ButtonStyle, ClickContext, Keyboard, WhenCondition,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    widgets::Text,
};

#[derive(Clone)]
enum ReplyButtonKind {
    Text,
    Contact,
    Location,
    Poll(Option<PollType>),
    Users(KeyboardButtonRequestUsers),
    Chat(KeyboardButtonRequestChat),
    ManagedBot(KeyboardButtonRequestManagedBot),
    WebApp(Arc<dyn Text>),
}

/// A native Telegram reply button. Text buttons send a message instead of a callback.
#[derive(Clone)]
pub struct ReplyButton {
    text: Arc<dyn Text>,
    kind: ReplyButtonKind,
    appearance: ButtonAppearance,
    when: Option<WhenCondition>,
}

impl ReplyButton {
    fn with_kind(text: impl Text, kind: ReplyButtonKind) -> Self {
        Self {
            text: Arc::new(text),
            kind,
            appearance: ButtonAppearance::default(),
            when: None,
        }
    }

    #[must_use]
    pub fn text(text: impl Text) -> Self {
        Self::with_kind(text, ReplyButtonKind::Text)
    }

    #[must_use]
    pub fn request_contact(text: impl Text) -> Self {
        Self::with_kind(text, ReplyButtonKind::Contact)
    }

    #[must_use]
    pub fn request_location(text: impl Text) -> Self {
        Self::with_kind(text, ReplyButtonKind::Location)
    }

    #[must_use]
    pub fn request_poll(text: impl Text, poll_type: Option<PollType>) -> Self {
        Self::with_kind(text, ReplyButtonKind::Poll(poll_type))
    }

    #[must_use]
    pub fn request_users(text: impl Text, request: KeyboardButtonRequestUsers) -> Self {
        Self::with_kind(text, ReplyButtonKind::Users(request))
    }

    #[must_use]
    pub fn request_chat(text: impl Text, request: KeyboardButtonRequestChat) -> Self {
        Self::with_kind(text, ReplyButtonKind::Chat(request))
    }

    #[must_use]
    pub fn request_managed_bot(text: impl Text, request: KeyboardButtonRequestManagedBot) -> Self {
        Self::with_kind(text, ReplyButtonKind::ManagedBot(request))
    }

    #[must_use]
    pub fn web_app(text: impl Text, web_app: WebAppInfo) -> Self {
        Self::web_app_dynamic(text, web_app.url.into_string())
    }

    #[must_use]
    pub fn web_app_dynamic(text: impl Text, url: impl Text) -> Self {
        Self::with_kind(text, ReplyButtonKind::WebApp(Arc::new(url)))
    }

    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.appearance = self.appearance.with_style(style);
        self
    }

    #[must_use]
    pub fn style_dynamic<F>(mut self, renderer: F) -> Self
    where
        F: AsyncFn(RenderContext) -> Option<ButtonStyle>
            + AsyncFn1<RenderContext, Output = Option<ButtonStyle>>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<RenderContext>>::OutputFuture: Send + 'static,
    {
        self.appearance = self.appearance.with_style_dynamic(renderer);
        self
    }

    #[must_use]
    pub fn appearance(mut self, appearance: ButtonAppearance) -> Self {
        self.appearance = appearance;
        self
    }

    #[must_use]
    pub fn icon_custom_emoji_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.appearance = self.appearance.with_emoji(value.into());
        self
    }

    #[must_use]
    pub fn icon_custom_emoji_id_dynamic(mut self, value: impl Text) -> Self {
        self.appearance = self.appearance.with_emoji(value);
        self
    }

    #[must_use]
    pub fn when(mut self, when: WhenCondition) -> Self {
        self.when = Some(when);
        self
    }

    async fn render(&self, render_ctx: &RenderContext) -> Option<KeyboardButton> {
        if !is_allowed(
            self.when.as_ref(),
            render_ctx.context.as_ref(),
            render_ctx.data.as_ref(),
        )
        .await
        {
            return None;
        }
        let button = KeyboardButton::new(self.text.render_text_in_context(render_ctx).await);
        let button = self.appearance.apply_reply(button, render_ctx).await;
        Some(match &self.kind {
            ReplyButtonKind::Text => button,
            ReplyButtonKind::Contact => button.request_contact(true),
            ReplyButtonKind::Location => button.request_location(true),
            ReplyButtonKind::Poll(poll_type) => button.request_poll(
                KeyboardButtonPollType::new().type_option(poll_type.map(Into::<Box<str>>::into)),
            ),
            ReplyButtonKind::Users(request) => button.request_users(request.clone()),
            ReplyButtonKind::Chat(request) => button.request_chat(request.clone()),
            ReplyButtonKind::ManagedBot(request) => button.request_managed_bot(request.clone()),
            ReplyButtonKind::WebApp(url) => button.web_app(
                WebAppInfo::new(url.render_text_in_context(render_ctx).await),
            ),
        })
    }
}

/// Reply keyboard composed of explicit rows of native buttons.
///
/// Pair with a message input to handle text choices or requested Telegram payloads.
#[derive(Default)]
pub struct ReplyKeyboard {
    rows: Vec<Vec<ReplyButton>>,
    options: ReplyKeyboardOptions,
    when: Option<WhenCondition>,
}

#[bon]
impl ReplyKeyboard {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field = Vec::new())] rows: Vec<Vec<ReplyButton>>,
        #[builder(field = ReplyKeyboardOptions::default())] options: ReplyKeyboardOptions,
        when: Option<WhenCondition>,
    ) -> Self {
        Self {
            rows,
            options,
            when,
        }
    }
}

impl<S> ReplyKeyboardBuilder<S>
where
    S: reply_keyboard_builder::State,
{
    pub fn row(mut self, row: impl IntoIterator<Item = ReplyButton>) -> Self {
        self.rows.push(row.into_iter().collect());
        self
    }

    pub fn push(mut self, button: ReplyButton) -> Self {
        match self.rows.last_mut() {
            Some(row) => row.push(button),
            None => self.rows.push(vec![button]),
        }
        self
    }

    impl_reply_keyboard_options_setters!();
}

#[async_trait]
impl Keyboard for ReplyKeyboard {
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
        let mut rows = Vec::with_capacity(self.rows.len());
        for row in &self.rows {
            let mut buttons = Vec::with_capacity(row.len());
            for button in row {
                if let Some(button) = button.render(render_ctx).await {
                    buttons.push(button);
                }
            }
            if !buttons.is_empty() {
                rows.push(buttons);
            }
        }
        if rows.is_empty() {
            return None;
        }
        Some(
            self.options
                .apply(ReplyKeyboardMarkup::new(rows), render_ctx)
                .await
                .into(),
        )
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

/// Explicitly remove the user's active reply keyboard.
#[derive(Default)]
pub struct ReplyKeyboardRemove {
    selective: Option<bool>,
    when: Option<WhenCondition>,
}

#[bon]
impl ReplyKeyboardRemove {
    #[builder]
    #[must_use]
    pub fn new(selective: Option<bool>, when: Option<WhenCondition>) -> Self {
        Self {
            selective,
            when,
        }
    }
}

#[async_trait]
impl Keyboard for ReplyKeyboardRemove {
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
        Some(
            TelegramReplyKeyboardRemove::new(true)
                .selective_option(self.selective)
                .into(),
        )
    }

    async fn handle_callback(&self, _click: &ClickContext) -> Option<ButtonAction> {
        None
    }
}

#[cfg(test)]
mod tests {
    use telers::{
        enums::PollType,
        serialization::Value,
        types::{
            KeyboardButtonRequestChat, KeyboardButtonRequestManagedBot, KeyboardButtonRequestUsers,
            ReplyKeyboardMarkup, ReplyMarkup, WebAppInfo,
        },
    };

    use super::{ReplyButton, ReplyKeyboard, ReplyKeyboardRemove};
    use crate::{
        entities::{Context, DataMap},
        widgets::{ButtonStyle, FormatText, Keyboard, WhenCondition},
    };

    fn context() -> Context {
        Context::new("", "state", Value::default())
    }

    async fn markup(keyboard: &impl Keyboard, data: &DataMap) -> ReplyKeyboardMarkup {
        let Some(ReplyMarkup::ReplyKeyboardMarkup(markup)) = keyboard
            .render_keyboard_for_test(&context(), data)
            .await
        else {
            panic!("reply keyboard");
        };
        markup
    }

    #[tokio::test]
    async fn rows_render_dynamic_text_placeholder_and_all_layout_options() {
        let data = [("name".into(), Value::from("Ada"))].into_iter().collect();
        let keyboard = ReplyKeyboard::builder()
            .push(ReplyButton::text(FormatText::new("Hello {name}")))
            .push(
                ReplyButton::text("Other")
                    .style(ButtonStyle::Primary)
                    .icon_custom_emoji_id("emoji"),
            )
            .row([ReplyButton::text("Last")])
            .is_persistent(true)
            .resize_keyboard(true)
            .one_time_keyboard(false)
            .input_field_placeholder(FormatText::new("Choose, {name}"))
            .selective(true)
            .build();

        let markup = markup(&keyboard, &data).await;
        assert_eq!(markup.keyboard.len(), 2);
        assert_eq!(markup.keyboard[0].len(), 2);
        assert_eq!(&*markup.keyboard[0][0].text, "Hello Ada");
        assert_eq!(markup.keyboard[0][1].style.as_deref(), Some("primary"));
        assert_eq!(
            markup.keyboard[0][1].icon_custom_emoji_id.as_deref(),
            Some("emoji"),
        );
        assert_eq!(markup.input_field_placeholder.as_deref(), Some("Choose, Ada"));
        assert_eq!(markup.is_persistent, Some(true));
        assert_eq!(markup.resize_keyboard, Some(true));
        assert_eq!(markup.one_time_keyboard, Some(false));
        assert_eq!(markup.selective, Some(true));
    }

    #[tokio::test]
    async fn mixed_native_button_types_preserve_their_request_payloads() {
        let keyboard = ReplyKeyboard::builder()
            .row([
                ReplyButton::text("Plain"),
                ReplyButton::request_contact("Phone"),
                ReplyButton::request_location("Location"),
                ReplyButton::request_poll("Quiz", Some(PollType::Quiz)),
            ])
            .row([
                ReplyButton::request_users(
                    "User",
                    KeyboardButtonRequestUsers::new(42).user_is_bot(false),
                ),
                ReplyButton::request_chat("Chat", KeyboardButtonRequestChat::new(43, false)),
                ReplyButton::request_managed_bot("Bot", KeyboardButtonRequestManagedBot::new(44)),
                ReplyButton::web_app("App", WebAppInfo::new("https://example.com")),
            ])
            .build();

        let markup = markup(&keyboard, &DataMap::new()).await;
        let first = &markup.keyboard[0];
        assert!(first[0].request_contact.is_none());
        assert!(first[0].request_location.is_none());
        assert!(first[0].request_poll.is_none());
        assert_eq!(first[1].request_contact, Some(true));
        assert_eq!(first[2].request_location, Some(true));
        assert_eq!(
            first[3]
                .request_poll
                .as_ref()
                .and_then(|poll| poll.r#type.as_deref()),
            Some("quiz"),
        );
        let second = &markup.keyboard[1];
        assert_eq!(second[0].request_users.as_ref().unwrap().request_id, 42);
        assert_eq!(
            second[0].request_users.as_ref().unwrap().user_is_bot,
            Some(false),
        );
        assert_eq!(second[1].request_chat.as_ref().unwrap().request_id, 43);
        assert_eq!(second[2].request_managed_bot.as_ref().unwrap().request_id, 44);
        assert_eq!(
            second[3].web_app.as_ref().unwrap().url.as_ref(),
            "https://example.com",
        );
    }

    #[tokio::test]
    async fn web_app_url_renders_from_current_data() {
        let data = [("path".into(), Value::from("form"))].into_iter().collect();
        let keyboard = ReplyKeyboard::builder()
            .push(ReplyButton::web_app_dynamic(
                "App",
                FormatText::new("https://example.com/{path}"),
            ))
            .build();

        assert_eq!(
            markup(&keyboard, &data).await.keyboard[0][0]
                .web_app
                .as_ref()
                .unwrap()
                .url
                .as_ref(),
            "https://example.com/form",
        );
    }

    #[tokio::test]
    async fn reply_button_renders_dynamic_style_and_emoji_with_native_request() {
        let data = [("emoji".into(), Value::from("location_emoji"))]
            .into_iter()
            .collect();
        let keyboard = ReplyKeyboard::builder()
            .push(
                ReplyButton::request_location("Location")
                    .style_dynamic(|render: crate::entities::RenderContext| async move {
                        render
                            .data
                            .contains_key("emoji")
                            .then_some(ButtonStyle::Success)
                    })
                    .icon_custom_emoji_id_dynamic(FormatText::new("{emoji}")),
            )
            .build();

        let markup = markup(&keyboard, &data).await;
        let button = &markup.keyboard[0][0];
        assert_eq!(button.request_location, Some(true));
        assert_eq!(button.style.as_deref(), Some("success"));
        assert_eq!(button.icon_custom_emoji_id.as_deref(), Some("location_emoji"));
    }

    #[tokio::test]
    async fn hidden_buttons_drop_empty_rows_and_reappear_when_data_changes() {
        let keyboard = ReplyKeyboard::builder()
            .row([ReplyButton::text("Hidden").when(WhenCondition::data_field("show"))])
            .row([])
            .row([ReplyButton::text("Always")])
            .build();

        let empty = DataMap::new();
        let visible = [("show".into(), Value::from(true))].into_iter().collect();
        assert_eq!(markup(&keyboard, &empty).await.keyboard.len(), 1);
        assert_eq!(markup(&keyboard, &visible).await.keyboard.len(), 2);
    }

    #[tokio::test]
    async fn absent_or_hidden_keyboards_produce_no_markup() {
        let empty = ReplyKeyboard::builder().row([]).build();
        let all_hidden = ReplyKeyboard::builder()
            .push(ReplyButton::text("Hidden").when(WhenCondition::data_field("show")))
            .build();
        let hidden = ReplyKeyboard::builder()
            .push(ReplyButton::text("Choice"))
            .when(WhenCondition::data_field("show"))
            .build();

        for keyboard in [empty, all_hidden, hidden] {
            assert!(keyboard
                .render_keyboard_for_test(&context(), &DataMap::new())
                .await
                .is_none());
        }
    }

    #[tokio::test]
    async fn native_reply_buttons_do_not_consume_callbacks() {
        let keyboard = ReplyKeyboard::builder()
            .push(ReplyButton::text("Choice"))
            .build();
        assert!(keyboard
            .handle_callback_for_test(&context(), "td:any:choice")
            .await
            .is_none());
        assert!(ReplyKeyboardRemove::default()
            .handle_callback_for_test(&context(), "td:any:choice")
            .await
            .is_none());
    }

    #[tokio::test]
    async fn remove_keyboard_renders_selective_markup_and_respects_visibility() {
        let remove = ReplyKeyboardRemove::builder().selective(true).build();
        let Some(ReplyMarkup::ReplyKeyboardRemove(markup)) = remove
            .render_keyboard_for_test(&context(), &DataMap::new())
            .await
        else {
            panic!("remove keyboard");
        };
        assert!(markup.remove_keyboard);
        assert_eq!(markup.selective, Some(true));

        let hidden = ReplyKeyboardRemove::builder()
            .when(WhenCondition::data_field("show"))
            .build();
        assert!(hidden
            .render_keyboard_for_test(&context(), &DataMap::new())
            .await
            .is_none());
    }
}
