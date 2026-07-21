use std::fmt::Display;

use crate::types::MessageEntity;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("The text is empty")]
    EmptyText,
    #[error("Index out of bounds")]
    IndexOutOfBounds,
}

/// Splits `text` into the `(before, inside, after)` parts delimited by a Telegram
/// [`MessageEntity`]'s `offset`/`length`.
///
/// Telegram entity offsets and lengths are measured in **UTF-16 code units** (not bytes or
/// `char`s), so the text is decoded to UTF-16, sliced in that domain, and re-encoded.
///
/// # Errors
/// Returns [`ErrorKind::IndexOutOfBounds`] if the offset/length are negative, their sum overflows,
/// they extend past the end of `text`, or a boundary splits a surrogate pair. It never panics, even
/// on attacker-controlled entities.
pub(crate) fn split_by_entity(
    text: &str,
    entity: &MessageEntity,
) -> Result<(String, String, String), ErrorKind> {
    let offset = usize::try_from(entity.offset()).map_err(|_| ErrorKind::IndexOutOfBounds)?;
    let length = usize::try_from(entity.length()).map_err(|_| ErrorKind::IndexOutOfBounds)?;
    let end = offset
        .checked_add(length)
        .ok_or(ErrorKind::IndexOutOfBounds)?;

    let units: Vec<u16> = text.encode_utf16().collect();
    if end > units.len() {
        return Err(ErrorKind::IndexOutOfBounds);
    }

    let decode = |slice: &[u16]| String::from_utf16(slice).map_err(|_| ErrorKind::IndexOutOfBounds);

    Ok((
        decode(&units[..offset])?,
        decode(&units[offset..end])?,
        decode(&units[end..])?,
    ))
}

/// The Bot API supports basic formatting for messages. You can use bold, italic, underlined, strikethrough, and spoiler text, as well as inline links and pre-formatted code in your bots' messages. Telegram clients will render them accordingly. You can specify text entities directly, or use markdown-style or HTML-style formatting.
///
/// Note that Telegram clients will display an **alert** to the user before opening an inline link ('Open this link?' together with the full URL).
///
/// Message entities can be nested, providing following restrictions are met:
/// - If two entities have common characters, then one of them is fully contained inside another.
/// - `bold`, `italic`, `underline`, `strikethrough`, and spoiler entities can contain and can be part of any other entities, except `pre` and `code`.
/// - All other entities can't contain each other.
///
/// Links `tg://user?id=<user_id>` can be used to mention a user by their ID without using a username. Please note:
/// - These links will work **only** if they are used inside an inline link or in an inline keyboard button. For example, they will not work, when used in a message text.
/// - Unless the user is a member in the chat where they were mentioned, these mentions are only guaranteed to work if the user has contacted the bot in private in the past or has sent a callback query to the bot via an inline button and doesn't have Forwarded Messages privacy enabled for the bot.
///
/// # Escaping
///
/// The formatting methods wrap the given text **as is**, without escaping, so their results
/// can be nested and composed freely (e.g. `italic(bold(text))`, or passing an HTML entity
/// like `&#8203;` as a link text).
/// Escape untrusted plain text explicitly with [`Formatter::quote`] before formatting it.
///
/// [`Formatter::apply_entity`] (and the [`Renderer`](crate::utils::text::Renderer)) receive
/// plain text by definition, so they escape it automatically.
///
/// # Examples
/// ```rust
/// use telers::utils::text::{Formatter as _, MarkdownFormatter};
///
/// let formatter = MarkdownFormatter::default();
///
/// // Formatting methods compose without corrupting each other's markup
/// assert_eq!(formatter.strikethrough(formatter.bold("text")), "~*text*~");
/// // Escape plain text explicitly when it can contain special characters
/// assert_eq!(
///     formatter.bold(formatter.quote("hello_world")),
///     "*hello\\_world*"
/// );
/// ```
pub trait Formatter {
    #[must_use]
    fn bold<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn italic<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn code<C>(&self, code: C) -> String
    where
        C: Display;

    #[must_use]
    fn underline<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn strikethrough<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn spoiler<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn blockquote<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn expandable_blockquote<T>(&self, text: T) -> String
    where
        T: Display;

    #[must_use]
    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: Display,
        U: Display;

    #[must_use]
    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: Display;

    #[must_use]
    fn custom_emoji<T, E>(&self, emoji: T, emoji_id: E) -> String
    where
        T: Display,
        E: Display;

    #[must_use]
    fn pre<C>(&self, code: C) -> String
    where
        C: Display;

    #[must_use]
    fn pre_language<C, L>(&self, code: C, language: L) -> String
    where
        C: Display,
        L: Display;

    #[must_use]
    fn date_time<T>(&self, text: T, unix_time: i64) -> String
    where
        T: Display;

    #[must_use]
    fn date_time_with_format<T, F>(&self, text: T, unix_time: i64, date_time_format: F) -> String
    where
        T: Display,
        F: Display;

    #[must_use]
    fn quote<T>(&self, text: T) -> String
    where
        T: Display;

    /// Apply the [`MessageEntity`] to the given text with offset and length.
    /// # Errors
    /// - If the given text is empty, then the [`ErrorKind::EmptyText`] will be returned.
    /// - If the given entity offset+length is out of bounds, then the [`ErrorKind::IndexOutOfBounds`] will be returned.
    fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, ErrorKind>
    where
        T: Display;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MessageEntityBold;

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn bold<T>(&self, text: T) -> String
        where
            T: Display,
        {
            format!("**{text}**")
        }

        fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, ErrorKind>
        where
            T: Display,
        {
            let text = text.to_string();
            let text_len = text.len();

            if text_len == 0 {
                return Err(ErrorKind::EmptyText);
            }

            let offset = entity.offset() as usize;
            let length = entity.length() as usize;

            if offset + length > text_len {
                return Err(ErrorKind::IndexOutOfBounds);
            }

            let editable_text = &text[offset..offset + length];

            if let MessageEntity::Bold(_) = entity {
            } else {
                unimplemented!();
            }

            let edited_text = self.bold(editable_text);

            let mut text = text.to_owned();
            text.replace_range(offset..offset + length, &edited_text);

            Ok(text)
        }

        fn italic<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn code<C>(&self, _code: C) -> String
        where
            C: Display,
        {
            todo!()
        }

        fn underline<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn strikethrough<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn spoiler<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn blockquote<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn expandable_blockquote<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn text_link<T, U>(&self, _text: T, _url: U) -> String
        where
            T: Display,
            U: Display,
        {
            todo!()
        }

        fn text_mention<T>(&self, _text: T, _user_id: i64) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn custom_emoji<T, E>(&self, _emoji: T, _emoji_id: E) -> String
        where
            T: Display,
            E: Display,
        {
            todo!()
        }

        fn pre<C>(&self, _code: C) -> String
        where
            C: Display,
        {
            todo!()
        }

        fn pre_language<C, L>(&self, _code: C, _language: L) -> String
        where
            C: Display,
            L: Display,
        {
            todo!()
        }

        fn date_time<T>(&self, _text: T, _unix_time: i64) -> String
        where
            T: Display,
        {
            todo!()
        }

        fn date_time_with_format<T, F>(
            &self,
            _text: T,
            _unix_time: i64,
            _date_time_format: F,
        ) -> String
        where
            T: Display,
            F: Display,
        {
            todo!()
        }

        fn quote<T>(&self, _text: T) -> String
        where
            T: Display,
        {
            todo!()
        }
    }

    #[test]
    fn test_apply_entity() {
        let formatter = TestFormatter;
        let text = "Hello, world!";

        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 5));

        assert_eq!(
            formatter.apply_entity(text, &entity).unwrap(),
            "**Hello**, world!"
        );

        let entity = MessageEntity::Bold(MessageEntityBold::new(7, 5));

        assert_eq!(
            formatter.apply_entity(text, &entity).unwrap(),
            "Hello, **world**!"
        );

        let entity = MessageEntity::Bold(MessageEntityBold::new(0, text.len() as i64));

        assert_eq!(
            formatter.apply_entity(text, &entity).unwrap(),
            "**Hello, world!**"
        );
    }

    #[test]
    #[should_panic]
    fn test_apply_entity_panic() {
        let formatter = TestFormatter;
        let text = "Hello, world!";
        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 15));

        formatter.apply_entity(text, &entity).unwrap();

        let entity = MessageEntity::Bold(MessageEntityBold::new(7, 9));

        formatter.apply_entity(text, &entity).unwrap();

        let entity = MessageEntity::Bold(MessageEntityBold::new(0, text.len() as i64 + 1));

        formatter.apply_entity(text, &entity).unwrap();

        let text = "";

        formatter.apply_entity(text, &entity).unwrap();
    }

    #[test]
    fn split_by_entity_uses_utf16_and_rejects_invalid_offsets() {
        // "a😀b": 'a' = unit 0, '😀' = units 1..3 (a surrogate pair), 'b' = unit 3 — 4 units total.
        let text = "a😀b";

        // Bolding "b" at UTF-16 offset 3, length 1 splits into ("a😀", "b", "").
        let entity = MessageEntity::Bold(MessageEntityBold::new(3, 1));
        assert_eq!(
            split_by_entity(text, &entity).unwrap(),
            ("a😀".to_owned(), "b".to_owned(), String::new())
        );

        // A negative offset returns an error instead of panicking on `usize::try_from`.
        let entity = MessageEntity::Bold(MessageEntityBold::new(-1, 1));
        assert!(matches!(
            split_by_entity(text, &entity),
            Err(ErrorKind::IndexOutOfBounds)
        ));

        // `offset + length` past the end (the text is 4 UTF-16 units) returns an error.
        let entity = MessageEntity::Bold(MessageEntityBold::new(0, 10));
        assert!(matches!(
            split_by_entity(text, &entity),
            Err(ErrorKind::IndexOutOfBounds)
        ));

        // An offset that falls between the surrogate halves of '😀' (unit 2) splits the pair, which
        // `String::from_utf16` rejects — so we return an error instead of producing garbage.
        let entity = MessageEntity::Bold(MessageEntityBold::new(2, 1));
        assert!(matches!(
            split_by_entity(text, &entity),
            Err(ErrorKind::IndexOutOfBounds)
        ));
    }
}
