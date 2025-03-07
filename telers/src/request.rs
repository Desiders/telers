use crate::{client::Reqwest, types::Update, Bot, Context, Extensions};

use std::{fmt, sync::Arc};

#[derive(Default, Clone)]
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
