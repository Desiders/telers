use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the list of the bot's commands. See this manual for more details about bot commands. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmycommands>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Box<[crate::types::BotCommand]>,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`BotCommandScopeDefault`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<Box<str>>,
}
impl SetMyCommands {
    /// Creates a new `SetMyCommands`.
    ///
    /// # Arguments
    /// * `commands` - A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::BotCommand>, T0: IntoIterator<Item = T0Item>>(
        commands: T0,
    ) -> Self {
        Self {
            commands: commands.into_iter().map(Into::into).collect(),
            scope: None,
            language_code: None,
        }
    }

    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn commands<TItem: Into<crate::types::BotCommand>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.commands = this
            .commands
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn command<T: Into<crate::types::BotCommand>>(self, val: T) -> Self {
        let mut this = self;
        this.commands = this
            .commands
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`BotCommandScopeDefault`].
    #[must_use]
    pub fn scope<T: Into<crate::types::BotCommandScope>>(self, val: T) -> Self {
        let mut this = self;
        this.scope = Some(val.into());
        this
    }

    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [`BotCommandScopeDefault`].
    #[must_use]
    pub fn scope_option<T: Into<crate::types::BotCommandScope>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.scope = val.map(Into::into);
        this
    }

    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[must_use]
    pub fn language_code<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.language_code = Some(val.into());
        this
    }

    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[must_use]
    pub fn language_code_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.language_code = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetMyCommands {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyCommands", self, None)
    }
}
