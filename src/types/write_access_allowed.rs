use serde::Deserialize;

/// This object represents a service message about a user allowing a bot added to the attachment menu to write messages. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#writeaccessallowed>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct WriteAccessAllowed {}
