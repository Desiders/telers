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

#[cfg(test)]
mod tests {
    use super::ButtonAction;
    use crate::entities::{DataMap, StartMode};
    use serde_json::json;

    #[test]
    fn noop_constructs_noop() {
        assert!(matches!(ButtonAction::noop(), ButtonAction::Noop));
    }

    #[test]
    fn next_constructs_next() {
        assert!(matches!(ButtonAction::next(), ButtonAction::Next));
    }

    #[test]
    fn back_constructs_back() {
        assert!(matches!(ButtonAction::back(), ButtonAction::Back));
    }

    #[test]
    fn done_constructs_done() {
        assert!(matches!(ButtonAction::done(), ButtonAction::Done));
    }

    #[test]
    fn switch_to_holds_state() {
        match ButtonAction::switch_to("s") {
            ButtonAction::SwitchTo(state) => assert_eq!(state, "s"),
            other => panic!("expected SwitchTo, got {other:?}"),
        }
    }

    #[test]
    fn start_holds_state_data_and_mode() {
        match ButtonAction::start("s", json!(1), StartMode::NewStack) {
            ButtonAction::Start {
                state,
                data,
                mode,
            } => {
                assert_eq!(state, "s");
                assert_eq!(data, json!(1));
                assert_eq!(mode, StartMode::NewStack);
            }
            other => panic!("expected Start, got {other:?}"),
        }
    }

    #[test]
    fn done_with_result_holds_value() {
        match ButtonAction::done_with_result(json!("r")) {
            ButtonAction::DoneWithResult(result) => assert_eq!(result, json!("r")),
            other => panic!("expected DoneWithResult, got {other:?}"),
        }
    }

    #[test]
    fn set_dialog_value_holds_key_and_value() {
        match ButtonAction::set_dialog_value("k", "v") {
            ButtonAction::SetDialogValue {
                key,
                value,
            } => {
                assert_eq!(key, "k");
                assert_eq!(value, json!("v"));
            }
            other => panic!("expected SetDialogValue, got {other:?}"),
        }
    }

    #[test]
    fn set_widget_value_holds_key_and_value() {
        match ButtonAction::set_widget_value("k", 3) {
            ButtonAction::SetWidgetValue {
                key,
                value,
            } => {
                assert_eq!(key, "k");
                assert_eq!(value, json!(3));
            }
            other => panic!("expected SetWidgetValue, got {other:?}"),
        }
    }

    #[test]
    fn extend_dialog_data_collects_entries() {
        match ButtonAction::extend_dialog_data([("a", "1"), ("b", "2")]) {
            ButtonAction::ExtendDialogData(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(map.get("a"), Some(&json!("1")));
                assert_eq!(map.get("b"), Some(&json!("2")));
            }
            other => panic!("expected ExtendDialogData, got {other:?}"),
        }
    }

    #[test]
    fn extend_widget_data_collects_entries() {
        match ButtonAction::extend_widget_data([("a", "1"), ("b", "2")]) {
            ButtonAction::ExtendWidgetData(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(map.get("a"), Some(&json!("1")));
                assert_eq!(map.get("b"), Some(&json!("2")));
            }
            other => panic!("expected ExtendWidgetData, got {other:?}"),
        }
    }

    #[test]
    fn set_dialog_data_holds_map() {
        let mut data = DataMap::new();
        data.insert("k".into(), json!("v"));
        match ButtonAction::set_dialog_data(data) {
            ButtonAction::SetDialogData(map) => assert_eq!(map.get("k"), Some(&json!("v"))),
            other => panic!("expected SetDialogData, got {other:?}"),
        }
    }

    #[test]
    fn set_widget_data_holds_map() {
        let mut data = DataMap::new();
        data.insert("k".into(), json!(3));
        match ButtonAction::set_widget_data(data) {
            ButtonAction::SetWidgetData(map) => assert_eq!(map.get("k"), Some(&json!(3))),
            other => panic!("expected SetWidgetData, got {other:?}"),
        }
    }

    #[test]
    fn chain_collects_actions_in_order() {
        match ButtonAction::chain([ButtonAction::noop(), ButtonAction::next()]) {
            ButtonAction::Chain(actions) => {
                assert_eq!(actions.len(), 2);
                assert!(matches!(actions[0], ButtonAction::Noop));
                assert!(matches!(actions[1], ButtonAction::Next));
            }
            other => panic!("expected Chain, got {other:?}"),
        }
    }

    #[test]
    fn chain_from_empty_iterator_is_empty() {
        match ButtonAction::chain(std::iter::empty()) {
            ButtonAction::Chain(actions) => assert_eq!(actions.len(), 0),
            other => panic!("expected Chain, got {other:?}"),
        }
    }
}
