use super::base::Filter;

use crate::{client::Bot, context::Context, types::Update};

use async_trait::async_trait;
use regex::Regex;
use std::borrow::Cow;

/// Represents a command pattern type for verification
/// # Variants
/// * [`PatternType::Text(Cow<str>)`]:
/// A command pattern with text
/// * [`PatternType::Regex(Regex)`]:
/// A command pattern with regex, compiled with [`Regex`] struct. \
/// If filter used with `ignore_case` flag, then the regex will be compiled with `(?i)` flag (ignore case sensitive flag).
#[derive(Debug, Clone)]
pub enum PatternType<'a> {
    Text(Cow<'a, str>),
    Regex(Regex),
}

impl<'a> From<Cow<'a, str>> for PatternType<'a> {
    fn from(text: Cow<'a, str>) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a str> for PatternType<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<Regex> for PatternType<'_> {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

/// This filter checks if the text matches the specified pattern
/// # Notes
/// Gets the text from the update, that is, the text of the message, the text of the inline query, the data of the callback query, etc.
#[derive(Debug, Default, Clone)]
pub struct Text<'a> {
    /// List of texts or compiled [`Regex`] patterns that must be equal to the text
    texts: Box<[PatternType<'a>]>,
    /// List of texts that must be contained in the text
    contains: Box<[Cow<'a, str>]>,
    /// List of texts that must be at the beginning of the text
    starts_with: Box<[Cow<'a, str>]>,
    /// List of texts that must be at the end of the text
    ends_with: Box<[Cow<'a, str>]>,
    /// Ignore case sensitive
    ignore_case: bool,
}

impl<'a> Text<'a> {
    /// Creates a new [`Text`] filter
    /// # Arguments
    /// * `texts` -
    /// List of texts or compiled [`Regex`] patterns that must be equal to the text
    /// * `contains` -
    /// List of texts that must be contained in the text
    /// * `starts_with` -
    /// List of texts that must be at the beginning of the text
    /// * `ends_with` -
    /// List of texts that must be at the end of the text
    /// # Panics
    /// If `ignore_case` is `true` and [`Regex`]
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    pub fn new<T, I1, C, I2, S, I3, E, I4>(
        texts: I1,
        contains: I2,
        starts_with: I3,
        ends_with: I4,
        ignore_case: bool,
    ) -> Self
    where
        T: Into<PatternType<'a>>,
        I1: IntoIterator<Item = T>,
        C: Into<Cow<'a, str>>,
        I2: IntoIterator<Item = C>,
        S: Into<Cow<'a, str>>,
        I3: IntoIterator<Item = S>,
        E: Into<Cow<'a, str>>,
        I4: IntoIterator<Item = E>,
    {
        if ignore_case {
            Self {
                texts: texts
                    .into_iter()
                    .map(|text| match text.into() {
                        PatternType::Text(text) => PatternType::Text(text.to_lowercase().into()),
                        PatternType::Regex(regex) => PatternType::Regex(
                            Regex::new(&format!("(?i){regex}"))
                                .expect("Failed to compile regex with (?i) flag"),
                        ),
                    })
                    .collect(),
                contains: contains
                    .into_iter()
                    .map(|val| val.into().to_lowercase().into())
                    .collect(),
                starts_with: starts_with
                    .into_iter()
                    .map(|val| val.into().to_lowercase().into())
                    .collect(),
                ends_with: ends_with
                    .into_iter()
                    .map(|val| val.into().to_lowercase().into())
                    .collect(),
                ignore_case,
            }
        } else {
            Self {
                texts: texts.into_iter().map(Into::into).collect(),
                contains: contains.into_iter().map(Into::into).collect(),
                starts_with: starts_with.into_iter().map(Into::into).collect(),
                ends_with: ends_with.into_iter().map(Into::into).collect(),
                ignore_case,
            }
        }
    }

    /// Creates a new [`Text`] fitler with pass text or compiled [`Regex`] pattern that must be equal to the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn one(text: impl Into<PatternType<'a>>) -> Self {
        Self::builder().text(text).build()
    }

    /// Creates a new [`Text`] fitler with pass texts or compiled [`Regex`] patterns that must be equal to the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn many(texts: impl IntoIterator<Item = impl Into<PatternType<'a>>>) -> Self {
        Self::builder().texts(texts).build()
    }

    /// Creates a new [`Text`] filter with pass text that must be contained in the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn contains_single(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().contains_single(val).build()
    }

    /// Creates a new [`Text`] filter with pass texts that must be contained in the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn contains(val: impl IntoIterator<Item = impl Into<Cow<'a, str>>>) -> Self {
        Self::builder().contains(val).build()
    }

    /// Creates a new [`Text`] filter with pass text that must be at the beginning of the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn starts_with_single(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().starts_with_single(val).build()
    }

    /// Creates a new [`Text`] filter with pass texts that must be at the beginning of the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn starts_with(val: impl IntoIterator<Item = impl Into<Cow<'a, str>>>) -> Self {
        Self::builder().starts_with(val).build()
    }

    /// Creates a new [`Text`] filter with pass text that must be at the end of the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn ends_with_single(val: impl Into<Cow<'a, str>>) -> Self {
        Self::builder().ends_with_single(val).build()
    }

    /// Creates a new [`Text`] filter with pass texts that must be at the end of the text
    /// # Notes
    /// This method is just a shortcut to create a filter using the builder
    #[must_use]
    pub fn ends_with(val: impl IntoIterator<Item = impl Into<Cow<'a, str>>>) -> Self {
        Self::builder().ends_with(val).build()
    }

    /// # Panics
    /// If `ignore_case` is `true` and [`Regex`],
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn builder() -> TextBuilder<'a> {
        TextBuilder::default()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct TextBuilder<'a> {
    texts: Vec<PatternType<'a>>,
    contains: Vec<Cow<'a, str>>,
    starts_with: Vec<Cow<'a, str>>,
    ends_with: Vec<Cow<'a, str>>,
    ignore_case: bool,
}

impl<'a> TextBuilder<'a> {
    #[must_use]
    pub fn text(self, val: impl Into<PatternType<'a>>) -> Self {
        Self {
            texts: self.texts.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn texts<T, I>(self, val: I) -> Self
    where
        T: Into<PatternType<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            texts: self
                .texts
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn contains_single(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            contains: self.contains.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn contains<T, I>(self, val: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            contains: self
                .contains
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn starts_with_single(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            starts_with: self
                .starts_with
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn starts_with<T, I>(self, starts_with: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            starts_with: self
                .starts_with
                .into_iter()
                .chain(starts_with.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn ends_with_single(self, val: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ends_with: self.ends_with.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn ends_with<T, I>(self, ends_with: I) -> Self
    where
        T: Into<Cow<'a, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            ends_with: self
                .ends_with
                .into_iter()
                .chain(ends_with.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn ignore_case(self, ignore_case: bool) -> Self {
        Self {
            ignore_case,
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> Text<'a> {
        Text::new(
            self.texts,
            self.contains,
            self.starts_with,
            self.ends_with,
            self.ignore_case,
        )
    }
}

impl<'a> Text<'a> {
    #[must_use]
    fn prepare_text(&self, text: &str) -> Box<str> {
        if self.ignore_case {
            text.to_lowercase()
        } else {
            text.to_owned()
        }
        .into()
    }

    #[must_use]
    pub fn validate_texts(&self, text: &str) -> bool {
        let text = self.prepare_text(text);
        let text_ref = text.as_ref();

        self.texts.iter().any(|pattern| match pattern {
            PatternType::Text(allowed_text) => allowed_text == text_ref,
            PatternType::Regex(regex) => regex.is_match(&text),
        })
    }

    #[must_use]
    pub fn validate_contains(&self, text: &str) -> bool {
        let text = self.prepare_text(text);

        self.contains
            .iter()
            .any(|part_text| text.contains(part_text.as_ref()))
    }

    #[must_use]
    pub fn validate_starts_with(&self, text: &str) -> bool {
        let text = self.prepare_text(text);

        self.starts_with
            .iter()
            .any(|part_text| text.starts_with(part_text.as_ref()))
    }

    #[must_use]
    pub fn validate_ends_with(&self, text: &str) -> bool {
        let text = self.prepare_text(text);

        self.ends_with
            .iter()
            .any(|part_text| text.ends_with(part_text.as_ref()))
    }

    #[must_use]
    pub fn validate_text(&self, text: &str) -> bool {
        self.validate_texts(text)
            || self.validate_contains(text)
            || self.validate_starts_with(text)
            || self.validate_ends_with(text)
    }
}

#[async_trait]
impl<Client> Filter<Client> for Text<'_> {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        update.text().map_or(false, |text| self.validate_text(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_validate_texts() {
        let text = Text::builder().text("text").text("text2").build();

        assert!(text.validate_texts("text"));
        assert!(text.validate_texts("text2"));
        assert!(!text.validate_texts("text3"));
        assert!(!text.validate_texts("TEXT"));
        assert!(!text.validate_texts("TEXT2"));
        assert!(!text.validate_texts("TEXT3"));

        let text = Text::builder()
            .text("text")
            .text("text2")
            .ignore_case(true)
            .build();

        assert!(text.validate_texts("text"));
        assert!(text.validate_texts("text2"));
        assert!(!text.validate_texts("text3"));
        assert!(text.validate_texts("TEXT"));
        assert!(text.validate_texts("TEXT2"));
        assert!(!text.validate_texts("TEXT3"));
    }

    #[test]
    fn text_validate_contains() {
        let text = Text::builder()
            .contains_single("foo")
            .contains_single("bar")
            .build();

        assert!(text.validate_contains("foo"));
        assert!(text.validate_contains("bar"));
        assert!(text.validate_contains("foobar"));
        assert!(text.validate_contains("foob"));
        assert!(text.validate_contains("oobar"));
        assert!(!text.validate_contains("fo"));
        assert!(!text.validate_contains("ba"));
        assert!(!text.validate_contains("FOO"));
        assert!(!text.validate_contains("BAR"));
        assert!(!text.validate_contains("FOOBAR"));
        assert!(!text.validate_contains("FOOB"));
        assert!(!text.validate_contains("OOBAR"));

        let text = Text::builder()
            .contains_single("foo")
            .contains_single("bar")
            .ignore_case(true)
            .build();

        assert!(text.validate_contains("foo"));
        assert!(text.validate_contains("bar"));
        assert!(text.validate_contains("foobar"));
        assert!(text.validate_contains("foob"));
        assert!(text.validate_contains("oobar"));
        assert!(!text.validate_contains("fo"));
        assert!(!text.validate_contains("ba"));
        assert!(text.validate_contains("FOO"));
        assert!(text.validate_contains("BAR"));
        assert!(text.validate_contains("FOOBAR"));
        assert!(text.validate_contains("FOOB"));
        assert!(text.validate_contains("OOBAR"));
    }

    #[test]
    fn text_validate_starts_with() {
        let text = Text::builder()
            .starts_with_single("foo")
            .starts_with_single("bar")
            .build();

        assert!(text.validate_starts_with("foo"));
        assert!(text.validate_starts_with("bar"));
        assert!(text.validate_starts_with("foobar"));
        assert!(text.validate_starts_with("foob"));
        assert!(!text.validate_starts_with("oobar"));
        assert!(!text.validate_starts_with("fo"));
        assert!(!text.validate_starts_with("ba"));
        assert!(!text.validate_starts_with("FOO"));
        assert!(!text.validate_starts_with("BAR"));
        assert!(!text.validate_starts_with("FOOBAR"));
        assert!(!text.validate_starts_with("FOOB"));
        assert!(!text.validate_starts_with("OOBAR"));

        let text = Text::builder()
            .starts_with_single("foo")
            .starts_with_single("bar")
            .ignore_case(true)
            .build();

        assert!(text.validate_starts_with("foo"));
        assert!(text.validate_starts_with("bar"));
        assert!(text.validate_starts_with("foobar"));
        assert!(text.validate_starts_with("foob"));
        assert!(!text.validate_starts_with("oobar"));
        assert!(!text.validate_starts_with("fo"));
        assert!(!text.validate_starts_with("ba"));
        assert!(text.validate_starts_with("FOO"));
        assert!(text.validate_starts_with("BAR"));
        assert!(text.validate_starts_with("FOOBAR"));
        assert!(text.validate_starts_with("FOOB"));
        assert!(!text.validate_starts_with("OOBAR"));
    }

    #[test]
    fn text_validate_ends_with() {
        let text = Text::builder()
            .ends_with_single("foo")
            .ends_with_single("bar")
            .build();

        assert!(text.validate_ends_with("foo"));
        assert!(text.validate_ends_with("bar"));
        assert!(text.validate_ends_with("foobar"));
        assert!(!text.validate_ends_with("foob"));
        assert!(text.validate_ends_with("oobar"));
        assert!(!text.validate_ends_with("fo"));
        assert!(!text.validate_ends_with("ba"));
        assert!(!text.validate_ends_with("FOO"));
        assert!(!text.validate_ends_with("BAR"));
        assert!(!text.validate_ends_with("FOOBAR"));
        assert!(!text.validate_ends_with("FOOB"));
        assert!(!text.validate_ends_with("OOBAR"));

        let text = Text::builder()
            .ends_with_single("foo")
            .ends_with_single("bar")
            .ignore_case(true)
            .build();

        assert!(text.validate_ends_with("foo"));
        assert!(text.validate_ends_with("bar"));
        assert!(text.validate_ends_with("foobar"));
        assert!(!text.validate_ends_with("foob"));
        assert!(text.validate_ends_with("oobar"));
        assert!(!text.validate_ends_with("fo"));
        assert!(!text.validate_ends_with("ba"));
        assert!(text.validate_ends_with("FOO"));
        assert!(text.validate_ends_with("BAR"));
        assert!(text.validate_ends_with("FOOBAR"));
        assert!(!text.validate_ends_with("FOOB"));
        assert!(text.validate_ends_with("OOBAR"));
    }
}
