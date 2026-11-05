//! Telegram Bot API methods and request preparation without a bot framework or HTTP client.
//!
//! ```
//! use telers_methods::{SendMessage, TelegramMethod as _};
//!
//! let request = SendMessage::new(1_i64, "Hello").build_request();
//! assert_eq!(request.method_name, "sendMessage");
//! assert!(request.files.is_none());
//! ```
//!
//! Transport implementations can use the prepared parameters and attachments, then parse
//! the API response through [`TelegramMethod::build_response`]. Serde is the default backend;
//! enabling `deser` selects Deser. Select the same backend across all `telers` crates.

pub mod either;
pub mod methods;
pub mod serialization;

pub use either::Either;
pub use methods::{non_telegram::base, *};
/// Data objects shared with the standalone types crate and framework facade.
pub use telers_types::{enums, types};
