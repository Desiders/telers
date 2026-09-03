//! Helpers for working with Telegram deep links (`t.me/<bot>?start=<payload>`).
//!
//! The payload is encoded with base64 using the URL-safe alphabet without padding.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

/// Encodes a payload for use in a deep link start parameter.
///
/// # Examples
/// ```rust
/// use telers::utils::encode_payload;
///
/// assert_eq!(encode_payload("hello world"), "aGVsbG8gd29ybGQ");
/// ```
#[must_use]
pub fn encode_payload(payload: &str) -> String {
    URL_SAFE_NO_PAD.encode(payload.as_bytes())
}

/// Decodes a payload that was encoded with [`encode_payload`].
///
/// # Errors
/// Returns an error if the payload is not valid base64 or UTF-8.
///
/// # Examples
/// ```rust
/// use telers::utils::{decode_payload, encode_payload};
///
/// assert_eq!(
///     decode_payload(&encode_payload("hello world")).unwrap(),
///     "hello world"
/// );
/// ```
pub fn decode_payload(payload: &str) -> Result<String, DecodeError> {
    Ok(String::from_utf8(URL_SAFE_NO_PAD.decode(payload)?)?)
}

/// Maximum length of a deep-link payload allowed by Telegram.
pub const DEEPLINK_PAYLOAD_LENGTH: usize = 64;

/// Validates a raw deep-link payload the same way as Telegram:
/// only `a-zA-Z0-9_-` characters and at most [`DEEPLINK_PAYLOAD_LENGTH`] bytes.
///
/// # Errors
/// Returns an error if the payload contains characters outside `a-zA-Z0-9_-`
/// or is longer than [`DEEPLINK_PAYLOAD_LENGTH`] bytes.
pub fn validate_payload(payload: &str) -> Result<(), DeepLinkError> {
    if payload.len() > DEEPLINK_PAYLOAD_LENGTH {
        return Err(DeepLinkError::PayloadTooLong(payload.len()));
    }

    if payload
        .bytes()
        .any(|byte| !(byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'))
    {
        return Err(DeepLinkError::InvalidPattern);
    }

    Ok(())
}

/// Creates a deep link.
///
/// The `link_type` is the query key: `start` opens a private chat with the bot,
/// `startgroup` asks to pick a group, `startapp` opens a Mini App.
/// The payload is validated as-is, or encoded with [`encode_payload`] first.
///
/// # Errors
/// Returns an error if the payload is not valid for Telegram
/// (invalid characters or too long, see [`validate_payload`]).
///
/// # Examples
/// ```rust
/// use telers::utils::create_deep_link;
///
/// assert_eq!(
///     create_deep_link("my_bot", "start", "ref123", false).unwrap(),
///     "https://t.me/my_bot?start=ref123"
/// );
/// ```
pub fn create_deep_link(
    username: &str,
    link_type: &str,
    payload: &str,
    encode: bool,
) -> Result<String, DeepLinkError> {
    let payload = if encode {
        encode_payload(payload)
    } else {
        payload.to_owned()
    };
    validate_payload(&payload)?;
    Ok(format!("https://t.me/{username}?{link_type}={payload}"))
}

/// Creates a `t.me` start link for a bot.
///
/// The payload is used as the `start` query parameter, optionally encoded
/// with [`encode_payload`]. Raw payloads are limited to `a-zA-Z0-9_-` and
/// [`DEEPLINK_PAYLOAD_LENGTH`] bytes by Telegram, so encoding is
/// recommended for anything else.
///
/// # Errors
/// Returns an error if the payload is not valid for Telegram.
///
/// # Examples
/// ```rust
/// use telers::utils::create_start_link;
///
/// assert_eq!(
///     create_start_link("my_bot", "ref123", false).unwrap(),
///     "https://t.me/my_bot?start=ref123"
/// );
/// assert_eq!(
///     create_start_link("my_bot", "hello world", true).unwrap(),
///     "https://t.me/my_bot?start=aGVsbG8gd29ybGQ"
/// );
/// ```
pub fn create_start_link(
    username: &str,
    payload: &str,
    encode: bool,
) -> Result<String, DeepLinkError> {
    create_deep_link(username, "start", payload, encode)
}

/// Creates a `t.me` start link that opens the bot in a group.
///
/// # Errors
/// Returns an error if the payload is not valid for Telegram.
///
/// # Examples
/// ```rust
/// use telers::utils::create_startgroup_link;
///
/// assert_eq!(
///     create_startgroup_link("my_bot", "ref123", false).unwrap(),
///     "https://t.me/my_bot?startgroup=ref123"
/// );
/// ```
pub fn create_startgroup_link(
    username: &str,
    payload: &str,
    encode: bool,
) -> Result<String, DeepLinkError> {
    create_deep_link(username, "startgroup", payload, encode)
}

/// Creates a `t.me` start link that opens a Mini App.
///
/// If `app_name` is given, a direct Mini App link to the specified app is created.
///
/// # Errors
/// Returns an error if the payload is not valid for Telegram.
///
/// # Examples
/// ```rust
/// use telers::utils::create_startapp_link;
///
/// assert_eq!(
///     create_startapp_link("my_bot", "ref123", None, false).unwrap(),
///     "https://t.me/my_bot?startapp=ref123"
/// );
/// ```
pub fn create_startapp_link(
    username: &str,
    payload: &str,
    app_name: Option<&str>,
    encode: bool,
) -> Result<String, DeepLinkError> {
    let link = create_deep_link(username, "startapp", payload, encode)?;

    Ok(match app_name {
        Some(app_name) => link.replacen(
            &format!("https://t.me/{username}?"),
            &format!("https://t.me/{username}/{app_name}?"),
            1,
        ),
        None => link,
    })
}

/// Error returned by deep-link creation helpers.
#[derive(Debug, thiserror::Error)]
pub enum DeepLinkError {
    /// The payload contains characters outside `a-zA-Z0-9_-`.
    #[error("payload contains invalid characters, allowed: a-zA-Z0-9_-")]
    InvalidPattern,
    /// The payload is longer than [`DEEPLINK_PAYLOAD_LENGTH`] bytes.
    #[error("payload is too long: {0} bytes, maximum is 64")]
    PayloadTooLong(usize),
}

/// Error returned by [`decode_payload`].
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// The payload is not valid base64.
    #[error("invalid base64 payload: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    /// The decoded payload is not valid UTF-8.
    #[error("decoded payload is not valid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

#[cfg(test)]
mod tests {
    use super::{
        create_deep_link, create_start_link, create_startapp_link, create_startgroup_link,
        decode_payload, encode_payload, validate_payload, DecodeError, DeepLinkError,
        DEEPLINK_PAYLOAD_LENGTH,
    };

    #[test]
    fn test_roundtrip() {
        for payload in [
            "hello world",
            "",
            "telegram",
            "https://example.com/?a=1&b=2",
        ] {
            assert_eq!(decode_payload(&encode_payload(payload)).unwrap(), payload);
        }
    }

    #[test]
    fn test_url_safe() {
        assert_eq!(encode_payload("hello world"), "aGVsbG8gd29ybGQ");
        assert_eq!(encode_payload("тест"), "0YLQtdGB0YI");
        assert!(!encode_payload("+/=").contains(['+', '/', '=']));
    }

    #[test]
    fn test_decode_invalid_base64() {
        assert!(matches!(
            decode_payload("!!!").unwrap_err(),
            DecodeError::InvalidBase64(_)
        ));
    }

    #[test]
    fn test_decode_invalid_utf8() {
        assert!(matches!(
            decode_payload("_w").unwrap_err(),
            DecodeError::InvalidUtf8(_)
        ));
    }

    #[test]
    fn test_create_start_link() {
        assert_eq!(
            create_start_link("my_bot", "ref123", false).unwrap(),
            "https://t.me/my_bot?start=ref123"
        );
        assert_eq!(
            create_start_link("my_bot", "hello world", true).unwrap(),
            "https://t.me/my_bot?start=aGVsbG8gd29ybGQ"
        );
    }

    #[test]
    fn test_create_deep_link() {
        assert_eq!(
            create_deep_link("my_bot", "start", "ref123", false).unwrap(),
            "https://t.me/my_bot?start=ref123"
        );
        assert_eq!(
            create_deep_link("my_bot", "startgroup", "ref123", false).unwrap(),
            "https://t.me/my_bot?startgroup=ref123"
        );
        assert_eq!(
            create_deep_link("my_bot", "startapp", "ref", false).unwrap(),
            "https://t.me/my_bot?startapp=ref"
        );
    }

    #[test]
    fn test_create_startgroup_link() {
        assert_eq!(
            create_startgroup_link("my_bot", "ref123", false).unwrap(),
            "https://t.me/my_bot?startgroup=ref123"
        );
    }

    #[test]
    fn test_create_startapp_link() {
        assert_eq!(
            create_startapp_link("my_bot", "ref123", None, false).unwrap(),
            "https://t.me/my_bot?startapp=ref123"
        );
        assert_eq!(
            create_startapp_link("my_bot", "ref123", Some("my_app"), false).unwrap(),
            "https://t.me/my_bot/my_app?startapp=ref123"
        );
    }

    #[test]
    fn test_validate_payload() {
        assert!(validate_payload("ref123").is_ok());
        assert!(validate_payload("a-b_c").is_ok());
        assert!(validate_payload("").is_ok());
        assert!(validate_payload("hello world").is_err());
        assert!(validate_payload("привет").is_err());
        assert!(matches!(
            validate_payload("hello world").unwrap_err(),
            DeepLinkError::InvalidPattern
        ));
        assert!(matches!(
            validate_payload(&"a".repeat(DEEPLINK_PAYLOAD_LENGTH + 1)).unwrap_err(),
            DeepLinkError::PayloadTooLong(_)
        ));
        assert!(validate_payload(&"a".repeat(DEEPLINK_PAYLOAD_LENGTH)).is_ok());
    }

    #[test]
    fn test_encoded_payload_skips_validation() {
        assert!(create_start_link("my_bot", "hello world", true).is_ok());
        assert!(create_start_link("my_bot", "hello world", false).is_err());
    }
}
