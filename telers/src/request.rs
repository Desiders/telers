use crate::{client::Reqwest, types::Update, Bot, Context, Extensions};

use std::{fmt, sync::Arc};

/// Everything a processing-unit (middleware, filter or handler) receives for an incoming event.
///
/// The request is created by the [`Dispatcher`](crate::Dispatcher) for every incoming [`Update`]
/// and passed through the routing chain; middlewares can modify its [`context`](Self::context)
/// and [`extensions`](Self::extensions) to pass data to the units that run after them.
#[derive(Clone)]
pub struct Request<Client = Reqwest> {
    pub bot: Bot<Client>,
    pub update: Arc<Update>,
    pub context: Context,
    pub extensions: Extensions,
}

impl<Client> fmt::Debug for Request<Client> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("bot", &self.bot)
            .field("update", &self.update)
            .field("context", &self.context)
            .field("extensions", &self.extensions)
            .finish()
    }
}
