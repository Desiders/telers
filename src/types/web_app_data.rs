use serde::Deserialize;

/// Describes data sent from a [`Web App`](https://core.telegram.org/bots/webapps) to the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#webappdata>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: Box<str>,
    /// Text of the `web_app` keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: Box<str>,
}
