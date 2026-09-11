use telers::{
    callback_data::{CallbackData as _, CallbackDataError},
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    filters::{CallbackData as CallbackDataFilter, CommandStart},
    methods::{AnswerCallbackQuery, EditMessageText, SendMessage},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot, CallbackData, Dispatcher, Router,
};

/// Callback data of the buttons that change the counter
/// # Notes
/// [`CallbackData`] macros generates `pack` and `unpack` methods, which convert the struct
/// to the string `counter:<value>:<step>` (e.g. `counter:5:-1`) and back
#[derive(Debug, Clone, CallbackData)]
#[callback_data(prefix = "counter")]
struct Counter {
    value: i64,
    step: i64,
}

fn counter_keyboard(value: i64) -> Result<InlineKeyboardMarkup, CallbackDataError> {
    let decrement = Counter {
        value,
        step: -1,
    }
    .pack()?;
    let increment = Counter {
        value,
        step: 1,
    }
    .pack()?;

    Ok(InlineKeyboardMarkup::new([[
        InlineKeyboardButton::new("-1").callback_data(decrement),
        InlineKeyboardButton::new("+1").callback_data(increment),
    ]]))
}

async fn start_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(
        SendMessage::new(message.chat().id(), "Counter: 0").reply_markup(counter_keyboard(0)?),
    )
    .await?;

    Ok(())
}

async fn counter_handler(
    bot: Bot,
    callback_query: CallbackQuery,
    Counter {
        value,
        step,
    }: Counter,
) -> HandlerResult<()> {
    let value = value + step;

    if let Some(message) = &callback_query.message {
        bot.send(
            EditMessageText::new()
                .chat_id(message.chat().id())
                .message_id(message.message_id())
                .text(format!("Counter: {value}"))
                .reply_markup(counter_keyboard(value)?),
        )
        .await?;
    }

    bot.send(AnswerCallbackQuery::new(callback_query.id))
        .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_message(|observer| {
            observer.register(Handler::new(start_handler).filter(CommandStart::default()))
        })
        .on_callback_query(|observer| {
            observer.register(
                Handler::new(counter_handler).filter(CallbackDataFilter::<Counter>::new()),
            )
        });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates([UpdateType::Message, UpdateType::CallbackQuery])
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
