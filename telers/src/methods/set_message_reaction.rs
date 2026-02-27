use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the chosen reactions on a message. Service messages of some types can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Bots can't use paid reactions. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmessagereaction>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMessageReaction {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
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
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
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

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn reactions<TItem: Into<crate::types::ReactionType>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.reaction = Some(
            this.reaction
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators. Paid reactions can't be used by bots.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn reaction<T: Into<crate::types::ReactionType>>(self, val: T) -> Self {
        let mut this = self;
        this.reaction = Some(
            this.reaction
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
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
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reaction = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }

    /// Pass `true` to set the reaction with a big animation
    #[must_use]
    pub fn is_big<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_big = Some(val.into());
        this
    }

    /// Pass `true` to set the reaction with a big animation
    #[must_use]
    pub fn is_big_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_big = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetMessageReaction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMessageReaction", self, None)
    }
}
