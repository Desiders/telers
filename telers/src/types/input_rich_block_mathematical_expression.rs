use serde::{Deserialize, Serialize};
/// A block with a mathematical expression in `LaTeX` format, corresponding to the custom HTML tag `<tg-math-block>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockmathematicalexpression>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockMathematicalExpression {
    /// The mathematical expression in `LaTeX` format
    pub expression: Box<str>,
}
impl InputRichBlockMathematicalExpression {
    /// Creates a new `InputRichBlockMathematicalExpression`.
    ///
    /// # Arguments
    /// * `expression` - The mathematical expression in `LaTeX` format
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(expression: T0) -> Self {
        Self {
            expression: expression.into(),
        }
    }

    /// The mathematical expression in `LaTeX` format
    #[must_use]
    pub fn expression<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.expression = val.into();
        self
    }
}
