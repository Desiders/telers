//! This example shows how to use the `Stickers` and `StickerSet`
//! types and how to use the Telegram bot API methods for processing stickers.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package random_sticker
//! ```

use rand::Rng;

use telers::{
    enums::{ContentType as ContentTypeEnum, UpdateType},
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    filters::{Command, ContentType},
    methods::{GetStickerSet, SendMessage, SendSticker},
    types::{InputFile, Message, MessageSticker, MessageText},
    Bot, Dispatcher, Filter, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

/// This handler send greeting message to chat.
async fn start_handler(bot: Bot, message: MessageText) -> HandlerResult {
    bot.send(SendMessage::new(
        message.chat.id(),
        "Hello! Send me a sticker, and I'll send you a random sticker from this sticker pack!",
    ))
    .await?;

    Ok(EventReturn::Finish)
}

/// This handler get sticker set from sent sticker and send random sticker
/// from this sticker set.
async fn sticker_handler(bot: Bot, message: MessageSticker) -> HandlerResult {
    // get the the sticker set name of the sent sticker
    let Some(sticker_set_name) = message.sticker.set_name else {
        // if the sticker does not have the name of the sticker set to which it belongs,
        // then the sticker does not have a sticker set and exit from handler
        bot.send(SendMessage::new(
            message.chat.id(),
            "Sorry, but this sticker without sticker set. Try send another sticker.",
        ))
        .await?;

        return Ok(EventReturn::Finish);
    };

    // get sticker set using sent sticker set name
    let sticker_set = bot.send(GetStickerSet::new(sticker_set_name)).await?;

    // generate a random number no longer than the number of stickers in the sticker set
    let rand_index_of_sticker_set = rand::thread_rng().gen_range(0..sticker_set.stickers.len());

    // get a sticker by random index in a sticker pack
    let sticker_to_send = &sticker_set.stickers[rand_index_of_sticker_set];

    // send sticker by file ID of specify sticker
    bot.send(SendSticker::new(
        message.chat.id(),
        InputFile::id(sticker_to_send.file_id.as_ref()),
    ))
    .await?;

    Ok(EventReturn::Finish)
}

/// This handler process all non-sticker messages
async fn wrong_message_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Please, send me any sticker.",
    ))
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let bot = Bot::from_env_by_key("BOT_TOKEN");

    let mut router = Router::new("main");

    // register handler that sends a greeting message when you use commands `/start` and `/help`
    router
        .message
        .register(start_handler)
        .filter(ContentType::one(ContentTypeEnum::Text))
        .filter(Command::many(["help", "start"]));

    // register handler that process sent sticker and send random sticker from this sticker set
    router
        .message
        .register(sticker_handler)
        .filter(ContentType::one(ContentTypeEnum::Sticker));

    // register handler that handles all non-sticker messages
    router
        .message
        .register(wrong_message_handler)
        .filter(ContentType::one(ContentTypeEnum::Sticker).invert());

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
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
