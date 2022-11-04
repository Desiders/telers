use serde::{Deserialize, Serialize};

/// Represents the default `scope <https://core.telegram.org/bots/api#botcommandscope>` of bot commands. Default commands are used if no commands with a `narrower scope <https://core.telegram.org/bots/api#determining-list-of-commands>` are specified for the user.
/// <https://core.telegram.org/bots/api#botcommandscopedefault>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
    /// Scope type, must be *default*
    #[serde(rename = "type", default = "default")]
    scope_type: String,
}

impl Default for BotCommandScopeDefault {
    fn default() -> Self {
        Self {
            scope_type: default(),
        }
    }
}

fn default() -> String {
    "default".to_string()
}
