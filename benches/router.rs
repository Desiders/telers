use criterion::{black_box, criterion_group, criterion_main, Criterion};
use telers::{
    client::{Bot, Reqwest},
    context::Context,
    dispatcher::{
        event::{
            telegram::{HandlerRequest, HandlerResponse},
            EventReturn, ToServiceProvider as _,
        },
        middlewares::inner::Next,
        router::Request,
        Router,
    },
    enums::UpdateType,
    error::AppErrorKind,
    types::Update,
};

fn router_benchmark(c: &mut Criterion) {
    async fn test_middleware<Client>(
        _: HandlerRequest<Client>,
        _: Next<Client>,
    ) -> Result<HandlerResponse<Client>, AppErrorKind> {
        unimplemented!()
    }

    c.bench_function("create", |b| b.iter(|| Router::<Reqwest>::default()));
    c.bench_function("to_service_provider", |b| {
        b.iter(|| {
            let router = black_box(Router::<Reqwest>::default());

            router.to_service_provider(()).unwrap();
        })
    });
    c.bench_function("include_sub", |b| {
        b.iter(|| {
            let mut router = black_box(Router::<Reqwest>::default());
            let sub_router = black_box(Router::<Reqwest>::default());

            router.include_router(sub_router);
        })
    });
    c.bench_function("include_sub_with_middleware", |b| {
        b.iter(|| {
            let mut router = black_box(Router::<Reqwest>::default());
            router.message.inner_middlewares.register(test_middleware);

            let sub_router = black_box(Router::<Reqwest>::default());

            router.include_router(sub_router);
        })
    });
    c.bench_function("include_sub_with_middleware_for_each_observer", |b| {
        b.iter(|| {
            let mut router = black_box(Router::<Reqwest>::default());
            router.message.inner_middlewares.register(test_middleware);
            router
                .edited_message
                .inner_middlewares
                .register(test_middleware);
            router
                .channel_post
                .inner_middlewares
                .register(test_middleware);
            router
                .edited_channel_post
                .inner_middlewares
                .register(test_middleware);
            router
                .inline_query
                .inner_middlewares
                .register(test_middleware);
            router
                .chosen_inline_result
                .inner_middlewares
                .register(test_middleware);
            router
                .callback_query
                .inner_middlewares
                .register(test_middleware);
            router
                .shipping_query
                .inner_middlewares
                .register(test_middleware);
            router
                .pre_checkout_query
                .inner_middlewares
                .register(test_middleware);
            router.poll.inner_middlewares.register(test_middleware);
            router
                .poll_answer
                .inner_middlewares
                .register(test_middleware);
            router
                .my_chat_member
                .inner_middlewares
                .register(test_middleware);
            router
                .chat_member
                .inner_middlewares
                .register(test_middleware);
            router
                .chat_join_request
                .inner_middlewares
                .register(test_middleware);

            let sub_router = black_box(Router::<Reqwest>::default());

            router.include_router(sub_router);
        })
    });
    c.bench_function("resolve_used_update_types", |b| {
        b.iter(|| {
            let mut router = black_box(Router::<Reqwest>::default());
            router
                .message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .edited_message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .channel_post
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .edited_channel_post
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .inline_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .chosen_inline_result
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .callback_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .shipping_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .pre_checkout_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .poll
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .poll_answer
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .my_chat_member
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .chat_member
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router
                .chat_join_request
                .register_no_filters(|| async { Ok(EventReturn::Finish) });

            router.resolve_used_update_types()
        })
    });
    c.bench_function("resolve_used_update_types_from_sub", |b| {
        b.iter(|| {
            let mut router = black_box(Router::<Reqwest>::default());
            let mut sub_router = black_box(Router::<Reqwest>::default());

            sub_router
                .message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .edited_message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .channel_post
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .edited_channel_post
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .inline_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .chosen_inline_result
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .callback_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .shipping_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .pre_checkout_query
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .poll
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .poll_answer
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .my_chat_member
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .chat_member
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            sub_router
                .chat_join_request
                .register_no_filters(|| async { Ok(EventReturn::Finish) });

            router.include_router(sub_router);
            router.resolve_used_update_types()
        })
    });
    c.bench_function("propagate_event", |b| {
        b.iter(|| async {
            let mut router = black_box(Router::<Reqwest>::default());
            router
                .message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });

            let router_service = router.to_service_provider(()).unwrap();

            let bot = black_box(Bot::<Reqwest>::default());
            let context = black_box(Context::new());
            let update = black_box(Update::default());

            let request = black_box(Request::new(bot, update, context));

            router_service
                .propagate_event(black_box(&UpdateType::Message), request)
                .await
                .unwrap();
        })
    });
    c.bench_function("propagate_event_with_middleware", |b| {
        b.iter(|| async {
            let mut router = black_box(Router::<Reqwest>::default());
            router
                .message
                .register_no_filters(|| async { Ok(EventReturn::Finish) });
            router.message.inner_middlewares.register(test_middleware);

            let router_service = router.to_service_provider(()).unwrap();

            let bot = black_box(Bot::<Reqwest>::default());
            let context = black_box(Context::new());
            let update = black_box(Update::default());

            let request = black_box(Request::new(bot, update, context));

            router_service
                .propagate_event(black_box(&UpdateType::Message), request)
                .await
                .unwrap();
        })
    });
}

criterion_group!(benches, router_benchmark);
criterion_main!(benches);
