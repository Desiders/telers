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
    Url(Cow<'static, str>),
    WebApp(WebAppInfo),
    LoginUrl(LoginUrl),
    SwitchInlineQuery(Cow<'static, str>),
    SwitchInlineQueryCurrentChat(Cow<'static, str>),
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),
    CopyText(CopyTextButton),
}

/// Inline keyboard button with a stable widget id.
///
/// Most constructors are thin convenience helpers over [`ButtonAction`].
#[derive(Clone)]
pub struct Button {
    id: Cow<'static, str>,
    text: Arc<dyn Text>,
    kind: ButtonKind,
}

impl Button {
    /// Create a callback button with an explicit action.
    #[must_use]
    pub fn action(id: impl Into<Cow<'static, str>>, text: impl Text, action: ButtonAction) -> Self {
        Self {
            id: id.into(),
            text: Arc::new(text),
            kind: ButtonKind::Callback(action),
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

    /// Create a URL button.
    #[must_use]
    pub fn url(text: impl Text, url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::Url(url.into()),
        }
    }

    /// Create a Web App button.
    #[must_use]
    pub fn web_app(text: impl Text, web_app: impl Into<WebAppInfo>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::WebApp(web_app.into()),
        }
    }

    /// Create a login URL button.
    #[must_use]
    pub fn login_url(text: impl Text, login_url: impl Into<LoginUrl>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::LoginUrl(login_url.into()),
        }
    }

    /// Create a button that opens inline mode in another chat.
    #[must_use]
    pub fn switch_inline_query(text: impl Text, query: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::SwitchInlineQuery(query.into()),
        }
    }

    /// Create a button that opens inline mode in the current chat.
    #[must_use]
    pub fn switch_inline_query_current_chat(
        text: impl Text,
        query: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::SwitchInlineQueryCurrentChat(query.into()),
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
        }
    }

    /// Create a button that copies text to the clipboard.
    #[must_use]
    pub fn copy_text(text: impl Text, copy_text: impl Into<CopyTextButton>) -> Self {
        Self {
            id: String::new().into(),
            text: Arc::new(text),
            kind: ButtonKind::CopyText(copy_text.into()),
        }
    }

    pub(crate) fn render<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, InlineKeyboardButton> {
        Box::pin(async move {
            let ctx = render_ctx.context.as_ref();
            let button =
                InlineKeyboardButton::new(self.text.render_text_in_context(render_ctx).await);
            match &self.kind {
                ButtonKind::Callback(_) | ButtonKind::OnClick(_) => {
                    button.callback_data(format_callback_data(ctx, &self.id, None))
                }
                ButtonKind::Url(url) => button.url(url.clone()),
                ButtonKind::WebApp(web_app) => button.web_app(web_app.clone()),
                ButtonKind::LoginUrl(login_url) => button.login_url(login_url.clone()),
                ButtonKind::SwitchInlineQuery(query) => button.switch_inline_query(query.clone()),
                ButtonKind::SwitchInlineQueryCurrentChat(query) => {
                    button.switch_inline_query_current_chat(query.clone())
                }
                ButtonKind::SwitchInlineQueryChosenChat(query) => {
                    button.switch_inline_query_chosen_chat(query.clone())
                }
                ButtonKind::CopyText(copy_text) => button.copy_text(copy_text.clone()),
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
