//! This example shows how to use the `Stickers` and `StickerSet`
//! types and how to use the Telegram bot API methods for processing stickers.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package random_sticker
//! ```

use rand::RngExt as _;
use telers::{
    enums::{
        MessageType::{Sticker, Text},
        UpdateType,
    },
    event::telegram::{Handler, HandlerResult},
    filters::{Command, MessageType},
    methods::{GetStickerSet, SendMessage, SendSticker},
    types::{InputFile, Message, MessageSticker, MessageText},
    Bot, Dispatcher, Filter, Router,
};

/// This handler send greeting message to chat.
async fn start_handler(bot: Bot, message: MessageText) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat.id(),
        "Hello! Send me a sticker, and I'll send you a random sticker from this sticker pack!",
    ))
    .await?;

    Ok(())
}

/// This handler get sticker set from sent sticker and send random sticker from this sticker set.
async fn sticker_handler(bot: Bot, message: MessageSticker) -> HandlerResult<()> {
    // get the the sticker set name of the sent sticker
    let Some(sticker_set_name) = message.sticker.set_name() else {
        // if the sticker does not have the name of the sticker set to which it belongs,
        // then the sticker does not have a sticker set and exit from handler
        bot.send(SendMessage::new(
            message.chat.id(),
            "Sorry, but this sticker without sticker set. Try send another sticker.",
        ))
        .await?;
        return Ok(());
    };

    // get sticker set using sent sticker set name
    let sticker_set = bot.send(GetStickerSet::new(sticker_set_name)).await?;
    // generate a random number no longer than the number of stickers in the sticker set
    let rand_index_of_sticker_set = rand::rng().random_range(0..sticker_set.stickers.len());
    // get a sticker by random index in a sticker pack
    let sticker_to_send = &sticker_set.stickers[rand_index_of_sticker_set];

    // send sticker by file ID of specify sticker
    bot.send(SendSticker::new(
        message.chat.id(),
        InputFile::id(sticker_to_send.file_id()),
    ))
    .await?;
    Ok(())
}

/// This handler process all non-sticker messages
async fn wrong_message_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Please, send me any sticker.",
    ))
    .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer.registers([
            // register handler that sends a greeting message when you use commands `/start` and `/help`
            Handler::new(start_handler)
                .filter(MessageType::one(Text))
                .filter(Command::many(["help", "start"])),
            // register handler that process sent sticker and send random sticker from this sticker set
            Handler::new(sticker_handler).filter(MessageType::one(Sticker)),
            // register handler that handles all non-sticker messages
            Handler::new(wrong_message_handler).filter(MessageType::one(Sticker).invert()),
        ])
    });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
