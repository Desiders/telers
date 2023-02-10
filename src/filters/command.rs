use super::base::Filter;

use crate::{
    client::{Bot, Session},
    context::Context,
    error::session,
    types::{BotCommand, Update},
};

use async_trait::async_trait;
use regex::Regex;
use std::borrow::Cow;
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
    #[error(transparent)]
    Session(#[from] session::ErrorKind),
}

/// Represents a command pattern type for verification
/// # Variants
/// * `Text(str)` - A command pattern with text
/// * `Object(BotCommand)` - A command pattern with [`BotCommand`] object
/// * `Regex(Regex)` - A command pattern with regex
#[derive(Debug, Clone)]
pub enum PatternType<'a> {
    Text(Cow<'a, str>),
    Object(BotCommand),
    Regex(Regex),
}

impl<'a> From<Cow<'a, str>> for PatternType<'a> {
    fn from(text: Cow<'a, str>) -> Self {
        Self::Text(text)
    }
}

impl<'a> From<&'a str> for PatternType<'a> {
    fn from(text: &'a str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<BotCommand> for PatternType<'_> {
    fn from(command: BotCommand) -> Self {
        Self::Object(command)
    }
}

impl From<Regex> for PatternType<'_> {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

#[derive(Debug, Clone)]
pub struct Command<'a> {
    /// List of commands ([`Cow`], [`BotCommand`] or compiled [`Regex`] patterns)
    commands: Vec<PatternType<'a>>,
    /// Command prefix
    prefix: &'a str,
    /// Ignore other command case (Does not work with regexp, use flags instead)
    ignore_case: bool,
    /// Ignore bot mention. By default, bot can not handle commands intended for other bots
    ignore_mention: bool,
}

impl<'a> Command<'a> {
    /// Creates a new [`Command`] filter
    /// # Arguments
    /// * `commands` - List of commands ([`Cow`], [`BotCommand`] or compiled [`Regex`] patterns)
    /// * `prefix` - Command prefix
    /// * `ignore_case` - Ignore other command case (Does not work with regexp, use flags instead)
    /// * `ignore_mention` - Ignore bot mention. By default, bot can not handle commands intended for other bots
    /// # Panics
    /// If `ignore_case` is `true` and `command`, which contains [`Regex`] pattern,
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn new<T: Into<PatternType<'a>>>(
        commands: Vec<T>,
        prefix: &'a str,
        ignore_case: bool,
        ignore_mention: bool,
    ) -> Self {
        let commands = if ignore_case {
            commands
                .into_iter()
                .map(|command| match command.into() {
                    PatternType::Text(text) => PatternType::Text(text.to_lowercase().into()),
                    PatternType::Object(command) => {
                        PatternType::Text(command.command.to_lowercase().into())
                    }
                    PatternType::Regex(regex) => PatternType::Regex(
                        Regex::new(&format!("(?i){regex}"))
                            .expect("Failed to compile regex with (?i) flag"),
                    ),
                })
                .collect()
        } else {
            commands.into_iter().map(Into::into).collect()
        };

        Self {
            commands,
            prefix,
            ignore_case,
            ignore_mention,
        }
    }

    #[must_use]
    pub fn builder() -> CommandBuilder<'a> {
        CommandBuilder::new()
    }
}

impl Default for Command<'_> {
    #[must_use]
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct CommandBuilder<'a> {
    commands: Vec<PatternType<'a>>,
    prefix: &'a str,
    ignore_case: bool,
    ignore_mention: bool,
}

impl<'a> CommandBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    /// If `ignore_case` is `true` and `command`, which contains [`Regex`] pattern,
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn commands<T: Into<PatternType<'a>>>(mut self, val: Vec<T>) -> Self {
        self.commands = val.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn command<T: Into<PatternType<'a>>>(mut self, val: T) -> Self {
        self.commands.push(val.into());
        self
    }

    #[must_use]
    pub fn prefix(mut self, val: &'a str) -> Self {
        self.prefix = val;
        self
    }

    #[must_use]
    pub fn ignore_case(mut self, val: bool) -> Self {
        self.ignore_case = val;
        self
    }

    #[must_use]
    pub fn ignore_mention(mut self, val: bool) -> Self {
        self.ignore_mention = val;
        self
    }

    /// # Panics
    /// If `ignore_case` is `true` and `command`, which contains [`Regex`] pattern,
    /// can't be compiled with `(?i)` flag (ignore case sensitive flag)
    #[must_use]
    pub fn build(self) -> Command<'a> {
        let commands = if self.ignore_case {
            self.commands
                .into_iter()
                .map(|command| match command {
                    PatternType::Text(text) => PatternType::Text(text.to_lowercase().into()),
                    PatternType::Object(command) => {
                        PatternType::Text(command.command.to_lowercase().into())
                    }
                    PatternType::Regex(regex) => PatternType::Regex(
                        Regex::new(&format!("(?i){regex}"))
                            .expect("Failed to compile regex with (?i) flag"),
                    ),
                })
                .collect()
        } else {
            self.commands
        };

        Command {
            commands,
            prefix: self.prefix,
            ignore_case: self.ignore_case,
            ignore_mention: self.ignore_mention,
        }
    }
}

impl Default for CommandBuilder<'_> {
    #[must_use]
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: "/",
            ignore_case: false,
            ignore_mention: false,
        }
    }
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
    pub async fn validate_mention<Client>(
        &self,
        command: &CommandObject,
        bot: &Bot<Client>,
    ) -> Result<()>
    where
        Client: Session + Send + Sync,
    {
        if self.ignore_mention {
            Ok(())
        } else if let Some(ref mention) = command.mention {
            if let Some(ref username) = bot.get_me(None).await?.username {
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
                PatternType::Regex(other_command) => {
                    if other_command.is_match(&command) {
                        return Ok(());
                    }
                }
                PatternType::Object(_) => unreachable!(
                    "PatternType::Object should be converted to PatternType::Text before validation"
                ),
            }
        }

        Err(Error::InvalidCommand)
    }

    /// # Errors
    /// - If prefix is invalid
    /// - If mention is invalid
    /// - If command is invalid
    pub async fn parse_command<Client>(
        &self,
        text: &str,
        bot: &Bot<Client>,
    ) -> Result<CommandObject>
    where
        Client: Session + Send + Sync,
    {
        let command = CommandObject::extract(text);

        self.validate_prefix(&command)?;
        self.validate_command(&command)?;
        self.validate_mention(&command, bot).await?;

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
        let command = full_command[1..].to_string();

        // Check if command contains mention, e.g. `/command@mention`, `/command@mention args`
        // and extract it, if it exists and isn't empty
        let (command, mention) = if command.contains('@') {
            let result: Vec<_> = command.split('@').collect();

            let command = result[0].to_string();
            let mention = result[1].to_string();

            let mention = if mention.is_empty() {
                None
            } else {
                Some(mention)
            };

            (command, mention)
        } else {
            (command, None)
        };

        CommandObject {
            command,
            prefix,
            mention,
            args,
        }
    }
}

#[async_trait]
impl<Client> Filter<Client> for Command<'_>
where
    Client: Session + Send + Sync,
{
    async fn check(&self, bot: &Bot<Client>, update: &Update, context: &Context) -> bool {
        let Some(ref message) = update.message else { return false; };
        let Some(text) = message.get_text_or_caption() else { return false; };

        match self.parse_command(text, bot).await {
            Ok(command) => {
                context.insert("command", Box::new(command));
                true
            }
            Err(_) => false,
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
        let command = Command::builder().prefix("/").command("start").build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/start_other");
        assert!(command.validate_prefix(&command_obj).is_ok());

        let command_obj = CommandObject::extract("!start");
        assert!(command.validate_prefix(&command_obj).is_err());
    }

    #[test]
    fn test_validate_command() {
        let command = Command::builder()
            .prefix("/")
            .command("start")
            .ignore_case(false)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        let command = Command::builder()
            .prefix("/")
            .command("start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());

        // Special case: `command` with uppercase letters and `ignore_case` is `true`
        // command should be converted to lowercase
        let command = Command::builder()
            .prefix("/")
            .command("Start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/START");
        assert!(command.validate_command(&command_obj).is_ok());

        let command_obj = CommandObject::extract("/stop");
        assert!(command.validate_command(&command_obj).is_err());

        let command_obj = CommandObject::extract("/STOP");
        assert!(command.validate_command(&command_obj).is_err());
    }

    // TODO: Add tests for `validate_mention` method
}
