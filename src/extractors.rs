//! This module contains functionality for extracting data from the event and context to the handler arguments.
//!
//! [`FromEventAndContext`] is the main trait to extracts data from the event and context to the handler arguments.
//!
//! The trait is implements for [`Option<T>`], [`Result<T, E>`], [`Box<T>`] and [`Pin<Box<T>>`] where `T: FromEventAndContext`,
//! also for the most common middlewares, types and filters (some of them creates structs, for example [`CommandObject`] by command filter).
//! You can use these as handler arguments.
//!
//! You can implement [`FromEventAndContext`] for your own types directly or using the [`from_context`] or [`from_context_into`] macro.
//! Check out the [`from_context`] and [`from_context_into`] macro documentation for more information.
//!
//! [`CommandObject`]: crate::filters::CommandObject
//! [`Pin<Box<T>>`]: std::pin::Pin

mod extractor;
mod filters;
mod from_context;
mod from_event;
mod middlewares;
mod types;

pub use crate::{from_context, from_context_into};
pub use crate::{from_update, try_from_update};
pub use extractor::FromEventAndContext;
