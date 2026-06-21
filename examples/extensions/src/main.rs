use std::{convert::Infallible, future::Future};
use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{
        telegram::{Handler, HandlerResult},
        EventReturn,
    },
    filters::Command,
    methods::SendMessage,
    middlewares::outer::MiddlewareResponse,
    types::Message,
    Bot, Dispatcher, Extension, Extensions, FilterResult, Request, Router,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct NumData(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrData(&'static str);

#[derive(Clone)]
struct EmptyData;

async fn to_extensions_middleware(
    mut request: Request,
) -> Result<MiddlewareResponse, EventErrorKind> {
    request.extensions.insert(NumData(1));

    Ok((request, EventReturn::default()))
}

fn to_extensions_filter(request: &mut Request) -> impl Future<Output = FilterResult<Infallible>> {
    request.extensions.insert(StrData("1"));
    async move { Ok(true) }
}

async fn send_data_handler(
    bot: Bot,
    message: Message,
    // Data has been extracted automatically
    Extension(num_data): Extension<NumData>,
    Extension(str_data): Extension<StrData>,
    Extension(_): Extension<EmptyData>,
    // You can use extensions by yourself to extract data
    extensions: Extensions,
) -> HandlerResult<()> {
    assert_eq!(num_data, extensions.get::<NumData>().unwrap().clone());
    assert_eq!(str_data, extensions.get::<StrData>().unwrap().clone());

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("NumData: {:?}. StrData: {:?}", num_data.0, str_data.0),
    ))
    .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer
            .filter(to_extensions_filter)
            // Register middleware that adds data to extensions.
            // Be aware, we register middleware for message observer, so it will be called only for messages.
            // If you want to register middleware for any update, you should register it for update observer.
            .register_outer_middleware(to_extensions_middleware)
            // Register handler that sends data from extensions to chat
            .register(Handler::new(send_data_handler).filter(Command::one("data")))
    });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        // You also can register an extension using builder methods
        .extension(EmptyData)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
