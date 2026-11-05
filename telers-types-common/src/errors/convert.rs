//! This module contains the error [`ConvertToType`] that can occur when type be converted to the desired type.
//! For example, if you try to convert an [`Update`] to a [`Message`] type, but the update represents a [`CallbackQuery`], you got this error.
//!
//!
//! [`Update`]: https://docs.rs/telers-types/latest/telers_types/types/update/enum.Update.html
//! [`Message`]: https://docs.rs/telers-types-messages/latest/telers_types_messages/types/message/enum.Message.html
//! [`CallbackQuery`]: https://docs.rs/telers-types/latest/telers_types/types/callback_query/struct.CallbackQuery.html

/// This error can occur when type be converted to the desired type.
/// For example, if you try to convert an [`Update`](https://docs.rs/telers-types/latest/telers_types/types/update/enum.Update.html) to a [`Message`](https://docs.rs/telers-types-messages/latest/telers_types_messages/types/message/enum.Message.html) type,
/// but the update represents a [`CallbackQuery`](https://docs.rs/telers-types/latest/telers_types/types/callback_query/struct.CallbackQuery.html), you got this error.
#[derive(Debug, thiserror::Error)]
#[error("Can't convert from `{from_raw_type}` to `{to_raw_type}`")]
pub struct ConvertToType {
    from_raw_type: &'static str,
    to_raw_type: &'static str,
}

impl ConvertToType {
    /// # Arguments
    /// * `from_raw_type` - The type from which the conversion is performed.
    /// * `to_raw_type` - The type to which the conversion is performed.
    #[must_use]
    pub const fn new(from_raw_type: &'static str, to_raw_type: &'static str) -> Self {
        Self {
            from_raw_type,
            to_raw_type,
        }
    }
}
