use crate::client::Bot;
use serde::Serialize;
/// Gifts a Telegram Premium subscription to the given user. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#giftpremiumsubscription>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct GiftPremiumSubscription {
    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    pub user_id: i64,
    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    pub month_count: i64,
    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    pub star_count: i64,
    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl GiftPremiumSubscription {
    /// Creates a new `GiftPremiumSubscription`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user who will receive a Telegram Premium subscription
    /// * `month_count` - Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    /// * `star_count` - Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>>(
        user_id: T0,
        month_count: T1,
        star_count: T2,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            month_count: month_count.into(),
            star_count: star_count.into(),
            text: None,
            text_parse_mode: None,
            text_entities: None,
        }
    }

    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    #[must_use]
    pub fn month_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.month_count = val.into();
        this
    }

    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    #[must_use]
    pub fn star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.star_count = val.into();
        this
    }

    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = Some(val.into());
        this
    }

    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    #[must_use]
    pub fn text_parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text_parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    #[must_use]
    pub fn text_parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text_parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of `text_parse_mode`. Entities other than `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` are ignored.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.text_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }
}
impl super::TelegramMethod for GiftPremiumSubscription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("giftPremiumSubscription", self, None)
    }
}
