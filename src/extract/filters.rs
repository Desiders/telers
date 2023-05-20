use crate::{
    client::Bot, context::Context, error::ExtractionError, extract::FromEventAndContext,
    filters::command::CommandObject, types::Update,
};

use std::sync::Arc;

/// To be able to use [`CommandObject`] from [`crate::filters::command::Command`] filter as handler argument
impl<Client> FromEventAndContext<Client> for CommandObject {
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match context.get("command") {
            Some(command) => match command.downcast_ref::<Self>() {
                Some(command) => Ok((*command).clone()),
                None => Err(ExtractionError::new(format!(
                    "Failed to downcast command, got `{command:?}` instead `CommandObject`"
                ))),
            },
            None => Err(ExtractionError::new(
                "Key `command` not found in the context",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Reqwest, event::telegram::handler::Handler};

    #[test]
    fn test_filters_extract() {
        fn assert_impl_handler<T: FromEventAndContext<Reqwest>>(_: impl Handler<T>) {}

        assert_impl_handler(|_: CommandObject| async { unreachable!() });
        assert_impl_handler(|_: Option<CommandObject>| async { unreachable!() });
        assert_impl_handler(|_: Result<CommandObject, ExtractionError>| async { unreachable!() });
    }
}
