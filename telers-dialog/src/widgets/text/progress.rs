use bon::bon;
use std::borrow::Cow;

use super::Text;
use crate::entities::{Data, DataMap};
use async_trait::async_trait;

/// Render a textual progress bar from a percentage value in `DataMap`.
///
/// The widget reads `field` from the render data, clamps it to `[0, 100]`, and
/// fills `width` cells in proportion to the percentage. Both the filled and
/// empty glyphs are configurable.
///
/// # Example
///
/// ```ignore
/// use telers_dialog::widgets::Progress;
///
/// let bar = Progress::builder("download_percent")
///     .width(20)
///     .filled("█")
///     .empty("░")
///     .build();
/// ```
pub struct Progress {
    field: Cow<'static, str>,
    width: usize,
    filled: Cow<'static, str>,
    empty: Cow<'static, str>,
}

#[bon]
impl Progress {
    /// Create a progress widget that reads percent from `field`.
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn, into)] field: Cow<'static, str>,
        #[builder(default = 10)] width: usize,
        #[builder(default = Cow::Borrowed("#"), into)] filled: Cow<'static, str>,
        #[builder(default = Cow::Borrowed("-"), into)] empty: Cow<'static, str>,
    ) -> Self {
        Self {
            field,
            width,
            filled,
            empty,
        }
    }
}

#[async_trait]
impl Text for Progress {
    async fn render_text(&self, data: &DataMap) -> Box<str> {
        let percent = data
            .get(self.field.as_ref())
            .and_then(|value| match value {
                Data::Number(value) => value.as_f64(),
                Data::String(value) => value.parse::<f64>().ok(),
                _ => None,
            })
            .unwrap_or_default()
            .clamp(0.0, 100.0);
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_precision_loss,
            clippy::cast_sign_loss
        )]
        let done = ((self.width as f64 * percent) / 100.0).round() as usize;
        let rest = self.width.saturating_sub(done);

        format!(
            "{}{} {:>3.0}%",
            self.filled.repeat(done),
            self.empty.repeat(rest),
            percent
        )
        .into_boxed_str()
    }
}
