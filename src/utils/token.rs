/// Hide telegram token for privacy. \
/// For example,
/// `1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` will be hidden as `12********11`
pub(crate) fn hide_token(token: &str) -> String {
    let token_len = token.len();

    let mut hidden = String::with_capacity(token_len);
    hidden.push_str(&token[..2]);
    hidden.push_str(&"*".repeat(8));
    hidden.push_str(&token[token_len - 2..]);
    hidden
}

/// Validate telegram token
pub fn validate_token(token: &str) -> bool {
    for symbol in token.chars() {
        if symbol.is_whitespace() {
            return false;
        }
    }

    token
        .split_once(':')
        .map(|(left, right)| {
            if left.is_empty() || right.is_empty() {
                return false;
            }

            match left.parse::<i64>().ok() {
                Some(_) => true,
                None => false,
            }
        })
        .unwrap_or(false)
}

pub fn extract_bot_id(token: &str) -> Option<i64> {
    // `unwrap`'s is safe here because we already checked token in `validate_token`
    validate_token(token).then(|| token.split_once(':').unwrap().0.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hide_token() {
        assert_eq!(
            hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"),
            "12********11"
        );
        assert_eq!(
            hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew1"),
            "12********w1"
        );
        assert_eq!(
            hide_token("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew"),
            "12********ew"
        );
    }

    #[test]
    fn test_validate_token() {
        assert!(validate_token(
            "5645341478:AAERH8MzJYL8zacQ_ht5oeg4tjYx_ZhTmxA"
        ));
        assert!(validate_token(
            "6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"
        ));
        assert!(!validate_token(
            "6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA "
        ));
        assert!(!validate_token(":AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"));
        assert!(!validate_token("6289679497:"));
    }
}
