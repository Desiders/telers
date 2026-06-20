use crate::{
    entities::{Context, LaunchMode, NewMessage, RenderContext, ResultContext},
    future::BoxFuture,
    widgets::{ButtonAction, ClickContext},
    IntoWindow, Window,
};
use async_fn_traits::AsyncFn1;
use async_trait::async_trait;
use std::{collections::BTreeMap, sync::Arc};
use telers::types::Message;
use tracing::warn;

type ProcessResultHandler =
    dyn Fn(ResultContext) -> BoxFuture<'static, Option<ButtonAction>> + Send + Sync + 'static;

#[async_trait]
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

    async fn render(&self, state: &str, render_ctx: &RenderContext) -> Option<NewMessage>;

    #[must_use]
    async fn handle_callback(&self, state: &str, click: &ClickContext) -> Option<ButtonAction>;

    #[must_use]
    async fn handle_message(
        &self,
        state: &str,
        ctx: &Context,
        message: Message,
    ) -> Option<ButtonAction>;

    /// Process a result from a completed child dialog.
    ///
    /// Called when a child dialog completes with `done_with_result()`.
    /// The handler receives a [`ResultContext`] with access to the parent context,
    /// child start data, and result data.
    ///
    /// Returns an optional action to execute in the parent context.
    #[must_use]
    async fn process_result(&self, _state: &str, _ctx: ResultContext) -> Option<ButtonAction> {
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

    /// Set an async handler for processing child dialog results.
    ///
    /// The handler receives a [`ResultContext`] with access to the parent context,
    /// child start data, and result data.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use telers_dialog::{dialog, window, widgets::{text, ButtonAction}, ResultContext};
    ///
    /// async fn handle_result(ctx: ResultContext) -> Option<ButtonAction> {
    ///     let username = ctx.result_value("username")?;
    ///     Some(ButtonAction::set_dialog_value("selected_user", username.clone()))
    /// }
    ///
    /// let d = dialog([window("parent", [text("Parent")])])
    ///     .on_process_result(handle_result);
    /// ```
    #[must_use]
    pub fn on_process_result<F>(mut self, handler: F) -> Self
    where
        F: AsyncFn(ResultContext) -> Option<ButtonAction>
            + AsyncFn1<ResultContext, Output = Option<ButtonAction>>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<ResultContext>>::OutputFuture: Send + 'static,
    {
        let handler = Arc::new(handler);
        self.on_process_result = Some(Arc::new(move |ctx| {
            let handler = handler.clone();
            Box::pin(async move { handler(ctx).await })
        }));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_window(&self, state: impl AsRef<str>) -> Option<&Arc<dyn Window>> {
        self.windows.get(state.as_ref())
    }
}

#[async_trait]
impl Dialog for DialogImpl {
    #[inline]
    fn states(&self) -> &[String] {
        &self.states
    }

    #[inline]
    fn launch_mode(&self) -> LaunchMode {
        self.launch_mode
    }

    async fn render(&self, state: &str, render_ctx: &RenderContext) -> Option<NewMessage> {
        match self.get_window(state) {
            Some(window) => Some(window.render(render_ctx).await),
            None => None,
        }
    }

    async fn handle_callback(&self, state: &str, click: &ClickContext) -> Option<ButtonAction> {
        match self.get_window(state) {
            Some(window) => window.handle_callback(click).await,
            None => None,
        }
    }

    async fn handle_message(
        &self,
        state: &str,
        ctx: &Context,
        message: Message,
    ) -> Option<ButtonAction> {
        match self.get_window(state) {
            Some(window) => window.handle_message(ctx, message).await,
            None => None,
        }
    }

    async fn process_result(&self, state: &str, ctx: ResultContext) -> Option<ButtonAction> {
        // First try dialog-level handler
        if let Some(handler) = &self.on_process_result {
            if let Some(action) = handler(ctx.clone()).await {
                return Some(action);
            }
        }
        // Then try window-level handler
        if let Some(window) = self.get_window(state) {
            return window.process_result(ctx).await;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{dialog, Dialog};
    use crate::{entities::LaunchMode, widgets::text, window};

    #[tokio::test]
    async fn dialog_knows_next_and_previous_states() {
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

    #[test]
    fn dialog_lists_states_in_order() {
        let dialog = dialog([window("a", [text("x")]), window("b", [text("y")])]);

        let states: Vec<String> = dialog.states().to_vec();
        assert_eq!(states, vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn dialog_contains_known_states_only() {
        let dialog = dialog([window("a", [text("x")]), window("b", [text("y")])]);

        assert!(dialog.contains_state("a"));
        assert!(dialog.contains_state("b"));
        assert!(!dialog.contains_state("z"));
    }

    #[test]
    fn dialog_skips_duplicate_window_states() {
        let dialog = dialog([
            window("a", [text("x")]),
            window("b", [text("y")]),
            window("a", [text("z")]),
        ]);

        let states: Vec<String> = dialog.states().to_vec();
        assert_eq!(states, vec!["a".to_owned(), "b".to_owned()]);
    }

    #[test]
    fn dialog_default_launch_mode_is_standard() {
        let dialog = dialog([window("a", [text("x")])]);

        assert_eq!(dialog.launch_mode(), LaunchMode::Standard);
        assert_eq!(dialog.launch_mode(), LaunchMode::default());
    }

    #[test]
    fn dialog_with_root_launch_mode() {
        let dialog = dialog([window("a", [text("x")])]).with_launch_mode(LaunchMode::Root);

        assert_eq!(dialog.launch_mode(), LaunchMode::Root);
    }

    #[test]
    fn dialog_with_exclusive_launch_mode() {
        let dialog = dialog([window("a", [text("x")])]).with_launch_mode(LaunchMode::Exclusive);

        assert_eq!(dialog.launch_mode(), LaunchMode::Exclusive);
    }

    #[test]
    fn dialog_with_single_top_launch_mode() {
        let dialog = dialog([window("a", [text("x")])]).with_launch_mode(LaunchMode::SingleTop);

        assert_eq!(dialog.launch_mode(), LaunchMode::SingleTop);
    }

    #[test]
    fn dialog_get_window_returns_some_for_known_state() {
        let dialog = dialog([window("a", [text("x")]), window("b", [text("y")])]);

        assert!(dialog.get_window("a").is_some());
        assert!(dialog.get_window("b").is_some());
        assert!(dialog.get_window("z").is_none());
    }
}
