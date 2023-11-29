#[must_use]
pub fn add_surrogates(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[must_use]
pub fn remove_surrogates(text: &[u16]) -> String {
    String::from_utf16_lossy(text)
}

/// The Bot API supports basic formatting for messages. You can use bold, italic, underlined, strikethrough, and spoiler text, as well as inline links and pre-formatted code in your bots' messages. Telegram clients will render them accordingly. You can specify text entities directly, or use markdown-style or HTML-style formatting.
///
/// Note that Telegram clients will display an **alert** to the user before opening an inline link ('Open this link?' together with the full URL).
///
/// Message entities can be nested, providing following restrictions are met:
/// - If two entities have common characters, then one of them is fully contained inside another.
/// - `bold`, `italic`, `underline`, `strikethrough`, and spoiler entities can contain and can be part of any other entities, except pre and code.
/// - All other entities can't contain each other.
///
/// Links `tg://user?id=<user_id>` can be used to mention a user by their ID without using a username. Please note:
/// - These links will work **only** if they are used inside an inline link or in an inline keyboard button. For example, they will not work, when used in a message text.
/// - Unless the user is a member in the chat where they were mentioned, these mentions are only guaranteed to work if the user has contacted the bot in private in the past or has sent a callback query to the bot via an inline button and doesn't have Forwarded Messages privacy enabled for the bot.
pub trait Decoration {
    /// Decorate text with `bold` tag
    #[must_use]
    fn bold(&self, text: &str) -> String;

    /// Decorate text with `italic` tag
    #[must_use]
    fn italic(&self, text: &str) -> String;

    /// Decorate text with `code` tag
    #[must_use]
    fn code(&self, text: &str) -> String;

    /// Decorate text with `underline` tag
    #[must_use]
    fn underline(&self, text: &str) -> String;

    /// Decorate text with `strikethrough` tag
    #[must_use]
    fn strikethrough(&self, text: &str) -> String;

    /// Decorate text with `spoiler` tag
    #[must_use]
    fn spoiler(&self, text: &str) -> String;

    /// Decorate text with `pre` tag
    #[must_use]
    fn pre(&self, text: &str) -> String;

    /// Decorate text with `pre_language` tag
    #[must_use]
    fn pre_language(&self, text: &str, language: &str) -> String;

    /// Decorate text with `link` tag
    #[must_use]
    fn link(&self, text: &str, url: &str) -> String;

    /// Decorate text with `custom_emoji` tag
    #[must_use]
    fn custom_emoji(&self, text: &str, emoji_id: &str) -> String;

    /// Quote symbols
    #[must_use]
    fn quote(&self, text: &str) -> String;
}
