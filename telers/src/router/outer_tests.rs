use super::{
    Config, Configured, InnerMiddlewaresConfig, OuterMiddlewaresConfig, PropagateEvent as _,
    Response, Router,
};
use crate::{
    client::Reqwest,
    enums::UpdateType,
    errors::{EventErrorKind, FilterError, HandlerError, MiddlewareError},
    event::{
        bases::PropagateEventResult,
        simple::Handler as SimpleHandler,
        telegram::{Handler, HandlerFn},
        EventReturn,
    },
    middlewares::{InnerMiddleware, InnerNext, OuterMiddleware, OuterNext},
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    Bot, Context, Extensions, Request,
};

use std::{
    convert::Infallible,
    future::{ready, Ready},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

type CallLog = Arc<Mutex<Vec<&'static str>>>;

fn test_request() -> Request<Reqwest> {
    Request::<Reqwest> {
        update: Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        bot: Bot::default(),
        context: Context::default(),
        extensions: Extensions::default(),
    }
}

fn configure_router(router: Router<Reqwest>) -> Configured<Reqwest> {
    router.configure(Config::new(
        OuterMiddlewaresConfig::new(),
        InnerMiddlewaresConfig::new(),
    ))
}

fn assert_finished(response: &Response<Reqwest>) {
    let PropagateEventResult::Handled(handler_response) = &response.propagate_result else {
        panic!("Expected a handled response, got {response:?}");
    };
    assert_eq!(
        handler_response.result.as_ref().unwrap(),
        &EventReturn::Finish
    );
}

fn recording_outer_middleware(
    calls: CallLog,
    before: &'static str,
    after: &'static str,
) -> impl OuterMiddleware<Reqwest> {
    move |request: Request<Reqwest>, next: OuterNext<Reqwest>| {
        let calls = calls.clone();

        async move {
            calls.lock().unwrap().push(before);
            let result = next(request).await;
            calls.lock().unwrap().push(after);
            result
        }
    }
}

fn recording_inner_middleware(calls: CallLog) -> impl InnerMiddleware<Reqwest> {
    move |request: Request<Reqwest>, next: InnerNext<Reqwest>| {
        let calls = calls.clone();

        async move {
            calls.lock().unwrap().push("inner before");
            let result = next(request).await;
            calls.lock().unwrap().push("inner after");
            result
        }
    }
}

fn recording_handler(calls: CallLog, label: &'static str, result: EventReturn) -> Handler<Reqwest> {
    Handler::new(move || {
        let calls = calls.clone();

        async move {
            calls.lock().unwrap().push(label);
            result
        }
    })
}

struct CloneTrackedHandler {
    clones: Arc<AtomicUsize>,
    calls: Arc<AtomicUsize>,
}

impl Clone for CloneTrackedHandler {
    fn clone(&self) -> Self {
        self.clones.fetch_add(1, Ordering::SeqCst);

        Self {
            clones: self.clones.clone(),
            calls: self.calls.clone(),
        }
    }
}

impl HandlerFn<()> for CloneTrackedHandler {
    type Future = Ready<EventReturn>;
    type Response = EventReturn;

    fn call(&mut self, _args: ()) -> Self::Future {
        self.calls.fetch_add(1, Ordering::SeqCst);
        ready(EventReturn::Finish)
    }
}

fn clone_tracked_router(
    name: &'static str,
    clones: Arc<AtomicUsize>,
    calls: Arc<AtomicUsize>,
) -> Router<Reqwest> {
    Router::<Reqwest>::new(name).on_message(|observer| {
        observer.register(Handler::new(CloneTrackedHandler {
            clones,
            calls,
        }))
    })
}

fn recording_lifecycle_handler(calls: CallLog, label: &'static str) -> SimpleHandler {
    SimpleHandler::new(
        move || {
            let calls = calls.clone();

            async move {
                calls.lock().unwrap().push(label);
                Ok::<_, Infallible>(())
            }
        },
        (),
    )
}

#[tokio::test]
async fn test_outer_cleanup_keeps_inner_and_outer_changes() {
    let mut router = configure_router(Router::new("router").on_message(|observer| {
        observer
            .register_outer_middleware(|mut request: Request, next: OuterNext| async move {
                request.context.insert("outer before", true);
                let mut response = next(request).await?;
                assert_eq!(
                    response.request.context.get::<bool>("inner before"),
                    Some(&true)
                );
                assert_eq!(
                    response.request.context.get::<bool>("inner after"),
                    Some(&true)
                );
                response.request.context.insert("outer after", true);
                Ok(response)
            })
            .register_inner_middleware(|mut request: Request, next: InnerNext| async move {
                request.context.insert("inner before", true);
                let mut response = next(request).await?;
                response.request.context.insert("inner after", true);
                Ok(response)
            })
            .register(Handler::new(|context: Context| async move {
                assert_eq!(context.get::<bool>("outer before"), Some(&true));
                assert_eq!(context.get::<bool>("inner before"), Some(&true));
                EventReturn::Finish
            }))
    }));

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();
    let PropagateEventResult::Handled(handler_response) = response.propagate_result else {
        panic!("the handler should finish through both middleware chains");
    };

    assert_eq!(handler_response.result.unwrap(), EventReturn::Finish);
    for key in ["outer before", "inner before", "inner after", "outer after"] {
        assert_eq!(response.request.context.get::<bool>(key), Some(&true));
        assert_eq!(
            handler_response.request.context.get::<bool>(key),
            Some(&true)
        );
    }
}

#[tokio::test]
async fn test_outer_wraps_update_observers_and_child_routes() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let child = Router::new("child")
        .on_update(|observer| {
            observer
                .register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "child update before",
                    "child update after",
                ))
                .register(recording_handler(
                    calls.clone(),
                    "child update handler",
                    EventReturn::Skip,
                ))
        })
        .on_message(|observer| {
            observer
                .register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "child message before",
                    "child message after",
                ))
                .register(recording_handler(
                    calls.clone(),
                    "message handler",
                    EventReturn::Finish,
                ))
        });
    let mut router = configure_router(
        Router::new("parent")
            .on_update(|observer| {
                observer
                    .register_outer_middleware(recording_outer_middleware(
                        calls.clone(),
                        "parent update before",
                        "parent update after",
                    ))
                    .register(recording_handler(
                        calls.clone(),
                        "parent update handler",
                        // An update Cancel must still allow typed and child routing.
                        EventReturn::Cancel,
                    ))
            })
            .on_message(|observer| {
                observer.register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "parent message before",
                    "parent message after",
                ))
            })
            .include(child),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "parent update before",
            "parent update handler",
            "parent message before",
            "child update before",
            "child update handler",
            "child message before",
            "message handler",
            "child message after",
            "child update after",
            "parent message after",
            "parent update after",
        ]
    );
}

#[tokio::test]
async fn test_outer_cleanup_after_common_filter_rejection() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let filter_calls = calls.clone();
    let mut router = configure_router(
        Router::new("parent")
            .on_message(|observer| {
                observer
                    .register_outer_middleware(recording_outer_middleware(
                        calls.clone(),
                        "outer before",
                        "outer after",
                    ))
                    .filter(move |_request: &mut Request| {
                        let calls = filter_calls.clone();

                        async move {
                            calls.lock().unwrap().push("common filter");
                            Ok::<_, Infallible>(false)
                        }
                    })
                    .register(recording_handler(
                        calls.clone(),
                        "parent handler",
                        EventReturn::Finish,
                    ))
            })
            .include(Router::new("child").on_message(|observer| {
                observer.register(recording_handler(
                    calls.clone(),
                    "child handler",
                    EventReturn::Finish,
                ))
            })),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert!(matches!(
        response.propagate_result,
        PropagateEventResult::Unhandled
    ));
    assert_eq!(
        *calls.lock().unwrap(),
        ["outer before", "common filter", "outer after"]
    );
}

#[tokio::test]
async fn test_handler_skip_repeats_inner_chain_only() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut router = configure_router(Router::new("router").on_message(|observer| {
        observer
            .register_outer_middleware(recording_outer_middleware(
                calls.clone(),
                "outer before",
                "outer after",
            ))
            .register_inner_middleware(recording_inner_middleware(calls.clone()))
            .register(recording_handler(
                calls.clone(),
                "skip handler",
                EventReturn::Skip,
            ))
            .register(recording_handler(
                calls.clone(),
                "finish handler",
                EventReturn::Finish,
            ))
    }));

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "outer before",
            "inner before",
            "skip handler",
            "inner after",
            "inner before",
            "finish handler",
            "inner after",
            "outer after",
        ]
    );
}

#[tokio::test]
async fn test_handler_cancel_prunes_children_but_allows_siblings() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let cancelled_child = Router::new("cancelled child")
        .on_message(|observer| {
            observer
                .register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "child before",
                    "child after",
                ))
                .register_inner_middleware(recording_inner_middleware(calls.clone()))
                .register(recording_handler(
                    calls.clone(),
                    "cancel handler",
                    EventReturn::Cancel,
                ))
                .register(recording_handler(
                    calls.clone(),
                    "later handler",
                    EventReturn::Finish,
                ))
        })
        .include(Router::new("descendant").on_message(|observer| {
            observer.register(recording_handler(
                calls.clone(),
                "descendant handler",
                EventReturn::Finish,
            ))
        }));
    let sibling = Router::new("sibling").on_message(|observer| {
        observer.register(recording_handler(
            calls.clone(),
            "sibling handler",
            EventReturn::Finish,
        ))
    });
    let mut router = configure_router(
        Router::new("parent")
            .on_update(|observer| {
                observer.register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "parent before",
                    "parent after",
                ))
            })
            .include(cancelled_child)
            .include(sibling),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "parent before",
            "child before",
            "inner before",
            "cancel handler",
            "inner after",
            "child after",
            "sibling handler",
            "parent after",
        ]
    );
}

#[tokio::test]
async fn test_outer_rejection_stops_siblings() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let rejecting_calls = calls.clone();
    let rejecting_child = Router::new("rejecting child").on_message(|observer| {
        observer
            .register_outer_middleware(move |request: Request, _next: OuterNext| {
                let calls = rejecting_calls.clone();

                async move {
                    calls.lock().unwrap().push("rejecting outer");
                    Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    })
                }
            })
            .register(recording_handler(
                calls.clone(),
                "rejected handler",
                EventReturn::Finish,
            ))
    });
    let sibling = Router::new("sibling").on_message(|observer| {
        observer.register(recording_handler(
            calls.clone(),
            "sibling handler",
            EventReturn::Finish,
        ))
    });
    let mut router = configure_router(
        Router::new("parent")
            .on_update(|observer| {
                observer.register_outer_middleware(recording_outer_middleware(
                    calls.clone(),
                    "parent before",
                    "parent after",
                ))
            })
            .include(rejecting_child)
            .include(sibling),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert!(matches!(
        response.propagate_result,
        PropagateEventResult::Rejected
    ));
    assert_eq!(
        *calls.lock().unwrap(),
        ["parent before", "rejecting outer", "parent after"]
    );
}

#[tokio::test]
async fn test_outer_error_keeps_context_for_recovery() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let update_calls = calls.clone();
    let failing_calls = calls.clone();
    let error_calls = calls.clone();
    let mut router = configure_router(
        Router::new("router")
            .on_update(|observer| {
                observer.register_outer_middleware(move |mut request: Request, next: OuterNext| {
                    let calls = update_calls.clone();

                    async move {
                        calls.lock().unwrap().push("update before");
                        request.context.insert("update", true);
                        let result = next(request).await;
                        calls.lock().unwrap().push("update after");
                        result.map_err(|(error, mut request)| {
                            request.context.insert("cleanup", true);
                            (error, request)
                        })
                    }
                })
            })
            .on_message(|observer| {
                observer.register_outer_middleware(move |mut request: Request, _next: OuterNext| {
                    let calls = failing_calls.clone();

                    async move {
                        calls.lock().unwrap().push("failing outer");
                        request.context.insert("failure", true);
                        Err((
                            EventErrorKind::Middleware(MiddlewareError::from_display(
                                "outer failed",
                            )),
                            request,
                        ))
                    }
                })
            })
            .on_error(|observer| {
                observer.register(Handler::new(
                    move |context: Context, error: EventErrorKind| {
                        let calls = error_calls.clone();

                        async move {
                            calls.lock().unwrap().push("error handler");
                            assert!(matches!(error, EventErrorKind::Middleware(_)));
                            assert_eq!(context.get::<bool>("update"), Some(&true));
                            assert_eq!(context.get::<bool>("failure"), Some(&true));
                            assert_eq!(context.get::<bool>("cleanup"), Some(&true));
                            EventReturn::Finish
                        }
                    },
                ))
            }),
    );

    let response = router
        .propagate_event_with_error_handling(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "update before",
            "failing outer",
            "update after",
            "error handler"
        ]
    );
}

#[tokio::test]
async fn test_error_outer_chain_wraps_child_recovery() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let parent_calls = calls.clone();
    let child_calls = calls.clone();
    let handler_calls = calls.clone();
    let child = Router::new("child").on_error(|observer| {
        observer
            .register_outer_middleware(move |request: Request, next: OuterNext| {
                let calls = child_calls.clone();

                async move {
                    calls.lock().unwrap().push("child error before");
                    let mut response = next(request).await?;
                    calls.lock().unwrap().push("child error after");
                    response.request.context.insert("child cleanup", true);
                    Ok(response)
                }
            })
            .register(Handler::new(
                move |context: Context, error: EventErrorKind| {
                    let calls = handler_calls.clone();

                    async move {
                        assert!(matches!(error, EventErrorKind::Handler(_)));
                        assert_eq!(context.get::<bool>("parent before"), Some(&true));
                        calls.lock().unwrap().push("error handler");
                        EventReturn::Finish
                    }
                },
            ))
    });
    let mut router = configure_router(
        Router::new("parent")
            .on_message(|observer| {
                observer.register(Handler::new(|| async {
                    Err::<EventReturn, _>(HandlerError::from_display("handler failed"))
                }))
            })
            .on_error(|observer| {
                observer.register_outer_middleware(move |mut request: Request, next: OuterNext| {
                    let calls = parent_calls.clone();

                    async move {
                        calls.lock().unwrap().push("parent error before");
                        request.context.insert("parent before", true);
                        let mut response = next(request).await?;
                        assert_eq!(
                            response.request.context.get::<bool>("child cleanup"),
                            Some(&true)
                        );
                        calls.lock().unwrap().push("parent error after");
                        response.request.context.insert("parent cleanup", true);
                        Ok(response)
                    }
                })
            })
            .include(child),
    );

    let response = router
        .propagate_event_with_error_handling(UpdateType::Message, test_request())
        .await
        .unwrap();
    let PropagateEventResult::Handled(handler_response) = response.propagate_result else {
        panic!("the child error handler should recover the original failure");
    };

    assert_eq!(handler_response.result.unwrap(), EventReturn::Finish);
    for key in ["parent before", "child cleanup", "parent cleanup"] {
        assert_eq!(response.request.context.get::<bool>(key), Some(&true));
        assert_eq!(
            handler_response.request.context.get::<bool>(key),
            Some(&true)
        );
    }
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "parent error before",
            "child error before",
            "error handler",
            "child error after",
            "parent error after",
        ]
    );
}

#[tokio::test]
async fn test_outer_handler_error_restored_for_public_propagation() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let outer_calls = calls.clone();
    let handler_calls = calls.clone();
    let error_calls = calls.clone();
    let mut router = configure_router(
        Router::new("router")
            .on_message(|observer| {
                observer
                    .register_outer_middleware(move |mut request: Request, next: OuterNext| {
                        let calls = outer_calls.clone();

                        async move {
                            calls.lock().unwrap().push("outer before");
                            request.context.insert("outer before", true);
                            let (error, mut request) = next(request).await.unwrap_err();
                            assert!(matches!(error, EventErrorKind::Handler(_)));
                            assert_eq!(request.context.get::<bool>("outer before"), Some(&true));
                            calls.lock().unwrap().push("outer saw handler error");
                            request.context.insert("outer saw error", true);
                            Err((error, request))
                        }
                    })
                    .register_inner_middleware(|request: Request, next: InnerNext| next(request))
                    .register(Handler::new(move || {
                        let calls = handler_calls.clone();

                        async move {
                            calls.lock().unwrap().push("handler failed");
                            Err::<EventReturn, _>(HandlerError::from_display("handler failed"))
                        }
                    }))
            })
            .on_error(|observer| {
                observer.register(Handler::new(
                    move |context: Context, error: EventErrorKind| {
                        let calls = error_calls.clone();

                        async move {
                            assert!(matches!(error, EventErrorKind::Handler(_)));
                            assert_eq!(context.get::<bool>("outer before"), Some(&true));
                            assert_eq!(context.get::<bool>("outer saw error"), Some(&true));
                            calls.lock().unwrap().push("error handler");
                            EventReturn::Finish
                        }
                    },
                ))
            }),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();
    let PropagateEventResult::Handled(handler_response) = response.propagate_result else {
        panic!("public propagation should preserve the handled handler error");
    };
    assert_eq!(
        handler_response.result.unwrap_err().to_string(),
        "handler failed"
    );
    assert_eq!(
        handler_response
            .request
            .context
            .get::<bool>("outer saw error"),
        Some(&true)
    );
    assert_eq!(
        *calls.lock().unwrap(),
        ["outer before", "handler failed", "outer saw handler error"]
    );

    calls.lock().unwrap().clear();
    let response = router
        .propagate_event_with_error_handling(UpdateType::Message, test_request())
        .await
        .unwrap();
    let PropagateEventResult::Handled(handler_response) = response.propagate_result else {
        panic!("the global error observer should recover the handler error");
    };
    assert_eq!(handler_response.result.unwrap(), EventReturn::Finish);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            "outer before",
            "handler failed",
            "outer saw handler error",
            "error handler",
        ]
    );
}

#[tokio::test]
async fn test_filter_error_keeps_context_and_cleanup() {
    let mut router = configure_router(
        Router::new("router")
            .on_message(|observer| {
                observer
                    .register_outer_middleware(|mut request: Request, next: OuterNext| async move {
                        request.context.insert("outer before", true);
                        let (error, mut request) = next(request).await.unwrap_err();
                        assert!(matches!(error, EventErrorKind::Filter(_)));
                        assert_eq!(request.context.get::<bool>("filter"), Some(&true));
                        request.context.insert("outer after", true);
                        Err((error, request))
                    })
                    .filter(|request: &mut Request| {
                        assert_eq!(request.context.get::<bool>("outer before"), Some(&true));
                        request.context.insert("filter", true);
                        async move { Err::<bool, _>(FilterError::from_display("filter failed")) }
                    })
            })
            .on_error(|observer| {
                observer.register(Handler::new(
                    |context: Context, error: EventErrorKind| async move {
                        assert!(matches!(error, EventErrorKind::Filter(_)));
                        assert_eq!(context.get::<bool>("outer before"), Some(&true));
                        assert_eq!(context.get::<bool>("filter"), Some(&true));
                        assert_eq!(context.get::<bool>("outer after"), Some(&true));
                        EventReturn::Finish
                    },
                ))
            }),
    );

    let response = router
        .propagate_event_with_error_handling(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
}

#[tokio::test]
async fn test_unhandled_child_context_isolated_from_siblings() {
    let mut router = configure_router(
        Router::new("parent")
            .on_update(|observer| {
                observer.register_outer_middleware(
                    |mut request: Request, next: OuterNext| async move {
                        request.context.insert("parent", true);
                        next(request).await
                    },
                )
            })
            .include(Router::new("unhandled child").on_message(|observer| {
                observer.register_outer_middleware(
                    |mut request: Request, next: OuterNext| async move {
                        request.context.insert("child before", true);
                        let mut response = next(request).await?;
                        assert!(matches!(
                            response.propagate_result,
                            PropagateEventResult::Unhandled
                        ));
                        response.request.context.insert("child after", true);
                        Ok(response)
                    },
                )
            }))
            .include(Router::new("handling sibling").on_message(|observer| {
                observer.register(Handler::new(|context: Context| async move {
                    assert_eq!(context.get::<bool>("parent"), Some(&true));
                    assert!(context.get::<bool>("child before").is_none());
                    assert!(context.get::<bool>("child after").is_none());
                    EventReturn::Finish
                }))
            })),
    );

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(response.request.context.get::<bool>("parent"), Some(&true));
    assert!(response
        .request
        .context
        .get::<bool>("child before")
        .is_none());
    assert!(response
        .request
        .context
        .get::<bool>("child after")
        .is_none());
}

#[tokio::test]
async fn test_direct_update_propagation_only_visits_current_observer() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut router = configure_router(
        Router::new("parent")
            .on_update(|observer| {
                observer
                    .register_outer_middleware(recording_outer_middleware(
                        calls.clone(),
                        "update before",
                        "update after",
                    ))
                    .register(recording_handler(
                        calls.clone(),
                        "update handler",
                        EventReturn::Skip,
                    ))
            })
            .on_message(|observer| {
                observer.register(recording_handler(
                    calls.clone(),
                    "message handler",
                    EventReturn::Finish,
                ))
            })
            .include(Router::new("child").on_update(|observer| {
                observer.register(recording_handler(
                    calls.clone(),
                    "child update handler",
                    EventReturn::Finish,
                ))
            })),
    );

    let response = router.propagate_update_event(test_request()).await.unwrap();

    assert!(matches!(
        response.propagate_result,
        PropagateEventResult::Unhandled
    ));
    assert_eq!(
        *calls.lock().unwrap(),
        ["update before", "update handler", "update after"]
    );
}

#[tokio::test]
async fn test_handled_root_does_not_clone_unused_descendant_handlers() {
    let clones = Arc::new(AtomicUsize::new(0));
    let calls = Arc::new(AtomicUsize::new(0));
    let child = clone_tracked_router("child", clones.clone(), calls.clone()).include(
        clone_tracked_router("grandchild", clones.clone(), calls.clone()),
    );
    let mut router = configure_router(
        Router::<Reqwest>::new("parent")
            .on_message(|observer| {
                observer.register(Handler::new(|| async { EventReturn::Finish }))
            })
            .include(child),
    );
    // Registration may clone services; measure only processing an update.
    clones.store(0, Ordering::SeqCst);

    let response = router
        .propagate_event(UpdateType::Message, test_request())
        .await
        .unwrap();

    assert_finished(&response);
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(clones.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn test_nested_routing_does_not_clone_unused_branches() {
    let unused_clones = Arc::new(AtomicUsize::new(0));
    let unused_calls = Arc::new(AtomicUsize::new(0));
    let handled_clones = Arc::new(AtomicUsize::new(0));
    let handled_calls = Arc::new(AtomicUsize::new(0));
    let unused_descendant = clone_tracked_router(
        "unused descendant",
        unused_clones.clone(),
        unused_calls.clone(),
    );
    let handled = clone_tracked_router("handled", handled_clones, handled_calls.clone())
        .include(unused_descendant);
    let unused_sibling = clone_tracked_router(
        "unused sibling",
        unused_clones.clone(),
        unused_calls.clone(),
    )
    .include(clone_tracked_router(
        "unused sibling descendant",
        unused_clones.clone(),
        unused_calls.clone(),
    ));
    let mut router = configure_router(
        Router::<Reqwest>::new("parent")
            .include(Router::new("branch").include(handled))
            .include(unused_sibling),
    );
    unused_clones.store(0, Ordering::SeqCst);

    for _ in 0..3 {
        let response = router
            .propagate_event(UpdateType::Message, test_request())
            .await
            .unwrap();
        assert_finished(&response);
    }

    assert_eq!(handled_calls.load(Ordering::SeqCst), 3);
    assert_eq!(unused_calls.load(Ordering::SeqCst), 0);
    assert_eq!(unused_clones.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn test_reached_handler_clone_count_does_not_grow_with_router_depth() {
    let shallow_clones = Arc::new(AtomicUsize::new(0));
    let shallow_calls = Arc::new(AtomicUsize::new(0));
    let deep_clones = Arc::new(AtomicUsize::new(0));
    let deep_calls = Arc::new(AtomicUsize::new(0));
    let mut shallow = configure_router(Router::<Reqwest>::new("parent").include(
        clone_tracked_router("leaf", shallow_clones.clone(), shallow_calls.clone()),
    ));
    let mut deep = clone_tracked_router("leaf", deep_clones.clone(), deep_calls.clone());
    for _ in 0..8 {
        deep = Router::new("parent").include(deep);
    }
    let mut deep = configure_router(deep);
    shallow_clones.store(0, Ordering::SeqCst);
    deep_clones.store(0, Ordering::SeqCst);

    for router in [&mut shallow, &mut deep] {
        for _ in 0..3 {
            let response = router
                .propagate_event(UpdateType::Message, test_request())
                .await
                .unwrap();
            assert_finished(&response);
        }
    }

    assert_eq!(shallow_calls.load(Ordering::SeqCst), 3);
    assert_eq!(deep_calls.load(Ordering::SeqCst), 3);
    let shallow_count = shallow_clones.load(Ordering::SeqCst);
    assert!(shallow_count > 0);
    assert_eq!(deep_clones.load(Ordering::SeqCst), shallow_count);
}

#[tokio::test]
async fn test_cloned_router_keeps_nested_startup_and_shutdown_order() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let grandchild = Router::new("grandchild")
        .on_startup(|observer| {
            observer.register(recording_lifecycle_handler(
                calls.clone(),
                "grandchild startup",
            ))
        })
        .on_shutdown(|observer| {
            observer.register(recording_lifecycle_handler(
                calls.clone(),
                "grandchild shutdown",
            ))
        });
    let child = Router::new("child")
        .on_startup(|observer| {
            observer.register(recording_lifecycle_handler(calls.clone(), "child startup"))
        })
        .on_shutdown(|observer| {
            observer.register(recording_lifecycle_handler(calls.clone(), "child shutdown"))
        })
        .include(grandchild);
    let sibling = Router::new("sibling")
        .on_startup(|observer| {
            observer.register(recording_lifecycle_handler(
                calls.clone(),
                "sibling startup",
            ))
        })
        .on_shutdown(|observer| {
            observer.register(recording_lifecycle_handler(
                calls.clone(),
                "sibling shutdown",
            ))
        });
    let mut router = configure_router(
        Router::<Reqwest>::new("parent")
            .on_startup(|observer| {
                observer.register(recording_lifecycle_handler(calls.clone(), "parent startup"))
            })
            .on_shutdown(|observer| {
                observer.register(recording_lifecycle_handler(
                    calls.clone(),
                    "parent shutdown",
                ))
            })
            .include(child)
            .include(sibling),
    );
    let mut cloned = router.clone();

    for router in [&mut cloned, &mut router] {
        calls.lock().unwrap().clear();
        router.emit_startup().await.unwrap();
        router.emit_shutdown().await.unwrap();

        assert_eq!(
            *calls.lock().unwrap(),
            [
                "parent startup",
                "child startup",
                "grandchild startup",
                "sibling startup",
                "parent shutdown",
                "child shutdown",
                "grandchild shutdown",
                "sibling shutdown",
            ]
        );
    }
}
