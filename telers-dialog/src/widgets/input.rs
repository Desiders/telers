mod base;
mod message;
mod text;

#[cfg(test)]
mod tests;

pub(crate) use base::MultiInput;

pub use base::Input;
pub use message::MessageInput;
pub use text::TextInput;
