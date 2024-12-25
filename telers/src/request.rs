use crate::{client::Reqwest, types::Update, Bot, Context, Extensions};

use std::{fmt, sync::Arc};

#[derive(Default)]
pub struct Request<Client = Reqwest> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Context,
    pub extensions: Extensions,
}

impl<Client> Clone for Request<Client> {
    fn clone(&self) -> Self {
        Self {
            bot: Arc::clone(&self.bot),
            update: Arc::clone(&self.update),
            context: self.context.clone(),
            extensions: self.extensions.clone(),
        }
    }
}

impl<Client> PartialEq for Request<Client> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot) && Arc::ptr_eq(&self.update, &other.update)
    }
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
