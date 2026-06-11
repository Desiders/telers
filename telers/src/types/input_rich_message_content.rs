use serde::{Deserialize, Serialize};
/// Represents the content of a rich message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessagecontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichMessageContent {
    /// The message to be sent
    pub rich_message: crate::types::InputRichMessage,
}
impl InputRichMessageContent {
    /// Creates a new `InputRichMessageContent`.
    ///
    /// # Arguments
    /// * `rich_message` - The message to be sent
    #[must_use]
    pub fn new<T0: Into<crate::types::InputRichMessage>>(rich_message: T0) -> Self {
        Self {
            rich_message: rich_message.into(),
        }
    }

    /// The message to be sent
    #[must_use]
    pub fn rich_message<T: Into<crate::types::InputRichMessage>>(mut self, val: T) -> Self {
        self.rich_message = val.into();
        self
    }
}
