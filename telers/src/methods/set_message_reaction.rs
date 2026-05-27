use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmessagereaction>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMessageReaction {
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub message_id: i64,
    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Box<[crate::types::ReactionType]>>,
    /// Pass `true` to set the reaction with a big animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}
impl SetMessageReaction {
    /// Creates a new `SetMessageReaction`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `message_id` - Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        message_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            reaction: None,
            is_big: None,
        }
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn reactions<TItem: Into<crate::types::ReactionType>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.reaction = Some(
            self.reaction
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn reaction<T: Into<crate::types::ReactionType>>(mut self, val: T) -> Self {
        self.reaction = Some(
            self.reaction
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn reaction_option<
        TItem: Into<crate::types::ReactionType>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reaction = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Pass `true` to set the reaction with a big animation
    #[must_use]
    pub fn is_big<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_big = Some(val.into());
        self
    }

    /// Pass `true` to set the reaction with a big animation
    #[must_use]
    pub fn is_big_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_big = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetMessageReaction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMessageReaction", self, None)
    }
}
