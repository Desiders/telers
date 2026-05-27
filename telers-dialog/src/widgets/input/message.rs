use std::{marker::PhantomData, sync::Arc};

use async_fn_traits::AsyncFn2;
use async_trait::async_trait;
use telers::types::Message;

use super::Input;
use crate::{entities::Context, widgets::ButtonAction};

/// Runtime inputs available to a [`MessageInput`] handler.
#[derive(Clone, Debug)]
pub struct MessageInputContext {
    /// Stored dialog context for the active intent.
    pub context: Arc<Context>,
}

/// Input widget that forwards strongly-typed incoming messages to an async handler.
///
/// `MessageType` is any value implementing [`TryFrom<Message>`]; the widget
/// attempts the conversion and silently skips messages that don't match, so
/// other input widgets on the same window can still claim them.
///
/// # Example
///
/// ```ignore
/// use telers::types::MessageContact;
/// use telers_dialog::widgets::{ButtonAction, MessageInput};
///
/// let input = MessageInput::new(|_ctx, message: MessageContact| async move {
///     ButtonAction::set_dialog_value("phone", message.contact.phone_number)
/// });
/// ```
pub struct MessageInput<Handler, MessageType> {
    handler: Handler,
    marker: PhantomData<fn() -> MessageType>,
}

impl<Handler, MessageType> MessageInput<Handler, MessageType> {
    /// Create a new message-input widget bound to `handler`.
    #[inline]
    #[must_use]
    pub const fn new(handler: Handler) -> Self
    where
        Handler: AsyncFn(MessageInputContext, MessageType) -> ButtonAction
            + AsyncFn2<MessageInputContext, MessageType, Output = ButtonAction>,
        <Handler as AsyncFn2<MessageInputContext, MessageType>>::OutputFuture: Send + 'static,
        MessageType: TryFrom<Message> + Send + 'static,
    {
        Self {
            handler,
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<Handler, MessageType> Input for MessageInput<Handler, MessageType>
where
    Handler: AsyncFn(MessageInputContext, MessageType) -> ButtonAction
        + AsyncFn2<MessageInputContext, MessageType, Output = ButtonAction>
        + Send
        + Sync
        + 'static,
    <Handler as AsyncFn2<MessageInputContext, MessageType>>::OutputFuture: Send + 'static,
    MessageType: TryFrom<Message> + Send + 'static,
{
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        Some(
            (self.handler)(
                MessageInputContext {
                    context: Arc::new(ctx.clone()),
                },
                message.try_into().ok()?,
            )
            .await,
        )
    }
}
