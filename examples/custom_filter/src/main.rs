//! This example shows how to create a simple filter
//! that allows only uppercase messages for the first handler and lowercase for the second.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! cd examples
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --bin custom_filter
//! ```

use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::SendMessage,
    types::{Message, Update},
    Bot, Context, Dispatcher, Filter, Router,
};

use async_trait::async_trait;

struct UppercaseFilter;

#[async_trait]
impl<Client> Filter<Client> for UppercaseFilter {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        update
            .text()
            .map_or(false, |text| text.to_uppercase() == text)
    }
}

async fn uppercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        &SendMessage::new(message.chat.id, "Uppercase message!"),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

async fn lowercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        &SendMessage::new(message.chat.id, "Lowercase message!"),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");
    router
        .message
        .register(uppercase_handler)
        .filter(UppercaseFilter);
    router.message.register(lowercase_handler).filter(
        // This filter will allow only lowercase messages.
        // Closure is used here, but you can use any type which implements `Filter` trait, such as `UppercaseFilter`,
        // but using closure can be not so convenient (lifetimes, type inference).
        |_: &Bot<_>, update: &Update, _: &Context| {
            let result = update
                .text()
                .map_or(false, |text| text.to_lowercase() == text);

            async move { result }
        },
    );
    router
        .message
        .register(|bot: Bot, message: Message| async move {
            bot.send(&SendMessage::new(message.chat.id, "Any message!"), None)
                .await?;

            Ok(EventReturn::Finish)
        })
        .filter(
            // This filter will allow messages, that are't uppercase and lowercase.
            // We use `Invert` filter to invert result of `UppercaseFilter` and closure,
            // and then combine them with `And` filter.
            UppercaseFilter.invert().and(
                // This filter will allow only lowercase messages.
                // we use closure here for example, but you can use any type which implements `Filter` trait, such as `UppercaseFilter`,
                // but using closure can be not so convenient (lifetimes, type inference).
                (|_: &Bot<_>, update: &Update, _: &Context| {
                    let result = update
                        .text()
                        .map_or(false, |text| text.to_lowercase() == text);

                    async move { result }
                })
                .invert(),
            ),
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
