#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{
    entities::{Context, DataGetter, NewMessage, RenderContext, ResultContext, ShowMode},
    future::BoxFuture,
    widgets::{
        ensure_widgets, ButtonAction, ClickContext, Input, Keyboard, LinkPreviewWidget, Media,
        Text, WidgetKind,
    },
};
use async_fn_traits::AsyncFn1;
use async_trait::async_trait;
use std::sync::Arc;
use telers::types::{LinkPreviewOptions, Message};

type WindowResultHandler =
    dyn Fn(ResultContext) -> BoxFuture<'static, Option<ButtonAction>> + Send + Sync + 'static;

#[async_trait]
pub trait Window: Send + Sync {
    fn get_state(&self) -> &str;
    async fn load_data(&self, render_ctx: &RenderContext) -> Result<RenderContext, crate::DialogError> {
        Ok(render_ctx.clone())
    }
    async fn render(&self, render_ctx: &RenderContext) -> NewMessage;
    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction>;
    #[cfg(test)]
    async fn handle_callback_for_test(
        &self,
        ctx: &Context,
        callback_data: &str,
    ) -> Option<ButtonAction> {
        use telers::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, User},
            Bot,
        };

        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(10), "/test")
                .from(User::new(10, false, "tester"))
                .into(),
        );
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let runtime_context = telers::Context::default();
        let click = ClickContext::new(ctx, callback_data, &event, &event_context, &runtime_context);
        self.handle_callback(&click).await
    }
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;

    /// Process a result from a completed child dialog.
    ///
    /// Called when a child dialog completes while this window is active.
    /// Returns an optional action to execute in response.
    async fn process_result(&self, _ctx: ResultContext) -> Option<ButtonAction> {
        None
    }
}

pub trait IntoWindow {
    fn into_window(self) -> Arc<dyn Window>;
}

impl<W> IntoWindow for W
where
    W: Window + 'static,
{
    fn into_window(self) -> Arc<dyn Window> {
        Arc::new(self)
    }
}

impl<W> IntoWindow for Arc<W>
where
    W: Window + 'static,
{
    fn into_window(self) -> Arc<dyn Window> {
        self
    }
}

impl IntoWindow for Arc<dyn Window> {
    fn into_window(self) -> Arc<dyn Window> {
        self
    }
}

#[must_use]
pub fn window(
    state: impl Into<String>,
    widgets: impl IntoIterator<Item = WidgetKind>,
) -> WindowImpl {
    WindowImpl::new(state, widgets)
}

/// Minimal window with widget kinds and standard message fields.
pub struct WindowImpl {
    state: String,
    text: Box<dyn Text>,
    keyboard: Option<Box<dyn Keyboard>>,
    input: Option<Box<dyn Input>>,
    link_preview: Option<Box<dyn LinkPreviewWidget>>,
    media: Option<Box<dyn Media>>,
    parse_mode: Option<Box<str>>,
    protect_content: Option<bool>,
    show_mode: ShowMode,
    link_preview_options: Option<LinkPreviewOptions>,
    on_process_result: Option<Arc<WindowResultHandler>>,
    getters: Vec<DataGetter>,
}

impl WindowImpl {
    /// Create a window with widgets and state id.
    #[must_use]
    pub fn new(state: impl Into<String>, widgets: impl IntoIterator<Item = WidgetKind>) -> Self {
        let (text, keyboard, input, link_preview, media) = ensure_widgets(widgets);
        let state = state.into();
        Self {
            state,
            text,
            keyboard,
            input,
            link_preview,
            media,
            parse_mode: None,
            protect_content: None,
            show_mode: ShowMode::Auto,
            link_preview_options: None,
            on_process_result: None,
            getters: Vec::new(),
        }
    }

    /// Add a data provider, evaluated after dialog getters in registration order.
    #[must_use]
    pub fn getter(mut self, getter: DataGetter) -> Self {
        self.getters.push(getter);
        self
    }

    /// Set parse mode.
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: impl Into<Box<str>>) -> Self {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    /// Set protect content flag.
    #[must_use]
    pub fn protect_content(mut self, protect: bool) -> Self {
        self.protect_content = Some(protect);
        self
    }

    /// Set show mode for this window.
    #[must_use]
    pub fn show_mode(mut self, show_mode: ShowMode) -> Self {
        self.show_mode = show_mode;
        self
    }

    /// Set link preview options.
    #[must_use]
    pub fn link_preview_options(mut self, opts: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(opts);
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
    /// use telers_dialog::{window, widgets::{text, ButtonAction}, ResultContext};
    ///
    /// async fn handle_result(ctx: ResultContext) -> Option<ButtonAction> {
    ///     let selection = ctx.result_value("item")?;
    ///     Some(ButtonAction::set_dialog_value("chosen", selection.clone()))
    /// }
    ///
    /// let w = window("select_parent", [text("Select an item")])
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
}

#[async_trait]
impl Window for WindowImpl {
    fn get_state(&self) -> &str {
        &self.state
    }

    async fn load_data(&self, render_ctx: &RenderContext) -> Result<RenderContext, crate::DialogError> {
        let mut context = render_ctx.clone();
        for getter in &self.getters {
            context = getter.load(&context).await?;
        }
        Ok(context)
    }

    async fn render(&self, render_ctx: &RenderContext) -> NewMessage {
        let event_ctx = render_ctx.event_context.as_ref();
        let reply_markup = match &self.keyboard {
            Some(kbd) => kbd.render_keyboard(render_ctx).await,
            None => None,
        };
        let link_preview_options = if let Some(options) = &self.link_preview_options {
            Some(options.clone())
        } else if let Some(link_preview) = &self.link_preview {
            link_preview.render_link_preview(render_ctx).await
        } else {
            None
        };
        let media = if let Some(media_widget) = &self.media {
            media_widget.render_media(render_ctx).await
        } else {
            None
        };

        NewMessage::new(
            event_ctx.chat.clone(),
            event_ctx.thread_id,
            event_ctx.business_connection_id.clone(),
            self.text.render_text_in_context(render_ctx).await,
            reply_markup,
            self.parse_mode.clone(),
            self.protect_content,
            self.show_mode,
            link_preview_options,
        )
        .with_media(media)
    }

    async fn handle_callback(&self, click: &ClickContext) -> Option<ButtonAction> {
        match &self.keyboard {
            Some(kbd) => kbd.handle_callback(click).await,
            None => None,
        }
    }

    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        match &self.input {
            Some(input) => input.handle_message(ctx, message).await,
            None => None,
        }
    }

    async fn process_result(&self, ctx: ResultContext) -> Option<ButtonAction> {
        match &self.on_process_result {
            Some(handler) => handler(ctx).await,
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{window, Window};
    use crate::{
        entities::{
            ChatEvent, Context, DataMap, EventContext, RenderContext, ResultContext, ShowMode,
        },
        widgets::{
            input, keyboard, link_preview, text, Button, ButtonAction, InlineKeyboard, LinkPreview,
            MessageInput, MessageInputContext,
        },
    };
    use telers::{
        client::Reqwest,
        serialization::Value,
        types::{ChatPrivate, Message, MessageText, User},
        Bot,
    };

    fn test_message(text: &str) -> Message {
        MessageText::new(1, 1, ChatPrivate::new(10), text)
            .from(User::new(10, false, "tester"))
            .into()
    }

    async fn noop_message(_ctx: MessageInputContext, _message: Message) -> ButtonAction {
        ButtonAction::noop()
    }

    async fn store_name(_ctx: MessageInputContext, message: MessageText) -> ButtonAction {
        ButtonAction::set_dialog_value("name", message.text.to_string())
    }

    #[tokio::test]
    async fn getters_merge_in_order_without_persisting_derived_data() {
        use crate::entities::{base_data, DataGetter};
        use telers::serialization::json;
        let mut context = Context::new("", "state", json!({"seed": 5}));
        context.dialog_data.insert("name".into(), json!("original"));
        context.widget_data.insert("choice".into(), json!("pear"));
        let event = ChatEvent::Message(test_message("input"));
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let mut runtime = telers::Context::default();
        runtime.insert("prefix", String::from("Hello"));
        let render = RenderContext::new(&context, &base_data(&context), &event, &event_context)
            .with_runtime_context(&runtime);
        let window = window("state", [text("{name}")])
            .getter(DataGetter::new(|render: RenderContext| async move {
                assert_eq!(render.data["start_data"]["seed"], 5);
                assert_eq!(render.data["widget_data"]["choice"], "pear");
                Ok(DataMap::from([(
                    "name".into(),
                    json!(render.runtime_data::<String>("prefix").unwrap()),
                )]))
            }))
            .getter(DataGetter::new(|render: RenderContext| async move {
                Ok(DataMap::from([(
                    "name".into(),
                    json!(format!("{} world", render.data["name"].as_str().unwrap())),
                )]))
            }));
        let loaded = window.load_data(&render).await.expect("getters");
        assert_eq!(loaded.data["name"], "Hello world");
        assert_eq!(context.dialog_data["name"], "original");
        assert_eq!(loaded.context.dialog_data["name"], "original");
    }

    #[tokio::test]
    async fn getter_failure_stops_the_pipeline() {
        use crate::entities::{DataGetter, base_data};
        let context = Context::new("", "state", Value::default());
        let event = ChatEvent::Message(test_message("input"));
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render = RenderContext::new(&context, &base_data(&context), &event, &event_context);
        let window = window("state", [text("Text")])
            .getter(DataGetter::new(|_: RenderContext| async {
                Err(crate::DialogError::NoContext)
            }))
            .getter(DataGetter::new(|_: RenderContext| async {
                panic!("must not run after failure")
            }));
        assert!(matches!(
            window.load_data(&render).await,
            Err(crate::DialogError::NoContext)
        ));
    }

    #[tokio::test]
    async fn window_getter_failure_keeps_inherited_dialog_getter_data_ephemeral() {
        use crate::{
            dialog,
            entities::{base_data, DataGetter},
            Dialog,
        };
        use telers::serialization::json;

        let mut context = Context::new("", "state", json!({"seed": 7}));
        context.dialog_data.insert("value".into(), json!("persisted"));
        let event = ChatEvent::Message(test_message("input"));
        let event_context = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render = RenderContext::new(&context, &base_data(&context), &event, &event_context);
        let dialog = dialog([window("state", [text("Text")])
            .getter(DataGetter::new(|render: RenderContext| async move {
                assert_eq!(render.data["value"].as_str(), Some("derived"));
                assert_eq!(render.data["start_data"]["seed"], 7);
                assert_eq!(
                    render.context.dialog_data["value"].as_str(),
                    Some("persisted")
                );
                Err(crate::DialogError::NoContext)
            }))
            .getter(DataGetter::new(|_: RenderContext| async {
                panic!("must not run after window getter fails")
            }))])
            .getter(DataGetter::new(|_: RenderContext| async {
                Ok(DataMap::from([("value".into(), json!("derived"))]))
            }));
        assert!(matches!(
            dialog.load_data("state", &render).await,
            Err(crate::DialogError::NoContext)
        ));
        assert_eq!(render.data["value"].as_str(), Some("persisted"));
        assert_eq!(context.dialog_data["value"].as_str(), Some("persisted"));
    }

    #[tokio::test]
    async fn window_combines_multiple_keyboard_and_input_widgets() {
        let window = window(
            "state",
            [
                text("Prompt"),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::action("first", "First", ButtonAction::next()))
                        .build(),
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .push(Button::action("second", "Second", ButtonAction::back()))
                        .build(),
                ),
                input(MessageInput::new(noop_message)),
                input(MessageInput::new(store_name)),
                link_preview(
                    LinkPreview::builder()
                        .url("https://example.com/menu")
                        .build(),
                ),
            ],
        );
        let ctx = Context::new("", "state", Value::default());
        let data = DataMap::new();
        let event = ChatEvent::Message(test_message("/start"));
        let event_ctx = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(&ctx, &data, &event, &event_ctx);

        let rendered = window.render(&render_ctx).await;
        let rows = rendered
            .reply_markup
            .as_ref()
            .and_then(telers::types::ReplyMarkup::inline_keyboard)
            .expect("inline keyboard");
        let first_callback = format!("td:{}:first", ctx.id);
        let second_callback = format!("td:{}:second", ctx.id);

        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0][0].callback_data.as_deref(),
            Some(first_callback.as_str())
        );
        assert_eq!(
            rows[1][0].callback_data.as_deref(),
            Some(second_callback.as_str())
        );
        assert_eq!(
            rendered
                .link_preview_options
                .as_ref()
                .and_then(|opts| opts.url.as_deref()),
            Some("https://example.com/menu")
        );

        let callback_action = window
            .handle_callback_for_test(&ctx, &format!("td:{}:second", ctx.id))
            .await
            .expect("callback action");
        assert!(matches!(callback_action, ButtonAction::Back));

        let input_action = window
            .handle_message(&ctx, test_message("Alice"))
            .await
            .expect("input action");
        assert!(matches!(input_action, ButtonAction::Noop));
    }

    #[tokio::test]
    async fn window_with_only_text_renders_without_keyboard() {
        let window = window("state", [text("Just a prompt")]);
        let ctx = Context::new("", "state", Value::default());
        let data = DataMap::new();
        let event = ChatEvent::Message(test_message("/start"));
        let event_ctx = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(&ctx, &data, &event, &event_ctx);

        let rendered = window.render(&render_ctx).await;

        assert!(rendered.reply_markup.is_none());
        assert_eq!(rendered.text.as_ref(), "Just a prompt");
    }

    #[tokio::test]
    async fn window_render_applies_message_options() {
        let window = window("state", [text("Body")])
            .parse_mode("HTML")
            .protect_content(true)
            .show_mode(ShowMode::Send);
        let ctx = Context::new("", "state", Value::default());
        let data = DataMap::new();
        let event = ChatEvent::Message(test_message("/start"));
        let event_ctx = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(&ctx, &data, &event, &event_ctx);

        let rendered = window.render(&render_ctx).await;

        assert_eq!(rendered.parse_mode.as_deref(), Some("HTML"));
        assert_eq!(rendered.protect_content, Some(true));
        assert_eq!(rendered.show_mode, ShowMode::Send);
    }

    #[tokio::test]
    async fn handle_message_without_input_widget_returns_none() {
        let window = window("state", [text("No input here")]);
        let ctx = Context::new("", "state", Value::default());

        let action = window.handle_message(&ctx, test_message("anything")).await;

        assert!(action.is_none());
    }

    #[tokio::test]
    async fn process_result_without_handler_returns_none() {
        let window = window("state", [text("Parent")]);
        let ctx = Context::new("", "state", Value::default());
        let start_data = Value::default();
        let result = Value::default();
        let event = ChatEvent::Message(test_message("/done"));
        let event_ctx = EventContext::new(Bot::<Reqwest>::default(), event.clone());
        let runtime_context = telers::Context::default();
        let result_ctx = ResultContext::new(
            &ctx,
            &start_data,
            &result,
            &event,
            &event_ctx,
            &runtime_context,
        );

        let action = window.process_result(result_ctx).await;

        assert!(action.is_none());
    }
}
