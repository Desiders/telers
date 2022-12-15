use super::base::Filter;

use crate::{
    client::Bot,
    context::Context,
    types::{BotCommand, Update},
};

use regex::Regex;
use std::sync::RwLock;
use thiserror;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid mention")]
    InvalidMention,
    #[error("Invalid command")]
    InvalidCommand,
}

/// Represents a command pattern type for verification
/// # Variants
/// * `Text(str)` - A command pattern with text
/// * `Object(BotCommand)` - A command pattern with [`BotCommand`] object
/// * `Regex(Regex)` - A command pattern with regex
#[derive(Debug, Clone)]
pub enum PatternType<'a> {
    Text(&'a str),
    Object(BotCommand),
    Regex(Regex),
}

#[derive(Default, Debug, Clone)]
pub struct Command<'a> {
    /// List of commands (str, [`BotCommand`] or compiled regexp patterns)
    pub commands: Vec<PatternType<'a>>,
    /// Command prefix
    pub prefix: &'a str,
    /// Ignore other command case (Does not work with regexp, use flags instead)
    pub ignore_case: bool,
    /// Ignore bot mention. By default, bot can not handle commands intended for other bots
    pub ignore_mention: bool,
}

impl<'a> Command<'a> {
    /// # Errors
    /// If prefix is invalid.
    pub fn validate_prefix(&self, command: &CommandObject) -> Result<()> {
        if command.prefix == self.prefix {
            Ok(())
        } else {
            Err(Error::InvalidPrefix)
        }
    }

    /// # Errors
    /// If mention is invalid.
    pub fn validate_mention(&self, command: &CommandObject, bot: &Bot) -> Result<()> {
        if self.ignore_mention {
            Ok(())
        } else if let Some(ref mention) = command.mention {
            if let Some(ref username) = bot.get_me().username {
                if mention == username {
                    Ok(())
                } else {
                    Err(Error::InvalidMention)
                }
            } else {
                Err(Error::InvalidMention)
            }
        } else {
            Ok(())
        }
    }

    /// # Errors
    /// If command is invalid.
    pub fn validate_command(&self, command: &CommandObject) -> Result<()> {
        let command = if self.ignore_case {
            command.command.to_lowercase()
        } else {
            command.command.clone()
        };

        for command_pattern in &self.commands {
            match command_pattern {
                PatternType::Text(other_command) => {
                    if command == *other_command {
                        return Ok(());
                    }
                }
                PatternType::Object(other_command) => {
                    if command == other_command.command {
                        return Ok(());
                    }
                }
                PatternType::Regex(other_command) => {
                    if other_command.is_match(&command) {
                        return Ok(());
                    }
                }
            }
        }

        Err(Error::InvalidCommand)
    }

    /// # Errors
    /// - If prefix is invalid
    /// - If mention is invalid
    /// - If command is invalid
    pub fn parse_command(&self, text: &str, bot: &Bot) -> Result<CommandObject> {
        let command = CommandObject::extract(text);

        self.validate_prefix(&command)?;
        self.validate_mention(&command, bot)?;
        self.validate_command(&command)?;

        Ok(command)
    }
}

/// Represents parsed command from text
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CommandObject {
    /// Command without prefix and mention
    pub command: String,
    /// Command prefix
    pub prefix: String,
    /// *Optional*. Mention in command
    pub mention: Option<String>,
    /// Command arguments
    pub args: Vec<String>,
}

impl CommandObject {
    /// Extracts [`CommandObject`] from text
    #[must_use]
    pub fn extract(text: &str) -> Self {
        let result: Vec<_> = text.trim().split(' ').collect();
        let full_command = result[0].to_string();
        let args: Vec<String> = result[1..].iter().map(ToString::to_string).collect();

        let prefix = full_command[0..1].to_string();
        let command_with_mention = full_command[1..].to_string();
        let (command, mention) = if command_with_mention.contains('@') {
            let result: Vec<_> = command_with_mention.split('@').collect();

            let command = result[0].to_string();
            let mention = result[1].to_string();

            (
                command,
                if mention.is_empty() {
                    None
                } else {
                    Some(mention)
                },
            )
        } else {
            (command_with_mention, None)
        };

        CommandObject {
            command,
            prefix,
            mention,
            args,
        }
    }
}

impl Filter for Command<'_> {
    fn check(&self, bot: &Bot, update: &Update, context: &RwLock<Context>) -> bool {
        if let Some(ref message) = update.message {
            let text = match message.get_text_or_caption() {
                Some(text) => text,
                None => return false,
            };

            match self.parse_command(text, bot) {
                Ok(command) => {
                    context
                        .write()
                        .unwrap()
                        .insert("command", Box::new(command));
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_extract() {
        let command_obj = CommandObject::extract("/start");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@bot_username");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, Some("bot_username".to_string()));
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, Vec::<String>::new());

        let command_obj = CommandObject::extract("/start@bot_username arg1 arg2");
        assert_eq!(command_obj.command, "start");
        assert_eq!(command_obj.prefix, "/");
        assert_eq!(command_obj.mention, Some("bot_username".to_string()));
        assert_eq!(command_obj.args, vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_validate_prefix() {
        let command = Command {
            commands: vec![PatternType::Text("start")],
            ignore_case: false,
            ignore_mention: false,
            prefix: "/",
        };

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("!start");
        assert!(command.validate_prefix(&command_obj).is_err());
    }

    #[test]
    fn test_validate_command() {
        let command = Command {
            commands: vec![PatternType::Text("start")],
            ignore_case: false,
            ignore_mention: false,
            prefix: "/",
        };

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        let command = Command {
            commands: vec![PatternType::Text("start")],
            ignore_case: true,
            ignore_mention: false,
            prefix: "/",
        };

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());
    }
}
