/// # Documentation
/// <https://core.telegram.org/bots/api#ephemeralmessageparameters>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct EphemeralMessageParameters {
    /// Identifier of the user who will receive the message; required unless the message is sent as an ephemeral reply to a callback query. If omitted, then the parameter `chat_id` of the method is ignored. If specified together with `callback_query_id`, then it must be the identifier of the user who pressed the button. Must be omitted for messages sent in chats where the bot isn't a member, unless the message is a reply to an ephemeral message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub receiver_user_id: Option<i64>,
    /// Identifier of the callback query which triggered the message, if any. If specified and `receiver_user_id` is omitted, then the message is sent as an ephemeral reply to the callback query and will be shown only to the user who pressed the button. Ephemeral replies can't be sent to callback queries from messages in channel direct messages chats. The bot must still answer the callback query using [`telers_methods::methods::AnswerCallbackQuery`](https://docs.rs/telers-methods/latest/telers_methods/methods/answer_callback_query/struct.AnswerCallbackQuery.html).
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub callback_query_id: Option<Box<str>>,
    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub replace_callback_query_message: Option<bool>,
}
impl EphemeralMessageParameters {
    /// Creates a new `EphemeralMessageParameters`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            receiver_user_id: None,
            callback_query_id: None,
            replace_callback_query_message: None,
        }
    }

    /// Identifier of the user who will receive the message; required unless the message is sent as an ephemeral reply to a callback query. If omitted, then the parameter `chat_id` of the method is ignored. If specified together with `callback_query_id`, then it must be the identifier of the user who pressed the button. Must be omitted for messages sent in chats where the bot isn't a member, unless the message is a reply to an ephemeral message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = Some(val.into());
        self
    }

    /// Identifier of the user who will receive the message; required unless the message is sent as an ephemeral reply to a callback query. If omitted, then the parameter `chat_id` of the method is ignored. If specified together with `callback_query_id`, then it must be the identifier of the user who pressed the button. Must be omitted for messages sent in chats where the bot isn't a member, unless the message is a reply to an ephemeral message. It is not guaranteed that the user will receive the message, especially if they are offline. See here for more details.
    #[must_use]
    pub fn receiver_user_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.receiver_user_id = val.map(Into::into);
        self
    }

    /// Identifier of the callback query which triggered the message, if any. If specified and `receiver_user_id` is omitted, then the message is sent as an ephemeral reply to the callback query and will be shown only to the user who pressed the button. Ephemeral replies can't be sent to callback queries from messages in channel direct messages chats. The bot must still answer the callback query using [`telers_methods::methods::AnswerCallbackQuery`](https://docs.rs/telers-methods/latest/telers_methods/methods/answer_callback_query/struct.AnswerCallbackQuery.html).
    #[must_use]
    pub fn callback_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_query_id = Some(val.into());
        self
    }

    /// Identifier of the callback query which triggered the message, if any. If specified and `receiver_user_id` is omitted, then the message is sent as an ephemeral reply to the callback query and will be shown only to the user who pressed the button. Ephemeral replies can't be sent to callback queries from messages in channel direct messages chats. The bot must still answer the callback query using [`telers_methods::methods::AnswerCallbackQuery`](https://docs.rs/telers-methods/latest/telers_methods/methods/answer_callback_query/struct.AnswerCallbackQuery.html).
    #[must_use]
    pub fn callback_query_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_query_id = val.map(Into::into);
        self
    }

    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[must_use]
    pub fn replace_callback_query_message<T: Into<bool>>(mut self, val: T) -> Self {
        self.replace_callback_query_message = Some(val.into());
        self
    }

    /// Pass `true` if the ephemeral message must be shown in place of the original message. Must be `false` for callback queries from ephemeral messages, which must be edited using regular `editEphemeralMessage`... methods.
    #[must_use]
    pub fn replace_callback_query_message_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.replace_callback_query_message = val.map(Into::into);
        self
    }
}
impl Default for EphemeralMessageParameters {
    fn default() -> Self {
        Self::new()
    }
}
