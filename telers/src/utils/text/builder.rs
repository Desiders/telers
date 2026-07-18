use std::fmt::Display;

use super::{Formatter, FormatterErrorKind};

use crate::types::{
    MessageEntity, MessageEntityBlockquote, MessageEntityBold, MessageEntityBotCommand,
    MessageEntityCashtag, MessageEntityCode, MessageEntityCustomEmoji, MessageEntityEmail,
    MessageEntityExpandableBlockquote, MessageEntityHashtag, MessageEntityItalic,
    MessageEntityMention, MessageEntityPhoneNumber, MessageEntityPre, MessageEntitySpoiler,
    MessageEntityStrikethrough, MessageEntityTextLink, MessageEntityTextMention,
    MessageEntityUnderline, MessageEntityUrl, User,
};

use tracing::{event, Level};

#[derive(Debug, Default)]
pub struct Builder<F> {
    formatter: F,
    text: String,
}

#[allow(clippy::cast_possible_truncation)]
fn utf16_len(text: &str) -> u16 {
    text.encode_utf16().count() as u16
}

impl<F> Builder<F>
where
    F: Formatter,
{
    #[inline]
    #[must_use]
    pub const fn new(formatter: F) -> Self {
        Self {
            formatter,
            text: String::new(),
        }
    }

    /// Add text without formatting.
    #[must_use]
    pub fn text(mut self, text: impl Display) -> Self {
        self.text.push_str(text.to_string().as_ref());
        self
    }

    /// Add texts without formatting.
    #[must_use]
    pub fn texts<T, I>(mut self, texts: I) -> Self
    where
        String: Extend<T>,
        I: IntoIterator<Item = T>,
        T: Display,
    {
        self.text.extend(texts);
        self
    }

    /// Add quote text without formatting.
    #[must_use]
    pub fn quote(mut self, text: impl Display) -> Self {
        self.text.push_str(self.formatter.quote(text).as_str());
        self
    }

    /// Add quote texts without formatting.
    #[must_use]
    pub fn quotes<T, I>(mut self, texts: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Display,
    {
        self.text.extend(
            texts
                .into_iter()
                .map(|text| self.formatter.quote(text))
                .collect::<Box<[_]>>()
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
    pub fn mention(self, username: impl Into<Box<str>>) -> Self {
        let mention = format!("@{}", username.into());
        let entity = MessageEntity::Mention(MessageEntityMention::new(
            utf16_len(&self.text),
            utf16_len(&mention),
        ));

        self.text(mention)
            .entity(&entity)
            .expect("Failed to add mention. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn hashtag(self, tag: impl Into<Box<str>>) -> Self {
        let hashtag = format!("#{}", tag.into());
        let entity = MessageEntity::Hashtag(MessageEntityHashtag::new(
            utf16_len(&self.text),
            utf16_len(&hashtag),
        ));

        self.text(hashtag)
            .entity(&entity)
            .expect("Failed to add hashtag. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn cashtag(self, tag: impl Into<Box<str>>) -> Self {
        let cashtag = format!("${}", tag.into());
        let entity = MessageEntity::Cashtag(MessageEntityCashtag::new(
            utf16_len(&self.text),
            utf16_len(&cashtag),
        ));

        self.text(cashtag)
            .entity(&entity)
            .expect("Failed to add cashtag. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn bot_command(self, command: impl Into<Box<str>>) -> Self {
        let bot_command = format!("/{}", command.into());
        let entity = MessageEntity::BotCommand(MessageEntityBotCommand::new(
            utf16_len(&self.text),
            utf16_len(&bot_command),
        ));

        self.text(bot_command)
            .entity(&entity)
            .expect("Failed to add bot command. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn url(self, url: impl Into<Box<str>>) -> Self {
        let url = url.into();
        let entity = MessageEntity::Url(MessageEntityUrl::new(
            utf16_len(&self.text),
            utf16_len(&url),
        ));

        self.text(url)
            .entity(&entity)
            .expect("Failed to add URL. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn email(self, email: impl Into<Box<str>>) -> Self {
        let email = email.into();
        let entity = MessageEntity::Email(MessageEntityEmail::new(
            utf16_len(&self.text),
            utf16_len(&email),
        ));

        self.text(email)
            .entity(&entity)
            .expect("Failed to add email. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn phone_number(self, phone_number: impl Into<Box<str>>) -> Self {
        let phone_number = phone_number.into();
        let entity = MessageEntity::PhoneNumber(MessageEntityPhoneNumber::new(
            utf16_len(&self.text),
            utf16_len(&phone_number),
        ));

        self.text(phone_number)
            .entity(&entity)
            .expect("Failed to add phone number. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn bold(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Bold(MessageEntityBold::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add bold. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn italic(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Italic(MessageEntityItalic::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add italic. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn underline(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Underline(MessageEntityUnderline::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add underline. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn strikethrough(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Strikethrough(MessageEntityStrikethrough::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add strikethrough. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn spoiler(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Spoiler(MessageEntitySpoiler::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add spoiler. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn blockquote(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::Blockquote(MessageEntityBlockquote::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add blockquote. Report this issue to the developers")
    }

    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn expandable_blockquote(self, text: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::ExpandableBlockquote(MessageEntityExpandableBlockquote::new(
            utf16_len(&self.text),
            utf16_len(&text),
        ));

        self.text(text)
            .entity(&entity)
            .expect("Failed to add expandable blockquote. Report this issue to the developers")
    }

    /// Add code as monowidth string.
    /// # Arguments
    /// * `code` - Code that will be added as monowidth string.
    /// # Notes
    /// If you want to use monowidth block, then use `pre` or `pre_language` method instead.
    /// # Warning
    /// If the given text length is greater than [`u16::MAX`], then the text will be truncated.
    #[must_use]
    pub fn code(self, code: impl Into<Box<str>>) -> Self {
        let code = code.into();
        let entity = MessageEntity::Code(MessageEntityCode::new(
            utf16_len(&self.text),
            utf16_len(&code),
        ));

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
    pub fn monowidth(self, text: impl Into<Box<str>>) -> Self {
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
    pub fn pre(self, code: impl Into<Box<str>>) -> Self {
        let code = code.into();
        let entity = MessageEntity::Pre(MessageEntityPre::new(
            utf16_len(&self.text),
            utf16_len(&code),
        ));

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
    pub fn pre_language(self, code: impl Into<Box<str>>, language: impl Into<Box<str>>) -> Self {
        let code = code.into();
        let entity = MessageEntity::Pre(
            MessageEntityPre::new(utf16_len(&self.text), utf16_len(&code)).language(language),
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
    pub fn text_link(self, text: impl Into<Box<str>>, url: impl Into<Box<str>>) -> Self {
        let text = text.into();
        let entity = MessageEntity::TextLink(MessageEntityTextLink::new(
            utf16_len(&self.text),
            utf16_len(&text),
            url,
        ));

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
    pub fn text_mention(self, text: impl Into<Box<str>>, user: User) -> Self {
        let text = text.into();
        let entity = MessageEntity::TextMention(MessageEntityTextMention::new(
            utf16_len(&self.text),
            utf16_len(&text),
            user,
        ));

        self.text(text).entity(&entity).expect(
            "Failed to add mention for the user without username. Report this issue to the \
             developers",
        )
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
    pub fn custom_emoji(
        self,
        emoji: impl Into<Box<str>>,
        custom_emoji_id: impl Into<Box<str>>,
    ) -> Self {
        let emoji = emoji.into();
        let entity = MessageEntity::CustomEmoji(MessageEntityCustomEmoji::new(
            utf16_len(&self.text),
            utf16_len(&emoji),
            custom_emoji_id,
        ));

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
            .text_mention("text_mention", User::new(0, true, ""))
            .text(" ")
            .custom_emoji("custom_emoji", "emoji_id")
            .text(" ")
            .blockquote("blockquote")
            .text(" ")
            .expandable_blockquote("expandable_blockquote");

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
            <tg-emoji emoji-id=\"emoji_id\">custom_emoji</tg-emoji> \
            <blockquote>blockquote</blockquote> \
            <blockquote expandable>expandable_blockquote</blockquote>\
            "
        );
    }
}
