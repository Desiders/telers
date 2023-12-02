use crate::types::{MessageEntity, MessageEntityKind};

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("The text is empty")]
    EmptyText,
    #[error("Index out of bounds")]
    IndexOutOfBounds,
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
pub trait Formatter {
    #[must_use]
    fn bold<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn italic<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn code<C>(&self, code: C) -> String
    where
        C: AsRef<str>;

    #[must_use]
    fn underline<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn strikethrough<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn spoiler<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn text_link<T, U>(&self, text: T, url: U) -> String
    where
        T: AsRef<str>,
        U: AsRef<str>;

    #[must_use]
    fn text_mention<T>(&self, text: T, user_id: i64) -> String
    where
        T: AsRef<str>;

    #[must_use]
    fn custom_emoji<T, E>(&self, emoji: T, emoji_id: E) -> String
    where
        T: AsRef<str>,
        E: AsRef<str>;

    #[must_use]
    fn pre<C>(&self, code: C) -> String
    where
        C: AsRef<str>;

    #[must_use]
    fn pre_language<C, L>(&self, code: C, language: L) -> String
    where
        C: AsRef<str>,
        L: AsRef<str>;

    #[must_use]
    fn quote<T>(&self, text: T) -> String
    where
        T: AsRef<str>;

    /// Apply the [`MessageEntity`] to the given text with offset and length.
    /// # Notes
    /// Differences between [`Formatter::apply_entity`] and [`Formatter::apply_entity_kind`]:
    /// - [`Formatter::apply_entity`] will apply the [`MessageEntityKind`] to the given text with offset and length
    /// - [`Formatter::apply_entity_kind`] will apply the [`MessageEntityKind`] to the whole given text
    /// # Errors
    /// - If the given text is empty, then the [`ErrorKind::EmptyText`] will be returned.
    /// - If the given entity offset+length is out of bounds, then the [`ErrorKind::IndexOutOfBounds`] will be returned.
    fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, ErrorKind>
    where
        T: AsRef<str>;

    /// Apply the [`MessageEntityKind`] to the whole given text.
    /// # Notes
    /// Differences between [`Formatter::apply_entity`] and [`Formatter::apply_entity_kind`]:
    /// - [`Formatter::apply_entity`] will apply the [`MessageEntityKind`] to the given text with offset and length
    /// - [`Formatter::apply_entity_kind`] will apply the [`MessageEntityKind`] to the whole given text
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    /// # Errors
    /// - If the given text is empty, then the [`ErrorKind::EmptyText`] will be returned.
    /// - If the given entity offset+length is out of bounds, then the [`ErrorKind::IndexOutOfBounds`] will be returned.
    #[allow(clippy::cast_possible_truncation)]
    fn apply_entity_kind<T>(
        &self,
        text: T,
        entity_kind: MessageEntityKind,
    ) -> Result<String, ErrorKind>
    where
        T: AsRef<str>,
    {
        let text = text.as_ref();
        let text_len = text.len();

        if text_len == 0 {
            return Err(ErrorKind::EmptyText);
        }

        let offset = 0;
        let length = text_len as u16;

        self.apply_entity(text, &MessageEntity::new(offset, length, entity_kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFormatter;

    impl Formatter for TestFormatter {
        fn bold<T>(&self, text: T) -> String
        where
            T: AsRef<str>,
        {
            let text = text.as_ref();

            format!("**{text}**")
        }

        fn apply_entity<T>(&self, text: T, entity: &MessageEntity) -> Result<String, ErrorKind>
        where
            T: AsRef<str>,
        {
            let text = text.as_ref();
            let text_len = text.len();

            if text_len == 0 {
                return Err(ErrorKind::EmptyText);
            }

            let offset = entity.offset as usize;
            let length = entity.length as usize;

            if offset + length > text_len {
                return Err(ErrorKind::IndexOutOfBounds);
            }

            let editable_text = &text[offset..offset + length];

            let edited_text = match entity.kind() {
                MessageEntityKind::Bold => self.bold(editable_text),
                _ => unimplemented!(),
            };

            let mut text = text.to_owned();
            text.replace_range(offset..offset + length, &edited_text);

            Ok(text)
        }

        fn italic<T>(&self, _text: T) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }

        fn code<C>(&self, _code: C) -> String
        where
            C: AsRef<str>,
        {
            todo!()
        }

        fn underline<T>(&self, _text: T) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }

        fn strikethrough<T>(&self, _text: T) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }

        fn spoiler<T>(&self, _text: T) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }

        fn text_link<T, U>(&self, _text: T, _url: U) -> String
        where
            T: AsRef<str>,
            U: AsRef<str>,
        {
            todo!()
        }

        fn text_mention<T>(&self, _text: T, _user_id: i64) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }

        fn custom_emoji<T, E>(&self, _emoji: T, _emoji_id: E) -> String
        where
            T: AsRef<str>,
            E: AsRef<str>,
        {
            todo!()
        }

        fn pre<C>(&self, _code: C) -> String
        where
            C: AsRef<str>,
        {
            todo!()
        }

        fn pre_language<C, L>(&self, _code: C, _language: L) -> String
        where
            C: AsRef<str>,
            L: AsRef<str>,
        {
            todo!()
        }

        fn quote<T>(&self, _text: T) -> String
        where
            T: AsRef<str>,
        {
            todo!()
        }
    }

    #[test]
    fn test_apply_entity() {
        let formatter = TestFormatter;
        let text = "Hello, world!";
        let entity = MessageEntity::new(0, 5, MessageEntityKind::Bold);

        assert_eq!(
            formatter.apply_entity(text, &entity).unwrap(),
            "**Hello**, world!"
        );

        let entity = MessageEntity::new(7, 5, MessageEntityKind::Bold);

        assert_eq!(
            formatter.apply_entity(text, &entity).unwrap(),
            "Hello, **world**!"
        );

        let entity = MessageEntity::new(0, text.len() as u16, MessageEntityKind::Bold);

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
        let entity = MessageEntity::new(0, 15, MessageEntityKind::Bold);

        formatter.apply_entity(text, &entity).unwrap();

        let entity = MessageEntity::new(7, 9, MessageEntityKind::Bold);

        formatter.apply_entity(text, &entity).unwrap();

        let entity = MessageEntity::new(0, text.len() as u16 + 1, MessageEntityKind::Bold);

        formatter.apply_entity(text, &entity).unwrap();

        let text = "";

        formatter.apply_entity(text, &entity).unwrap();
    }

    #[test]
    fn test_apply_entity_kind() {
        let formatter = TestFormatter;
        let text = "Hello, world!";

        assert_eq!(
            formatter
                .apply_entity_kind(text, MessageEntityKind::Bold)
                .unwrap(),
            "**Hello, world!**"
        );
    }

    #[test]
    #[should_panic]
    fn test_apply_entity_kind_panic() {
        let formatter = TestFormatter;
        let text = "";

        formatter
            .apply_entity_kind(text, MessageEntityKind::Bold)
            .unwrap();
    }
}
