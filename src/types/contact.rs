use serde::Deserialize;

/// This object represents a phone contact.
/// <https://core.telegram.org/bots/api#contact>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a [`vCard`](https://en.wikipedia.org/wiki/VCard)
    pub vcard: Option<String>,
}
