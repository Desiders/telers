use serde::{Deserialize, Serialize};

/// Represents the default [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands. Default commands are used if no commands with a [`narrower scope`](https://core.telegram.org/bots/api#determining-list-of-commands) are specified for the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopedefault>
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommandScopeDefault;

impl BotCommandScopeDefault {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
