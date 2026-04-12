use telers::types::{InlineKeyboardMarkup, ReplyMarkup};
use tracing::warn;

use super::{ButtonAction, ClickContext};
use crate::entities::{Context, DataMap, RenderContext};

/// Keyboard widget rendered inside a dialog window.
///
/// Implementors are responsible for producing reply markup and resolving
/// callback data that belongs to the widget.
pub trait Keyboard: Send + Sync + 'static {
    /// Render reply markup for the current dialog context.
    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup>;

    #[cfg(test)]
    fn render_keyboard_for_test(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        RenderContext::with_test(ctx, data, |render_ctx| self.render_keyboard(render_ctx))
    }

    /// Return whether this keyboard should render and handle callbacks.
    #[inline]
    #[must_use]
    fn is_visible(&self, _ctx: &Context, _data: &DataMap) -> bool {
        true
    }

    /// Resolve callback data into a dialog action.
    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction>;

    #[cfg(test)]
    fn handle_callback_for_test(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        ClickContext::with_test(ctx, callback_data, |click| self.handle_callback(click))
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
    fn render_keyboard(&self, render_ctx: &RenderContext<'_>) -> Option<ReplyMarkup> {
        let mut inline_rows = Vec::new();
        let mut non_inline_markup = None;

        for keyboard in &self.keyboards {
            if !keyboard.is_visible(render_ctx.context, render_ctx.data) {
                continue;
            }
            let Some(markup) = keyboard.render_keyboard(render_ctx) else {
                continue;
            };
            match markup {
                ReplyMarkup::InlineKeyboardMarkup(markup) => {
                    if non_inline_markup.is_some() {
                        warn!("Cannot combine inline and non-inline reply markups in one window");
                        continue;
                    }
                    inline_rows.extend(markup.inline_keyboard.into_vec());
                }
                other_markup => {
                    if !inline_rows.is_empty() {
                        warn!("Cannot combine non-inline reply markup with inline keyboard rows");
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
    }

    fn handle_callback(&self, click: &ClickContext<'_>) -> Option<ButtonAction> {
        self.keyboards
            .iter()
            .filter(|keyboard| keyboard.is_visible(click.context, &click.context.dialog_data))
            .find_map(|keyboard| keyboard.handle_callback(click))
    }
}
