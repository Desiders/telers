use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send answers to callback queries sent from [inline keyboards](https://core.telegram.org/bots/features#inline-keyboards). The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
/// # Documentation
/// <https://core.telegram.org/bots/api#answercallbackquery>
/// # Notes
/// Alternatively, the user can be redirected to the specified Game URL. For this option to work, you must first create a game for your bot via @BotFather and accept the terms. Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    pub text: Option<String>,
    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to `false`.
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a [`Game`](crate::types::Game) and accepted the conditions via @BotFather, specify the URL that opens your game - note that this will only work if the query comes from a [callback_game](crate::types::InlineKeyboardButton) button.
    /// Otherwise, you may use links like `t.me/your_bot?start=XXXX` that open your bot with a parameter.
    pub url: Option<String>,
    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Telegram apps will support caching starting in version 3.14. Defaults to `0`.
    pub cache_time: Option<i64>,
}

impl AnswerCallbackQuery {
    #[must_use]
    pub fn new(callback_query_id: impl Into<String>) -> Self {
        Self {
            callback_query_id: callback_query_id.into(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    #[must_use]
    pub fn callback_query_id(self, val: impl Into<String>) -> Self {
        Self {
            callback_query_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn show_alert(self, val: bool) -> Self {
        Self {
            show_alert: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn cache_time(self, val: i64) -> Self {
        Self {
            cache_time: Some(val),
            ..self
        }
    }
}

impl AnswerCallbackQuery {
    #[must_use]
    pub fn text_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            text: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn show_alert_option(self, val: Option<bool>) -> Self {
        Self {
            show_alert: val,
            ..self
        }
    }

    #[must_use]
    pub fn url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn cache_time_option(self, val: Option<i64>) -> Self {
        Self {
            cache_time: val,
            ..self
        }
    }
}

impl TelegramMethod for AnswerCallbackQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerCallbackQuery", self, None)
    }
}

impl AsRef<AnswerCallbackQuery> for AnswerCallbackQuery {
    fn as_ref(&self) -> &Self {
        self
    }
}
