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

    token.split_once(':').is_some_and(|(left, right)| {
        if left.is_empty() || right.is_empty() {
            return false;
        }

        left.parse::<i64>().ok().is_some()
    })
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn extract_bot_id(token: &str) -> Option<i64> {
    // `unwrap`s is safe here because we already checked token in `validate_token`
    validate(token).then(|| token.split_once(':').unwrap().0.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert!(validate("5645341478:AAERH8MzJYL8zacQ_ht5oeg4tjYx_ZhTmxA"));
        assert!(validate("6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"));
        assert!(!validate("6289679497:AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA "));
        assert!(!validate(":AAE6rlKdZBHrC1PdXXmeSY9TzAdh5dD7eGA"));
        assert!(!validate("6289679497:"));
    }
}
