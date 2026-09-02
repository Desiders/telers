//! Builder for media groups (albums).

use crate::{
    methods::SendMediaGroup,
    types::{
        ChatIdKind, InputFile, InputMedia, InputMediaAudio, InputMediaDocument, InputMediaPhoto,
        InputMediaVideo,
    },
};

/// Accumulates media items and builds a [`SendMediaGroup`].
///
/// Telegram requires 2-10 items per album; [`MediaGroupBuilder`] panics when
/// [`Self::MAX_SIZE`] is exceeded, the minimum is up to the Telegram API.
pub struct MediaGroupBuilder {
    chat_id: ChatIdKind,
    media: Vec<InputMedia>,
}

impl MediaGroupBuilder {
    /// Maximum number of media items per album.
    pub const MAX_SIZE: usize = 10;

    /// Creates a builder for the given chat.
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
            media: Vec::new(),
        }
    }

    fn assert_can_add(&self) {
        assert!(
            self.media.len() < Self::MAX_SIZE,
            "cannot add more than {} media items",
            Self::MAX_SIZE
        );
    }

    /// Appends a photo.
    ///
    /// # Panics
    ///
    /// Panics if the album already has [`Self::MAX_SIZE`] media items.
    #[must_use]
    pub fn add_photo(mut self, media: impl Into<InputFile>) -> Self {
        self.assert_can_add();
        self.media
            .push(InputMedia::Photo(InputMediaPhoto::new(media)));
        self
    }

    /// Appends a video.
    ///
    /// # Panics
    ///
    /// Panics if the album already has [`Self::MAX_SIZE`] media items.
    #[must_use]
    pub fn add_video(mut self, media: impl Into<InputFile>) -> Self {
        self.assert_can_add();
        self.media
            .push(InputMedia::Video(InputMediaVideo::new(media)));
        self
    }

    /// Appends an audio file.
    ///
    /// # Panics
    ///
    /// Panics if the album already has [`Self::MAX_SIZE`] media items.
    #[must_use]
    pub fn add_audio(mut self, media: impl Into<InputFile>) -> Self {
        self.assert_can_add();
        self.media
            .push(InputMedia::Audio(InputMediaAudio::new(media)));
        self
    }

    /// Appends a document.
    ///
    /// # Panics
    ///
    /// Panics if the album already has [`Self::MAX_SIZE`] media items.
    #[must_use]
    pub fn add_document(mut self, media: impl Into<InputFile>) -> Self {
        self.assert_can_add();
        self.media
            .push(InputMedia::Document(InputMediaDocument::new(media)));
        self
    }

    /// Builds a [`SendMediaGroup`].
    #[must_use]
    pub fn build(self) -> SendMediaGroup {
        SendMediaGroup::new(self.chat_id, self.media)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_contains_added_media() {
        let method = MediaGroupBuilder::new(42)
            .add_photo(InputFile::id("photo1"))
            .add_video(InputFile::id("video1"))
            .build();

        assert_eq!(method.media.len(), 2);
    }

    #[test]
    fn build_sets_chat_id() {
        let method = MediaGroupBuilder::new("@channel")
            .add_photo(InputFile::id("photo1"))
            .build();

        assert!(matches!(method.chat_id, ChatIdKind::Username(_)));
    }

    #[test]
    #[should_panic]
    fn add_panics_beyond_max() {
        let mut builder = MediaGroupBuilder::new(42);
        for _ in 0..MediaGroupBuilder::MAX_SIZE {
            builder = builder.add_photo(InputFile::id("photo"));
        }
        let _ = builder.add_photo(InputFile::id("photo"));
    }
}
