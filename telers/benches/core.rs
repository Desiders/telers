use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use std::{convert::Infallible, hint::black_box, sync::Arc};
use telers::{
    client::{
        session::{ClientResponse, ClientStreamResponse, Session},
        telegram::APIServer,
    },
    event::telegram::Handler,
    event::EventReturn,
    errors::EventErrorKind,
    types::{ChatPrivate, Message, MessageText, Update, UpdateMessage, User},
    Bot, Dispatcher, Filter, Request, Router, RouterConfigured,
};

#[derive(Clone)]
struct MockSession {
    api: APIServer,
}

impl Default for MockSession {
    fn default() -> Self {
        Self {
            api: APIServer::default(),
        }
    }
}

impl Session for MockSession {
    fn api(&self) -> &APIServer {
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
        T: telers::methods::TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        unreachable!("the propagation benchmark must not make network requests")
    }

    async fn stream_content(
        &self,
        _url: &str,
        _timeout: Option<f32>,
    ) -> Result<ClientStreamResponse, anyhow::Error> {
        unreachable!("the propagation benchmark must not download files")
    }
}

fn finish_handler() -> Handler<MockSession> {
    Handler::new(
        |_bot: Bot<MockSession>, _message: Message| async {
            Ok::<_, Infallible>(EventReturn::Finish)
        },
    )
}

fn skip_handler() -> Handler<MockSession> {
    Handler::new(
        |_bot: Bot<MockSession>, _message: Message| async {
            Ok::<_, Infallible>(EventReturn::Skip)
        },
    )
}

fn passing_filter() -> impl Filter<MockSession> {
    |_request: &mut Request<MockSession>| async { Ok::<_, Infallible>(true) }
}

fn fixture() -> (Bot<MockSession>, Arc<Update>) {
    let bot = Bot::with_client("123:benchmark", MockSession::default());
    let update = Arc::new(Update::Message(UpdateMessage::new(
        1,
        MessageText::new(
            42,
            1_700_000_000,
            ChatPrivate::new(42),
            "hello from a benchmark",
        )
        .from(User::new(7, false, "Benchmark User")),
    )));
    (bot, update)
}

fn benchmark_dispatcher<M: Measurement>(
    group: &mut BenchmarkGroup<'_, M>,
    runtime: &tokio::runtime::Runtime,
    name: &str,
    dispatcher: Dispatcher<MockSession, RouterConfigured<MockSession>>,
    bot: &Bot<MockSession>,
    update: &Arc<Update>,
) {
    let bot = bot.clone();
    let update = update.clone();

    group.bench_function(name, |b| {
        b.to_async(runtime).iter(|| {
            let mut dispatcher = dispatcher.clone();
            let bot = bot.clone();
            let update = update.clone();

            async move {
                let response = dispatcher.feed_update(bot, update).await.unwrap();
                black_box(response);
            }
        });
    });
}

fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

fn baseline(c: &mut Criterion) {
    let (bot, update) = fixture();
    let runtime = runtime();
    let mut group = c.benchmark_group("dispatcher/baseline");

    let router = Router::new("benchmark").on_message(|observer| {
        observer.register(finish_handler())
    });
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "handled_message",
        dispatcher,
        &bot,
        &update,
    );
    group.finish();
}

fn middleware(c: &mut Criterion) {
    let (bot, update) = fixture();
    let runtime = runtime();
    let mut group = c.benchmark_group("dispatcher/middleware");

    let router = Router::new("outer_middleware").on_message(|observer| {
        observer
            .register_outer_middleware(|request: Request<MockSession>| async move {
                Ok::<_, EventErrorKind>((request, EventReturn::Skip))
            })
            .register(finish_handler())
    });
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "outer",
        dispatcher,
        &bot,
        &update,
    );

    let router = Router::new("inner_middleware").on_message(|observer| {
        observer
            .register_inner_middleware(
                |request: Request<MockSession>, next: telers::middlewares::Next<MockSession>| {
                    next(request)
                },
            )
            .register(finish_handler())
    });
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "inner",
        dispatcher,
        &bot,
        &update,
    );
    group.finish();
}

fn filters(c: &mut Criterion) {
    let (bot, update) = fixture();
    let runtime = runtime();
    let mut group = c.benchmark_group("dispatcher/filters");
    let router = Router::new("many_filters").on_message(|observer| {
        let handler =
            (0..8).fold(finish_handler(), |handler, _| handler.filter(passing_filter()));
        observer.register(handler)
    });
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "8_passing",
        dispatcher,
        &bot,
        &update,
    );
    group.finish();
}

fn handlers(c: &mut Criterion) {
    let (bot, update) = fixture();
    let runtime = runtime();
    let mut group = c.benchmark_group("dispatcher/handlers");
    let router = Router::new("many_handlers").on_message(|observer| {
        let observer = (0..16).fold(observer, |observer, _| observer.register(skip_handler()));
        observer.register(finish_handler())
    });
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "16_skipping_then_handled",
        dispatcher,
        &bot,
        &update,
    );
    group.finish();
}

fn routers(c: &mut Criterion) {
    let (bot, update) = fixture();
    let runtime = runtime();
    let mut group = c.benchmark_group("dispatcher/routers");
    let router = (0..8).fold(Router::new("many_routers"), |router, _| {
        router.include_router(
            Router::new("child").on_message(|observer| observer.register(skip_handler())),
        )
    });
    let router = router.on_message(|observer| observer.register(finish_handler()));
    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .build();
    benchmark_dispatcher(
        &mut group,
        &runtime,
        "8_nested_then_handled",
        dispatcher,
        &bot,
        &update,
    );
    group.finish();
}

criterion_group!(
    propagation_benches,
    baseline,
    middleware,
    filters,
    handlers,
    routers,
);
criterion_main!(propagation_benches);
