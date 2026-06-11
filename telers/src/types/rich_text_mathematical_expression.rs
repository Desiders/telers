use serde::{Deserialize, Serialize};
/// A mathematical expression.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextmathematicalexpression>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextMathematicalExpression {
    /// The expression in `LaTeX` format
    pub expression: Box<str>,
}
impl RichTextMathematicalExpression {
    /// Creates a new `RichTextMathematicalExpression`.
    ///
    /// # Arguments
    /// * `expression` - The expression in `LaTeX` format
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(expression: T0) -> Self {
        Self {
            expression: expression.into(),
        }
    }

    /// The expression in `LaTeX` format
    #[must_use]
    pub fn expression<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.expression = val.into();
        self
    }
}
