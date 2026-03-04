use super::base::Filter;
use crate::{
    client::{Bot, Session},
    errors::SessionErrorKind,
    methods::GetMe,
    types::BotCommand,
    FromContext, Request,
};

use regex::Regex;
use std::{borrow::Cow, iter::once};
use tracing::{event, instrument, Level};

/// Represents a command pattern type for verification
/// # Variants
/// * `PatternType::Text(Cow<str>)` - A command pattern with text
/// * `PatternType::Object(BotCommand)` -
///   A command pattern with [`BotCommand`] object. \
///   Just a shortcut for `PatternType::Text(command.command)`.
/// * `PatternType::Regex(Regex)` -
///   A command pattern with regex, compiled with [`Regex`] struct. \
///   If filter used with `ignore_case` flag, then the regex will be compiled with `(?i)` flag (ignore case sensitive flag).
#[derive(Debug, Clone)]
pub enum PatternType {
    Text(Cow<'static, str>),
    Object(BotCommand),
    Regex(Regex),
}

impl From<Cow<'static, str>> for PatternType {
    fn from(text: Cow<'static, str>) -> Self {
        Self::Text(text)
    }
}

impl From<&'static str> for PatternType {
    fn from(text: &'static str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<BotCommand> for PatternType {
    fn from(command: BotCommand) -> Self {
        Self::Object(command)
    }
}

impl From<Regex> for PatternType {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

/// This filter checks if the message is a command.
///
/// Filter accepts [`PatternType`] that represents a command pattern type for verification,
/// for example, text, [`BotCommand`] or [`Regex`].
///
/// # Notes
/// You can use parsed command using [`CommandObject`] struct in handler arguments,
/// or get it from [`crate::context::Context`] by `command` key.
#[derive(Debug, Clone)]
pub struct Command {
    /// List of commands ([`Cow`], [`BotCommand`] or compiled [`Regex`] patterns)
    commands: Vec<PatternType>,
    /// Command prefix
    prefix: char,
    /// Ignore case sensitive
    ignore_case: bool,
    /// Ignore bot mention
    ignore_mention: bool,
}

impl Command {
    /// Creates a new [`Command`] filter
    /// # Arguments
    /// * `commands` - List of commands (texts, [`BotCommand`] or compiled [`Regex`] patterns)
    /// * `prefix` - Command prefix
    /// * `ignore_case` - Ignore other command case
    /// * `ignore_mention` - Ignore bot mention
    #[must_use]
    #[instrument(skip(commands))]
    pub fn new<CommandType, Commands>(
        commands: Commands,
        prefix: char,
        ignore_case: bool,
        ignore_mention: bool,
    ) -> Self
    where
        CommandType: Into<PatternType>,
        Commands: IntoIterator<Item = CommandType>,
    {
        let commands = if ignore_case {
            commands
                .into_iter()
                .map(|command| match command.into() {
                    PatternType::Text(text) => PatternType::Text(text.to_lowercase().into()),
                    // We convert object to text, because this pattern type is just a shortcut for text
                    PatternType::Object(command) => {
                        PatternType::Text(command.command.to_lowercase().into())
                    }
                    PatternType::Regex(regex) => {
                        if ignore_mention {
                            event!(Level::WARN, "Ignore mention flag doesn't work with regexes");
                        }

                        PatternType::Regex(regex)
                    }
                })
                .collect()
        } else {
            commands
                .into_iter()
                .map(|command| match command.into() {
                    PatternType::Text(text) => PatternType::Text(text),
                    // We convert object to text, because this pattern type is just a shortcut for text
                    PatternType::Object(command) => {
                        PatternType::Text(Cow::Owned(command.command.into_string()))
                    }
                    PatternType::Regex(regex) => {
                        if ignore_mention {
                            event!(Level::WARN, "Ignore mention flag doesn't work with regexes");
                        }

                        PatternType::Regex(regex)
                    }
                })
                .collect()
        };

        Self {
            commands,
            prefix,
            ignore_case,
            ignore_mention,
        }
    }

    /// Creates a new [`Command`] filter with pass command
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder
    /// - By default, the prefix is `/`. If you want to change it, use [`Command::one_with_prefix`] instead.
    #[must_use]
    pub fn one(command: impl Into<PatternType>) -> Self {
        Self::builder().command(command).build()
    }

    /// Creates a new [`Command`] filter with pass command and prefix
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder.
    /// - By default, the prefix is `/`, so you can use [`Command::one`] instead. Use this method if you want to change the it.
    #[must_use]
    pub fn one_with_prefix(command: impl Into<PatternType>, prefix: char) -> Self {
        Self::builder().command(command).prefix(prefix).build()
    }

    /// Creates a new [`Command`] filter with pass commands
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder
    /// - By default, the prefix is `/`. If you want to change it, use [`Command::many_with_prefix`] instead.
    #[must_use]
    pub fn many<T, I>(commands: I) -> Self
    where
        T: Into<PatternType>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().commands(commands).build()
    }

    /// Creates a new [`Command`] filter with pass commands and prefix
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder
    /// - By default, the prefix is `/`, so you can use [`Command::many`] instead. Use this method if you want to change the it.
    #[must_use]
    pub fn many_with_prefix<T, I>(commands: I, prefix: char) -> Self
    where
        T: Into<PatternType>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().commands(commands).prefix(prefix).build()
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: '/',
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Builder {
    commands: Vec<PatternType>,
    prefix: char,
    ignore_case: bool,
    ignore_mention: bool,
}

impl Builder {
    #[must_use]
    pub fn new() -> Builder {
        Self::default()
    }

    #[must_use]
    pub fn command(self, val: impl Into<PatternType>) -> Self {
        Self {
            commands: self.commands.into_iter().chain(once(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn commands<T, I>(self, val: I) -> Self
    where
        T: Into<PatternType>,
        I: IntoIterator<Item = T>,
    {
        Self {
            commands: self
                .commands
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn prefix(self, val: char) -> Self {
        Self {
            prefix: val,
            ..self
        }
    }

    #[must_use]
    pub fn ignore_case(self, val: bool) -> Self {
        Self {
            ignore_case: val,
            ..self
        }
    }

    #[must_use]
    pub fn ignore_mention(self, val: bool) -> Self {
        Self {
            ignore_mention: val,
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> Command {
        Command::new(
            self.commands,
            self.prefix,
            self.ignore_case,
            self.ignore_mention,
        )
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            commands: vec![],
            prefix: '/',
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

impl Command {
    #[must_use]
    pub fn validate_prefix(&self, command: &CommandObject) -> bool {
        command.prefix == self.prefix
    }

    /// # Errors
    /// If error occurred in the process of sending request to the Telegram API or parsing response
    #[allow(clippy::missing_panics_doc)]
    pub async fn validate_mention(
        &self,
        command: &CommandObject,
        bot: &Bot<impl Session>,
    ) -> Result<bool, SessionErrorKind> {
        if self.ignore_mention {
            Ok(true)
        } else if let Some(ref mention) = command.mention {
            bot.send(GetMe {}).await.map(|user| {
                // `unwrap` is safe here, because bot always has username
                user.username.unwrap().eq(mention)
            })
        } else {
            Ok(true)
        }
    }

    #[must_use]
    pub fn validate_command(&self, command: &CommandObject) -> bool {
        let command = if self.ignore_case {
            command.command.to_lowercase().into_boxed_str()
        } else {
            command.command.clone()
        };
        let command_ref = command.as_ref();

        for pattern in &*self.commands {
            match pattern {
                PatternType::Text(allowed_command) => {
                    if command_ref == allowed_command {
                        return true;
                    }
                }
                PatternType::Regex(regex) => {
                    if regex.is_match(&command) {
                        return true;
                    }
                }
                PatternType::Object(_) => {
                    unreachable!(
                        "`PatternType::Object` should be converted to `PatternType::Text` before \
                         validation"
                    )
                }
            }
        }

        false
    }

    /// # Errors
    /// If error occurred in the process of sending request to the Telegram API or parsing response
    pub async fn validate_command_object(
        &self,
        command: &CommandObject,
        bot: &Bot<impl Session>,
    ) -> Result<bool, SessionErrorKind> {
        Ok(self.validate_prefix(command)
            && self.validate_command(command)
            && self.validate_mention(command, bot).await?)
    }
}

/// Represents parsed command from text
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Hash, PartialEq, Eq, FromContext)]
#[context(
    key = "command",
    description = "Parsed command object. This type is available only if the command filter is \
                   used and filer is passed."
)]
pub struct CommandObject {
    /// Command without prefix and mention
    pub command: Box<str>,
    /// Command prefix
    pub prefix: char,
    /// Mention in command
    pub mention: Option<Box<str>>,
    /// Command arguments
    pub args: Box<[Box<str>]>,
}

impl CommandObject {
    /// Extracts [`CommandObject`] from text
    #[must_use]
    pub fn extract(text: &str) -> Option<Self> {
        let result: Box<[&str]> = text.trim().split(' ').collect();
        let full_command = result[0];
        let args = result[1..]
            .iter()
            .map(|arg| (*arg).to_owned().into_boxed_str())
            .collect();

        let mut full_command_chars = full_command.chars();

        let prefix = full_command_chars.next()?;

        let command = full_command_chars.as_str();
        if command.is_empty() {
            return None;
        }

        // Check if command contains mention, e.g. `/command@mention`, `/command@mention args`
        // and extract it, if it exists and isn't empty
        let (command, mention) = if !command.starts_with('@') && command.contains('@') {
            let result: Box<[&str]> = command.split('@').collect();

            let command = result[0];
            let mention = result[1];

            let mention = if mention.is_empty() {
                None
            } else {
                Some(mention)
            };

            (command, mention)
        } else {
            (command, None)
        };

        Some(CommandObject {
            command: command.into(),
            prefix,
            mention: mention.map(Into::into),
            args,
        })
    }
}

impl<Client> Filter<Client> for Command
where
    Client: Session + 'static,
{
    #[instrument]
    async fn check(&mut self, request: &mut Request<Client>) -> bool {
        let Some(message) = request.update.message() else {
            return false;
        };
        let Some(text) = message.text().or(message.caption()) else {
            return false;
        };
        let Some(command) = CommandObject::extract(text) else {
            return false;
        };

        match self.validate_command_object(&command, &request.bot).await {
            Ok(result) => {
                if result {
                    request.context.insert("command", command);
                    true
                } else {
                    false
                }
            }
            Err(err) => {
                event!(Level::ERROR, error = %err, "Failed to validate command object");
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_extract() {
        let command_obj = CommandObject::extract("/start").unwrap();
        assert_eq!(command_obj.command.as_ref(), "start");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("/start@bot_username").unwrap();
        assert_eq!(command_obj.command.as_ref(), "start");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention.as_deref(), Some("bot_username"));
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("/start@").unwrap();
        assert_eq!(command_obj.command.as_ref(), "start");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("/@").unwrap();
        assert_eq!(command_obj.command.as_ref(), "@");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("@/").unwrap();
        assert_eq!(command_obj.command.as_ref(), "/");
        assert_eq!(command_obj.prefix, '@');
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("/@ arg1 arg2").unwrap();
        assert_eq!(command_obj.command.as_ref(), "@");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention, None);
        assert!(command_obj.args == Box::new(["arg1".into(), "arg2".into()]) as Box<_>);

        let command_obj = CommandObject::extract("/@bot_username").unwrap();
        assert_eq!(command_obj.command.as_ref(), "@bot_username");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention, None);
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("@start@bot_username").unwrap();
        assert_eq!(command_obj.command.as_ref(), "start");
        assert_eq!(command_obj.prefix, '@');
        assert_eq!(command_obj.mention.as_deref(), Some("bot_username"));
        assert_eq!(command_obj.args, [].into());

        let command_obj = CommandObject::extract("/start@bot_username arg1 arg2").unwrap();
        assert_eq!(command_obj.command.as_ref(), "start");
        assert_eq!(command_obj.prefix, '/');
        assert_eq!(command_obj.mention.as_deref(), Some("bot_username"));
        assert!(command_obj.args == Box::new(["arg1".into(), "arg2".into()]) as Box<_>);

        let command_obj = CommandObject::extract("Telegram says: 123").unwrap();
        assert_eq!(command_obj.command.as_ref(), "elegram");
        assert_eq!(command_obj.prefix, 'T');
        assert_eq!(command_obj.mention, None);
        assert!(command_obj.args == Box::new(["says:".into(), "123".into()]) as Box<_>);

        let command_obj = CommandObject::extract("One two").unwrap();
        assert_eq!(command_obj.command.as_ref(), "ne");
        assert_eq!(command_obj.prefix, 'O');
        assert_eq!(command_obj.mention, None);
        assert!(command_obj.args == Box::new(["two".into()]) as Box<_>);

        let command_obj = CommandObject::extract("Один два").unwrap();
        assert_eq!(command_obj.command.as_ref(), "дин");
        assert_eq!(command_obj.prefix, 'О');
        assert_eq!(command_obj.mention, None);
        assert!(command_obj.args == Box::new(["два".into()]) as Box<_>);
    }

    #[test]
    #[should_panic]
    fn test_command_extract_panic() {
        assert!(
            // Returns `None`, because prefix is empty
            CommandObject::extract("").is_some()
            // Returns `None`, because command is empty
            || CommandObject::extract("/").is_some()
        );
    }

    #[test]
    fn test_validate_prefix() {
        let command = Command::builder().prefix('/').command("start").build();

        let command_obj = CommandObject::extract("/start").unwrap();
        assert!(command.validate_prefix(&command_obj));

        let command_obj = CommandObject::extract("/start_other").unwrap();
        assert!(command.validate_prefix(&command_obj));

        let command_obj = CommandObject::extract("!start").unwrap();
        assert!(!command.validate_prefix(&command_obj));
    }

    #[test]
    fn test_validate_command() {
        let command = Command::builder()
            .prefix('/')
            .command("start")
            .ignore_case(false)
            .build();

        let command_obj = CommandObject::extract("/start").unwrap();
        assert!(command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/START").unwrap();
        assert!(!command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/stop").unwrap();
        assert!(!command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/STOP").unwrap();
        assert!(!command.validate_command(&command_obj));

        let command = Command::builder()
            .prefix('/')
            .command("start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start").unwrap();
        assert!(command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/START").unwrap();
        assert!(command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/stop").unwrap();
        assert!(!command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/STOP").unwrap();
        assert!(!command.validate_command(&command_obj));

        // Special case: `command` with uppercase letters and `ignore_case` is `true`
        // command should be converted to lowercase
        let command = Command::builder()
            .prefix('/')
            .command("Start")
            .ignore_case(true)
            .build();

        let command_obj = CommandObject::extract("/start").unwrap();
        assert!(command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/START").unwrap();
        assert!(command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/stop").unwrap();
        assert!(!command.validate_command(&command_obj));

        let command_obj = CommandObject::extract("/STOP").unwrap();
        assert!(!command.validate_command(&command_obj));
    }

    // TODO: Add tests for `validate_mention` method
}
