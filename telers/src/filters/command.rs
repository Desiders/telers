use super::{Filter, FilterResult};
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

use crate::utils::decode_payload;

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
    #[inline]
    fn from(text: Cow<'static, str>) -> Self {
        Self::Text(text)
    }
}

impl From<&'static str> for PatternType {
    #[inline]
    fn from(text: &'static str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}

impl From<BotCommand> for PatternType {
    #[inline]
    fn from(command: BotCommand) -> Self {
        Self::Object(command)
    }
}

impl From<Regex> for PatternType {
    #[inline]
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
    /// # Panics
    /// If `ignore_case` is set and a [`Regex`] pattern can't be recompiled with the
    /// case-insensitive `(?i)` flag
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
                        PatternType::Regex(
                            Regex::new(&format!("(?i){regex}"))
                                .expect("Failed to compile regex with (?i) flag"),
                        )
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
    #[inline]
    #[must_use]
    pub fn one(command: impl Into<PatternType>) -> Self {
        Self::builder().command(command).build()
    }

    /// Creates a new [`Command`] filter with pass command and prefix
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder.
    /// - By default, the prefix is `/`, so you can use [`Command::one`] instead. Use this method if you want to change the it.
    #[inline]
    #[must_use]
    pub fn one_with_prefix(command: impl Into<PatternType>, prefix: char) -> Self {
        Self::builder().command(command).prefix(prefix).build()
    }

    /// Creates a new [`Command`] filter with pass commands
    /// # Notes
    /// - This method is just a shortcut to create a filter using the builder
    /// - By default, the prefix is `/`. If you want to change it, use [`Command::many_with_prefix`] instead.
    #[inline]
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
    #[inline]
    #[must_use]
    pub fn many_with_prefix<T, I>(commands: I, prefix: char) -> Self
    where
        T: Into<PatternType>,
        I: IntoIterator<Item = T>,
    {
        Self::builder().commands(commands).prefix(prefix).build()
    }

    #[inline]
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Default for Command {
    #[inline]
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
    #[inline]
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

    #[inline]
    #[must_use]
    pub fn prefix(self, val: char) -> Self {
        Self {
            prefix: val,
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn ignore_case(self, val: bool) -> Self {
        Self {
            ignore_case: val,
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn ignore_mention(self, val: bool) -> Self {
        Self {
            ignore_mention: val,
            ..self
        }
    }

    #[inline]
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
    #[inline]
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
    #[inline]
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
                // `unwrap` is safe here, because bot always has username.
                // Telegram usernames are case-insensitive, so compare accordingly.
                user.username.unwrap().eq_ignore_ascii_case(mention)
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

/// Deep-link payload validation rules for [`CommandStart`].
#[derive(Debug, Clone, Copy)]
pub enum DeepLink {
    /// The `/start` command must carry a plain text payload.
    Plain,
    /// The `/start` command must carry a base64url-encoded payload;
    /// it is decoded and replaces the raw arguments in the [`CommandObject`].
    Encoded,
}

/// Filter for `/start` commands with optional deep-link payload validation.
///
/// It is a specialized [`Command`] filter that always matches the `start` command.
///
/// # Notes
/// You can use parsed command using [`CommandObject`] struct in handler arguments,
/// or get it from [`crate::context::Context`] by `command` key.
/// If [`DeepLink::Encoded`] is used, the decoded payload replaces the raw command arguments.
#[derive(Debug, Clone)]
pub struct CommandStart {
    command: Command,
    deep_link: Option<DeepLink>,
}

impl Default for CommandStart {
    #[inline]
    fn default() -> Self {
        Self::new(None, '/', false, false)
    }
}

impl CommandStart {
    /// Creates a new [`CommandStart`] filter
    /// # Arguments
    /// * `deep_link` - Deep-link payload validation rule ([`None`] matches any `/start` command)
    /// * `prefix` - Command prefix
    /// * `ignore_case` - Ignore other command case
    /// * `ignore_mention` - Ignore bot mention
    #[must_use]
    pub fn new(
        deep_link: Option<DeepLink>,
        prefix: char,
        ignore_case: bool,
        ignore_mention: bool,
    ) -> Self {
        Self {
            command: Command::builder()
                .command("start")
                .prefix(prefix)
                .ignore_case(ignore_case)
                .ignore_mention(ignore_mention)
                .build(),
            deep_link,
        }
    }

    #[inline]
    #[must_use]
    pub fn builder() -> StartBuilder {
        StartBuilder::new()
    }
}

/// Builder for the [`CommandStart`] filter
#[derive(Debug, Clone)]
pub struct StartBuilder {
    deep_link: Option<DeepLink>,
    prefix: char,
    ignore_case: bool,
    ignore_mention: bool,
}

impl StartBuilder {
    #[inline]
    #[must_use]
    pub fn new() -> StartBuilder {
        Self::default()
    }

    #[inline]
    #[must_use]
    pub fn deep_link(mut self, val: Option<DeepLink>) -> Self {
        self.deep_link = val;
        self
    }

    #[inline]
    #[must_use]
    pub fn prefix(mut self, val: char) -> Self {
        self.prefix = val;
        self
    }

    #[inline]
    #[must_use]
    pub fn ignore_case(mut self, val: bool) -> Self {
        self.ignore_case = val;
        self
    }

    #[inline]
    #[must_use]
    pub fn ignore_mention(mut self, val: bool) -> Self {
        self.ignore_mention = val;
        self
    }

    #[inline]
    #[must_use]
    pub fn build(self) -> CommandStart {
        CommandStart::new(
            self.deep_link,
            self.prefix,
            self.ignore_case,
            self.ignore_mention,
        )
    }
}

impl Default for StartBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            deep_link: None,
            prefix: '/',
            ignore_case: false,
            ignore_mention: false,
        }
    }
}

impl<Client> Filter<Client> for CommandStart
where
    Client: Session + 'static,
{
    type Error = SessionErrorKind;

    #[instrument]
    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        let Some(message) = request.update.message() else {
            return Ok(false);
        };
        let Some(text) = message.text().or(message.caption()) else {
            return Ok(false);
        };
        let Some(command) = CommandObject::extract(text) else {
            return Ok(false);
        };

        match self
            .command
            .validate_command_object(&command, &request.bot)
            .await
        {
            Ok(true) => {}
            Ok(false) => return Ok(false),
            Err(err) => {
                event!(Level::ERROR, error = %err, "Failed to validate command object");
                return Err(err);
            }
        }

        match self.deep_link {
            None => {
                request.context.insert("command", command);
                Ok(true)
            }
            Some(DeepLink::Plain) => {
                if command.args.is_empty() {
                    return Ok(false);
                }
                request.context.insert("command", command);
                Ok(true)
            }
            Some(DeepLink::Encoded) => {
                if command.args.is_empty() {
                    return Ok(false);
                }
                let Ok(payload) = decode_payload(command.args.join(" ").as_str()) else {
                    return Ok(false);
                };
                let mut command = command;
                command.args = Box::new([payload.into_boxed_str()]);
                request.context.insert("command", command);
                Ok(true)
            }
        }
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
        // Split on any run of whitespace (spaces, tabs, newlines), skipping empties — a
        // command is commonly followed by a newline (`/start\nfoo`), and splitting only on
        // a single `' '` left the newline stuck to the command so it never matched.
        let mut parts = text.split_whitespace();
        let full_command = parts.next()?;
        let args = parts.map(|arg| arg.to_owned().into_boxed_str()).collect();

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
    type Error = SessionErrorKind;

    #[instrument]
    async fn check(&mut self, request: &mut Request<Client>) -> FilterResult<Self::Error> {
        let Some(message) = request.update.message() else {
            return Ok(false);
        };
        let Some(text) = message.text().or(message.caption()) else {
            return Ok(false);
        };
        let Some(command) = CommandObject::extract(text) else {
            return Ok(false);
        };

        match self.validate_command_object(&command, &request.bot).await {
            Ok(result) => {
                if result {
                    request.context.insert("command", command);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => {
                event!(Level::ERROR, error = %err, "Failed to validate command object");
                Err(err)
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

    #[test]
    fn test_validate_command_regex_ignore_case() {
        let command = Command::builder()
            .prefix('/')
            .command(Regex::new("Start").unwrap())
            .ignore_case(true)
            .build();

        for input in ["/start", "/START", "/Start"] {
            let command_obj = CommandObject::extract(input).unwrap();
            assert!(
                command.validate_command(&command_obj),
                "ignore_case regex should match {input}"
            );
        }

        let command_obj = CommandObject::extract("/stop").unwrap();
        assert!(!command.validate_command(&command_obj));
    }

    // TODO: Add tests for `validate_mention` method

    #[cfg(test)]
    mod command_start_tests {
        use super::*;
        use crate::{
            client::Reqwest,
            types::{ChatPrivate, MessageText, Update, UpdateMessage},
        };

        use std::sync::Arc;

        fn request(text: &str) -> Request<Reqwest> {
            Request {
                update: Arc::new(Update::Message(UpdateMessage::new(
                    0,
                    MessageText::new(0, 0, ChatPrivate::new(0), text),
                ))),
                bot: crate::Bot::default(),
                context: crate::Context::default(),
                extensions: crate::Extensions::default(),
            }
        }

        #[tokio::test]
        async fn test_rejects_other_commands() {
            let mut req = request("/help");
            assert!(!CommandStart::new(None, '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_rejects_non_commands() {
            let mut req = request("just text");
            assert!(!CommandStart::new(None, '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_rejects_wrong_prefix() {
            let mut req = request("!start ref123");
            assert!(!CommandStart::new(None, '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_matches_any_start_with_none() {
            let mut req = request("/start");
            assert!(CommandStart::new(None, '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_plain_requires_payload() {
            let mut req = request("/start");
            assert!(!CommandStart::new(Some(DeepLink::Plain), '/', false, false)
                .check(&mut req)
                .await
                .unwrap());

            let mut req = request("/start ref123");
            assert!(CommandStart::new(Some(DeepLink::Plain), '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_encoded_validates_and_decodes() {
            let mut req = request("/start aGVsbG8gd29ybGQ");
            assert!(
                CommandStart::new(Some(DeepLink::Encoded), '/', false, false)
                    .check(&mut req)
                    .await
                    .unwrap()
            );
            let command = req.context.get::<CommandObject>("command").unwrap();
            assert_eq!(
                command.args,
                Box::new(["hello world".to_string().into_boxed_str()]) as Box<_>
            );

            let mut req = request("/start ref123");
            assert!(
                !CommandStart::new(Some(DeepLink::Encoded), '/', false, false)
                    .check(&mut req)
                    .await
                    .unwrap()
            );
        }

        #[tokio::test]
        async fn test_ignore_case() {
            let mut req = request("/START ref123");
            assert!(CommandStart::new(Some(DeepLink::Plain), '/', true, false)
                .check(&mut req)
                .await
                .unwrap());

            let mut req = request("/START ref123");
            assert!(!CommandStart::new(Some(DeepLink::Plain), '/', false, false)
                .check(&mut req)
                .await
                .unwrap());
        }

        #[tokio::test]
        async fn test_builder() {
            let mut req = request("/start aGVsbG8gd29ybGQ");
            assert!(CommandStart::builder()
                .deep_link(Some(DeepLink::Encoded))
                .ignore_case(true)
                .build()
                .check(&mut req)
                .await
                .unwrap());

            let mut req = request("/START aGVsbG8gd29ybGQ");
            assert!(CommandStart::builder()
                .deep_link(Some(DeepLink::Encoded))
                .ignore_case(true)
                .build()
                .check(&mut req)
                .await
                .unwrap());

            let mut req = request("!start aGVsbG8gd29ybGQ");
            assert!(!CommandStart::builder()
                .deep_link(Some(DeepLink::Encoded))
                .build()
                .check(&mut req)
                .await
                .unwrap());
        }
    }
}
