//! Enum helpers and discriminator types for Telegram objects.
//!
//! This module contains:
//! - generated `*Type` enums for polymorphic Telegram objects (for example `MessageType`)
//! - hand-authored enums like [`ParseMode`]
pub use telers_types_common::enums::*;
pub use telers_types_input::enums::*;
pub use telers_types_messages::enums::*;
pub mod chat_full_info;
pub mod update;
pub use chat_full_info::ChatFullInfoType;
pub use update::UpdateType;
