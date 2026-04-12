use crate::{
    entities::{Context, NewMessage, RenderContext, ShowMode},
    widgets::{
        ensure_widgets, ButtonAction, ClickContext, Input, Keyboard, LinkPreviewWidget, Text,
        WidgetKind,
    },
};
use std::sync::Arc;
use telers::types::{LinkPreviewOptions, Message};

pub trait Window: Send + Sync {
    fn get_state(&self) -> &str;
    fn render(&self, render_ctx: &RenderContext<'_>) -> NewMessage;
    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction>;
    #[cfg(test)]
    fn handle_callback_for_test(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        ClickContext::with_test(ctx, callback_data, |click| self.handle_callback(click))
    }
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;
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
    parse_mode: Option<Box<str>>,
    protect_content: Option<bool>,
    show_mode: ShowMode,
    link_preview_options: Option<LinkPreviewOptions>,
}

impl WindowImpl {
    /// Create a window with widgets and state id.
    #[must_use]
    pub fn new(state: impl Into<String>, widgets: impl IntoIterator<Item = WidgetKind>) -> Self {
        let (text, keyboard, input, link_preview) = ensure_widgets(widgets);
        let state = state.into();
        Self {
            state,
            text,
            keyboard,
            input,
            link_preview,
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

    fn render(&self, render_ctx: &RenderContext<'_>) -> NewMessage {
        let event_ctx = render_ctx.event_context;

        NewMessage::new(
            event_ctx.chat.clone(),
            event_ctx.thread_id,
            event_ctx.business_connection_id.clone(),
            self.text.render_text_in_context(render_ctx),
            self.keyboard
                .as_ref()
                .filter(|kbd| kbd.is_visible(render_ctx.context, render_ctx.data))
                .and_then(|kbd| kbd.render_keyboard(render_ctx)),
            self.parse_mode.clone(),
            self.protect_content,
            self.show_mode,
            self.link_preview_options.clone().or_else(|| {
                self.link_preview
                    .as_ref()
                    .and_then(|link_preview| link_preview.render_link_preview(render_ctx))
            }),
        )
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        self.keyboard
            .as_ref()
            .filter(|kbd| kbd.is_visible(click.context, &click.context.dialog_data))
            .and_then(|kbd| kbd.handle_callback(click))
    }

    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        self.input
            .as_ref()
            .and_then(|input| input.handle_message(ctx, message))
    }
}

#[cfg(test)]
mod tests {
    use super::{window, Window};
    use crate::{
        entities::{ChatEvent, Context, DataMap, EventContext, RenderContext},
        widgets::{
            input, keyboard, link_preview, text, Button, ButtonAction, InlineKeyboard, LinkPreview,
            MessageInput,
        },
    };
    use serde_json::Value;
    use telers::{
        client::Reqwest,
        types::{ChatPrivate, Message, MessageText, User},
        Bot,
    };

    fn test_message(text: &str) -> Message {
        MessageText::new(1, 1, ChatPrivate::new(10), text)
            .from(User::new(10, false, "tester"))
            .into()
    }

    #[test]
    fn window_combines_multiple_keyboard_and_input_widgets() {
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
                input(MessageInput::new(|_ctx, _message: Message| {
                    ButtonAction::noop()
                })),
                input(MessageInput::new(|_ctx, message: MessageText| {
                    ButtonAction::set_dialog_value("name", message.text.to_string())
                })),
                link_preview(
                    LinkPreview::builder()
                        .url("https://example.com/menu")
                        .build(),
                ),
            ],
        );
        let ctx = Context::new("", "state", Value::Null);
        let data = DataMap::new();
        let event = ChatEvent::Message(test_message("/start"));
        let event_ctx = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        let render_ctx = RenderContext::new(&ctx, &data, &event, &event_ctx);

        let rendered = window.render(&render_ctx);
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
            .expect("callback action");
        assert!(matches!(callback_action, ButtonAction::Back));

        let input_action = window
            .handle_message(&ctx, test_message("Alice"))
            .expect("input action");
        assert!(matches!(input_action, ButtonAction::Noop));
    }
}
