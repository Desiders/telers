use serde::Deserialize;

/// This object contains information about one answer option in a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#polloption>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: Box<str>,
    /// Number of users that voted for this option
    pub voter_count: i64,
}
