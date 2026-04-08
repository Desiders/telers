use std::borrow::Cow;

use telers::types::InlineKeyboardButton;
use tracing::debug;

use super::{format_callback_data, parse_callback_data, ButtonAction};
use crate::{
    entities::{Context, Data, DataMap, StartMode},
    widgets::Text,
};

enum ButtonKind {
    Callback(ButtonAction),
    Url(Cow<'static, str>),
}

/// Inline keyboard button with a stable widget id.
///
/// Most constructors are thin convenience helpers over [`ButtonAction`].
pub struct Button {
    id: Cow<'static, str>,
    text: Box<dyn Text>,
    kind: ButtonKind,
}

impl Button {
    /// Create a callback button with an explicit action.
    #[must_use]
    pub fn action(id: impl Into<Cow<'static, str>>, text: impl Text, action: ButtonAction) -> Self {
        Self {
            id: id.into(),
            text: Box::new(text),
            kind: ButtonKind::Callback(action),
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
            text: Box::new(text),
            kind: ButtonKind::Url(url.into()),
        }
    }

    pub(crate) fn render(&self, ctx: &Context, data: &DataMap) -> InlineKeyboardButton {
        let button = InlineKeyboardButton::new(self.text.render_text(data));
        match &self.kind {
            ButtonKind::Callback(_) => {
                button.callback_data(format_callback_data(ctx, &self.id, None))
            }
            ButtonKind::Url(url) => button.url(url.clone()),
        }
    }

    pub(crate) fn resolve_callback(
        &self,
        ctx: &Context,
        callback_data: &str,
    ) -> Option<ButtonAction> {
        let parsed = parse_callback_data(ctx, callback_data)?;
        if parsed.target_id != self.id.as_ref() || parsed.payload.is_some() {
            return None;
        }
        match &self.kind {
            ButtonKind::Callback(action) => {
                debug!(context_id = %ctx.id, button_id = %self.id, "Resolved button callback");
                Some(action.clone())
            }
            ButtonKind::Url(_) => None,
        }
    }
}
