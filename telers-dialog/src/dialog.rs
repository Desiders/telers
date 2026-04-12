use crate::{
    entities::{Context, Data, LaunchMode, NewMessage, RenderContext},
    widgets::{ButtonAction, ClickContext},
    IntoWindow, Window,
};
use std::{collections::BTreeMap, sync::Arc};
use telers::types::Message;
use tracing::warn;

type ProcessResultHandler =
    dyn Fn(&Context, &Data, &Data) -> Option<ButtonAction> + Send + Sync + 'static;

pub trait Dialog: Send + Sync {
    #[must_use]
    fn states(&self) -> &[String];

    #[inline]
    #[must_use]
    fn launch_mode(&self) -> LaunchMode {
        LaunchMode::default()
    }

    #[must_use]
    fn contains_state(&self, state: &str) -> bool {
        self.states().iter().any(|candidate| candidate == state)
    }

    #[must_use]
    fn next_state(&self, current: &str) -> Option<&str> {
        let states = self.states();
        let index = states.iter().position(|state| state == current)?;
        states.get(index + 1).map(String::as_str)
    }

    #[must_use]
    fn prev_state(&self, current: &str) -> Option<&str> {
        let states = self.states();
        let index = states.iter().position(|state| state == current)?;
        index
            .checked_sub(1)
            .and_then(|index| states.get(index).map(String::as_str))
    }

    fn render(&self, state: &str, render_ctx: &RenderContext<'_>) -> Option<NewMessage>;

    #[must_use]
    fn handle_callback(&self, state: &str, click: &ClickContext<'_>) -> Option<ButtonAction>;

    #[must_use]
    fn handle_message(&self, state: &str, ctx: &Context, message: Message) -> Option<ButtonAction>;

    #[must_use]
    fn process_result(
        &self,
        _state: &str,
        _ctx: &Context,
        _start_data: &Data,
        _result: &Data,
    ) -> Option<ButtonAction> {
        None
    }
}

pub trait IntoDialog {
    fn into_dialog(self) -> Arc<dyn Dialog>;
}

impl<D> IntoDialog for D
where
    D: Dialog + 'static,
{
    fn into_dialog(self) -> Arc<dyn Dialog> {
        Arc::new(self)
    }
}

impl<D> IntoDialog for Arc<D>
where
    D: Dialog + 'static,
{
    fn into_dialog(self) -> Arc<dyn Dialog> {
        self
    }
}

impl IntoDialog for Arc<dyn Dialog> {
    fn into_dialog(self) -> Arc<dyn Dialog> {
        self
    }
}

#[must_use]
pub fn dialog<W>(windows: impl IntoIterator<Item = W>) -> DialogImpl
where
    W: IntoWindow,
{
    DialogImpl::new(windows)
}

pub struct DialogImpl {
    states: Vec<String>,
    windows: BTreeMap<String, Arc<dyn Window>>,
    launch_mode: LaunchMode,
    on_process_result: Option<Arc<ProcessResultHandler>>,
}

impl DialogImpl {
    #[must_use]
    pub fn new<W>(windows: impl IntoIterator<Item = W>) -> Self
    where
        W: IntoWindow,
    {
        let windows = windows.into_iter().map(W::into_window);
        let mut states = Vec::new();
        let mut map = BTreeMap::new();
        for window in windows {
            let state = window.get_state().to_owned();
            if map.contains_key(&state) {
                warn!(state = %state, "Skipping duplicate window state in dialog");
                continue;
            }
            states.push(state.clone());
            map.insert(state, window);
        }
        Self {
            states,
            windows: map,
            launch_mode: LaunchMode::default(),
            on_process_result: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn with_launch_mode(mut self, mode: LaunchMode) -> Self {
        self.launch_mode = mode;
        self
    }

    #[must_use]
    pub fn on_process_result<F>(mut self, handler: F) -> Self
    where
        F: Fn(&Context, &Data, &Data) -> Option<ButtonAction> + Send + Sync + 'static,
    {
        self.on_process_result = Some(Arc::new(handler));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_window(&self, state: impl AsRef<str>) -> Option<&Arc<dyn Window>> {
        self.windows.get(state.as_ref())
    }
}

impl Dialog for DialogImpl {
    #[inline]
    fn states(&self) -> &[String] {
        &self.states
    }

    #[inline]
    fn launch_mode(&self) -> LaunchMode {
        self.launch_mode
    }

    fn render(&self, state: &str, render_ctx: &RenderContext<'_>) -> Option<NewMessage> {
        self.get_window(state)
            .map(|window| window.render(render_ctx))
    }

    fn handle_callback(&self, state: &str, click: &ClickContext<'_>) -> Option<ButtonAction> {
        self.get_window(state)
            .and_then(|window| window.handle_callback(click))
    }

    fn handle_message(&self, state: &str, ctx: &Context, message: Message) -> Option<ButtonAction> {
        self.get_window(state)
            .and_then(|window| window.handle_message(ctx, message))
    }

    fn process_result(
        &self,
        _state: &str,
        ctx: &Context,
        start_data: &Data,
        result: &Data,
    ) -> Option<ButtonAction> {
        self.on_process_result
            .as_ref()
            .and_then(|handler| handler(ctx, start_data, result))
    }
}

#[cfg(test)]
mod tests {
    use super::{dialog, Dialog};
    use crate::{widgets::text, window};

    #[test]
    fn dialog_knows_next_and_previous_states() {
        let dialog = dialog([
            window("first", [text("one")]),
            window("second", [text("two")]),
            window("third", [text("three")]),
        ]);

        assert_eq!(dialog.next_state("first"), Some("second"));
        assert_eq!(dialog.prev_state("third"), Some("second"));
        assert_eq!(dialog.next_state("third"), None);
        assert_eq!(dialog.prev_state("first"), None);
    }
}
