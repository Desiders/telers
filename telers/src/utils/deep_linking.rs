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

#[inline]
fn encode(payload: &str, encode: bool) -> String {
    if encode {
        encode_payload(payload)
    } else {
        payload.to_owned()
    }
}

/// Creates a `t.me` start link for a bot.
///
/// The payload is used as the `start` query parameter, optionally encoded
/// with [`encode_payload`]. Raw payloads are limited to `a-zA-Z0-9_-` by
/// Telegram, so encoding is recommended for anything else.
///
/// # Examples
/// ```rust
/// use telers::utils::create_start_link;
///
/// assert_eq!(
///     create_start_link("my_bot", "ref123", false),
///     "https://t.me/my_bot?start=ref123"
/// );
/// assert_eq!(
///     create_start_link("my_bot", "hello world", true),
///     "https://t.me/my_bot?start=aGVsbG8gd29ybGQ"
/// );
/// ```
#[must_use]
pub fn create_start_link(bot_username: &str, payload: &str, encoded: bool) -> String {
    format!(
        "https://t.me/{bot_username}?start={}",
        encode(payload, encoded)
    )
}

/// Creates a `t.me` deep link for an arbitrary command.
///
/// An empty payload produces a link without the `start` parameter.
///
/// # Examples
/// ```rust
/// use telers::utils::create_deep_link;
///
/// assert_eq!(
///     create_deep_link("my_bot", "start", "ref123", false),
///     "https://t.me/my_bot/start?start=ref123"
/// );
/// assert_eq!(
///     create_deep_link("my_bot", "help", "", false),
///     "https://t.me/my_bot/help"
/// );
/// ```
#[must_use]
pub fn create_deep_link(bot_username: &str, command: &str, payload: &str, encoded: bool) -> String {
    if payload.is_empty() {
        return format!("https://t.me/{bot_username}/{command}");
    }

    format!(
        "https://t.me/{bot_username}/{command}?start={}",
        encode(payload, encoded)
    )
}

/// Creates a `t.me` link that starts the bot in a group.
///
/// # Examples
/// ```rust
/// use telers::utils::create_startgroup_link;
///
/// assert_eq!(
///     create_startgroup_link("my_bot", "ref123", false),
///     "https://t.me/my_bot?startgroup=ref123"
/// );
/// ```
#[must_use]
pub fn create_startgroup_link(bot_username: &str, payload: &str, encoded: bool) -> String {
    format!(
        "https://t.me/{bot_username}?startgroup={}",
        encode(payload, encoded)
    )
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
    use super::{decode_payload, encode_payload, DecodeError};

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
}
