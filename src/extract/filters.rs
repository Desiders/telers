use crate::{
    client::Bot, context::Context, error::ExtractionError, extract::FromEventAndContext,
    filters::command::CommandObject, types::Update,
};

use std::sync::Arc;

impl FromEventAndContext for CommandObject {
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match context.get("command") {
            Some(command) => match command.downcast_ref::<CommandObject>() {
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
    use crate::dispatcher::event::telegram::handler::Handler;

    #[test]
    fn test_filters_extract() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

        assert_impl_handler(|_: CommandObject| async { unreachable!() });
    }
}
