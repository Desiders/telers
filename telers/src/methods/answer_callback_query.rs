use crate::client::Bot;
use serde::Serialize;
/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#answercallbackquery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: Box<str>,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @`BotFather`, specify the URL that opens your game - note that this will only work if the query comes from a `callback_game` button. Otherwise, you may use links like <https://t.me/your_bot?start=XXXX> that open your bot with a parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}
impl AnswerCallbackQuery {
    /// Creates a new `AnswerCallbackQuery`.
    ///
    /// # Arguments
    /// * `callback_query_id` - Unique identifier for the query to be answered
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(callback_query_id: T0) -> Self {
        Self {
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn callback_query_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.callback_query_id = val.into();
        this
    }

    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = Some(val.into());
        this
    }

    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text = val.map(Into::into);
        this
    }

    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[must_use]
    pub fn show_alert<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.show_alert = Some(val.into());
        this
    }

    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[must_use]
    pub fn show_alert_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.show_alert = val.map(Into::into);
        this
    }

    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @`BotFather`, specify the URL that opens your game - note that this will only work if the query comes from a `callback_game` button. Otherwise, you may use links like <https://t.me/your_bot?start=XXXX> that open your bot with a parameter.
    #[must_use]
    pub fn url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.url = Some(val.into());
        this
    }

    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @`BotFather`, specify the URL that opens your game - note that this will only work if the query comes from a `callback_game` button. Otherwise, you may use links like <https://t.me/your_bot?start=XXXX> that open your bot with a parameter.
    #[must_use]
    pub fn url_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.url = val.map(Into::into);
        this
    }

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[must_use]
    pub fn cache_time<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.cache_time = Some(val.into());
        this
    }

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to 0.
    #[must_use]
    pub fn cache_time_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.cache_time = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for AnswerCallbackQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerCallbackQuery", self, None)
    }
}
