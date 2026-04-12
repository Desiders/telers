use std::marker::PhantomData;

use telers::types::Message;

use super::Input;
use crate::{entities::Context, widgets::ButtonAction};

pub struct MessageInput<Handler, MessageType> {
    handler: Handler,
    marker: PhantomData<fn() -> MessageType>,
}

impl<Handler, MessageType> MessageInput<Handler, MessageType> {
    #[inline]
    #[must_use]
    pub const fn new(handler: Handler) -> Self
    where
        Handler: Fn(&Context, MessageType) -> ButtonAction,
        MessageType: TryFrom<Message>,
    {
        Self {
            handler,
            marker: PhantomData,
        }
    }
}

impl<Handler, MessageType> Input for MessageInput<Handler, MessageType>
where
    Handler: Fn(&Context, MessageType) -> ButtonAction + Send + Sync + 'static,
    MessageType: TryFrom<Message> + 'static,
{
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        Some((self.handler)(ctx, message.try_into().ok()?))
    }
}
