use crate::{
    entities::{Context, Data, DataGetter, LaunchMode, NewMessage, RenderContext, ResultContext},
    errors::DialogError,
    future::BoxFuture,
    manager::DialogControl,
    widgets::{ButtonAction, ClickContext},
    IntoWindow, Window,
};
use async_fn_traits::{AsyncFn1, AsyncFn2, AsyncFn3};
use async_trait::async_trait;
use std::{collections::BTreeMap, sync::Arc};
use telers::types::Message;
use tracing::warn;

type ProcessResultHandler =
    dyn Fn(ResultContext) -> BoxFuture<'static, Option<ButtonAction>> + Send + Sync + 'static;

type StartHandler = dyn for<'a> Fn(
        Context,
        &'a dyn DialogControl,
    ) -> BoxFuture<'a, Result<Option<ButtonAction>, DialogError>>
    + Send
    + Sync;
type CloseHandler = dyn for<'a> Fn(
        Context,
        Option<Data>,
        &'a dyn DialogControl,
    ) -> BoxFuture<'a, Result<(), DialogError>>
    + Send
    + Sync;
type ManagedResultHandler = dyn for<'a> Fn(
        ResultContext,
        &'a dyn DialogControl,
    ) -> BoxFuture<'a, Result<Option<ButtonAction>, DialogError>>
    + Send
    + Sync;

fn run_hook<'a, F, C>(
    handler: &F,
    ctx: C,
    manager: &'a (dyn DialogControl + 'a),
) -> BoxFuture<'a, Result<Option<ButtonAction>, DialogError>>
where
    F: for<'b> AsyncFn2<
        C,
        &'b (dyn DialogControl + 'b),
        Output = Result<Option<ButtonAction>, DialogError>,
    >,
    for<'b> <F as AsyncFn2<C, &'b (dyn DialogControl + 'b)>>::OutputFuture: Send + 'b,
{
    let future: <F as AsyncFn2<C, &'a (dyn DialogControl + 'a)>>::OutputFuture = handler(ctx, manager);
    Box::pin(future)
}

fn run_close_hook<'a, F>(
    handler: &F,
    ctx: Context,
    result: Option<Data>,
    manager: &'a (dyn DialogControl + 'a),
) -> BoxFuture<'a, Result<(), DialogError>>
where
    F: for<'b> AsyncFn3<
        Context,
        Option<Data>,
        &'b (dyn DialogControl + 'b),
        Output = Result<(), DialogError>,
    >,
    for<'b> <F as AsyncFn3<Context, Option<Data>, &'b (dyn DialogControl + 'b)>>::OutputFuture: Send + 'b,
{
    let future: <F as AsyncFn3<Context, Option<Data>, &'a (dyn DialogControl + 'a)>>::OutputFuture =
        handler(ctx, result, manager);
    Box::pin(future)
}

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

    /// # Errors
    /// Returns data-provider errors.
    async fn load_data(
        &self,
        _state: &str,
        render_ctx: &RenderContext,
    ) -> Result<RenderContext, DialogError> {
        Ok(render_ctx.clone())
    }

    async fn render(&self, state: &str, render_ctx: &RenderContext) -> Option<NewMessage>;

    async fn handle_callback(&self, state: &str, click: &ClickContext) -> Option<ButtonAction>;

    async fn handle_message(
        &self,
        state: &str,
        ctx: &Context,
        message: Message,
    ) -> Option<ButtonAction>;

    /// Process a result from a completed child dialog.
    ///
    /// Called when a child dialog completes. Plain `done()` supplies null result data.
    /// The handler receives a [`ResultContext`] with access to the parent context,
    /// child start data, and result data.
    ///
    /// Returns an optional action to execute in the parent context.
    async fn process_result(&self, _state: &str, _ctx: ResultContext) -> Option<ButtonAction> {
        None
    }

    /// # Errors
    /// Returns errors from the start hook.
    async fn process_start(
        &self,
        _ctx: Context,
        _manager: &dyn DialogControl,
    ) -> Result<Option<ButtonAction>, DialogError> {
        Ok(None)
    }

    /// # Errors
    /// Returns errors from the close hook.
    async fn process_close(
        &self,
        _ctx: Context,
        _result: Option<Data>,
        _manager: &dyn DialogControl,
    ) -> Result<(), DialogError> {
        Ok(())
    }

    /// # Errors
    /// Returns errors from result processing.
    async fn process_result_with_manager(
        &self,
        state: &str,
        ctx: ResultContext,
        _manager: &dyn DialogControl,
    ) -> Result<Option<ButtonAction>, DialogError> {
        Ok(self.process_result(state, ctx).await)
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
    on_start: Option<Arc<StartHandler>>,
    on_close: Option<Arc<CloseHandler>>,
    on_managed_result: Option<Arc<ManagedResultHandler>>,
    getters: Vec<DataGetter>,
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
            on_start: None,
            on_close: None,
            on_managed_result: None,
            getters: Vec::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn with_launch_mode(mut self, mode: LaunchMode) -> Self {
        self.launch_mode = mode;
        self
    }

    /// Add a provider evaluated before window getters, in registration order.
    #[must_use]
    pub fn getter(mut self, getter: DataGetter) -> Self {
        self.getters.push(getter);
        self
    }

    /// Run before the first render, after the new context has been persisted.
    #[must_use]
    pub fn on_start<F>(mut self, handler: F) -> Self
    where
        F: for<'a> AsyncFn2<
                Context,
                &'a dyn DialogControl,
                Output = Result<Option<ButtonAction>, DialogError>,
            > + Send
            + Sync
            + 'static,
        for<'a> <F as AsyncFn2<Context, &'a dyn DialogControl>>::OutputFuture: Send + 'a,
    {
        let handler = Arc::new(handler);
        self.on_start = Some(Arc::new(move |ctx, manager| {
            run_hook(handler.as_ref(), ctx, manager)
        }));
        self
    }

    /// Run while the closing context is still current. Errors leave it open.
    #[must_use]
    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: for<'a> AsyncFn3<
                Context,
                Option<Data>,
                &'a dyn DialogControl,
                Output = Result<(), DialogError>,
            > + Send
            + Sync
            + 'static,
        for<'a> <F as AsyncFn3<Context, Option<Data>, &'a dyn DialogControl>>::OutputFuture: Send + 'a,
    {
        let handler = Arc::new(handler);
        self.on_close = Some(Arc::new(move |ctx, result, manager| {
            run_close_hook(handler.as_ref(), ctx, result, manager)
        }));
        self
    }

    /// Process child results with access to the live parent manager.
    #[must_use]
    pub fn on_process_result_with_manager<F>(mut self, handler: F) -> Self
    where
        F: for<'a> AsyncFn2<
                ResultContext,
                &'a dyn DialogControl,
                Output = Result<Option<ButtonAction>, DialogError>,
            > + Send
            + Sync
            + 'static,
        for<'a> <F as AsyncFn2<ResultContext, &'a dyn DialogControl>>::OutputFuture: Send + 'a,
    {
        let handler = Arc::new(handler);
        self.on_managed_result = Some(Arc::new(move |ctx, manager| {
            run_hook(handler.as_ref(), ctx, manager)
        }));
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

    async fn load_data(
        &self,
        state: &str,
        render_ctx: &RenderContext,
    ) -> Result<RenderContext, DialogError> {
        let mut render_ctx = render_ctx.clone();
        for getter in &self.getters {
            render_ctx = getter.load(&render_ctx).await?;
        }
        match self.get_window(state) {
            Some(window) => window.load_data(&render_ctx).await,
            None => Err(DialogError::DialogNotFound),
        }
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

    async fn process_start(
        &self,
        ctx: Context,
        manager: &dyn DialogControl,
    ) -> Result<Option<ButtonAction>, DialogError> {
        match &self.on_start {
            Some(handler) => handler(ctx, manager).await,
            None => Ok(None),
        }
    }

    async fn process_close(
        &self,
        ctx: Context,
        result: Option<Data>,
        manager: &dyn DialogControl,
    ) -> Result<(), DialogError> {
        match &self.on_close {
            Some(handler) => handler(ctx, result, manager).await,
            None => Ok(()),
        }
    }

    async fn process_result_with_manager(
        &self,
        state: &str,
        ctx: ResultContext,
        manager: &dyn DialogControl,
    ) -> Result<Option<ButtonAction>, DialogError> {
        if let Some(handler) = &self.on_managed_result {
            let action = handler(ctx.clone(), manager).await?;
            if action.is_some() {
                return Ok(action);
            }
            let current = match manager.current_context().await {
                Ok(current) if current.id == ctx.context.id => current,
                Ok(_) | Err(DialogError::NoContext) => return Ok(None),
                Err(err) => return Err(err),
            };
            let state = current.state.clone();
            let mut ctx = ctx;
            ctx.context = Arc::new(current);
            return Ok(self.process_result(&state, ctx).await);
        }
        Ok(self.process_result(state, ctx).await)
    }
}

#[cfg(test)]
mod tests {
    use super::{dialog, Dialog};
    use crate::{
        entities::{Context, DataGetter, DataMap, LaunchMode, RenderContext},
        widgets::text,
        window,
    };
    use telers::serialization::{json, Value};

    #[tokio::test]
    async fn dialog_getters_chain_before_window_getters() {
        let dialog = dialog([window("a", [text("A")]).getter(DataGetter::new(
            |render: RenderContext| async move {
                assert_eq!(render.data.get("source"), Some(&json!("second")));
                Ok(DataMap::from([("source".into(), json!("window"))]))
            },
        ))])
        .getter(DataGetter::new(|_render: RenderContext| async move {
            Ok(DataMap::from([
                ("source".into(), json!("first")),
                ("first_only".into(), json!(true)),
            ]))
        }))
        .getter(DataGetter::new(|render: RenderContext| async move {
            assert_eq!(render.data.get("source"), Some(&json!("first")));
            Ok(DataMap::from([("source".into(), json!("second"))]))
        }));
        let ctx = Context::new("", "a", Value::default());
        let loaded = RenderContext::with_test(&ctx, &DataMap::new(), |render| {
            Box::pin(async move { dialog.load_data("a", &render).await.unwrap() })
        })
        .await;

        assert_eq!(loaded.data.get("source"), Some(&json!("window")));
        assert_eq!(loaded.data.get("first_only"), Some(&json!(true)));
        assert!(loaded.context.dialog_data.is_empty());
    }

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
