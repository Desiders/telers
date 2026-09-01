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
/// Telegram requires 2-10 items per album; [`MediaGroupBuilder`] caps additions
/// at [`Self::MAX_SIZE`], the minimum is up to the Telegram API.
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

    /// Appends a photo.
    #[must_use]
    pub fn add_photo(mut self, media: impl Into<InputFile>) -> Self {
        if self.media.len() == Self::MAX_SIZE {
            return self;
        }
        self.media
            .push(InputMedia::Photo(InputMediaPhoto::new(media)));
        self
    }

    /// Appends a video.
    #[must_use]
    pub fn add_video(mut self, media: impl Into<InputFile>) -> Self {
        if self.media.len() == Self::MAX_SIZE {
            return self;
        }
        self.media
            .push(InputMedia::Video(InputMediaVideo::new(media)));
        self
    }

    /// Appends an audio file.
    #[must_use]
    pub fn add_audio(mut self, media: impl Into<InputFile>) -> Self {
        if self.media.len() == Self::MAX_SIZE {
            return self;
        }
        self.media
            .push(InputMedia::Audio(InputMediaAudio::new(media)));
        self
    }

    /// Appends a document.
    #[must_use]
    pub fn add_document(mut self, media: impl Into<InputFile>) -> Self {
        if self.media.len() == Self::MAX_SIZE {
            return self;
        }
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
    fn add_ignores_media_beyond_max() {
        let mut builder = MediaGroupBuilder::new(42);
        for i in 0..MediaGroupBuilder::MAX_SIZE + 1 {
            builder = builder.add_photo(InputFile::id(format!("photo{i}")));
        }
        assert_eq!(builder.build().media.len(), MediaGroupBuilder::MAX_SIZE);
    }
}
