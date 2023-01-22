use serde::Deserialize;

/// This object represents a service message about a video chat scheduled in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatscheduled>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}
