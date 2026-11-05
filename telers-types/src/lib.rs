//! Telegram Bot API objects, builders and serialization, independent of the bot framework.
//!
//! ```
//! use telers_types::{serialization, types::User};
//!
//! let user: User = serialization::from_str(r#"{"id": 1, "is_bot": false, "first_name": "Ada"}"#)?;
//! assert_eq!(user.first_name.as_ref(), "Ada");
//! # Ok::<(), telers_types::serialization::Error>(())
//! ```
//!
//! The default `serde` feature uses Serde. Enabling `deser` selects Deser instead.
//! File upload descriptors support buffered, file-system and streaming sources without
//! depending on an HTTP implementation. Routing, handler extraction and request shortcuts
//! are provided by the `telers` crate.

pub mod enums;
pub mod errors;
pub mod serialization;
pub mod types;
