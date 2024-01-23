use super::MessageEntity;

use serde::Deserialize;

/// This object contains information about the quoted part of a message that is replied to by the given message.
/// # Documentation
/// <https://core.telegram.org/bots/api#textquote>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message
    pub text: Box<str>,
    /// Special entities that appear in the quote. Currently, only `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` entities are kept in quotes.
    pub entities: Option<Box<[MessageEntity]>>,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: u16,
    /// `true`, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    pub is_manual: bool,
}
