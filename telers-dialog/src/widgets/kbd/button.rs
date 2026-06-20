use std::{borrow::Cow, sync::Arc};

use async_fn_traits::AsyncFn1;
use telers::types::{
    CopyTextButton, InlineKeyboardButton, LoginUrl, SwitchInlineQueryChosenChat, WebAppInfo,
};
use tracing::debug;

use super::{format_callback_data, parse_callback_data, ButtonAction, ClickContext};
use crate::{
    entities::{Data, RenderContext, StartMode},
    future::BoxFuture,
    widgets::Text,
};

type ButtonClickHandler =
    dyn Fn(ClickContext) -> BoxFuture<'static, ButtonAction> + Send + Sync + 'static;

/// Distinguishes the underlying Telegram action carried by a [`Button`].
///
/// Callback variants ([`ButtonKind::Callback`], [`ButtonKind::OnClick`]) carry
/// a widget id used to match the callback data; all other variants are passive
/// (the Telegram client handles them) and never produce a [`ButtonAction`].
#[derive(Clone)]
enum ButtonKind {
    Callback(ButtonAction),
    OnClick(Arc<ButtonClickHandler>),
    Url(Arc<dyn Text>),
    WebApp(Arc<dyn Text>),
    LoginUrl(LoginUrl),
    SwitchInlineQuery(Arc<dyn Text>),
    SwitchInlineQueryCurrentChat(Arc<dyn Text>),
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),
    CopyText(Arc<dyn Text>),
}

impl ButtonKind {
    /// Returns true when this kind is resolved via a callback data round-trip
    /// (and therefore needs a stable widget id).
    #[inline]
    fn is_callback(&self) -> bool {
        matches!(self, Self::Callback(_) | Self::OnClick(_))
    }
}

/// Visual style for an inline keyboard button.
///
/// Maps directly onto the optional `style` field supported by telers'
/// [`InlineKeyboardButton`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Red-tinted button for destructive or dangerous actions.
    Danger,
    /// Green-tinted button for success or confirmation actions.
    Success,
    /// Blue-tinted button for primary actions.
    Primary,
}

impl ButtonStyle {
    /// String value expected by the Telegram client.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Danger => "danger",
            Self::Success => "success",
            Self::Primary => "primary",
        }
    }
}

/// Inline-keyboard button rendered inside an [`InlineKeyboard`] row.
///
/// Construct a button via one of the typed constructors:
/// - callback-style: [`Button::action`], [`Button::on_click`], plus the
///   convenience wrappers ([`Button::next`], [`Button::back`],
///   [`Button::switch_to`], [`Button::start`], [`Button::done`],
///   [`Button::done_with_result`], [`Button::set_dialog_value`]);
/// - passive: [`Button::url`] / [`Button::url_dynamic`],
///   [`Button::web_app`] / [`Button::web_app_dynamic`], [`Button::login_url`],
///   the `switch_inline_query*` family, and
///   [`Button::copy_text`] / [`Button::copy_text_dynamic`].
///
/// Callback buttons require a stable widget id (`id`) so [`InlineKeyboard`] can
/// dispatch the callback back to the right button. Passive buttons are
/// resolved entirely on the Telegram client and have no id.
///
/// Visual styling is applied with [`Button::style`] (or one of the shortcuts
/// [`Button::danger`], [`Button::success`], [`Button::primary`]) and
/// [`Button::icon_custom_emoji_id`].
///
/// [`InlineKeyboard`]: crate::widgets::InlineKeyboard
#[derive(Clone)]
pub struct Button {
    id: Option<Cow<'static, str>>,
    text: Arc<dyn Text>,
    kind: ButtonKind,
    style: Option<ButtonStyle>,
    icon_custom_emoji_id: Option<Cow<'static, str>>,
}

impl Button {
    #[inline]
    fn callback(id: Cow<'static, str>, text: Arc<dyn Text>, kind: ButtonKind) -> Self {
        debug_assert!(kind.is_callback(), "non-callback kind used with widget id");
        Self {
            id: Some(id),
            text,
            kind,
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    #[inline]
    fn passive(text: Arc<dyn Text>, kind: ButtonKind) -> Self {
        debug_assert!(!kind.is_callback(), "callback kind missing a widget id");
        Self {
            id: None,
            text,
            kind,
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a callback button bound to an explicit [`ButtonAction`].
    #[must_use]
    pub fn action(id: impl Into<Cow<'static, str>>, text: impl Text, action: ButtonAction) -> Self {
        Self::callback(id.into(), Arc::new(text), ButtonKind::Callback(action))
    }

    /// Create a callback button whose action is produced by an async closure.
    #[must_use]
    pub fn on_click<F>(id: impl Into<Cow<'static, str>>, text: impl Text, handler: F) -> Self
    where
        F: AsyncFn(ClickContext) -> ButtonAction
            + AsyncFn1<ClickContext, Output = ButtonAction>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<ClickContext>>::OutputFuture: Send + 'static,
    {
        let handler = Arc::new(handler);
        let kind = ButtonKind::OnClick(Arc::new(move |click| {
            let handler = handler.clone();
            Box::pin(async move { handler(click).await })
        }));
        Self::callback(id.into(), Arc::new(text), kind)
    }

    /// Create a button that advances to the next dialog state.
    #[inline]
    #[must_use]
    pub fn next(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::next())
    }

    /// Create a button that returns to the previous dialog state.
    #[inline]
    #[must_use]
    pub fn back(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::back())
    }

    /// Create a button that switches to a specific state inside the current dialog.
    #[inline]
    #[must_use]
    pub fn switch_to(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        state: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::action(id, text, ButtonAction::switch_to(state))
    }

    /// Create a button that starts another dialog state with start data and a stack mode.
    #[inline]
    #[must_use]
    pub fn start(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        state: impl Into<Cow<'static, str>>,
        data: impl Into<Data>,
        mode: StartMode,
    ) -> Self {
        Self::action(id, text, ButtonAction::start(state, data, mode))
    }

    /// Create a button that closes the current dialog context.
    #[inline]
    #[must_use]
    pub fn done(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::done())
    }

    /// Create a button that closes the dialog and forwards a result to its parent.
    #[inline]
    #[must_use]
    pub fn done_with_result(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        result: impl Into<Data>,
    ) -> Self {
        Self::action(id, text, ButtonAction::done_with_result(result))
    }

    /// Create a button that writes a single value into `dialog_data`.
    #[inline]
    #[must_use]
    pub fn set_dialog_value(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Data>,
    ) -> Self {
        Self::action(id, text, ButtonAction::set_dialog_value(key, value))
    }

    /// Create a URL button with a static URL.
    #[must_use]
    pub fn url(text: impl Text, url: impl Into<Cow<'static, str>>) -> Self {
        Self::url_dynamic(text, url.into().to_string())
    }

    /// Create a URL button whose target is rendered from dialog data.
    #[must_use]
    pub fn url_dynamic(text: impl Text, url: impl Text) -> Self {
        Self::passive(Arc::new(text), ButtonKind::Url(Arc::new(url)))
    }

    /// Create a Web App button with a static URL.
    #[must_use]
    pub fn web_app(text: impl Text, web_app: impl Into<WebAppInfo>) -> Self {
        let url = web_app.into().url.to_string();
        Self::web_app_dynamic(text, url)
    }

    /// Create a Web App button whose URL is rendered from dialog data.
    #[must_use]
    pub fn web_app_dynamic(text: impl Text, url: impl Text) -> Self {
        Self::passive(Arc::new(text), ButtonKind::WebApp(Arc::new(url)))
    }

    /// Create a login-URL button.
    #[must_use]
    pub fn login_url(text: impl Text, login_url: impl Into<LoginUrl>) -> Self {
        Self::passive(Arc::new(text), ButtonKind::LoginUrl(login_url.into()))
    }

    /// Create a button that opens inline mode in another chat with a static query.
    #[must_use]
    pub fn switch_inline_query(text: impl Text, query: impl Into<Cow<'static, str>>) -> Self {
        Self::switch_inline_query_dynamic(text, query.into().to_string())
    }

    /// Create a button that opens inline mode in another chat with a dynamic query.
    #[must_use]
    pub fn switch_inline_query_dynamic(text: impl Text, query: impl Text) -> Self {
        Self::passive(
            Arc::new(text),
            ButtonKind::SwitchInlineQuery(Arc::new(query)),
        )
    }

    /// Create a button that opens inline mode in the current chat with a static query.
    #[must_use]
    pub fn switch_inline_query_current_chat(
        text: impl Text,
        query: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::switch_inline_query_current_chat_dynamic(text, query.into().to_string())
    }

    /// Create a button that opens inline mode in the current chat with a dynamic query.
    #[must_use]
    pub fn switch_inline_query_current_chat_dynamic(text: impl Text, query: impl Text) -> Self {
        Self::passive(
            Arc::new(text),
            ButtonKind::SwitchInlineQueryCurrentChat(Arc::new(query)),
        )
    }

    /// Create a button that opens inline mode in a chosen chat.
    #[must_use]
    pub fn switch_inline_query_chosen_chat(
        text: impl Text,
        query: impl Into<SwitchInlineQueryChosenChat>,
    ) -> Self {
        Self::passive(
            Arc::new(text),
            ButtonKind::SwitchInlineQueryChosenChat(query.into()),
        )
    }

    /// Create a button that copies static text to the clipboard.
    #[must_use]
    pub fn copy_text(text: impl Text, copy_text: impl Into<CopyTextButton>) -> Self {
        let copy = copy_text.into().text.to_string();
        Self::copy_text_dynamic(text, copy)
    }

    /// Create a button that copies dynamic text to the clipboard.
    #[must_use]
    pub fn copy_text_dynamic(text: impl Text, copy_text: impl Text) -> Self {
        Self::passive(Arc::new(text), ButtonKind::CopyText(Arc::new(copy_text)))
    }

    /// Apply the given style to this button.
    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Style this button as a destructive (red) action.
    #[inline]
    #[must_use]
    pub fn danger(self) -> Self {
        self.style(ButtonStyle::Danger)
    }

    /// Style this button as a success (green) action.
    #[inline]
    #[must_use]
    pub fn success(self) -> Self {
        self.style(ButtonStyle::Success)
    }

    /// Style this button as a primary (blue) action.
    #[inline]
    #[must_use]
    pub fn primary(self) -> Self {
        self.style(ButtonStyle::Primary)
    }

    /// Set the custom emoji shown before the button text.
    #[must_use]
    pub fn icon_custom_emoji_id(mut self, emoji_id: impl Into<Cow<'static, str>>) -> Self {
        self.icon_custom_emoji_id = Some(emoji_id.into());
        self
    }

    pub(crate) fn render<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, InlineKeyboardButton> {
        Box::pin(async move {
            let ctx = render_ctx.context.as_ref();
            let button =
                InlineKeyboardButton::new(self.text.render_text_in_context(render_ctx).await)
                    .style_option(self.style.map(|style| style.as_str()))
                    .icon_custom_emoji_id_option(self.icon_custom_emoji_id.as_deref());

            match &self.kind {
                ButtonKind::Callback(_) | ButtonKind::OnClick(_) => {
                    let id = self.id.as_deref().unwrap_or("");
                    button.callback_data(format_callback_data(ctx, id, None))
                }
                ButtonKind::Url(url) => {
                    let rendered_url = url.render_text_in_context(render_ctx).await;
                    button.url(rendered_url)
                }
                ButtonKind::WebApp(url) => {
                    let rendered_url = url.render_text_in_context(render_ctx).await;
                    button.web_app(WebAppInfo::new(rendered_url))
                }
                ButtonKind::LoginUrl(login_url) => button.login_url(login_url.clone()),
                ButtonKind::SwitchInlineQuery(query) => {
                    let rendered_query = query.render_text_in_context(render_ctx).await;
                    button.switch_inline_query(rendered_query)
                }
                ButtonKind::SwitchInlineQueryCurrentChat(query) => {
                    let rendered_query = query.render_text_in_context(render_ctx).await;
                    button.switch_inline_query_current_chat(rendered_query)
                }
                ButtonKind::SwitchInlineQueryChosenChat(query) => {
                    button.switch_inline_query_chosen_chat(query.clone())
                }
                ButtonKind::CopyText(copy_text) => {
                    let rendered_copy = copy_text.render_text_in_context(render_ctx).await;
                    button.copy_text(CopyTextButton::new(rendered_copy))
                }
            }
        })
    }

    pub(crate) fn resolve_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            let id = self.id.as_deref()?;
            let ctx = click.context.as_ref();
            let parsed = parse_callback_data(ctx, click.callback_data.as_str())?;
            if parsed.target_id != id || parsed.payload.is_some() {
                return None;
            }
            match &self.kind {
                ButtonKind::Callback(action) => {
                    debug!(context_id = %ctx.id, button_id = %id, "Resolved button callback");
                    Some(action.clone())
                }
                ButtonKind::OnClick(handler) => {
                    debug!(context_id = %ctx.id, button_id = %id, "Resolved button click handler");
                    Some(handler(click.clone()).await)
                }
                ButtonKind::Url(_)
                | ButtonKind::WebApp(_)
                | ButtonKind::LoginUrl(_)
                | ButtonKind::SwitchInlineQuery(_)
                | ButtonKind::SwitchInlineQueryCurrentChat(_)
                | ButtonKind::SwitchInlineQueryChosenChat(_)
                | ButtonKind::CopyText(_) => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::ButtonStyle;
    use crate::{
        entities::{Context, DataMap},
        widgets::{Button, ButtonAction, FormatText, InlineKeyboard, Keyboard},
    };

    /// Render a single button inside an [`InlineKeyboard`] and return the
    /// resulting first-row, first-column [`InlineKeyboardButton`].
    async fn render_single(
        button: Button,
        ctx: &Context,
        data: &DataMap,
    ) -> telers::types::InlineKeyboardButton {
        let keyboard = InlineKeyboard::builder().row([button]).build();
        let markup = keyboard
            .render_keyboard_for_test(ctx, data)
            .await
            .expect("keyboard should be visible");
        let rows = markup.inline_keyboard().expect("inline keyboard markup");
        rows[0][0].clone()
    }

    #[test]
    fn button_style_as_str_maps_each_variant() {
        assert_eq!(ButtonStyle::Danger.as_str(), "danger");
        assert_eq!(ButtonStyle::Success.as_str(), "success");
        assert_eq!(ButtonStyle::Primary.as_str(), "primary");
    }

    #[tokio::test]
    async fn danger_shortcut_renders_danger_style() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop()).danger();

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.style.as_deref(), Some("danger"));
    }

    #[tokio::test]
    async fn success_shortcut_renders_success_style() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop()).success();

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.style.as_deref(), Some("success"));
    }

    #[tokio::test]
    async fn primary_shortcut_renders_primary_style() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop()).primary();

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.style.as_deref(), Some("primary"));
    }

    #[tokio::test]
    async fn style_method_renders_primary_style() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop()).style(ButtonStyle::Primary);

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.style.as_deref(), Some("primary"));
    }

    #[tokio::test]
    async fn no_style_renders_none() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop());

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.style, None);
    }

    #[tokio::test]
    async fn icon_custom_emoji_id_is_rendered() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::action("a", "A", ButtonAction::noop()).icon_custom_emoji_id("123");

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.icon_custom_emoji_id.as_deref(), Some("123"));
    }

    #[tokio::test]
    async fn static_url_button_renders_url_without_callback() {
        let ctx = Context::new("", "state", Value::Null);
        let button = Button::url("Open", "https://x.test");

        let rendered = render_single(button, &ctx, &DataMap::new()).await;

        assert_eq!(rendered.url.as_deref(), Some("https://x.test"));
        assert_eq!(rendered.callback_data, None);
    }

    #[tokio::test]
    async fn dynamic_url_button_renders_from_data() {
        let ctx = Context::new("", "state", Value::Null);
        let mut data = DataMap::new();
        data.insert("u".into(), json!("https://dyn.test"));
        let button = Button::url_dynamic("Open", FormatText::new("{u}"));

        let rendered = render_single(button, &ctx, &data).await;

        assert_eq!(rendered.url.as_deref(), Some("https://dyn.test"));
        assert_eq!(rendered.callback_data, None);
    }

    #[tokio::test]
    async fn dynamic_copy_text_button_renders_from_data() {
        let ctx = Context::new("", "state", Value::Null);
        let mut data = DataMap::new();
        data.insert("c".into(), json!("code123"));
        let button = Button::copy_text_dynamic("C", FormatText::new("{c}"));

        let rendered = render_single(button, &ctx, &data).await;

        let copy_text = rendered.copy_text.expect("copy_text should be present");
        assert_eq!(&*copy_text.text, "code123");
        assert_eq!(rendered.callback_data, None);
    }

    #[tokio::test]
    async fn passive_button_yields_no_callback_action() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::builder()
            .row([Button::url("Open", "https://x.test")])
            .build();

        let action = keyboard
            .handle_callback_for_test(&ctx, &format!("td:{}:anything", ctx.id))
            .await;

        assert!(action.is_none());
    }

    #[tokio::test]
    async fn callback_button_resolves_done_action() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::builder()
            .row([Button::done("close", "Close")])
            .build();

        let action = keyboard
            .handle_callback_for_test(&ctx, &format!("td:{}:close", ctx.id))
            .await;

        assert!(matches!(action, Some(ButtonAction::Done)));
    }
}
