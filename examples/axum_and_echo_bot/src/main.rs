use axum::{routing, Router as AxumRouter};
use std::sync::Arc;
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::CopyMessage,
    types::Message,
    utils::shutdown_signal,
    Bot, Dispatcher, Router as TelersRouter,
};
use tokio::{net::TcpListener, sync::Notify};

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.message_id(),
    ))
    .await?;
    Ok(())
}

async fn hello_world_handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = TelersRouter::new("main")
        .on_message(|observer| observer.register(Handler::new(echo_handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    let app = AxumRouter::new().route("/", routing::get(hello_world_handler));

    let shutdown = Arc::new(Notify::new());
    let _ = tokio::join!(
        tokio::spawn(run_server(app, shutdown.clone())),
        tokio::spawn(run_dispatcher(dispatcher, shutdown.clone())),
        tokio::spawn(handle_shutdown(shutdown))
    );
}

async fn run_server(app: AxumRouter, shutdown: Arc<Notify>) {
    let listener = TcpListener::bind(format!("{SERVER_HOST}:{SERVER_PORT}"))
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { shutdown.notified().await })
        .await
        .unwrap();
}

async fn run_dispatcher(dispatcher: Dispatcher, shutdown: Arc<Notify>) {
    dispatcher
        .run_polling()
        .with_graceful_shutdown(async move { shutdown.notified().await })
        .await
        .unwrap();
}

async fn handle_shutdown(shutdown: Arc<Notify>) {
    let () = shutdown_signal().await;
    shutdown.notify_waiters();
}
