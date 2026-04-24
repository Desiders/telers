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

/// Button style for Telegram inline keyboard buttons.
///
/// These map directly to Telegram's button styles:
/// - `Danger`: Red-tinted button (for destructive actions)
/// - `Success`: Green-tinted button (for confirmation actions)
/// - `Primary`: Blue-tinted button (default styling emphasis)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Red-tinted button for destructive/dangerous actions.
    Danger,
    /// Green-tinted button for success/confirmation actions.
    Success,
    /// Blue-tinted button for primary actions.
    Primary,
}

impl ButtonStyle {
    /// Convert to the string value expected by Telegram API.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Danger => "danger",
            Self::Success => "success",
            Self::Primary => "primary",
        }
    }
}

/// Inline keyboard button with a stable widget id.
///
/// Most constructors are thin convenience helpers over [`ButtonAction`].
/// Supports optional `style` and `icon_custom_emoji_id` for visual customization.
#[derive(Clone)]
pub struct Button {
    id: Cow<'static, str>,
    text: Arc<dyn Text>,
    kind: ButtonKind,
    /// Optional button style (danger, success, primary).
    style: Option<ButtonStyle>,
    /// Optional custom emoji ID to display before button text.
    icon_custom_emoji_id: Option<Cow<'static, str>>,
}

impl Button {
    /// Create a callback button with an explicit action.
    #[must_use]
    pub fn action(id: impl Into<Cow<'static, str>>, text: impl Text, action: ButtonAction) -> Self {
        Self {
            id: id.into(),
            text: Arc::new(text),
            kind: ButtonKind::Callback(action),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a callback button with an async click handler.
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
        Self {
            id: id.into(),
            text: Arc::new(text),
            kind: ButtonKind::OnClick(Arc::new(move |click| {
                let handler = handler.clone();
                Box::pin(async move { handler(click).await })
            })),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a button that moves to the next dialog state.
    #[inline]
    #[must_use]
    pub fn next(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::next())
    }

    /// Create a button that moves to the previous dialog state.
    #[inline]
    #[must_use]
    pub fn back(id: impl Into<Cow<'static, str>>, text: impl Text) -> Self {
        Self::action(id, text, ButtonAction::back())
    }

    /// Create a button that switches to a specific state in the current dialog.
    #[inline]
    #[must_use]
    pub fn switch_to(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        state: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::action(id, text, ButtonAction::switch_to(state))
    }

    /// Create a button that starts another dialog state.
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

    /// Create a button that closes the current dialog and returns a result.
    #[inline]
    #[must_use]
    pub fn done_with_result(
        id: impl Into<Cow<'static, str>>,
        text: impl Text,
        result: impl Into<Data>,
    ) -> Self {
        Self::action(id, text, ButtonAction::done_with_result(result))
    }

    /// Create a button that writes one value into `dialog_data`.
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

    /// Create a URL button with a dynamic URL rendered from data.
    #[must_use]
    pub fn url_dynamic(text: impl Text, url: impl Text) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::Url(Arc::new(url)),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a Web App button with a static URL.
    #[must_use]
    pub fn web_app(text: impl Text, web_app: impl Into<WebAppInfo>) -> Self {
        let url = web_app.into().url.to_string();
        Self::web_app_dynamic(text, url)
    }

    /// Create a Web App button with a dynamic URL rendered from data.
    #[must_use]
    pub fn web_app_dynamic(text: impl Text, url: impl Text) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::WebApp(Arc::new(url)),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a login URL button.
    #[must_use]
    pub fn login_url(text: impl Text, login_url: impl Into<LoginUrl>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::LoginUrl(login_url.into()),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a button that opens inline mode in another chat with a static query.
    #[must_use]
    pub fn switch_inline_query(text: impl Text, query: impl Into<Cow<'static, str>>) -> Self {
        Self::switch_inline_query_dynamic(text, query.into().to_string())
    }

    /// Create a button that opens inline mode in another chat with a dynamic query.
    #[must_use]
    pub fn switch_inline_query_dynamic(text: impl Text, query: impl Text) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::SwitchInlineQuery(Arc::new(query)),
            style: None,
            icon_custom_emoji_id: None,
        }
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
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::SwitchInlineQueryCurrentChat(Arc::new(query)),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Create a button that opens inline mode in a chosen chat.
    #[must_use]
    pub fn switch_inline_query_chosen_chat(
        text: impl Text,
        query: impl Into<SwitchInlineQueryChosenChat>,
    ) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::SwitchInlineQueryChosenChat(query.into()),
            style: None,
            icon_custom_emoji_id: None,
        }
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
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::CopyText(Arc::new(copy_text)),
            style: None,
            icon_custom_emoji_id: None,
        }
    }

    /// Set the button style.
    #[must_use]
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Set the button to danger style (red-tinted).
    #[must_use]
    pub fn danger(self) -> Self {
        self.style(ButtonStyle::Danger)
    }

    /// Set the button to success style (green-tinted).
    #[must_use]
    pub fn success(self) -> Self {
        self.style(ButtonStyle::Success)
    }

    /// Set the button to primary style (blue-tinted).
    #[must_use]
    pub fn primary(self) -> Self {
        self.style(ButtonStyle::Primary)
    }

    /// Set the custom emoji ID to display before button text.
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
            let mut button =
                InlineKeyboardButton::new(self.text.render_text_in_context(render_ctx).await);

            // Apply style if set
            if let Some(style) = &self.style {
                button = button.style(style.as_str());
            }

            // Apply custom emoji if set
            if let Some(emoji_id) = &self.icon_custom_emoji_id {
                button = button.icon_custom_emoji_id(emoji_id.as_ref());
            }

            match &self.kind {
                ButtonKind::Callback(_) | ButtonKind::OnClick(_) => {
                    button.callback_data(format_callback_data(ctx, &self.id, None))
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
            let ctx = click.context.as_ref();
            let parsed = parse_callback_data(ctx, click.callback_data.as_str())?;
            if parsed.target_id != self.id.as_ref() || parsed.payload.is_some() {
                return None;
            }
            match &self.kind {
                ButtonKind::Callback(action) => {
                    debug!(context_id = %ctx.id, button_id = %self.id, "Resolved button callback");
                    Some(action.clone())
                }
                ButtonKind::OnClick(handler) => {
                    debug!(context_id = %ctx.id, button_id = %self.id, "Resolved button click handler");
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
