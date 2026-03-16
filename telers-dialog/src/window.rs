use crate::{
    entities::{Context, DataMap, EventContext, NewMessage, ShowMode},
    widgets::{ensure_widgets, ButtonAction, Keyboard, Text, WidgetKind},
};
use std::sync::Arc;
use telers::types::LinkPreviewOptions;

pub trait Window: Send + Sync {
    fn get_state(&self) -> &str;
    fn render(&self, ctx: &Context, data: &DataMap, event_ctx: &EventContext) -> NewMessage;
    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction>;
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

/// Minimal window with widget kinds and standard message fields.
pub struct WindowImpl {
    state: String,
    text: Box<dyn Text>,
    keyboard: Option<Box<dyn Keyboard>>,
    parse_mode: Option<Box<str>>,
    protect_content: Option<bool>,
    show_mode: ShowMode,
    link_preview_options: Option<LinkPreviewOptions>,
}

impl WindowImpl {
    /// Create a window with widgets and state id.
    #[must_use]
    pub fn new(state: impl Into<String>, widgets: impl IntoIterator<Item = WidgetKind>) -> Self {
        let (text, keyboard) = ensure_widgets(widgets);
        let state = state.into();
        Self {
            state,
            text,
            keyboard,
            parse_mode: None,
            protect_content: None,
            show_mode: ShowMode::Auto,
            link_preview_options: None,
        }
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
}

impl Window for WindowImpl {
    fn get_state(&self) -> &str {
        &self.state
    }

    fn render(&self, ctx: &Context, data: &DataMap, event_ctx: &EventContext) -> NewMessage {
        NewMessage::new(
            event_ctx.chat.clone(),
            event_ctx.thread_id,
            event_ctx.business_connection_id.clone(),
            self.text.render_text(data),
            self.keyboard
                .as_ref()
                .and_then(|kbd| kbd.render_keyboard(ctx, data)),
            self.parse_mode.clone(),
            self.protect_content,
            self.show_mode,
            self.link_preview_options.clone(),
        )
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        self.keyboard
            .as_ref()
            .and_then(|kbd| kbd.handle_callback(ctx, callback_data))
    }
}
