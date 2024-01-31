use super::{Formatter, FormatterErrorKind};

use crate::types::{MessageEntity, User};

use tracing::{event, Level};

#[derive(Debug, Default)]
pub struct Builder<F> {
    formatter: F,
    text: String,
}

#[allow(clippy::cast_possible_truncation, clippy::missing_panics_doc)]
impl<F> Builder<F>
where
    F: Formatter,
{
    #[must_use]
    pub const fn new(formatter: F) -> Self {
        Self {
            formatter,
            text: String::new(),
        }
    }

    /// Add text without formatting.
    #[must_use]
    pub fn text(mut self, text: impl AsRef<str>) -> Self {
        self.text.push_str(text.as_ref());
        self
    }

    /// Add texts without formatting.
    #[must_use]
    pub fn texts<T, I>(mut self, texts: I) -> Self
    where
        I: AsRef<[T]>,
        T: AsRef<str>,
    {
        self.text.extend(texts.as_ref().iter().map(AsRef::as_ref));
        self
    }

    /// Add quote text without formatting.
    #[must_use]
    pub fn quote(mut self, text: impl AsRef<str>) -> Self {
        self.text.push_str(self.formatter.quote(text).as_str());
        self
    }

    /// Add quote texts without formatting.
    #[must_use]
    pub fn quotes<T, I>(mut self, texts: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        self.text.extend(
            texts
                .into_iter()
                .map(|text| self.formatter.quote(text))
                .collect::<Vec<_>>()
                .iter()
                .map(String::as_str),
        );
        self
    }

    /// Add entity to the builder.
    /// # Arguments
    /// * `entity` - Entity that will be added to the builder.
    /// # Notes
    /// You can use this method if you want to add entity that is not supported by this builder
    /// # Errors
    /// - If the given text is empty, then the [`FormatterErrorKind::EmptyText`] will be returned.
    /// - If the given entity offset+length is out of bounds, then the [`FormatterErrorKind::IndexOutOfBounds`] will be returned.
    pub fn entity(mut self, entity: &MessageEntity) -> Result<Self, FormatterErrorKind> {
        event!(
            Level::TRACE,
            text = self.text,
            ?entity,
            "Add entity for the text"
        );

        self.formatter
            .apply_entity(self.text.as_str(), entity)
            .map(|text| {
                self.text = text;
                self
            })
    }

    /// Add mention by username.
    /// # Arguments
    /// * `username` - Username which will be mentioned.
    /// # Notes
    /// If you want to mention user without username, then use `text_mention` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn mention(self, username: impl AsRef<str>) -> Self {
        let username = username.as_ref();
        let entity = MessageEntity::new_mention(self.text.len() as u16, username.len() as u16);

        self.text(username)
            .entity(&entity)
            .expect("Failed to add mention. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn hashtag(self, tag: impl AsRef<str>) -> Self {
        let tag = tag.as_ref();
        let entity = MessageEntity::new_hashtag(self.text.len() as u16, tag.len() as u16);

        self.text(tag)
            .entity(&entity)
            .expect("Failed to add hashtag. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn cashtag(self, tag: impl AsRef<str>) -> Self {
        let tag = tag.as_ref();
        let entity = MessageEntity::new_cashtag(self.text.len() as u16, tag.len() as u16);

        self.text(tag)
            .entity(&entity)
            .expect("Failed to add cashtag. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn bot_command(self, command: impl AsRef<str>) -> Self {
        let command = command.as_ref();
        let entity = MessageEntity::new_bot_command(self.text.len() as u16, command.len() as u16);

        self.text(command)
            .entity(&entity)
            .expect("Failed to add bot command. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn url(self, url: impl AsRef<str>) -> Self {
        let url = url.as_ref();
        let entity = MessageEntity::new_url(self.text.len() as u16, url.len() as u16);

        self.text(url)
            .entity(&entity)
            .expect("Failed to add URL. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn email(self, email: impl AsRef<str>) -> Self {
        let email = email.as_ref();
        let entity = MessageEntity::new_email(self.text.len() as u16, email.len() as u16);

        self.text(email)
            .entity(&entity)
            .expect("Failed to add email. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn phone_number(self, phone_number: impl AsRef<str>) -> Self {
        let phone_number = phone_number.as_ref();
        let entity =
            MessageEntity::new_phone_number(self.text.len() as u16, phone_number.len() as u16);

        self.text(phone_number)
            .entity(&entity)
            .expect("Failed to add phone number. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn bold(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity = MessageEntity::new_bold(self.text.len() as u16, text.len() as u16);

        self.text(text)
            .entity(&entity)
            .expect("Failed to add bold. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn italic(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity = MessageEntity::new_italic(self.text.len() as u16, text.len() as u16);

        self.text(text)
            .entity(&entity)
            .expect("Failed to add italic. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn underline(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity = MessageEntity::new_underline(self.text.len() as u16, text.len() as u16);

        self.text(text)
            .entity(&entity)
            .expect("Failed to add underline. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn strikethrough(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity = MessageEntity::new_strikethrough(self.text.len() as u16, text.len() as u16);

        self.text(text)
            .entity(&entity)
            .expect("Failed to add strikethrough. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn spoiler(self, text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity = MessageEntity::new_spoiler(self.text.len() as u16, text.len() as u16);

        self.text(text)
            .entity(&entity)
            .expect("Failed to add spoiler. Report this issue to the developers")
    }

    /// Add code as monowidth string.
    /// # Arguments
    /// * `code` - Code that will be added as monowidth string.
    /// # Notes
    /// If you want to use monowidth block, then use `pre` or `pre_language` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn code(self, code: impl AsRef<str>) -> Self {
        let code = code.as_ref();
        let entity = MessageEntity::new_code(self.text.len() as u16, code.len() as u16);

        self.text(code)
            .entity(&entity)
            .expect("Failed to add code. Report this issue to the developers")
    }

    /// Add text as monowidth string.
    /// # Arguments
    /// * `text` - Text that will be added as monowidth string.
    /// # Notes
    /// If you want to use monowidth block, then use `pre` or `pre_language` method instead.
    ///
    /// This method is shorthand for `code` method. Using this method is the same as using `code` method,
    /// but it's more readable for `text` than `code`.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn monowidth(self, text: impl AsRef<str>) -> Self {
        self.code(text)
    }

    /// Add code to the monowidth block.
    /// # Arguments
    /// * `code` - Code that will be added to the monowidth block.
    /// # Notes
    /// If you want to highlight code with programming language, then use `pre_language` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn pre(self, code: impl AsRef<str>) -> Self {
        let code = code.as_ref();
        let entity = MessageEntity::new_pre(self.text.len() as u16, code.len() as u16);

        self.text(code)
            .entity(&entity)
            .expect("Failed to add pre. Report this issue to the developers")
    }

    /// Add code with programming language to the monowidth block and highlight it.
    /// # Arguments
    /// * `code` - Code that will be added to the monowidth block and will be highlighted.
    /// * `language` - Programming language that will be used to highlight the text.
    /// # Notes
    /// If you want to highlight code without programming language, then use `pre` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn pre_language(self, code: impl AsRef<str>, language: impl AsRef<str>) -> Self {
        let code = code.as_ref();
        let entity = MessageEntity::new_pre_language(
            self.text.len() as u16,
            code.len() as u16,
            language.as_ref(),
        );

        self.text(code).entity(&entity).expect(
            "Failed to add pre with programming language. Report this issue to the developers",
        )
    }

    /// Add clickable text link.
    /// # Arguments
    /// * `text` - Text that will be replaced with clickable text link.
    /// * `url` - URL that will be opened after user clicks on the text link.
    /// # Notes
    /// If you want to use link without text, then use `url` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn text_link(self, text: impl AsRef<str>, url: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let entity =
            MessageEntity::new_text_link(self.text.len() as u16, text.len() as u16, url.as_ref());

        self.text(text)
            .entity(&entity)
            .expect("Failed to add clickable text link. Report this issue to the developers")
    }

    /// Add mention for the user without username to the text.
    /// # Arguments
    /// * `text` - Text that will be added to the text and will be replaced with mention.
    /// * `user` - User that will be mentioned.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn text_mention(self, text: impl AsRef<str>, user: User) -> Self {
        let text = text.as_ref();
        let entity =
            MessageEntity::new_text_mention(self.text.len() as u16, text.len() as u16, user);

        self.text(text).entity(&entity).expect(
            "Failed to add mention for the user without username. Report this issue to the developers",)
    }

    /// Add custom emoji to the text instead of unicode emoji.
    /// # Arguments
    /// * `emoji` - Emoji that will be added to the text and will be replaced with custom emoji.
    /// * `custom_emoji_id` - ID of the custom emoji.
    /// # Notes
    /// If user doesn't have custom emoji (premium feature), then unicode emoji will be used instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn custom_emoji(self, emoji: impl AsRef<str>, custom_emoji_id: impl AsRef<str>) -> Self {
        let emoji = emoji.as_ref();
        let entity = MessageEntity::new_custom_emoji(
            self.text.len() as u16,
            emoji.len() as u16,
            custom_emoji_id.as_ref(),
        );

        self.text(emoji)
            .entity(&entity)
            .expect("Failed to add custom emoji. Report this issue to the developers")
    }

    /// Get formatted text.
    #[must_use]
    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::text::HTMLFormatter;

    #[test]
    fn test_text_builder() {
        let builder = Builder::new(HTMLFormatter::default())
            .text("Hello, ")
            .bold("world")
            .text("!")
            .texts(["\n", "How are you?"])
            .text(" ")
            .italic("I'm fine")
            .bold("!");

        assert_eq!(
            builder.get_text(),
            "Hello, <b>world</b>!\nHow are you? <i>I'm fine</i><b>!</b>"
        );

        let builder = Builder::new(HTMLFormatter::default())
            .mention("username")
            .text(" ")
            .hashtag("hashtag")
            .text(" ")
            .cashtag("cashtag")
            .text(" ")
            .bot_command("command")
            .text(" ")
            .url("https://example.com")
            .text(" ")
            .email("test@mail.pu")
            .text(" ")
            .phone_number("+1234567890")
            .text(" ")
            .bold("bold")
            .text(" ")
            .italic("italic")
            .text(" ")
            .underline("underline")
            .text(" ")
            .strikethrough("strikethrough")
            .text(" ")
            .spoiler("spoiler")
            .text(" ")
            .code("code")
            .text(" ")
            .pre("pre")
            .text(" ")
            .pre_language("pre_language", "python")
            .text(" ")
            .text_link("text_link", "https://example.com")
            .text(" ")
            .text_mention("text_mention", User::default())
            .text(" ")
            .custom_emoji("custom_emoji", "emoji_id");

        assert_eq!(
            builder.get_text(),
            "@username #hashtag $cashtag /command https://example.com test@mail.pu \
            +1234567890 <b>bold</b> <i>italic</i> <u>underline</u> <s>strikethrough</s> \
            <tg-spoiler>spoiler</tg-spoiler> \
            <code>code</code> \
            <pre>pre</pre> \
            <pre><code class=\"language-python\">pre_language</code></pre> \
            <a href=\"https://example.com\">text_link</a> \
            <a href=\"tg://user?id=0\">text_mention</a> \
            <tg-emoji data-emoji-id=\"emoji_id\">custom_emoji</tg-emoji>\
            "
        );
    }
}
