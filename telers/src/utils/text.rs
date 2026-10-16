//! Build the text of a message in one of Telegram's parse modes, and render it back from
//! entities.
//!
//! [`Builder`] assembles a message from plain and formatted fragments, [`Formatter`] and its HTML
//! and `MarkdownV2` implementations wrap single fragments in the markup of that parse mode, and
//! [`Renderer`] turns a text plus its [`MessageEntity`](crate::types::MessageEntity) list back
//! into one formatted string.

pub mod builder;
pub mod formatter;
pub mod html_formatter;
pub mod markdown_formatter;
pub mod render;

pub use builder::Builder;
pub use formatter::{ErrorKind as FormatterErrorKind, Formatter};
pub use html_formatter::{
    blockquote as html_blockquote, bold as html_bold, code as html_code,
    custom_emoji as html_custom_emoji, expandable_blockquote as html_expandable_blockquote,
    italic as html_italic, pre as html_pre, pre_language as html_pre_language, quote as html_quote,
    spoiler as html_spoiler, strikethrough as html_strikethrough, text_link as html_text_link,
    text_mention as html_text_mention, underline as html_underline, Formatter as HTMLFormatter,
};
pub use markdown_formatter::{
    blockquote as markdown_blockquote, bold as markdown_bold, code as markdown_code,
    custom_emoji as markdown_custom_emoji, expandable_blockquote as markdown_expandable_blockquote,
    italic as markdown_italic, pre as markdown_pre, pre_language as markdown_pre_language,
    quote as markdown_quote, spoiler as markdown_spoiler, strikethrough as markdown_strikethrough,
    text_link as markdown_text_link, text_mention as markdown_text_mention,
    underline as markdown_underline, Formatter as MarkdownFormatter,
};
pub use render::Renderer;
