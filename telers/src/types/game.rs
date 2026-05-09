use serde::{Deserialize, Serialize};
/// This object represents a game. Use `BotFather` to create and edit games, their short names will act as unique identifiers.
/// # Documentation
/// <https://core.telegram.org/bots/api#game>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: Box<str>,
    /// Description of the game
    pub description: Box<str>,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Box<[crate::types::PhotoSize]>,
    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Animation that will be displayed in the game message in chats. Upload via `BotFather`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<crate::types::Animation>>,
}
impl Game {
    /// Creates a new `Game`.
    ///
    /// # Arguments
    /// * `title` - Title of the game
    /// * `description` - Description of the game
    /// * `photo` - Photo that will be displayed in the game message in chats.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2Item: Into<crate::types::PhotoSize>,
        T2: IntoIterator<Item = T2Item>,
    >(
        title: T0,
        description: T1,
        photo: T2,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            photo: photo.into_iter().map(Into::into).collect(),
            text: None,
            text_entities: None,
            animation: None,
        }
    }

    /// Title of the game
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Description of the game
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }

    /// Photo that will be displayed in the game message in chats.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: T) -> Self {
        self.photo = self
            .photo
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Photo that will be displayed in the game message in chats.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.photo = self
            .photo
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = Some(val.into());
        self
    }

    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.text = val.map(Into::into);
        self
    }

    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.text_entities = val.map(Into::into);
        self
    }

    /// Animation that will be displayed in the game message in chats. Upload via `BotFather`
    #[must_use]
    pub fn animation<T: Into<crate::types::Animation>>(mut self, val: T) -> Self {
        self.animation = Some(Box::new(val.into()));
        self
    }

    /// Animation that will be displayed in the game message in chats. Upload via `BotFather`
    #[must_use]
    pub fn animation_option<T: Into<crate::types::Animation>>(mut self, val: Option<T>) -> Self {
        self.animation = val.map(|val| Box::new(val.into()));
        self
    }
}
