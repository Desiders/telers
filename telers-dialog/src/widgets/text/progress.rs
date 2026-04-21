use std::borrow::Cow;

use super::Text;
use crate::entities::{Data, DataMap};
use async_trait::async_trait;

/// Render a textual progress bar from a percentage field in `DataMap`.
pub struct Progress {
    field: Cow<'static, str>,
    width: usize,
    filled: Cow<'static, str>,
    empty: Cow<'static, str>,
}

impl Progress {
    /// Create a new progress widget reading percent from `field`.
    #[must_use]
    pub fn new(field: impl Into<Cow<'static, str>>) -> Self {
        Self {
            field: field.into(),
            width: 10,
            filled: "#".into(),
            empty: "-".into(),
        }
    }

    /// Set the progress bar width in cells.
    #[must_use]
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set the filled-cell glyph.
    #[must_use]
    pub fn filled(mut self, filled: impl Into<Cow<'static, str>>) -> Self {
        self.filled = filled.into();
        self
    }

    /// Set the empty-cell glyph.
    #[must_use]
    pub fn empty(mut self, empty: impl Into<Cow<'static, str>>) -> Self {
        self.empty = empty.into();
        self
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
