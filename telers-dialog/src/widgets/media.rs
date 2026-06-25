//! Media widgets for rendering media content in dialogs.
//!
//! This module provides:
//! - [`Media`] trait for rendering media content
//! - [`StaticMedia`] for static media (URL or file ID)
//! - [`DynamicMedia`] for dynamic media from render data
//! - [`MediaScroll`] for paginated media with scroll support
//! - [`MediaAttachment`] for representing rendered media

mod base;
mod dynamic;
mod scroll;
mod r#static;
mod storage;

pub use base::{Media, MediaAttachment, MediaContentType, MediaId, MultiMedia};
pub use dynamic::DynamicMedia;
pub use r#static::StaticMedia;
pub use scroll::MediaScroll;
pub use storage::{InMemoryMediaIdStorage, MediaIdStorage};

#[cfg(test)]
mod tests;
