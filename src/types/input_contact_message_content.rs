use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a contact message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputcontactmessagecontent>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a `vCard <https://en.wikipedia.org/wiki/VCard>`, 0-2048 bytes
    pub vcard: Option<String>,
}
