use telers::{
    enums::{MessageType::Text, UpdateType},
    event::telegram::{Handler, HandlerResult},
    filters::{CommandStart, MessageType, State as StateFilter},
    fsm::{Context as FSMContext, MemoryStorage, Strategy::UserInChatAndConnection},
    methods::SendMessage,
    middlewares::outer::FSMContext as FSMContextMiddleware,
    types::{Message, MessageText},
    Bot, Dispatcher, Router,
};

/// Shorthand for the FSM context with in-memory storage. Replace `MemoryStorage` with your own `Storage` impl if needed.
type Fsm = FSMContext<MemoryStorage>;

/// State of conversation.
///
/// We use it to determine what we should ask user next and implement [`From<State>`] for [`str`]
/// for possible save this state in `Storage`.
/// We also implement [`PartialEq<&str>`] for comparing states with other in [`StateFilter`].
#[derive(Clone)]
enum State {
    /// User is asked for his name
    Name,
    /// User is asked for his language
    Language,
}

impl AsRef<str> for State {
    fn as_ref(&self) -> &str {
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
        self.as_ref() == *other
    }
}

async fn start_handler(bot: Bot, message: Message, fsm: Fsm) -> HandlerResult<()> {
    bot.send(
        SendMessage::new(message.chat().id(), "Hello! What's your name?").business_connection_id(
            message.business_connection_id().expect(
                "Business connection id should be set, because we registered this handler for \
                 business connections only",
            ),
        ),
    )
    .await?;

    // We set state to `State::Name` to point that we are waiting for user's name.
    // `name_handler` will be called when user will send message,
    // because we set `State::Name` as state and this handler is registered for this state
    fsm.set_state(State::Name).await?;
    Ok(())
}

async fn name_handler(bot: Bot, message: MessageText, fsm: Fsm) -> HandlerResult<()> {
    let name = message.text;

    // Save name to FSM storage, because we will need it in `language_handler`
    fsm.set_value("name", name.clone()).await?;
    // Set state to `State::Language` to point that we are waiting for user's language
    fsm.set_state(State::Language).await?;

    // Usually state and data set to FSM storage before sending message to user,
    // because we want to be sure that we will receive message from user in the same state
    // (user can send message to bot before we set state and data to FSM storage, but it's rare case)

    bot.send(
        SendMessage::new(
            message.chat.id(),
            format!("Nice to meet you, {name}! What's your native language?"),
        )
        .business_connection_id(message.business_connection_id.expect(
            "Business connection id should be set, because we registered this handler for \
             business connections only",
        )),
    )
    .await?;
    Ok(())
}

async fn language_handler(bot: Bot, message: MessageText, fsm: Fsm) -> HandlerResult<()> {
    let language = message.text;

    // Get user's name from FSM storage
    // TODO: Add validation, e.g. check that name isn't empty
    let name: Box<str> = fsm.get_value("name").await?.expect("Name should be set");

    // Check if user's language is acceptable
    match language.to_lowercase().as_str() {
        "english" | "en" => {
            bot.send(
                SendMessage::new(message.chat.id(), format!("{name}, let's talk!"))
                    .business_connection_id(message.business_connection_id.expect(
                        "Business connection id should be set, because we registered this handler \
                         for business connections only",
                    )),
            )
            .await?;

            // Remove state and data from FSM storage, because we don't need them anymore
            fsm.finish().await?;
        }
        _ => {
            bot.send(
                SendMessage::new(
                    message.chat.id(),
                    format!("{name}, I don't speak your language. Please, choose another :(",),
                )
                .business_connection_id(message.business_connection_id.expect(
                    "Business connection id should be set, because we registered this handler for \
                     business connections only",
                )),
            )
            .await?;

            // We don't need this, because `State::Language` is already set and doesn't change automatically
            // fsm.set_state(State::Language).await?;
        }
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    // You can use any storage, which implements `Storage` trait
    let storage = MemoryStorage::new();

    let router = Router::new("main")
        // Register fsm middleware for possible managing states and fsm data (e.g. user's name and language for this example)
        // We use here `Strategy::UserInChatAndConnection` to have different states for business connections and other chats
        .on_update(|observer| {
            observer.register_outer_middleware(
                FSMContextMiddleware::new(storage).strategy(UserInChatAndConnection),
            )
        })
        .on_business_message(|observer| {
            observer.registers([
                Handler::new(start_handler)
                    .filter(CommandStart::default())
                    .filter(StateFilter::none()),
                Handler::new(name_handler)
                    .filter(MessageType::one(Text))
                    .filter(StateFilter::one(State::Name)),
                Handler::new(language_handler)
                    .filter(MessageType::one(Text))
                    .filter(StateFilter::one(State::Language)),
            ])
        });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::BusinessMessage)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
