use serde::Deserialize;

/// This object contains the data of the Mini App user.
/// <https://core.telegram.org/bots/webapps#webappuser>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct WebAppUser {
    /// A unique identifier for the user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. It has at most 52 significant bits, so a 64-bit integer or a double-precision float type is safe for storing this identifier.
    pub id: i64,
    /// `true`, if this user is a bot. Returns in the [receiver](https://core.telegram.org/bots/webapps#webappinitdata) field only.
    pub is_bot: Option<bool>,
    /// First name of the user or bot.
    pub first_name: Box<str>,
    /// Last name of the user or bot.
    pub last_name: Option<Box<str>>,
    /// Username of the user or bot.
    pub username: Option<Box<str>>,
    /// [IETF language tag language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language. Returns in user field only.
    pub language_code: Option<Box<str>>,
    /// `true`, if this user is a Telegram Premium user.
    pub is_premium: Option<bool>,
    /// `true`, if this user added the bot to the attachment menu.
    pub added_to_attachment_menu: Option<bool>,
    /// `true`, if this user allowed the bot to message them.
    pub allows_write_to_pm: Option<bool>,
    /// URL of the user’s profile photo. The photo can be in .jpeg or .svg formats. Only returned for Mini Apps launched from the attachment menu.
    pub photo_url: Option<Box<str>>,
}
