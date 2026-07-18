//! HTML tag table and writers for the [`Renderer`](super::Renderer).

use std::fmt::Write;

use super::tag::{
    ComplexTag, DateTimeTag, Kind, NewLineRepeatedTag, Place, SimpleTag, Tag, TagWriter,
};

pub(crate) static HTML: TagWriter = TagWriter {
    bold: SimpleTag::new("<b>", "</b>"),
    blockquote: NewLineRepeatedTag::new("<blockquote>", "", "</blockquote>"),
    expandable_blockquote: NewLineRepeatedTag::new("<blockquote expandable>", "", "</blockquote>"),
    italic: SimpleTag::new("<i>", "</i>"),
    underline: SimpleTag::new("<u>", "</u>"),
    strikethrough: SimpleTag::new("<s>", "</s>"),
    spoiler: SimpleTag::new("<tg-spoiler>", "</tg-spoiler>"),
    code: SimpleTag::new("<code>", "</code>"),
    pre_no_lang: SimpleTag::new("<pre>", "</pre>"),
    pre: ComplexTag::new("<pre><code class=\"language-", "\">", "</code></pre>"),
    text_link: ComplexTag::new("<a href=\"", "\">", "</a>"),
    text_mention: ComplexTag::new("<a href=\"tg://user?id=", "\">", "</a>"),
    custom_emoji: ComplexTag::new("<tg-emoji emoji-id=\"", "\">", "</tg-emoji>"),
    date_time: DateTimeTag::new("<tg-time unix=\"", "\">", "\" format=\"", "</tg-time>"),
    write_tag_fn: write_tag,
    write_char_fn: write_char,
};

#[allow(clippy::too_many_lines)]
fn write_tag(tag: &Tag, buf: &mut String) {
    match tag.kind {
        Kind::Bold => buf.push_str(HTML.bold.get_tag(tag.place)),
        Kind::Italic => buf.push_str(HTML.italic.get_tag(tag.place)),
        Kind::Underline => buf.push_str(HTML.underline.get_tag(tag.place)),
        Kind::Strikethrough => buf.push_str(HTML.strikethrough.get_tag(tag.place)),
        Kind::Spoiler => buf.push_str(HTML.spoiler.get_tag(tag.place)),
        Kind::Code => buf.push_str(HTML.code.get_tag(tag.place)),
        Kind::Blockquote => match tag.place {
            Place::Start => buf.push_str(HTML.blockquote.start),
            Place::MidNewLine => (), // HTML keeps the newlines inside the tag.
            Place::End => buf.push_str(HTML.blockquote.end),
        },
        Kind::ExpandableBlockquote => match tag.place {
            Place::Start => buf.push_str(HTML.expandable_blockquote.start),
            Place::MidNewLine => (),
            Place::End => buf.push_str(HTML.expandable_blockquote.end),
        },
        Kind::Pre(lang) => match tag.place {
            Place::Start => match lang {
                Some(lang) => {
                    write!(buf, "{}{}{}", HTML.pre.start, lang, HTML.pre.middle).unwrap();
                }
                None => buf.push_str(HTML.pre_no_lang.start),
            },
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(lang.map_or(HTML.pre_no_lang.end, |_| HTML.pre.end)),
        },
        Kind::TextLink(url) => match tag.place {
            Place::Start => {
                write!(
                    buf,
                    "{}{}{}",
                    HTML.text_link.start, url, HTML.text_link.middle
                )
                .unwrap();
            }
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(HTML.text_link.end),
        },
        Kind::TextMention(id) => match tag.place {
            Place::Start => {
                write!(
                    buf,
                    "{}{}{}",
                    HTML.text_mention.start, id, HTML.text_mention.middle
                )
                .unwrap();
            }
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(HTML.text_mention.end),
        },
        Kind::CustomEmoji(id) => match tag.place {
            Place::Start => {
                write!(
                    buf,
                    "{}{}{}",
                    HTML.custom_emoji.start, id, HTML.custom_emoji.middle
                )
                .unwrap();
            }
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(HTML.custom_emoji.end),
        },
        Kind::DateTime {
            unix_time,
            format,
        } => match tag.place {
            Place::Start => match format {
                Some(format) => write!(
                    buf,
                    "{}{unix_time}{}{format}{}",
                    HTML.date_time.start, HTML.date_time.format_sep, HTML.date_time.middle
                )
                .unwrap(),
                None => {
                    write!(
                        buf,
                        "{}{unix_time}{}",
                        HTML.date_time.start, HTML.date_time.middle
                    )
                    .unwrap();
                }
            },
            Place::MidNewLine => unreachable!(),
            Place::End => buf.push_str(HTML.date_time.end),
        },
    }
}

fn write_char(ch: char, buf: &mut String, _verbatim: bool) {
    match ch {
        '&' => buf.push_str("&amp;"),
        '<' => buf.push_str("&lt;"),
        '>' => buf.push_str("&gt;"),
        ch => buf.push(ch),
    }
}
