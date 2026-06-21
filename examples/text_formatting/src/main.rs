use telers::{
    enums::{ParseMode, UpdateType},
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::Message,
    utils::text::{html_text_link, Builder as TextBuilder, Formatter as _, HTMLFormatter},
    Bot, Dispatcher, Router,
};

async fn handler(bot: Bot, message: Message) -> HandlerResult<()> {
    // First way to format text by using formatting directly in the text.
    let text =
        "This is <b>bold</b> text.\nThis is <i>italic</i> text.\nThis is <a href=\"https://example.com\">link</a>.";

    // We should use `parse_mode` to specify that we use HTML formatting.
    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    // Second way to format text by using `TextBuilder`.
    let text_builder = TextBuilder::new(HTMLFormatter::new())
        .text("This is ")
        .bold("bold")
        .text(" text.\nThis is ")
        .italic("italic")
        .text(" text.\nThis is ")
        .text_link("link", "https://example.com")
        .text(".");

    let text = text_builder.get_text();

    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    // Third way to format text by using `HTMLFormatter`.
    let html = HTMLFormatter::default();

    let text = format!(
        "This is {bold} text.\nThis is {italic} text.\nThis is {link}.",
        bold = html.bold("bold"),
        italic = html.italic("italic"),
        // We also can use short functions to format text. They are the same as methods of `HTMLFormatter`
        link = html_text_link("link", "https://example.com")
    );

    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router =
        Router::new("main").on_message(|observer| observer.register(Handler::new(handler)));

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
