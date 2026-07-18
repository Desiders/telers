//! Tag primitives shared by the [`Renderer`](super::Renderer) and the per-format writers:
//! the open/close/mid-newline markers parsed from entities, and the tag-string tables.

use std::cmp::Ordering;

/// Where a [`Tag`] sits relative to its entity span.
///
/// Ordered so `End` sorts before `Start` at the same offset: a previous entity must be
/// closed before the next one opens.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Place {
    End,
    MidNewLine,
    Start,
}

/// The kinds of entity that produce markup. Auto-detected entities are intentionally absent.
#[derive(Clone, PartialEq, Eq)]
pub(crate) enum Kind<'a> {
    Bold,
    Blockquote,
    ExpandableBlockquote,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre(Option<&'a str>),
    TextLink(&'a str),
    TextMention(i64),
    CustomEmoji(&'a str),
    DateTime {
        unix_time: i64,
        format: Option<&'a str>,
    },
}

/// An opening, closing, or mid-newline marker for an entity at a UTF-16 `offset`.
#[derive(Clone)]
pub(crate) struct Tag<'a> {
    pub place: Place,
    pub kind: Kind<'a>,
    pub offset: usize,
    pub index: usize,
}

impl<'a> Tag<'a> {
    pub fn start(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self {
            place: Place::Start,
            kind,
            offset,
            index,
        }
    }

    pub fn mid_new_line(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self {
            place: Place::MidNewLine,
            kind,
            offset,
            index,
        }
    }

    pub fn end(kind: Kind<'a>, offset: usize, index: usize) -> Self {
        Self {
            place: Place::End,
            kind,
            offset,
            index,
        }
    }
}

impl Eq for Tag<'_> {}

impl PartialEq for Tag<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.place == other.place && self.offset == other.offset && self.index == other.index
    }
}

impl Ord for Tag<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset
            .cmp(&other.offset)
            .then_with(|| self.place.cmp(&other.place))
            .then_with(|| match other.place {
                // Opening tags: earlier entity (outer) opens first.
                Place::Start | Place::MidNewLine => self.index.cmp(&other.index),
                // Closing tags: later entity (inner) closes first.
                Place::End => other.index.cmp(&self.index),
            })
    }
}

impl PartialOrd for Tag<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A tag with the same string for opening and closing (e.g. `*bold*`).
pub(crate) struct SimpleTag {
    pub start: &'static str,
    pub end: &'static str,
}

impl SimpleTag {
    pub const fn new(start: &'static str, end: &'static str) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn get_tag(&self, place: Place) -> &'static str {
        match place {
            Place::Start => self.start,
            Place::End => self.end,
            Place::MidNewLine => unreachable!("simple tags are never placed at a mid-newline"),
        }
    }
}

/// A tag that wraps a data value (url, language, …) between three parts.
pub(crate) struct ComplexTag {
    pub start: &'static str,
    pub middle: &'static str,
    pub end: &'static str,
}

impl ComplexTag {
    pub const fn new(start: &'static str, middle: &'static str, end: &'static str) -> Self {
        Self {
            start,
            middle,
            end,
        }
    }
}

/// Like [`ComplexTag`], but for a date-time tag that also carries an **optional** `format`
/// value, hence the extra `format_sep` placed before the format when present.
pub(crate) struct DateTimeTag {
    pub start: &'static str,
    pub middle: &'static str,
    pub format_sep: &'static str,
    pub end: &'static str,
}

impl DateTimeTag {
    pub const fn new(
        start: &'static str,
        middle: &'static str,
        format_sep: &'static str,
        end: &'static str,
    ) -> Self {
        Self {
            start,
            middle,
            format_sep,
            end,
        }
    }
}

/// A blockquote-style tag whose marker is repeated after each inner newline.
pub(crate) struct NewLineRepeatedTag {
    pub start: &'static str,
    pub repeat: &'static str,
    pub end: &'static str,
}

impl NewLineRepeatedTag {
    pub const fn new(start: &'static str, repeat: &'static str, end: &'static str) -> Self {
        Self {
            start,
            repeat,
            end,
        }
    }
}

/// The set of tag strings and writer callbacks for one output format.
pub(crate) struct TagWriter {
    pub bold: SimpleTag,
    pub blockquote: NewLineRepeatedTag,
    pub expandable_blockquote: NewLineRepeatedTag,
    pub italic: SimpleTag,
    pub underline: SimpleTag,
    pub strikethrough: SimpleTag,
    pub spoiler: SimpleTag,
    pub code: SimpleTag,
    pub pre_no_lang: SimpleTag,
    pub pre: ComplexTag,
    pub text_link: ComplexTag,
    pub text_mention: ComplexTag,
    pub custom_emoji: ComplexTag,
    pub date_time: DateTimeTag,
    pub write_tag_fn: fn(&Tag, &mut String),
    pub write_char_fn: fn(char, &mut String, bool),
}
