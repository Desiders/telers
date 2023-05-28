use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use telers::{
    client::Reqwest,
    enums::UpdateType,
    event::{telegram::HandlerRequest, EventReturn, ToServiceProvider as _},
    middlewares::inner::Next,
    router::{PropagateEvent as _, Request, RouterService},
    types::{CallbackQuery, Message, Update},
    Bot, Context, Router,
};
use tokio::runtime::Builder;

fn propagate_event_benchmark(c: &mut Criterion) {
    async fn propagate_event<Client>(
        router_service: &RouterService<Client>,
        request: Request<Client>,
        update_type: UpdateType,
    ) where
        Client: Send + Sync + Clone + 'static,
    {
        router_service
            .propagate_event(update_type, request)
            .await
            .unwrap();
    }

    c.bench_with_input(
        BenchmarkId::new("propagate_event", "no display data"),
        &{
            let mut router = Router::<Reqwest>::default();
            router
                .message
                .register(|| async { Ok(EventReturn::Finish) });

            let router_service = router.to_service_provider_default().unwrap();

            let bot = Bot::<Reqwest>::default();
            let context = Context::new();
            let update = Update {
                message: Some(Message::default()),
                ..Default::default()
            };

            let request = Request::new(bot, update, context);

            (router_service, request, UpdateType::Message)
        },
        |b, (router_service, request, update_type)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| propagate_event(router_service, request.clone(), *update_type))
        },
    );

    c.bench_with_input(
        BenchmarkId::new("propagate_event_with_sub_router", "no display data"),
        &{
            let mut router = Router::<Reqwest>::default();
            router
                .message
                .register(|| async { Ok(EventReturn::Finish) });
            let mut sub_router = Router::<Reqwest>::default();
            sub_router
                .message
                .register(|| async { Ok(EventReturn::Finish) });
            router.include(sub_router);

            let router_service = router.to_service_provider_default().unwrap();

            let bot = Bot::<Reqwest>::default();
            let context = Context::new();
            let update = Update {
                message: Some(Message::default()),
                ..Default::default()
            };

            let request = Request::new(bot, update, context);

            (router_service, request, UpdateType::Message)
        },
        |b, (router_service, request, update_type)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| propagate_event(router_service, request.clone(), *update_type))
        },
    );

    c.bench_with_input(
        BenchmarkId::new("propagate_event_failed", "no display data"),
        &{
            let mut router = Router::<Reqwest>::default();
            router
                .message
                .register(|| async { Ok(EventReturn::Finish) });

            let router_service = router.to_service_provider_default().unwrap();

            let bot = Bot::<Reqwest>::default();
            let context = Context::new();
            let update = Update {
                callback_query: Some(CallbackQuery::default()),
                ..Default::default()
            };

            let request = Request::new(bot, update, context);

            (router_service, request, UpdateType::CallbackQuery)
        },
        |b, (router_service, request, update_type)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| propagate_event(router_service, request.clone(), *update_type))
        },
    );

    c.bench_with_input(
        BenchmarkId::new("propagate_event_with_middleware", "no display data"),
        &{
            let mut router = Router::<Reqwest>::default();
            router
                .message
                .register(|| async { Ok(EventReturn::Finish) });
            router
                .message
                .inner_middlewares
                .register(|request: HandlerRequest<_>, next: Next<_>| next(request));

            let router_service = router.to_service_provider_default().unwrap();

            let bot = Bot::<Reqwest>::default();
            let context = Context::new();
            let update = Update {
                message: Some(Message::default()),
                ..Default::default()
            };

            let request = Request::new(bot, update, context);

            (router_service, request, UpdateType::Message)
        },
        |b, (router_service, request, update_type)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| propagate_event(router_service, request.clone(), *update_type))
        },
    );

    c.bench_with_input(
        BenchmarkId::new(
            "propagate_event_with_middleware_and_sub_router",
            "no display data",
        ),
        &{
            let mut router = Router::<Reqwest>::default();
            router
                .message
                .register(|| async { Ok(EventReturn::Finish) });
            router
                .message
                .inner_middlewares
                .register(|request: HandlerRequest<_>, next: Next<_>| next(request));
            let mut sub_router = Router::<Reqwest>::default();
            sub_router
                .message
                .register(|| async { Ok(EventReturn::Finish) });
            router.include(sub_router);

            let router_service = router.to_service_provider_default().unwrap();

            let bot = Bot::<Reqwest>::default();
            let context = Context::new();
            let update = Update {
                message: Some(Message::default()),
                ..Default::default()
            };

            let request = Request::new(bot, update, context);

            (router_service, request, UpdateType::Message)
        },
        |b, (router_service, request, update_type)| {
            b.to_async(Builder::new_current_thread().build().unwrap())
                .iter(|| propagate_event(router_service, request.clone(), *update_type))
        },
    );
}

criterion_group!(benches, propagate_event_benchmark);
criterion_main!(benches);
