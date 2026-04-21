use telers::types::{InlineKeyboardMarkup, ReplyMarkup};
use tracing::warn;

use super::{ButtonAction, ClickContext};
#[cfg(test)]
use crate::entities::{ChatEvent, EventContext};
use crate::{
    entities::{Context, DataMap, RenderContext},
    future::BoxFuture,
};

/// Keyboard widget rendered inside a dialog window.
///
/// Implementors are responsible for producing reply markup and resolving
/// callback data that belongs to the widget.
pub trait Keyboard: Send + Sync + 'static {
    /// Render reply markup for the current dialog context.
    fn render_keyboard<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Option<ReplyMarkup>>;

    #[cfg(test)]
    fn render_keyboard_for_test<'a>(
        &'a self,
        ctx: &'a Context,
        data: &'a DataMap,
    ) -> BoxFuture<'a, Option<ReplyMarkup>> {
        Box::pin(async move {
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
            let event_context =
                EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
            let render_ctx = RenderContext::new(ctx, data, &event, &event_context);
            self.render_keyboard(&render_ctx).await
        })
    }

    /// Return whether this keyboard should render and handle callbacks.
    #[inline]
    #[must_use]
    fn is_visible<'a>(&'a self, _ctx: &'a Context, _data: &'a DataMap) -> BoxFuture<'a, bool> {
        Box::pin(async { true })
    }

    /// Resolve callback data into a dialog action.
    fn handle_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>>;

    #[cfg(test)]
    fn handle_callback_for_test<'a>(
        &'a self,
        ctx: &'a Context,
        callback_data: &'a str,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
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
            let event_context =
                EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
            let runtime_context = telers::Context::default();
            let click =
                ClickContext::new(ctx, callback_data, &event, &event_context, &runtime_context);
            self.handle_callback(&click).await
        })
    }
}

pub(crate) struct MultiKeyboard {
    keyboards: Vec<Box<dyn Keyboard>>,
}

impl MultiKeyboard {
    #[inline]
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            keyboards: Vec::new(),
        }
    }

    #[must_use]
    pub(crate) fn kbd_boxed(mut self, keyboard: Box<dyn Keyboard>) -> Self {
        self.keyboards.push(keyboard);
        self
    }
}

impl Keyboard for MultiKeyboard {
    fn render_keyboard<'a>(
        &'a self,
        render_ctx: &'a RenderContext,
    ) -> BoxFuture<'a, Option<ReplyMarkup>> {
        Box::pin(async move {
            let mut inline_rows = Vec::new();
            let mut non_inline_markup = None;

            for keyboard in &self.keyboards {
                if !keyboard
                    .is_visible(render_ctx.context.as_ref(), render_ctx.data.as_ref())
                    .await
                {
                    continue;
                }
                let Some(markup) = keyboard.render_keyboard(render_ctx).await else {
                    continue;
                };
                match markup {
                    ReplyMarkup::InlineKeyboardMarkup(markup) => {
                        if non_inline_markup.is_some() {
                            warn!(
                                "Cannot combine inline and non-inline reply markups in one window"
                            );
                            continue;
                        }
                        inline_rows.extend(markup.inline_keyboard.into_vec());
                    }
                    other_markup => {
                        if !inline_rows.is_empty() {
                            warn!(
                                "Cannot combine non-inline reply markup with inline keyboard rows"
                            );
                            continue;
                        }
                        if non_inline_markup.is_some() {
                            warn!("Only one non-inline reply markup can be used in a window");
                            continue;
                        }
                        non_inline_markup = Some(other_markup);
                    }
                }
            }

            if inline_rows.is_empty() {
                non_inline_markup
            } else {
                Some(ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
                    inline_keyboard: inline_rows.into_boxed_slice(),
                }))
            }
        })
    }

    fn handle_callback<'a>(
        &'a self,
        click: &'a ClickContext,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            for keyboard in &self.keyboards {
                if keyboard
                    .is_visible(click.context.as_ref(), &click.context.dialog_data)
                    .await
                {
                    if let Some(action) = keyboard.handle_callback(click).await {
                        return Some(action);
                    }
                }
            }
            None
        })
    }
}
