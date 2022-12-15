use crate::{
    client::Bot, context::Context, error::app, extract::FromEventAndContext,
    filters::command::CommandObject, types::Update,
};

use futures::future::{err, ok, Ready};
use std::{sync::Arc, sync::RwLock};

impl FromEventAndContext for CommandObject {
    type Error = app::ErrorKind;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, _: Arc<Update>, context: Arc<RwLock<Context>>) -> Self::Future {
        context.read().unwrap().get("command").map_or(
            err(app::ErrorKind::ExtractError(
                "Key `command` not found in the context".into(),
            )),
            |command| {
                command.downcast_ref::<CommandObject>().map_or(
                    err(app::ErrorKind::ExtractError(
                        format!(
                            "Failed to downcast command, got `{command:?}` instead `CommandObject`"
                        )
                        .into(),
                    )),
                    |command| ok(command.clone()),
                )
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dispatcher::event::telegram::Handler;

    #[test]
    fn test_filters_extract() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

        assert_impl_handler(|_: CommandObject| async { unreachable!() });
    }
}
