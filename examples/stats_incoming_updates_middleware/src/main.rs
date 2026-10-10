use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};
use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::telegram::{Handler, HandlerResponse, HandlerResult},
    methods::SendMessage,
    middlewares::{
        outer::MiddlewareResult, InnerMiddleware, InnerNext, OuterMiddleware, OuterNext,
    },
    types::Update,
    Bot, Context, Dispatcher, Request, Router,
};

#[derive(Default, Clone)]
struct IncomingUpdates {
    counter: Arc<AtomicUsize>,
}

impl OuterMiddleware for IncomingUpdates {
    async fn call(&mut self, mut request: Request, next: OuterNext) -> MiddlewareResult {
        self.counter.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_updates_counter",
            self.counter.load(Ordering::SeqCst),
        );

        let started = Instant::now();
        let result = next(request).await;
        tracing::debug!(elapsed = ?started.elapsed(), "Incoming update processing finished");

        result
    }
}

/// # Warning
/// If the handler returns an error, the counter not increments
#[derive(Default, Clone)]
struct ProcessedHandlers {
    counter: Arc<AtomicUsize>,
}

impl InnerMiddleware for ProcessedHandlers {
    async fn call(
        &mut self,
        mut request: Request,
        next: InnerNext,
    ) -> Result<HandlerResponse, EventErrorKind> {
        request.context.insert(
            "processed_handlers_counter",
            self.counter.load(Ordering::SeqCst),
        );

        let response = next(request).await?;

        self.counter.fetch_add(1, Ordering::SeqCst);

        Ok(response)
    }
}

async fn handler(bot: Bot, update: Update, context: Context) -> HandlerResult<()> {
    let text = format!(
        "Hello! Users sent me {} updates and I processed {} handlers successfully for them.",
        context.get::<usize>("incoming_updates_counter").unwrap(),
        context.get::<usize>("processed_handlers_counter").unwrap()
    );
    if let Some(chat) = update.chat() {
        bot.send(SendMessage::new(chat.id(), text)).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        // Register inner middleware for all telegram observers
        .on_all(|observer| observer.register_inner_middleware(ProcessedHandlers::default()))
        // Register outer middleware for update
        .on_update(|observer| observer.register_outer_middleware(IncomingUpdates::default()))
        .on_message(|observer| observer.register(Handler::new(handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
