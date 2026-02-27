use crate::client::Bot;
use serde::Serialize;
/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletemessages>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// A JSON-serialized list of 1-100 identifiers of messages to delete. See deleteMessage for limitations on which messages can be deleted
    pub message_ids: Box<[u8]>,
}
impl DeleteMessages {
    /// Creates a new `DeleteMessages`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// * `message_ids` - A JSON-serialized list of 1-100 identifiers of messages to delete. See deleteMessage for limitations on which messages can be deleted
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1Item: Into<u8>,
        T1: IntoIterator<Item = T1Item>,
    >(
        chat_id: T0,
        message_ids: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_ids: message_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages to delete. See deleteMessage for limitations on which messages can be deleted
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn message_ids<TItem: Into<u8>, T: IntoIterator<Item = TItem>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages to delete. See deleteMessage for limitations on which messages can be deleted
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn message_id<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
impl super::TelegramMethod for DeleteMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteMessages", self, None)
    }
}
