use std::{borrow::Cow, collections::HashMap, vec};
use telers::{
    client::Bot,
    dispatcher::{
        event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
        middlewares::outer::FSMContext as FSMContextMiddleware,
        Dispatcher, Router,
    },
    enums::UpdateType,
    filters::State as StateFilter,
    fsm::{Context as FSMContext, MemoryStorage, Storage, Strategy},
    methods::SendMessage,
    types::Message,
};

#[derive(Clone)]
enum State {
    Name,
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

    fsm.set_state(State::Name).await.map_err(Into::into)?;

    Ok(EventReturn::Finish)
}

async fn name_handler<S: Storage>(bot: Bot, message: Message, fsm: FSMContext<S>) -> HandlerResult {
    // TODO: Add validation, e.g. check that name isn't empty
    let name = message.text.unwrap();

    let mut user_info = HashMap::new();
    user_info.insert("name", name.clone());

    fsm.set_data(user_info).await.map_err(Into::into)?;
    fsm.set_state(State::Language).await.map_err(Into::into)?;

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
    // TODO: Add validation, e.g. check that language isn't empty
    let language = message.text.unwrap();

    let user_info = fsm.get_data::<String>().await.map_err(Into::into)?;
    let name = user_info.get("name").unwrap();

    match language.to_lowercase().as_str() {
        "english" | "en" => {
            bot.send(
                &SendMessage::new(message.chat.id, format!("{name}, let's talk!")),
                None,
            )
            .await?;

            fsm.remove_state().await.map_err(Into::into)?;
            fsm.remove_data().await.map_err(Into::into)?;
        }
        _ => {
            bot.send(
                &SendMessage::new(
                    message.chat.id,
                    format!(
                        "{name}, I don't speak your language. Please, choose another language :(",
                    ),
                ),
                None,
            )
            .await?;

            // Go back to the previous state for choosing another language
            fsm.set_state(State::Language).await.map_err(Into::into)?;
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
    let storage = MemoryStorage::new();

    let fsm_context_middleware = FSMContextMiddleware::new(storage).strategy(Strategy::UserInChat);

    let mut router = Router::new("main");

    router
        .message
        .outer_middlewares
        .register(fsm_context_middleware);

    router
        .message
        .register(start_handler::<MemoryStorage>, vec![StateFilter::none()]);
    router.message.register(
        name_handler::<MemoryStorage>,
        vec![StateFilter::one(State::Name)],
    );
    router.message.register(
        language_handler::<MemoryStorage>,
        vec![StateFilter::one(State::Language)],
    );

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
