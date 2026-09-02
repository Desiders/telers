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
pub const DEEPLINK_PAYLOAD_MAX_LEN: usize = 64;

/// Validates a raw deep-link payload the same way as Telegram:
/// only `a-zA-Z0-9_-` characters and at most [`DEEPLINK_PAYLOAD_MAX_LEN`] bytes.
///
/// # Errors
/// Returns an error if the payload contains characters outside `a-zA-Z0-9_-`
/// or is longer than [`DEEPLINK_PAYLOAD_MAX_LEN`] bytes.
pub fn validate_payload(payload: &str) -> Result<(), DeepLinkError> {
    if payload.len() > DEEPLINK_PAYLOAD_MAX_LEN {
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

#[inline]
fn checked_encode(payload: &str, encode: bool) -> Result<String, DeepLinkError> {
    if encode {
        Ok(encode_payload(payload))
    } else {
        validate_payload(payload)?;
        Ok(payload.to_owned())
    }
}

/// Creates a `t.me` start link for a bot.
///
/// The payload is used as the `start` query parameter, optionally encoded
/// with [`encode_payload`]. Raw payloads are limited to `a-zA-Z0-9_-` and
/// [`DEEPLINK_PAYLOAD_MAX_LEN`] bytes by Telegram, so encoding is
/// recommended for anything else.
///
/// # Errors
/// Returns an error if a raw (not encoded) payload is invalid for Telegram.
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
    bot_username: &str,
    payload: &str,
    encoded: bool,
) -> Result<String, DeepLinkError> {
    Ok(format!(
        "https://t.me/{bot_username}?start={}",
        checked_encode(payload, encoded)?
    ))
}

/// Creates a `t.me` deep link with the payload attached as a query parameter.
///
/// The `parameter` is the query key: `start` opens a private chat with the bot,
/// `startgroup` asks to pick a group, `startapp` opens a Mini App.
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
    bot_username: &str,
    parameter: &str,
    payload: &str,
    encoded: bool,
) -> Result<String, DeepLinkError> {
    Ok(format!(
        "https://t.me/{bot_username}?{parameter}={}",
        checked_encode(payload, encoded)?
    ))
}

/// Creates a `t.me` link that starts the bot in a group.
///
/// # Errors
/// Returns an error if a raw (not encoded) payload is invalid for Telegram.
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
    bot_username: &str,
    payload: &str,
    encoded: bool,
) -> Result<String, DeepLinkError> {
    Ok(format!(
        "https://t.me/{bot_username}?startgroup={}",
        checked_encode(payload, encoded)?
    ))
}

/// Error returned by deep-link creation helpers.
#[derive(Debug, thiserror::Error)]
pub enum DeepLinkError {
    /// The raw payload contains characters outside `a-zA-Z0-9_-`.
    #[error("payload contains invalid characters, allowed: a-zA-Z0-9_-")]
    InvalidPattern,
    /// The raw payload is longer than [`DEEPLINK_PAYLOAD_MAX_LEN`] bytes.
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
        create_deep_link, create_start_link, create_startgroup_link, decode_payload,
        encode_payload, validate_payload, DecodeError, DeepLinkError, DEEPLINK_PAYLOAD_MAX_LEN,
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
            validate_payload(&"a".repeat(DEEPLINK_PAYLOAD_MAX_LEN + 1)).unwrap_err(),
            DeepLinkError::PayloadTooLong(_)
        ));
        assert!(validate_payload(&"a".repeat(DEEPLINK_PAYLOAD_MAX_LEN)).is_ok());
    }

    #[test]
    fn test_raw_payload_validation() {
        assert!(matches!(
            create_start_link("my_bot", "hello world", false).unwrap_err(),
            DeepLinkError::InvalidPattern
        ));
        assert!(create_start_link("my_bot", "hello world", true).is_ok());
    }
}
