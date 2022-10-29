use crate::types::MessageEntity;

#[must_use]
pub fn add_surrogates(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[must_use]
pub fn remove_surrogates(text: &[u16]) -> String {
    String::from_utf16_lossy(text)
}

#[allow(clippy::module_name_repetitions)]
pub trait TextDecoration {
    /// Decorate text by [`MessageEntity`]
    fn apply_entity(&self, entity: &MessageEntity, text: &str) -> String {
        match entity.entity_type.as_str() {
            "mention" | "hashtag" | "cashtag" | "bot_command" => text.to_string(),
            "bold" => self.bold(text),
            "italic" => self.italic(text),
            "underline" => self.underline(text),
            "strikethrough" => self.strikethrough(text),
            "spoiler" => self.spoiler(text),
            "pre" => {
                if let Some(language) = &entity.language {
                    self.pre_language(text, language)
                } else {
                    self.pre(text)
                }
            }
            "text_mention" => self.link(
                text,
                &format!(
                    "tg://user?id={user_id}",
                    user_id = entity.user.as_ref().unwrap().id
                ),
            ),
            "text_link" => self.link(text, entity.url.as_ref().unwrap()),
            "custom_emoji" => self.custom_emoji(text, entity.custom_emoji_id.as_ref().unwrap()),
            _ => self.quote(text),
        }
    }

    /// Decorate text with `bold` tag
    fn bold(&self, text: &str) -> String;

    /// Decorate text with `italic` tag
    fn italic(&self, text: &str) -> String;

    /// Decorate text with `code` tag
    fn code(&self, text: &str) -> String;

    /// Decorate text with `underline` tag
    fn underline(&self, text: &str) -> String;

    /// Decorate text with `strikethrough` tag
    fn strikethrough(&self, text: &str) -> String;

    /// Decorate text with `spoiler` tag
    fn spoiler(&self, text: &str) -> String;

    /// Decorate text with `pre` tag
    fn pre(&self, text: &str) -> String;

    /// Decorate text with `pre_language` tag
    fn pre_language(&self, text: &str, language: &str) -> String;

    /// Decorate text with `link` tag
    fn link(&self, text: &str, url: &str) -> String;

    /// Decorate text with `custom_emoji` tag
    fn custom_emoji(&self, text: &str, emoji_id: &str) -> String;

    /// Quote symbols
    fn quote(&self, text: &str) -> String;
}
