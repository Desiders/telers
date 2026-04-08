use std::borrow::Cow;

use crate::entities::{Data, DataMap, StartMode};

#[derive(Clone, Debug)]
pub enum ButtonAction {
    Noop,
    Next,
    Back,
    SwitchTo(Cow<'static, str>),
    Start {
        state: Cow<'static, str>,
        data: Data,
        mode: StartMode,
    },
    Done,
    DoneWithResult(Data),
    SetDialogData(DataMap),
    SetDialogValue {
        key: Cow<'static, str>,
        value: Data,
    },
    SetWidgetData(DataMap),
    SetWidgetValue {
        key: Cow<'static, str>,
        value: Data,
    },
    Chain(Box<[ButtonAction]>),
}

impl ButtonAction {
    #[inline]
    #[must_use]
    pub const fn noop() -> Self {
        Self::Noop
    }

    #[inline]
    #[must_use]
    pub const fn next() -> Self {
        Self::Next
    }

    #[inline]
    #[must_use]
    pub const fn back() -> Self {
        Self::Back
    }

    #[must_use]
    pub fn switch_to(state: impl Into<Cow<'static, str>>) -> Self {
        Self::SwitchTo(state.into())
    }

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

    #[must_use]
    pub fn done_with_result(result: impl Into<Data>) -> Self {
        Self::DoneWithResult(result.into())
    }

    #[inline]
    #[must_use]
    pub fn set_dialog_data(data: DataMap) -> Self {
        Self::SetDialogData(data)
    }

    #[must_use]
    pub fn set_dialog_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetDialogValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[inline]
    #[must_use]
    pub fn set_widget_data(data: DataMap) -> Self {
        Self::SetWidgetData(data)
    }

    #[must_use]
    pub fn set_widget_value(key: impl Into<Cow<'static, str>>, value: impl Into<Data>) -> Self {
        Self::SetWidgetValue {
            key: key.into(),
            value: value.into(),
        }
    }

    #[must_use]
    pub fn chain(actions: impl IntoIterator<Item = ButtonAction>) -> Self {
        Self::Chain(actions.into_iter().collect())
    }
}
