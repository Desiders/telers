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
/// Telegram requires 2-10 items per album; [`MediaGroupBuilder`] does not
/// enforce this on its own.
pub struct MediaGroupBuilder {
    media: Vec<InputMedia>,
}

impl MediaGroupBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            media: Vec::new(),
        }
    }

    /// Appends a photo.
    #[must_use]
    pub fn add_photo(mut self, media: impl Into<InputFile>) -> Self {
        self.media
            .push(InputMedia::Photo(InputMediaPhoto::new(media)));
        self
    }

    /// Appends a video.
    #[must_use]
    pub fn add_video(mut self, media: impl Into<InputFile>) -> Self {
        self.media
            .push(InputMedia::Video(InputMediaVideo::new(media)));
        self
    }

    /// Appends an audio file.
    #[must_use]
    pub fn add_audio(mut self, media: impl Into<InputFile>) -> Self {
        self.media
            .push(InputMedia::Audio(InputMediaAudio::new(media)));
        self
    }

    /// Appends a document.
    #[must_use]
    pub fn add_document(mut self, media: impl Into<InputFile>) -> Self {
        self.media
            .push(InputMedia::Document(InputMediaDocument::new(media)));
        self
    }

    /// Builds a [`SendMediaGroup`] for the given chat.
    #[must_use]
    pub fn build(self, chat_id: impl Into<ChatIdKind>) -> SendMediaGroup {
        SendMediaGroup::new(chat_id, self.media)
    }
}

impl Default for MediaGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_contains_added_media() {
        let method = MediaGroupBuilder::new()
            .add_photo(InputFile::id("photo1"))
            .add_video(InputFile::id("video1"))
            .build(42);

        assert_eq!(method.media.len(), 2);
    }

    #[test]
    fn build_sets_chat_id() {
        let method = MediaGroupBuilder::new()
            .add_photo(InputFile::id("photo1"))
            .build("@channel");

        assert!(matches!(method.chat_id, ChatIdKind::Username(_)));
    }
}
