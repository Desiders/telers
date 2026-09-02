use crate::client::Bot;
use serde::Serialize;
/// Use this method to stream a partial rich message to a user while the message is being generated. Note that the streamed draft is ephemeral and acts as a temporary 30-second preview - once the output is finalized, you must call [`crate::methods::SendRichMessage`] with the complete message to persist it in the user's chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendrichmessagedraft>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SendRichMessageDraft {
    /// Unique identifier for the target private chat
    pub chat_id: i64,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Unique identifier of the message draft; must be non-zero. Changes to drafts with the same identifier are animated. Otherwise, the draft is replaced without animation.
    pub draft_id: i64,
    /// The partial message to be streamed. Direct upload of new files and explicit upload of files by a URL isn't supported.
    pub rich_message: crate::types::InputRichMessage,
    /// Pass `true` to show the user a button to stop further drafts. The bot will receive an Update `stopped_message_generation` if the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_stop: Option<bool>,
    /// Pass `true` to keep the draft in the chat when the button is pressed. The draft will still disappear after a short time or if the bot sends a message. To fully preserve the partial draft, the bot should send it as a new message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_on_stop: Option<bool>,
}
impl SendRichMessageDraft {
    /// Creates a new `SendRichMessageDraft`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target private chat
    /// * `draft_id` - Unique identifier of the message draft; must be non-zero. Changes to drafts with the same identifier are animated. Otherwise, the draft is replaced without animation.
    /// * `rich_message` - The partial message to be streamed. Direct upload of new files and explicit upload of files by a URL isn't supported.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<crate::types::InputRichMessage>>(
        chat_id: T0,
        draft_id: T1,
        rich_message: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            draft_id: draft_id.into(),
            rich_message: rich_message.into(),
            can_stop: None,
            keep_on_stop: None,
        }
    }

    /// Unique identifier for the target private chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message thread
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier for the target message thread
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Unique identifier of the message draft; must be non-zero. Changes to drafts with the same identifier are animated. Otherwise, the draft is replaced without animation.
    #[must_use]
    pub fn draft_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.draft_id = val.into();
        self
    }

    /// The partial message to be streamed. Direct upload of new files and explicit upload of files by a URL isn't supported.
    #[must_use]
    pub fn rich_message<T: Into<crate::types::InputRichMessage>>(mut self, val: T) -> Self {
        self.rich_message = val.into();
        self
    }

    /// Pass `true` to show the user a button to stop further drafts. The bot will receive an Update `stopped_message_generation` if the user presses the button.
    #[must_use]
    pub fn can_stop<T: Into<bool>>(mut self, val: T) -> Self {
        self.can_stop = Some(val.into());
        self
    }

    /// Pass `true` to show the user a button to stop further drafts. The bot will receive an Update `stopped_message_generation` if the user presses the button.
    #[must_use]
    pub fn can_stop_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.can_stop = val.map(Into::into);
        self
    }

    /// Pass `true` to keep the draft in the chat when the button is pressed. The draft will still disappear after a short time or if the bot sends a message. To fully preserve the partial draft, the bot should send it as a new message.
    #[must_use]
    pub fn keep_on_stop<T: Into<bool>>(mut self, val: T) -> Self {
        self.keep_on_stop = Some(val.into());
        self
    }

    /// Pass `true` to keep the draft in the chat when the button is pressed. The draft will still disappear after a short time or if the bot sends a message. To fully preserve the partial draft, the bot should send it as a new message.
    #[must_use]
    pub fn keep_on_stop_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.keep_on_stop = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SendRichMessageDraft {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_rich_message(&mut files, &mut self.rich_message);
        super::Request::new("sendRichMessageDraft", self, Some(files))
    }
}
