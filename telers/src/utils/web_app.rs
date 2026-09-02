//! Validation of `WebApp` init data and login widget signatures.
//!
//! Implements the `HMAC-SHA256` checks `Telegram` documents for
//! [Web Apps](https://core.telegram.org/bots/webapps#validating-data-received-via-the-mini-app)
//! and the [login widget](https://core.telegram.org/widgets/login#checking-authorization).

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use std::fmt::Write as _;

/// Validates the signature of `WebApp` init data.
///
/// The check passes only if the `hash` parameter is present and matches the
/// `HMAC-SHA256` of the data computed with the bot token.
///
/// Per the `Telegram` algorithm, the data check string is every parameter
/// except `hash`, sorted by key and joined with a line feed, so the parameter
/// order in `init_data` does not matter.
///
/// # Examples
/// ```rust
/// use telers::utils::check_webapp_signature;
///
/// assert!(check_webapp_signature(
///     "123456:ABC",
///     "query_id=abc&user=%7B%22id%22%3A42%7D&auth_date=100&\
///      hash=9778490b4477d9ccfcf47b4041c9b5c65bcdced53b105a5a3e1ca39e57e1f06a"
/// ));
/// ```
#[must_use]
pub fn check_webapp_signature(bot_token: &str, init_data: &str) -> bool {
    let mut data_check_string = Vec::new();
    let mut hash_value = None;

    for pair in init_data.split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        if key == "hash" {
            hash_value = Some(value);
        } else {
            data_check_string.push((key, value));
        }
    }

    let Some(hash_value) = hash_value else {
        return false;
    };
    data_check_string.sort_unstable();

    let data_check_string = data_check_string
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("\n");

    let secret_key = hmac_sha256(b"WebAppData", bot_token.as_bytes());

    signature_matches(&secret_key, data_check_string.as_bytes(), hash_value)
}

/// Validates the signature of `WebApp` init data and parses it.
///
/// # Errors
/// Returns an error if the signature is invalid or the data cannot be parsed.
///
/// # Examples
/// ```rust
/// use telers::utils::safe_parse_webapp_init_data;
///
/// let init_data = "query_id=abc&user=%7B%22id%22%3A42%7D&auth_date=100&\
///                  hash=9778490b4477d9ccfcf47b4041c9b5c65bcdced53b105a5a3e1ca39e57e1f06a";
/// let parsed = safe_parse_webapp_init_data("123456:ABC", init_data).unwrap();
/// assert_eq!(parsed.query_id.as_deref(), Some("abc"));
/// ```
pub fn safe_parse_webapp_init_data(
    bot_token: &str,
    init_data: &str,
) -> Result<WebAppInitData, WebAppValidationError> {
    if !check_webapp_signature(bot_token, init_data) {
        return Err(WebAppValidationError::InvalidSignature);
    }

    let mut parsed = WebAppInitData::default();
    for pair in init_data.split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        let value = percent_decode(value);
        match key {
            "query_id" => parsed.query_id = Some(value),
            "user" => parsed.user = Some(serde_json::from_str(&value)?),
            "receiver" => parsed.receiver = Some(serde_json::from_str(&value)?),
            "chat_type" => parsed.chat_type = Some(value),
            "chat_instance" => parsed.chat_instance = Some(value),
            "start_param" => parsed.start_param = Some(value),
            "can_send_after" => parsed.can_send_after = value.parse().ok(),
            "auth_date" => parsed.auth_date = value.parse().ok(),
            "hash" => parsed.hash = Some(value),
            _ => {}
        }
    }
    Ok(parsed)
}

/// Validates the signature of a login widget authorization.
///
/// `fields` are the `key=value` pairs from the widget callback, e.g.
/// `("auth_date", "1662771648")`. They are sorted by key and joined with
/// newlines before the `HMAC-SHA256` check.
///
/// # Examples
/// ```rust
/// use telers::utils::check_signature;
///
/// let fields = [
///     ("user_name", "Rogue"),
///     ("first_name", "Andrew"),
///     ("auth_date", "1662771648"),
/// ];
/// assert!(check_signature(
///     "123456:ABC",
///     &fields,
///     "6f59af49c462cb5db5f52f4d51e9dae40c09c562edeb424c823690a88dddc940"
/// ));
/// ```
#[must_use]
pub fn check_signature(bot_token: &str, fields: &[(&str, &str)], signature: &str) -> bool {
    let mut fields: Vec<(&str, &str)> = fields.to_vec();
    fields.sort_unstable();

    let mut message = String::new();
    for (index, (key, value)) in fields.iter().enumerate() {
        if index > 0 {
            message.push('\n');
        }
        let _ = write!(message, "{key}={value}");
    }

    signature_matches(bot_token.as_bytes(), message.as_bytes(), signature)
}

/// Error returned by [`safe_parse_webapp_init_data`].
#[derive(Debug, thiserror::Error)]
pub enum WebAppValidationError {
    /// The signature does not match the bot token.
    #[error("invalid signature")]
    InvalidSignature,
    /// The init data contains a field that failed to parse.
    #[error("failed to parse init data: {0}")]
    Json(#[from] serde_json::Error),
}

/// Init data of a `WebApp`, as sent by `Telegram`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WebAppInitData {
    /// Unique identifier of the query.
    pub query_id: Option<String>,
    /// The user who opened the `WebApp`.
    pub user: Option<WebAppUser>,
    /// The chat where the `WebApp` was opened, if it was opened in a chat.
    pub receiver: Option<WebAppChat>,
    /// The type of the chat where the `WebApp` was opened.
    pub chat_type: Option<String>,
    /// The unique identifier of the chat instance.
    pub chat_instance: Option<String>,
    /// The value of the `start_param` query parameter.
    pub start_param: Option<String>,
    /// Time in seconds after which a message can be sent.
    pub can_send_after: Option<u64>,
    /// Unix time when the init data was created.
    pub auth_date: Option<u64>,
    /// The signature of the init data.
    pub hash: Option<String>,
}

/// A user of a `WebApp`.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct WebAppUser {
    /// Unique identifier of the user.
    pub id: u64,
    /// Whether the user is a bot.
    pub is_bot: Option<bool>,
    /// First name of the user.
    pub first_name: Option<String>,
    /// Last name of the user.
    pub last_name: Option<String>,
    /// Username of the user.
    pub username: Option<String>,
    /// `IETF` language tag of the user's language.
    pub language_code: Option<String>,
    /// Whether the user is a `Telegram` Premium user.
    pub is_premium: Option<bool>,
    /// Whether the user allowed the bot to send messages.
    pub allows_write_to_pm: Option<bool>,
    /// URL of the user's profile photo.
    pub photo_url: Option<String>,
}

/// A chat that a `WebApp` was opened in.
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct WebAppChat {
    /// Unique identifier of the chat.
    pub id: i64,
    /// Type of the chat.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Title of the chat.
    pub title: String,
    /// Username of the chat.
    pub username: Option<String>,
    /// First name of the other party in a private chat.
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat.
    pub last_name: Option<String>,
}

fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC accepts keys of any length");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0F) as usize] as char);
    }
    out
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    bool::from(a.ct_eq(b))
}

fn signature_matches(secret: &[u8], message: &[u8], expected_hex: &str) -> bool {
    let calculated = hex_lower(&hmac_sha256(secret, message));

    constant_time_eq(calculated.as_bytes(), expected_hex.as_bytes())
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let high = hex_val(bytes[index + 1]);
                let low = hex_val(bytes[index + 2]);
                if let (Some(high), Some(low)) = (high, low) {
                    out.push((high << 4) | low);
                    index += 3;
                } else {
                    out.push(b'%');
                    index += 1;
                }
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = "123456:ABC";

    fn init_data() -> String {
        format!(
            "query_id=abc&user=%7B%22id%22%3A42%7D&auth_date=100&hash={}",
            "9778490b4477d9ccfcf47b4041c9b5c65bcdced53b105a5a3e1ca39e57e1f06a"
        )
    }

    const WIDGET_FIELDS: [(&str, &str); 3] = [
        ("user_name", "Rogue"),
        ("first_name", "Andrew"),
        ("auth_date", "1662771648"),
    ];

    #[test]
    fn test_check_webapp_signature_valid() {
        assert!(check_webapp_signature(TOKEN, &init_data()));
    }

    #[test]
    fn test_check_webapp_signature_tampered() {
        let data = init_data().replace("auth_date=100", "auth_date=101");
        assert!(!check_webapp_signature(TOKEN, &data));
    }

    #[test]
    fn test_check_webapp_signature_wrong_token() {
        assert!(!check_webapp_signature("other:token", &init_data()));
    }

    #[test]
    fn test_check_webapp_signature_missing_hash() {
        assert!(!check_webapp_signature(TOKEN, "query_id=abc&auth_date=100"));
    }

    #[test]
    fn test_check_webapp_signature_hash_first() {
        let data = format!(
            "hash={}&query_id=abc&user=%7B%22id%22%3A42%7D&auth_date=100",
            "9778490b4477d9ccfcf47b4041c9b5c65bcdced53b105a5a3e1ca39e57e1f06a"
        );
        assert!(check_webapp_signature(TOKEN, &data));
    }

    #[test]
    fn test_safe_parse_happy_path() {
        let parsed = safe_parse_webapp_init_data(TOKEN, &init_data()).unwrap();

        assert_eq!(parsed.query_id.as_deref(), Some("abc"));
        assert_eq!(parsed.auth_date, Some(100));
        let user = parsed.user.unwrap();
        assert_eq!(user.id, 42);
        assert_eq!(user.first_name, None);
        assert_eq!(parsed.hash.as_deref().unwrap().len(), 64);
    }

    #[test]
    fn test_safe_parse_invalid_signature() {
        let data = init_data().replace("hash=9778", "hash=0000");
        assert!(matches!(
            safe_parse_webapp_init_data(TOKEN, &data),
            Err(WebAppValidationError::InvalidSignature)
        ));
    }

    #[test]
    fn test_check_signature_valid() {
        assert!(check_signature(
            TOKEN,
            &WIDGET_FIELDS,
            "6f59af49c462cb5db5f52f4d51e9dae40c09c562edeb424c823690a88dddc940"
        ));
    }

    #[test]
    fn test_check_signature_invalid() {
        assert!(!check_signature(
            TOKEN,
            &WIDGET_FIELDS,
            "0000000000000000000000000000000000000000000000000000000000000000"
        ));
    }

    #[test]
    fn test_check_signature_order_insensitive() {
        let reversed: Vec<(&str, &str)> = WIDGET_FIELDS.iter().rev().copied().collect();
        assert!(check_signature(
            TOKEN,
            &reversed,
            "6f59af49c462cb5db5f52f4d51e9dae40c09c562edeb424c823690a88dddc940"
        ));
    }

    #[test]
    fn test_percent_decode() {
        assert_eq!(percent_decode("%7B%22id%22%3A42%7D"), "{\"id\":42}");
        assert_eq!(percent_decode("a+b"), "a b");
        assert_eq!(percent_decode("%ZZ"), "%ZZ");
        assert_eq!(percent_decode("plain"), "plain");
    }

    #[test]
    fn test_hex_helpers() {
        assert_eq!(hex_val(b'f'), Some(15));
        assert_eq!(hex_val(b'F'), Some(15));
        assert_eq!(hex_val(b'0'), Some(0));
        assert_eq!(hex_val(b'Z'), None);
        assert_eq!(hex_lower(&[0xde, 0xad, 0xbe, 0xef]), "deadbeef");
    }
}
