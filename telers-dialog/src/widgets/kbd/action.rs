use std::borrow::Cow;

use crate::entities::{Data, DataMap, StartMode};

/// Action produced by a keyboard widget callback.
///
/// These actions are executed by [`DialogManager`](crate::DialogManager) and
/// cover the common dialog transitions and state mutations needed by widgets.
#[derive(Clone, Debug)]
pub enum ButtonAction {
    /// Consume the callback without changing dialog state.
    Noop,
    /// Move to the next state in the current dialog declaration order.
    Next,
    /// Move to the previous state in the current dialog declaration order.
    Back,
    /// Switch to a specific state inside the current dialog.
    SwitchTo(Cow<'static, str>),
    /// Start another dialog state, optionally with start data and stack mode.
    Start {
        state: Cow<'static, str>,
        data: Data,
        mode: StartMode,
    },
    /// Close the current dialog context.
    Done,
    /// Close the current dialog context and pass a result to the parent dialog.
    DoneWithResult(Data),
    /// Replace the whole `dialog_data` map for the current context.
    SetDialogData(DataMap),
    /// Set one value in `dialog_data` for the current context.
    SetDialogValue { key: Cow<'static, str>, value: Data },
    /// Merge several entries into `dialog_data` for the current context.
    ExtendDialogData(DataMap),
    /// Replace the whole `widget_data` map for the current context.
    SetWidgetData(DataMap),
    /// Set one value in `widget_data` for the current context.
    SetWidgetValue { key: Cow<'static, str>, value: Data },
    /// Merge several entries into `widget_data` for the current context.
    ExtendWidgetData(DataMap),
    /// Execute several actions in order.
    Chain(Box<[ButtonAction]>),
}

impl ButtonAction {
    /// Create a no-op action.
    #[inline]
    #[must_use]
    pub const fn noop() -> Self {
        Self::Noop
    }

    /// Create a [`ButtonAction::Next`] action.
    #[inline]
    #[must_use]
    pub const fn next() -> Self {
        Self::Next
    }

    /// Create a [`ButtonAction::Back`] action.
    #[inline]
    #[must_use]
    pub const fn back() -> Self {
        Self::Back
    }

    /// Create a [`ButtonAction::SwitchTo`] action.
    #[must_use]
    pub fn switch_to(state: impl Into<Cow<'static, str>>) -> Self {
        Self::SwitchTo(state.into())
    }

    /// Create a [`ButtonAction::Start`] action.
    #[must_use]
    pub fn start(
        state: impl Into<Cow<'static, str>>,
        data: impl Into<Data>,
        mode: StartMode,
    ) -> Self {
        Self::Start {
            state: state.into(),
            data: data.into(),
            mode,
        }
    }

    #[inline]
    #[must_use]
    pub const fn done() -> Self {
        Self::Done
    }

    /// Create a [`ButtonAction::DoneWithResult`] action.
    #[must_use]
    pub fn done_with_result(result: impl Into<Data>) -> Self {
        Self::DoneWithResult(result.into())
    }

    /// Create a [`ButtonAction::SetDialogData`] action.
    #[inline]
    #[must_use]
    pub fn set_dialog_data(data: DataMap) -> Self {
        Self::SetDialogData(data)
    }

    /// Create a [`ButtonAction::SetDialogValue`] action.
    #[must_use]
    pub fn set_dialog_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetDialogValue {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Create a [`ButtonAction::SetWidgetData`] action.
    #[inline]
    #[must_use]
    pub fn set_widget_data(data: DataMap) -> Self {
        Self::SetWidgetData(data)
    }

    /// Create a [`ButtonAction::SetWidgetValue`] action.
    #[must_use]
    pub fn set_widget_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetWidgetValue {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Create a [`ButtonAction::ExtendDialogData`] action from key/value pairs.
    #[must_use]
    pub fn extend_dialog_data<I, K, V>(entries: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<Data>,
    {
        Self::ExtendDialogData(
            entries
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }

    /// Create a [`ButtonAction::ExtendWidgetData`] action from key/value pairs.
    #[must_use]
    pub fn extend_widget_data<I, K, V>(entries: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<Data>,
    {
        Self::ExtendWidgetData(
            entries
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }

    /// Create a [`ButtonAction::Chain`] action.
    #[must_use]
    pub fn chain(actions: impl IntoIterator<Item = ButtonAction>) -> Self {
        Self::Chain(actions.into_iter().collect())
    }
}
