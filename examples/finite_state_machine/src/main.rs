//! This example shows how to use [`FSMContextMiddleware`] and [`StateFilter`] to implement a simple conversation with a user.
//! In this example we will ask user for his name and language,
//! if languase isn't "acceptable", we will ask him to choose another one.
//! After that all steps will be finished and we will send a message with user's name and language to him and
//! finish conversation.
//!
//! In this example we will use [`MemoryStorage`] as storage for [`FSMContextMiddleware`], but you can use any storage,
//! which implements [`Storage`] trait.
//! This storage isn't recommended for production use, because it doesn't persist data between restarts, but it's
//! useful for testing and example purposes and easy to use.
//! We the same use [`StateFilter`] to filter states and call handlers only when state is equal to some value.
//!
//! More information about FSM you can find in [`telers::fsm`] and [`FSMContextMiddleware`] documentation.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! cd examples
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --bin finite_state_machine
//! ```

use std::borrow::Cow;
use telers::{
    enums::ContentType as ContentTypeEnum,
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    filters::{Command, ContentType, State as StateFilter},
    fsm::{Context as FSMContext, MemoryStorage, Storage, Strategy},
    methods::SendMessage,
    middlewares::outer::FSMContext as FSMContextMiddleware,
    types::Message,
    Bot, Dispatcher, Router,
};

/// State of conversation.
///
/// We use it to determine what we should ask user next and implement [`From<State>`] for [`Cow<'static, str>`]
/// for possible save this state in [`Storage`].
/// We also implement [`PartialEq<&str>`] for comparing states with other in [`StateFilter`].
#[derive(Clone)]
enum State {
    /// User is asked for his name
    Name,
    /// User is asked for his language
    Language,
}

impl State {
    const fn as_str(&self) -> &'static str {
        match self {
            State::Name => "name",
            State::Language => "language",
        }
    }
}

// Implementation `PartialEq<&str>` and `From<State> for Cow<'static, str>` for `State` is optional,
// but it's useful for using enum as state without boilerplate code as `State::Name.as_str()`,
// because we can use `State::Name` directly.
impl PartialEq<&str> for State {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl From<State> for Cow<'static, str> {
    fn from(state: State) -> Self {
        Cow::Borrowed(state.as_str())
    }
}

async fn start_handler<S: Storage>(
    bot: Bot,
    message: Message,
    fsm: FSMContext<S>,
) -> HandlerResult {
    bot.send(
        &SendMessage::new(message.chat.id, "Hello! What's your name?"),
        None,
    )
    .await?;

    // We set state to `State::Name` to point that we are waiting for user's name.
    // `name_handler` will be called when user will send message,
    // because we set `State::Name` as state and this handler is registered for this state
    fsm.set_state(State::Name).await.map_err(Into::into)?;

    Ok(EventReturn::Finish)
}

async fn name_handler<S: Storage>(bot: Bot, message: Message, fsm: FSMContext<S>) -> HandlerResult {
    // `unwrap` is safe here, because we set content type filter
    let name = message.text.unwrap();

    // Save name to FSM storage, because we will need it in `language_handler`
    fsm.set_value("name", name.clone())
        .await
        .map_err(Into::into)?;
    // Set state to `State::Language` to point that we are waiting for user's language
    fsm.set_state(State::Language).await.map_err(Into::into)?;

    // Usually state and data set to FSM storage before sending message to user,
    // because we want to be sure that we will receive message from user in the same state
    // (user can send message to bot before we set state and data to FSM storage, but it's rare case)

    bot.send(
        &SendMessage::new(
            message.chat.id,
            format!("Nice to meet you, {name}! What's your native language?"),
        ),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

async fn language_handler<S: Storage>(
    bot: Bot,
    message: Message,
    fsm: FSMContext<S>,
) -> HandlerResult {
    // `unwrap` is safe here, because we set content type filter
    let language = message.text.unwrap();

    // Get user's name from FSM storage
    // TODO: Add validation, e.g. check that name isn't empty
    let name: String = fsm
        .get_value("name")
        .await
        .map_err(Into::into)?
        .expect("Name should be set");

    // Check if user's language is acceptable
    match language.to_lowercase().as_str() {
        "english" | "en" => {
            bot.send(
                &SendMessage::new(message.chat.id, format!("{name}, let's talk!")),
                None,
            )
            .await?;

            // Remove state and data from FSM storage, because we don't need them anymore
            fsm.finish().await.map_err(Into::into)?;
        }
        _ => {
            bot.send(
                &SendMessage::new(
                    message.chat.id,
                    format!("{name}, I don't speak your language. Please, choose another :(",),
                ),
                None,
            )
            .await?;

            // We don't need this, because `State::Language` is already set and doesn't change automatically
            // fsm.set_state(State::Language).await.map_err(Into::into)?;
        }
    };

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    // You can use any storage, which implements `Storage` trait
    let storage = MemoryStorage::new();

    let mut router = Router::new("main");

    // Register fsm middleware for possible managing states and fsm data (e.g. user's name and language for this example)
    router
        .update
        .outer_middlewares
        .register(FSMContextMiddleware::new(storage).strategy(Strategy::UserInChat));

    router
        .message
        .register(start_handler::<MemoryStorage>)
        .filter(Command::one("start"))
        .filter(StateFilter::none());
    router
        .message
        .register(name_handler::<MemoryStorage>)
        .filter(ContentType::one(ContentTypeEnum::Text))
        .filter(StateFilter::one(State::Name));
    router
        .message
        .register(language_handler::<MemoryStorage>)
        .filter(ContentType::one(ContentTypeEnum::Text))
        .filter(StateFilter::one(State::Language));

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => log::info!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
