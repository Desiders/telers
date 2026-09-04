//! This module contains the [`FileIdGetter`] trait, which is implemented for objects that have a `file_id`,
//! so they can be passed to [`Bot::download`](crate::Bot::download).

/// Getter of the file ID, which can be used to download or reuse the file.
///
/// It's implemented for a plain file ID (`&str`, [`String`], `Box<str>`)
/// and generated for every Telegram object that has a `file_id` field, for example [`PhotoSize`], [`Document`]
/// or [`Sticker`], as well as for references and [`Box`] of them,
/// so any of them can be passed to [`Bot::download`](crate::Bot::download).
///
/// [`PhotoSize`]: crate::types::PhotoSize
/// [`Document`]: crate::types::Document
/// [`Sticker`]: crate::types::Sticker
pub trait FileIdGetter {
    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    fn file_id(&self) -> &str;
}

impl<T: FileIdGetter + ?Sized> FileIdGetter for &T {
    fn file_id(&self) -> &str {
        T::file_id(self)
    }
}

impl<T: FileIdGetter + ?Sized> FileIdGetter for Box<T> {
    fn file_id(&self) -> &str {
        T::file_id(self)
    }
}

impl FileIdGetter for str {
    fn file_id(&self) -> &str {
        self
    }
}

impl FileIdGetter for String {
    fn file_id(&self) -> &str {
        self
    }
}
