//! This example shows how to set a custom bot HTTP client.
//!
//! Usually you don't need to use a custom client, because [`telers`] provides default client,
//! but if you want to use a custom client, you can do it by using [`Bot::with_client`] method and use it in handlers.
//!
//! You can use any client, which implements [`Session`] trait and use it in handlers:
//! ```ignore
//! async fn handler(bot: Bot<impl Session>) -> HandlerResult {
//!    // ...
//! }
//! ```
//! You the same can use another client and use it directly:
//! ```ignore
//! async fn handler(bot: Bot<SomeClientDirectly>) -> HandlerResult {
//!     // ...
//! }
//! ```
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package bot_http_client
//! ```
//!
//! [`Bot::with_client`]: telers::Bot#method.with_client

use std::borrow::Cow;
use telers::{
    client::{session::ClientResponse, telegram, Session},
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::{CopyMessage, TelegramMethod},
    types::Message,
    Bot, Dispatcher, Router,
};

#[derive(Clone)]
struct CustomClient {
    api: Cow<'static, telegram::APIServer>,
}

impl Default for CustomClient {
    fn default() -> Self {
        Self {
            api: Cow::Borrowed(&telegram::PRODUCTION),
        }
    }
}

impl Session for CustomClient {
    fn api(&self) -> &telegram::APIServer {
        &self.api
    }

    async fn send_request<Client, T>(
        &self,
        _bot: &Bot<Client>,
        _method: T,
        _timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        unimplemented!(
            "Send request is not implemented for custom client. You can use default client or \
             implement it for your custom client."
        )
    }
}

async fn echo_handler(bot: Bot<impl Session>, message: Message) -> HandlerResult<()> {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.message_id(),
    ))
    .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN env variable is not set!");
    let bot = Bot::with_client(token, CustomClient::default());

    let mut router = Router::new("main");
    router.message.register(Handler::new(echo_handler));

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
