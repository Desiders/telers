use crate::client::Bot;
use serde::Serialize;
/// Use this method to create a subscription invite link for a channel chat. The bot must have the `can_invite_users` administrator rights. The link can be edited using the method edit[`ChatSubscriptionInviteLink`] or revoked using the method revoke[`ChatInviteLink`]. Returns the new invite link as a [`ChatInviteLink`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#createchatsubscriptioninvitelink>
/// # Returns
/// - `crate::types::ChatInviteLink`
#[derive(Clone, Debug, Serialize)]
pub struct CreateChatSubscriptionInviteLink {
    /// Unique identifier for the target channel chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    pub subscription_period: i64,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
    pub subscription_price: u16,
}
impl CreateChatSubscriptionInviteLink {
    /// Creates a new `CreateChatSubscriptionInviteLink`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target channel chat or username of the target channel (in the format @channelusername)
    /// * `subscription_period` - The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    /// * `subscription_price` - The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>, T2: Into<u16>>(
        chat_id: T0,
        subscription_period: T1,
        subscription_price: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: None,
            subscription_period: subscription_period.into(),
            subscription_price: subscription_price.into(),
        }
    }

    /// Unique identifier for the target channel chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Invite link name; 0-32 characters
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = Some(val.into());
        this
    }

    /// Invite link name; 0-32 characters
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.name = val.map(Into::into);
        this
    }

    /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    #[must_use]
    pub fn subscription_period<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.subscription_period = val.into();
        this
    }

    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
    #[must_use]
    pub fn subscription_price<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.subscription_price = val.into();
        this
    }
}
impl super::TelegramMethod for CreateChatSubscriptionInviteLink {
    type Method = Self;
    type Return = crate::types::ChatInviteLink;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("createChatSubscriptionInviteLink", self, None)
    }
}
