use axum::{
    extract::{FromRequest, Request},
    handler::Handler,
    response::{IntoResponse, Response},
    routing::{self, MethodRouter},
};
use futures_util::future::BoxFuture;
use reqwest::StatusCode;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{event, Level};

use crate::{
    either::Either::{self, Left, Right},
    router::PropagateEvent,
    types::{Update, UpdateUnparsed},
    webhooks::secret::{verify, XTelegramBotApiSecretToken, SECRET_TOKEN_HEADER_NAME},
    Bot, Dispatcher,
};

#[derive(Clone)]
pub struct UpdatesHandler<Client, Propagator, Backoff> {
    bot: Bot<Client>,
    dispatcher: Dispatcher<Client, Propagator, Backoff>,
    secret_token: Option<Vec<u8>>,
    /// If `true`, updates will be handled in background and a response to Telegram returns immediately.
    /// If `false`, wait for a handler execution to complete before send a response to Telegram back.
    handle_in_background: bool,
    /// Used together only with `handle_in_background` set to `false`.
    /// If `true`, will try to avoid get duplicate updates by Telegram.
    avoid_resend_updates: bool,
}

impl<Client, Propagator, Backoff> UpdatesHandler<Client, Propagator, Backoff> {
    #[inline]
    #[must_use]
    pub const fn new(
        bot: Bot<Client>,
        dispatcher: Dispatcher<Client, Propagator, Backoff>,
    ) -> Self {
        Self {
            bot,
            dispatcher,
            secret_token: None,
            handle_in_background: true,
            avoid_resend_updates: true,
        }
    }

    #[must_use]
    pub fn secret_token(self, val: impl Into<String>) -> Self {
        Self {
            secret_token: Some(val.into().into_bytes()),
            ..self
        }
    }

    #[must_use]
    pub fn handle_in_background(self, val: bool) -> Self {
        Self {
            handle_in_background: val,
            ..self
        }
    }

    #[must_use]
    pub fn avoid_resend_updates(self, val: bool) -> Self {
        Self {
            avoid_resend_updates: val,
            ..self
        }
    }
}

impl<Client, Propagator, Backoff> UpdatesHandler<Client, Propagator, Backoff> {
    #[must_use]
    pub fn secret_token_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            secret_token: val.map(|val| val.into().into_bytes()),
            ..self
        }
    }
}

impl<S, Client, Propagator, Backoff> Handler<((),), S>
    for UpdatesHandler<Client, Propagator, Backoff>
where
    S: Send + Sync + 'static,
    Client: Clone + Send + Sync + 'static,
    Propagator: PropagateEvent<Client>,
    Backoff: Clone + Send + Sync + 'static,
{
    type Future = BoxFuture<'static, Response>;

    fn call(mut self, req: Request, state: S) -> Self::Future {
        event!(Level::TRACE, "Received event");

        let (parts, body) = req.into_parts();
        let secret_token_header = match parts.headers.get(SECRET_TOKEN_HEADER_NAME) {
            Some(token) => XTelegramBotApiSecretToken(Some(token.as_ref().into())),
            None => XTelegramBotApiSecretToken(None),
        };
        if !verify(self.secret_token.as_deref(), &secret_token_header) {
            event!(Level::ERROR, secret_token = ?secret_token_header, "Invalid secret token");
            return Box::pin(async move { StatusCode::UNAUTHORIZED.into_response() });
        }
        let req = Request::from_parts(parts, body);

        Box::pin(async move {
            let update_raw = match String::from_request(req, &state).await {
                Ok(val) => val,
                Err(err) => {
                    event!(Level::ERROR, error = %err, "Failed to read body");
                    return err.into_response();
                }
            };
            let update = match serde_json::from_str::<Either<Update, UpdateUnparsed>>(&update_raw) {
                Ok(Left(update)) => update,
                Ok(Right(UpdateUnparsed { id, value })) => {
                    event!(
                        Level::ERROR,
                        update_id = id,
                        update = ?value,
                        "Failed to parse update kind",
                    );
                    return StatusCode::OK.into_response();
                }
                Err(err) => {
                    event!(Level::ERROR, error = %err, "Failed to parse update");
                    return StatusCode::UNPROCESSABLE_ENTITY.into_response();
                }
            };
            event!(Level::DEBUG, update_id = update.id, "Received update",);

            let update = Arc::new(update);

            if self.handle_in_background {
                tokio::spawn(async move { self.dispatcher.feed_update(self.bot, update).await });
            } else {
                let fut = self.dispatcher.feed_update(self.bot, update);
                if self.avoid_resend_updates {
                    tokio::select! {
                        _ = fut => {},
                        () = sleep(Duration::from_secs(55)) => event!(
                            Level::WARN,
                            "Detected slow handler execution. \
                            Telegram waits for a response for 60 seconds and re-send the update. \
                            To avoid this, the response will be sent immediately because execution takes more than 55 seconds."
                        )
                    }
                } else {
                    let _ = fut.await;
                }
            }

            StatusCode::OK.into_response()
        })
    }
}

#[inline]
pub fn get_updates_router<S, Client, Propagator, Backoff>(
    handler: UpdatesHandler<Client, Propagator, Backoff>,
) -> MethodRouter<S, Infallible>
where
    S: Clone + Send + Sync + 'static,
    Client: Clone + Send + Sync + 'static,
    Propagator: PropagateEvent<Client>,
    Backoff: Clone + Send + Sync + 'static,
{
    routing::post(handler)
}

#[cfg(test)]
mod tests {
    use axum::Router as AxumRouter;

    use super::{get_updates_router, UpdatesHandler};
    use crate::{client::Reqwest, enums::UpdateType, Bot, Dispatcher, Router as TelersRouter};

    #[tokio::test]
    async fn test_register() {
        let bot = Bot::<Reqwest>::default();
        let router = TelersRouter::default();

        let dispatcher = Dispatcher::builder()
            .main_router(router.configure_default())
            .bot(bot.clone())
            .allowed_update(UpdateType::Message)
            .build();

        let _ = AxumRouter::<()>::new().route(
            "/",
            get_updates_router(UpdatesHandler::new(bot, dispatcher)),
        );
    }
}
