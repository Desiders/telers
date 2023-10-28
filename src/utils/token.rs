/// Hide telegram token for privacy. \
/// For example,
/// `1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` will be hidden as `12********11`
pub(crate) fn hide(token: &str) -> Box<str> {
    let token_len = token.len();

    let mut hidden = String::with_capacity(token_len);
    hidden.push_str(&token[..2]);
    hidden.push_str(&"*".repeat(8));
    hidden.push_str(&token[token_len - 2..]);
    hidden.into()
}

/// Validate telegram token
/// # Returns
/// `true` if token is valid, otherwise `false`
#[must_use]
pub fn validate(token: &str) -> bool {
    for symbol in token.chars() {
        if symbol.is_whitespace() {
            return false;
        }
    }

    token.split_once(':').map_or(false, |(left, right)| {
        if left.is_empty() || right.is_empty() {
            return false;
        }

        left.parse::<i64>().ok().is_some()
    })
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn extract_bot_id(token: &str) -> Option<i64> {
    // `unwrap`'s is safe here because we already checked token in `validate_token`
    validate(token).then(|| token.split_once(':').unwrap().0.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hide() {
        assert_eq!(
            hide("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11").as_ref(),
            "12********11"
        );
        assert_eq!(
            hide("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew1").as_ref(),
            "12********w1"
        );
        assert_eq!(
            hide("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew").as_ref(),
            "12********ew"
        );
    }

    #[test]
    fn test_validate() {
        assert!(validate("5645341478:AAERH8MzJYL8zacQ_ht5oeg4tjYx_ZhTmxA"));
        assert!(validate("6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"));
        assert!(!validate("6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA "));
        assert!(!validate(":AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"));
        assert!(!validate("6289679497:"));
    }
}
