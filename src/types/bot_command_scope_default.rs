use serde::{Deserialize, Serialize};

/// Represents the default `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands. Default commands are used if no commands with a `narrower scope <https://core.telegram.org/bots/api#determining-list-of-commands>`_ are specified for the user.
/// <https://core.telegram.org/bots/api#botcommandscopedefault>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be *default*
    #[serde(rename = "type", default = "default")]
    scope_type: String,
}

fn default() -> String {
    "default".to_string()
}
