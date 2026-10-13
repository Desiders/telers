//! Message-side widgets that consume user replies inside a dialog window.
//!
//! Input widgets read the user's next [`Message`] (text, contact, location,
//! poll, etc.) and turn it into a [`ButtonAction`] — typically advancing the
//! dialog or writing a value into `dialog_data`. When a window declares multiple
//! inputs, the first one to claim the message wins.
//!
//! - [`MessageInput`]: typed wrapper that accepts any `MessageType: TryFrom<Message>`
//!   and forwards it to an async handler.
//! - [`TextInput`]: parses the message text into a target value via [`FromStr`]
//!   or a custom parser, writes the raw text into `widget_data`, and dispatches
//!   to success/error handlers.
//!
//! [`Message`]: telers::types::Message
//! [`ButtonAction`]: crate::widgets::ButtonAction
//! [`FromStr`]: std::str::FromStr

mod base;
mod message;
mod text;

#[cfg(test)]
mod tests;

pub(crate) use base::MultiInput;

pub use base::{CombinedInput, FilteredInput, Input, InputFilter, InputFilterContext};
pub use message::{MessageInput, MessageInputContext};
pub use text::{TextInput, TextInputContext};
