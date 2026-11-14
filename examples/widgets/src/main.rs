use telers::{
    enums::{ChatType::Private, UpdateType},
    event::telegram::{Handler, HandlerResult},
    filters::{ChatType, CommandStart},
    methods::{AnswerWidgetQuery, SendWidget},
    types::{Message, WidgetQuery},
    Bot, Dispatcher, Router,
};

const WIDGET_HTML: &str = include_str!("widget.html");

async fn start_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(
        SendWidget::new(message.chat().id(), WIDGET_HTML)
            .caption("Press the button inside the widget to say hello.")
            .allow_queries(true),
    )
    .await?;

    Ok(())
}

async fn widget_handler(bot: Bot, query: WidgetQuery) -> HandlerResult<()> {
    let answer = match query.data.as_ref() {
        "hello" => format!("Hello, {}!", query.from.first_name),
        _ => "Unknown request".to_owned(),
    };

    bot.send(AnswerWidgetQuery::new(query.id).data(answer))
        .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_message(|observer| {
            observer.register(
                Handler::new(start_handler)
                    .filter(CommandStart::default())
                    .filter(ChatType::one(Private)),
            )
        })
        .on_widget_query(|observer| observer.register(Handler::new(widget_handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates([UpdateType::Message, UpdateType::WidgetQuery])
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
