//! This example shows how to setup webhooks for a bot using `axum` server.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package axum_webhook
//! ```

use std::fmt::Display;

use axum::Router as AxumRouter;
use telers::{
    enums::UpdateType,
    event::{
        simple,
        telegram::{self, HandlerResult},
    },
    methods::{CopyMessage, SetWebhook},
    types::Message,
    utils::shutdown_signal,
    webhooks::axum::{get_updates_router, UpdatesHandler},
    Bot, Dispatcher, Router as TelersRouter,
};
use tokio::{
    net::TcpListener,
    sync::broadcast::{channel, Receiver, Sender},
};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

const WEBHOOK_URL: &str = "https://example.com";
const HANDLER_PATH: &str = "/";
const SECRET_TOKEN: &str = "123";

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.message_id(),
    ))
    .await?;
    Ok(())
}

async fn set_webhook(
    bot: Bot,
    webhook_url: impl Display,
    handler_path: impl Display,
    secret_token: Option<impl Into<Box<str>>>,
) -> simple::HandlerResult {
    bot.send(
        SetWebhook::new(format!("{webhook_url}{handler_path}"))
            .allowed_update(UpdateType::Message)
            .secret_token_option(secret_token),
    )
    .await?;
    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let bot = Bot::from_env_by_key("BOT_TOKEN");

    let mut router = TelersRouter::new("main");
    router
        .message
        .register(telegram::Handler::new(echo_handler));

    router.startup.register(simple::Handler::new(
        set_webhook,
        (bot.clone(), WEBHOOK_URL, HANDLER_PATH, Some(SECRET_TOKEN)),
    ));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot.clone())
        .build();

    let app = AxumRouter::new().route(
        HANDLER_PATH,
        get_updates_router(UpdatesHandler::new(bot, dispatcher.clone()).secret_token(SECRET_TOKEN)),
    );

    let (shutdown_tx, _) = channel(1);

    let _ = tokio::join!(
        tokio::spawn(run_server(app, shutdown_tx.subscribe())),
        tokio::spawn(run_dispatcher(dispatcher, shutdown_tx.subscribe())),
        tokio::spawn(handle_shutdown(shutdown_tx))
    );
}

async fn run_server(app: AxumRouter, mut shutdown_rx: Receiver<()>) {
    let listener = TcpListener::bind(format!("{SERVER_HOST}:{SERVER_PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await
        .unwrap();
}

async fn run_dispatcher(dispatcher: Dispatcher, mut shutdown_rx: Receiver<()>) {
    dispatcher
        // We don't need polling
        .run_no_polling()
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.recv().await;
        })
        .await
        .unwrap();
}

async fn handle_shutdown(shutdown_tx: Sender<()>) {
    let () = shutdown_signal().await;
    let _ = shutdown_tx.send(());
}
