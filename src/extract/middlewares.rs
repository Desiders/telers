use super::FromEventAndContext;

use crate::{
    client::Bot,
    context::Context,
    error::ExtractionError,
    types::{Chat, Update, User},
};

use std::sync::Arc;

/// To be able to use [`User`] from [`crate::dispatcher::middlewares::outer::user_context::UserContext`] middleware as handler argument
impl<Client> FromEventAndContext<Client> for User {
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match context.get("event_user") {
            Some(user) => match user.downcast_ref::<User>() {
                Some(user) => Ok((*user).clone()),
                None => Err(ExtractionError::new(format!(
                    "Failed to downcast user, got `{user:?}` instead `User`"
                ))),
            },
            None => Err(ExtractionError::new(
                "Key `event_user` not found in the context",
            )),
        }
    }
}

/// To be able to use [`Chat`] from [`crate::dispatcher::middlewares::outer::user_context::UserContext`] middleware as handler argument
impl<Client> FromEventAndContext<Client> for Chat {
    type Error = ExtractionError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match context.get("event_chat") {
            Some(chat) => match chat.downcast_ref::<Chat>() {
                Some(chat) => Ok((*chat).clone()),
                None => Err(ExtractionError::new(format!(
                    "Failed to downcast chat, got `{chat:?}` instead `Chat`"
                ))),
            },
            None => Err(ExtractionError::new(
                "Key `event_chat` not found in the context",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Reqwest, dispatcher::event::telegram::handler::Handler};

    #[test]
    fn test_middlewares_extract() {
        fn assert_impl_handler<T: FromEventAndContext<Reqwest>>(_: impl Handler<T>) {}

        assert_impl_handler(|_: User| async { unreachable!() });
        assert_impl_handler(|_: Option<User>| async { unreachable!() });
        assert_impl_handler(|_: Result<User, ExtractionError>| async { unreachable!() });

        assert_impl_handler(|_: Chat| async { unreachable!() });
        assert_impl_handler(|_: Option<Chat>| async { unreachable!() });
        assert_impl_handler(|_: Result<Chat, ExtractionError>| async { unreachable!() });
    }
}
