//! This example shows how to use [`InputFile`] and send files by the bot.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package input_file
//! ```

use bytes::BytesMut;
use futures::{TryFutureExt as _, TryStreamExt as _};
use telers::{
    enums::UpdateType,
    errors::HandlerError,
    event::{simple, telegram, EventReturn, ToServiceProvider as _},
    methods::{SendMediaGroup, SendPhoto},
    router::Router,
    types::{InputFile, InputMediaPhoto, Message},
    Bot, Dispatcher,
};
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const CAT_URL: &str = "https://http.cat/images/200.jpg";
const CAT_FS_PATH: &str = "cat.jpg";

const DEFAULT_CAPACITY: usize = 64 * 1024; // 64 KiB

/// This handler will be called on bot startup.
/// It will download file from URL and save it to the file system as `cat.jpg` for further usage in handlers.
async fn on_startup() -> simple::HandlerResult {
    let response = reqwest::get(CAT_URL).await.map_err(|err| {
        event!(Level::ERROR, url = CAT_URL, error = %err, "Failed to download file");

        HandlerError::new(err)
    })?;

    let bytes = response.bytes().await.map_err(|err| {
        event!(Level::ERROR, error = %err, "Failed to read file bytes");

        HandlerError::new(err)
    })?;

    let mut file = tokio::fs::File::create(CAT_FS_PATH).await.map_err(|err| {
        event!(Level::ERROR, path = CAT_FS_PATH, error = %err, "Failed to create file");

        HandlerError::new(err)
    })?;

    tokio::io::copy(&mut bytes.as_ref(), &mut file)
        .await
        .map_err(|err| {
            event!(Level::ERROR, error = %err, path = CAT_FS_PATH, "Failed to write file");

            HandlerError::new(err)
        })?;

    event!(
        Level::INFO,
        url = CAT_URL,
        path = CAT_FS_PATH,
        "File downloaded and saved to file system"
    );

    Ok(())
}

/// This handler will be called on bot shutdown.
/// It will remove file from file system, which was downloaded on bot startup.
async fn on_shutdown() -> simple::HandlerResult {
    tokio::fs::remove_file(CAT_FS_PATH).await.map_err(|err| {
        event!(Level::ERROR, error = %err, path = CAT_FS_PATH, "Failed to remove file");

        HandlerError::new(err)
    })?;

    event!(
        Level::INFO,
        path = CAT_FS_PATH,
        "File removed from file system"
    );

    Ok(())
}

async fn input_file_handler(bot: Bot, message: Message) -> telegram::HandlerResult {
    // Using `InputFile::url` to send file by URL
    let cat_url_input_file = InputFile::url(CAT_URL);

    // Using `InputFile::fs` to send file by path in file system
    let cat_fs_input_file = InputFile::fs(CAT_FS_PATH);

    // Using `InputFile::buffered` to send file by bytes
    let cat_buffered_input_file =
        InputFile::buffered(tokio::fs::read(CAT_FS_PATH).await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Failed to read file bytes");

            HandlerError::new(err)
        })?);

    // Using `InputFile::stream` to send file by stream
    let cat_stream_input_file = InputFile::stream(Box::pin(
        tokio::fs::File::open(CAT_FS_PATH)
            .map_ok(move |file| {
                FramedRead::with_capacity(file, BytesCodec::new(), DEFAULT_CAPACITY)
                    .map_ok(BytesMut::freeze)
            })
            .try_flatten_stream(),
    ));

    let result_message = bot
        .send(SendMediaGroup::new(
            message.chat_id(),
            [
                InputMediaPhoto::new(cat_url_input_file).caption("Cat by URL"),
                InputMediaPhoto::new(cat_fs_input_file).caption("Cat by file system"),
                InputMediaPhoto::new(cat_buffered_input_file).caption("Cat by bytes"),
                InputMediaPhoto::new(cat_stream_input_file).caption("Cat by stream"),
            ],
        ))
        .await?;

    // Using `InputFile::id` to send file by telegram file ID.
    // We can get file ID from result message and send it again by ID.
    let cat_id_input_file = InputFile::id(
        result_message[0]
            .photo
            .as_ref()
            .expect("Photo is empty")
            .last()
            .expect("Photo is empty")
            .file_id
            .as_ref(),
    );

    bot.send(
        &SendPhoto::new(message.chat_id(), cat_id_input_file).caption("Cat by telegram file ID"),
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");
    router.message.register(input_file_handler);

    router.startup.register(on_startup, ());
    router.shutdown.register(on_shutdown, ());

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
