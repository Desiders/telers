use super::Filter;
use crate::{
    client::Bot,
    types::{BotCommand, Message},
};

use regex::Regex;
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, CommandError>;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub enum CommandError {
    InvalidPrefix,
    InvalidMention,
    InvalidCommand,
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidPrefix => write!(f, "Invalid prefix"),
            CommandError::InvalidMention => write!(f, "Invalid mention"),
            CommandError::InvalidCommand => write!(f, "Invalid command"),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub enum CommandPatternType {
    Text(String),
    Object(BotCommand),
    Regex(Regex),
}

pub struct Command {
    /// List of commands (string or compiled regexp patterns)
    pub commands: Vec<CommandPatternType>,
    /// Command prefix
    pub prefix: String,
    /// Ignore other command case (Does not work with regexp, use flags instead)
    pub ignore_case: bool,
    /// Ignore bot mention. By default, bot can not handle commands intended for other bots
    pub ignore_mention: bool,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: "/".to_string(),
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

impl Command {
    #[must_use]
    pub fn new(
        commands: Vec<CommandPatternType>,
        prefix: String,
        ignore_case: bool,
        ignore_mention: bool,
    ) -> Self {
        Self {
            commands,
            prefix,
            ignore_case,
            ignore_mention,
        }
    }

    pub fn validate_prefix(&self, command: &CommandObject) -> Result<()> {
        if command.prefix == self.prefix {
            Ok(())
        } else {
            Err(CommandError::InvalidPrefix)
        }
    }

    pub fn validate_mention(&self, command: &CommandObject, bot: &Bot) -> Result<()> {
        if self.ignore_mention {
            Ok(())
        } else if let Some(ref mention) = command.mention {
            if let Some(ref username) = bot.get_me().username {
                if mention == username {
                    Ok(())
                } else {
                    Err(CommandError::InvalidMention)
                }
            } else {
                Err(CommandError::InvalidMention)
            }
        } else {
            Ok(())
        }
    }

    pub fn validate_command(&self, command: &CommandObject) -> Result<()> {
        let command = if self.ignore_case {
            command.command.to_lowercase()
        } else {
            command.command.clone()
        };

        for command_pattern in &self.commands {
            match command_pattern {
                CommandPatternType::Text(other_command) => {
                    if command == *other_command {
                        return Ok(());
                    }
                }
                CommandPatternType::Object(other_command) => {
                    if command == other_command.command {
                        return Ok(());
                    }
                }
                CommandPatternType::Regex(other_command) => {
                    if other_command.is_match(&command) {
                        return Ok(());
                    }
                }
            }
        }

        Err(CommandError::InvalidCommand)
    }

    pub fn parse_command(&self, text: &str, bot: &Bot) -> Result<CommandObject> {
        let command = CommandObject::extract(text);

        self.validate_prefix(&command)?;
        self.validate_mention(&command, bot)?;
        self.validate_command(&command)?;

        Ok(command)
    }
}

#[allow(clippy::module_name_repetitions)]
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

            (command, if mention == "" { None } else { Some(mention) })
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

impl Filter for Command {
    type Event = Message;

    fn check(&self, bot: &Bot, message: &Self::Event) -> bool {
        let text = match message.get_text_or_caption() {
            Some(text) => text,
            None => return false,
        };

        self.parse_command(text, bot).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, CommandObject, CommandPatternType};

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
        assert_eq!(
            command_obj.args,
            vec!["arg1".to_string(), "arg2".to_string()]
        );
    }

    #[test]
    fn test_validate_prefix() {
        let command = Command::new(
            vec![CommandPatternType::Text("start".to_string())],
            "/".to_string(),
            false,
            false,
        );

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("!start");
        assert!(command.validate_prefix(&command_obj).is_err());
    }

    #[test]
    fn test_validate_command() {
        let command = Command::new(
            vec![CommandPatternType::Text("start".to_string())],
            "/".to_string(),
            false,
            false,
        );

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        let command = Command::new(
            vec![CommandPatternType::Text("start".to_string())],
            "/".to_string(),
            true,
            false,
        );

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
