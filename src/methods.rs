pub mod base;
pub mod get_updates;
pub mod send_photo;

pub use base::{Request, Response, TelegramMethod};
pub use get_updates::GetUpdates;
pub use send_photo::SendPhoto;
