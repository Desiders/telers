//! Render a message's text/caption together with its [`MessageEntity`] list back into a
//! single HTML or MarkdownV2 string — the inverse of parsing formatted text into entities.
//!
//! This lets a formatted message be stored as one human-readable string and re-sent later
//! with a `parse_mode`, instead of persisting the text and entities separately.

mod html;
mod markdown;
mod tag;

use tag::{Kind, Tag, TagWriter};

use crate::types::MessageEntity;

/// Renders text and its message entities into HTML or MarkdownV2.
///
/// # Example
/// ```
/// use telers::{
///     types::{MessageEntity, MessageEntityBold},
///     utils::text::Renderer,
/// };
///
/// let text = "Bold text";
/// let entities = [MessageEntity::Bold(MessageEntityBold::new(0, 4))];
///
/// assert_eq!(Renderer::new(text, &entities).as_html(), "<b>Bold</b> text");
/// ```
#[derive(Clone)]
pub struct Renderer<'a> {
    text: &'a str,
    tags: Vec<Tag<'a>>,
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl<'a> Renderer<'a> {
    /// Creates a new [`Renderer`] for the given text and message entities.
    #[must_use]
    pub fn new(text: &'a str, entities: &'a [MessageEntity]) -> Self {
        let mut tags = Vec::with_capacity(entities.len() * 2);

        for (index, entity) in entities.iter().enumerate() {
            let kind = match entity {
                MessageEntity::Bold(_) => Kind::Bold,
                MessageEntity::Italic(_) => Kind::Italic,
                MessageEntity::Underline(_) => Kind::Underline,
                MessageEntity::Strikethrough(_) => Kind::Strikethrough,
                MessageEntity::Spoiler(_) => Kind::Spoiler,
                MessageEntity::Blockquote(_) => Kind::Blockquote,
                MessageEntity::ExpandableBlockquote(_) => Kind::ExpandableBlockquote,
                MessageEntity::Code(_) => Kind::Code,
                MessageEntity::Pre(pre) => Kind::Pre(pre.language.as_deref()),
                MessageEntity::TextLink(link) => Kind::TextLink(&link.url),
                MessageEntity::TextMention(mention) => Kind::TextMention(mention.user.id),
                MessageEntity::CustomEmoji(emoji) => Kind::CustomEmoji(&emoji.custom_emoji_id),
                MessageEntity::DateTime(date_time) => Kind::DateTime {
                    unix_time: date_time.unix_time,
                    format: date_time.date_time_format.as_deref(),
                },
                // Auto-detected entities (mention, hashtag, cashtag, bot command, url, email,
                // phone number) carry no markup — Telegram re-detects them — so they're skipped.
                _ => continue,
            };

            let offset = entity.offset() as usize;
            let length = entity.length() as usize;

            tags.push(Tag::start(kind.clone(), offset, index));

            // A blockquote can span multiple lines; MarkdownV2 needs the quote marker
            // repeated after every newline inside it.
            if matches!(kind, Kind::Blockquote | Kind::ExpandableBlockquote) {
                let new_line_indexes = text
                    .chars()
                    .skip(offset)
                    .take(length)
                    .enumerate()
                    .filter_map(|(idx, ch)| (ch == '\n').then_some(idx));

                for new_line_index in new_line_indexes {
                    tags.push(Tag::mid_new_line(
                        kind.clone(),
                        offset + new_line_index + 1,
                        index,
                    ));
                }
            }

            tags.push(Tag::end(kind, offset + length, index));
        }

        tags.sort_unstable();

        Self {
            text,
            tags,
        }
    }

    /// Renders the text with the given [`TagWriter`], inserting tags at their UTF-16 offsets.
    ///
    /// Unlike teloxide, text with no renderable entities is still escaped (rather than
    /// returned verbatim) so the result is always valid HTML / MarkdownV2.
    fn format(&self, writer: &TagWriter) -> String {
        let mut buffer = String::with_capacity(self.text.len() + self.tags.len() * 8);
        let mut tags = self.tags.iter();
        let mut current_tag = tags.next();
        let mut prev_point: Option<u16> = None;

        for (idx, point) in self.text.encode_utf16().enumerate() {
            while let Some(tag) = current_tag {
                if tag.offset == idx {
                    (writer.write_tag_fn)(tag, &mut buffer);
                    current_tag = tags.next();
                } else {
                    break;
                }
            }

            let ch = if let Some(previous) = prev_point.take() {
                char::decode_utf16([previous, point])
                    .next()
                    .unwrap()
                    .unwrap()
            } else {
                match char::decode_utf16([point]).next().unwrap() {
                    Ok(ch) => ch,
                    Err(unpaired) => {
                        prev_point = Some(unpaired.unpaired_surrogate());
                        continue;
                    }
                }
            };

            (writer.write_char_fn)(ch, &mut buffer);
        }

        for tag in current_tag.into_iter().chain(tags) {
            (writer.write_tag_fn)(tag, &mut buffer);
        }

        buffer
    }

    /// Renders the text as an **HTML-formatted** string.
    #[must_use]
    #[inline]
    pub fn as_html(&self) -> String {
        self.format(&html::HTML)
    }

    /// Renders the text as a **MarkdownV2-formatted** string.
    #[must_use]
    #[inline]
    pub fn as_markdown(&self) -> String {
        self.format(&markdown::MARKDOWN)
    }
}

#[cfg(test)]
mod tests {
    use super::Renderer;
    use crate::types::{
        MessageEntity, MessageEntityBold, MessageEntityCode, MessageEntityCustomEmoji,
        MessageEntityDateTime, MessageEntityHashtag, MessageEntityItalic, MessageEntityMention,
        MessageEntityPre, MessageEntityStrikethrough, MessageEntityTextLink,
        MessageEntityTextMention, MessageEntityUnderline, User,
    };

    #[test]
    fn render_simple() {
        let text = "Bold italic <underline_";
        let entities = [
            MessageEntity::Bold(MessageEntityBold::new(0, 4)),
            MessageEntity::Italic(MessageEntityItalic::new(5, 6)),
            MessageEntity::Underline(MessageEntityUnderline::new(12, 10)),
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "<b>Bold</b> <i>italic</i> <u>&lt;underline</u>_"
        );
        assert_eq!(
            render.as_markdown(),
            "*Bold* _\ritalic_\r __\r<underline__\r\\_"
        );
    }

    #[test]
    fn render_pre_with_lang() {
        let text = "Some pre, normal and rusty code";
        let entities = [
            MessageEntity::Pre(MessageEntityPre::new(5, 3)),
            MessageEntity::Code(MessageEntityCode::new(10, 6)),
            MessageEntity::Pre(MessageEntityPre::new(21, 5).language("rust")),
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "Some <pre>pre</pre>, <code>normal</code> and <pre><code \
             class=\"language-rust\">rusty</code></pre> code",
        );
        assert_eq!(
            render.as_markdown(),
            "Some ```\npre```\n, `normal` and ```rust\nrusty```\n code",
        );
    }

    #[test]
    fn render_nested() {
        let text = "Some bold both italics";
        let entities = [
            MessageEntity::Bold(MessageEntityBold::new(5, 9)),
            MessageEntity::Italic(MessageEntityItalic::new(10, 12)),
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(render.as_html(), "Some <b>bold <i>both</b> italics</i>");
        assert_eq!(render.as_markdown(), "Some *bold _\rboth* italics_\r");
    }

    #[test]
    fn render_overlapping_at_same_offset() {
        // Two entities starting at the same offset: outer (lower index) opens first, and
        // the inner closes first.
        let text = "este";
        let entities = [
            MessageEntity::Underline(MessageEntityUnderline::new(0, 4)),
            MessageEntity::Strikethrough(MessageEntityStrikethrough::new(0, 4)),
        ];

        assert_eq!(
            Renderer::new(text, &entities).as_html(),
            "<u><s>este</s></u>"
        );
    }

    #[test]
    fn render_custom_emoji() {
        let text = "👍";
        let entities = [MessageEntity::CustomEmoji(MessageEntityCustomEmoji::new(
            0,
            2,
            "5368324170671202286",
        ))];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "<tg-emoji emoji-id=\"5368324170671202286\">👍</tg-emoji>",
        );
        assert_eq!(
            render.as_markdown(),
            "[👍](tg://emoji?id=5368324170671202286)"
        );
    }

    #[test]
    fn render_date_time() {
        let text = "soon";
        let with_format = [MessageEntity::DateTime(
            MessageEntityDateTime::new(0, 4, 1).date_time_format("wDT"),
        )];
        let without_format = [MessageEntity::DateTime(MessageEntityDateTime::new(0, 4, 1))];

        assert_eq!(
            Renderer::new(text, &with_format).as_html(),
            "<tg-time unix=\"1\" format=\"wDT\">soon</tg-time>",
        );
        assert_eq!(
            Renderer::new(text, &without_format).as_html(),
            "<tg-time unix=\"1\">soon</tg-time>",
        );
        assert_eq!(
            Renderer::new(text, &with_format).as_markdown(),
            "![soon](tg://time?unix=1&format=wDT)",
        );
    }

    #[test]
    fn render_text_mention() {
        let text = "hi";
        let entities = [MessageEntity::TextMention(MessageEntityTextMention::new(
            0,
            2,
            User::new(123, false, "x"),
        ))];

        assert_eq!(
            Renderer::new(text, &entities).as_html(),
            "<a href=\"tg://user?id=123\">hi</a>",
        );
    }

    #[test]
    fn render_skips_auto_detected_entities() {
        // Mention/hashtag carry no markup, so the output is the (escaped) text only.
        let text = "@user #tag";
        let entities = [
            MessageEntity::Mention(MessageEntityMention::new(0, 5)),
            MessageEntity::Hashtag(MessageEntityHashtag::new(6, 4)),
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(render.as_html(), "@user #tag");
        assert_eq!(render.as_markdown(), "@user \\#tag");
    }

    #[test]
    fn render_complex() {
        let text = "Hi how are you?\nnested entities are cool\nIm in a Blockquote!\nIm in a \
                    multiline Blockquote!\n\nIm in a multiline Blockquote!\nIm in an expandable \
                    Blockquote!\nIm in an expandable multiline Blockquote!\n\nIm in an expandable \
                    multiline Blockquote!";
        let entities = [
            MessageEntity::Bold(MessageEntityBold::new(0, 2)),
            MessageEntity::Italic(MessageEntityItalic::new(3, 3)),
            MessageEntity::Underline(MessageEntityUnderline::new(7, 3)),
            MessageEntity::Strikethrough(MessageEntityStrikethrough::new(11, 3)),
            MessageEntity::Bold(MessageEntityBold::new(16, 1)),
            MessageEntity::Bold(MessageEntityBold::new(17, 5)),
            MessageEntity::Underline(MessageEntityUnderline::new(17, 4)),
            MessageEntity::Strikethrough(MessageEntityStrikethrough::new(17, 4)),
            MessageEntity::TextLink(MessageEntityTextLink::new(23, 8, "https://t.me/")),
            MessageEntity::TextLink(MessageEntityTextLink::new(32, 3, "tg://user?id=1234567")),
            MessageEntity::Code(MessageEntityCode::new(36, 4)),
            MessageEntity::Blockquote(crate::types::MessageEntityBlockquote::new(41, 19)),
            MessageEntity::Blockquote(crate::types::MessageEntityBlockquote::new(61, 60)),
            MessageEntity::ExpandableBlockquote(
                crate::types::MessageEntityExpandableBlockquote::new(122, 31),
            ),
            MessageEntity::ExpandableBlockquote(
                crate::types::MessageEntityExpandableBlockquote::new(154, 84),
            ),
        ];

        let render = Renderer::new(text, &entities);

        assert_eq!(
            render.as_html(),
            "<b>Hi</b> <i>how</i> <u>are</u> <s>you</s>?\n<b>n</b><b><u><s>este</s></u>d</b> \
            <a href=\"https://t.me/\">entities</a> <a href=\"tg://user?id=1234567\">are</a> <code>cool</code>\n\
            <blockquote>Im in a Blockquote!</blockquote>\n\
            <blockquote>Im in a multiline Blockquote!\n\nIm in a multiline Blockquote!</blockquote>\n\
            <blockquote expandable>Im in an expandable Blockquote!</blockquote>\n\
            <blockquote expandable>Im in an expandable multiline Blockquote!\n\nIm in an expandable multiline Blockquote!</blockquote>"
        );
        assert_eq!(
            render.as_markdown(),
            "*Hi* _\rhow_\r __\rare__\r ~you~?\n*n**__\r~este~__\rd* [entities](https://t.me/) \
             [are](tg://user?id=1234567) `cool`\n**>Im in a Blockquote\\!\n**>Im in a multiline \
             Blockquote\\!\n>\n>Im in a multiline Blockquote\\!\n**>Im in an expandable \
             Blockquote\\!||\n**>Im in an expandable multiline Blockquote\\!\n>\n>Im in an \
             expandable multiline Blockquote\\!||"
        );
    }
}
