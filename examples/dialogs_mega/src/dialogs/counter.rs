//! `Counter` paired with a progress bar.
//!
//! The counter stores its value in `widget_data`, so the progress bar is a
//! small custom [`Text`] that reads the same key from the render context (the
//! built-in `Progress` widget reads from `dialog_data` instead).

use telers_dialog::{
    async_trait,
    entities::{DataMap, RenderContext},
    widgets::{keyboard, text, Counter, InlineKeyboard, Text},
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "counter_main";

const COUNTER_ID: &str = "counter_value";
const MAX_VALUE: f64 = 10.0;

/// Renders a `[####------] 40%` bar from the counter's `widget_data` value.
struct CounterProgress;

impl CounterProgress {
    fn bar(value: f64) -> Box<str> {
        let width = 10usize;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let filled = ((value / MAX_VALUE) * width as f64).round() as usize;
        let filled = filled.min(width);
        let percent = ((value / MAX_VALUE) * 100.0).round();
        format!(
            "[{}{}] {percent:.0}%",
            "#".repeat(filled),
            "-".repeat(width - filled),
        )
        .into_boxed_str()
    }
}

#[async_trait]
impl Text for CounterProgress {
    async fn render_text(&self, _data: &DataMap) -> Box<str> {
        Self::bar(0.0)
    }

    async fn render_text_in_context(&self, render_ctx: &RenderContext) -> Box<str> {
        let value = render_ctx
            .context
            .widget_value_as::<f64>(COUNTER_ID)
            .unwrap_or(0.0);
        Self::bar(value)
    }
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            text("Counter & progress\n\nUse +/- to change the value; the bar tracks it."),
            text(CounterProgress),
            keyboard(
                Counter::builder(COUNTER_ID)
                    .default(0.0)
                    .min(0.0)
                    .max(MAX_VALUE)
                    .build(),
            ),
            keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
        ],
    )])
}
