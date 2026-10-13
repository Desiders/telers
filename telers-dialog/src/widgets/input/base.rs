use std::sync::Arc;

use async_fn_traits::AsyncFn1;
use bon::bon;
use telers::types::Message;

use crate::{entities::Context, future::BoxFuture, widgets::ButtonAction};
use async_trait::async_trait;

/// Widget that consumes an incoming user [`Message`] for the active dialog.
///
/// Implementations decide whether they can interpret the message; returning
/// `None` means "this widget did not claim the message" and the runtime falls
/// through to other inputs registered on the window.
#[async_trait]
pub trait Input: Send + Sync + 'static {
    /// Attempt to handle `message` and emit a [`ButtonAction`] when claimed.
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;

    #[must_use]
    fn filter(self, filter: InputFilter) -> FilteredInput<Self>
    where
        Self: Sized,
    {
        FilteredInput {
            input: self,
            filter,
        }
    }

    #[cfg(test)]
    async fn handle_message_for_test(
        &self,
        ctx: &Context,
        message: Message,
    ) -> Option<ButtonAction> {
        self.handle_message(ctx, message).await
    }
}

#[derive(Clone, Debug)]
pub struct InputFilterContext {
    pub context: Arc<Context>,
    pub message: Arc<Message>,
}

type InputPredicate = dyn Fn(InputFilterContext) -> BoxFuture<'static, bool> + Send + Sync;

/// An async predicate checked before an input consumes a message.
#[derive(Clone)]
pub struct InputFilter(Arc<InputPredicate>);

impl InputFilter {
    #[must_use]
    pub fn new<F>(predicate: F) -> Self
    where
        F: AsyncFn(InputFilterContext) -> bool
            + AsyncFn1<InputFilterContext, Output = bool>
            + Send
            + Sync
            + 'static,
        <F as AsyncFn1<InputFilterContext>>::OutputFuture: Send + 'static,
    {
        let predicate = Arc::new(predicate);
        Self(Arc::new(move |ctx| {
            let predicate = predicate.clone();
            Box::pin(async move { predicate(ctx).await })
        }))
    }

    async fn check(&self, ctx: &Context, message: &Message) -> bool {
        (self.0)(InputFilterContext {
            context: Arc::new(ctx.clone()),
            message: Arc::new(message.clone()),
        })
        .await
    }
}

pub struct FilteredInput<I> {
    input: I,
    filter: InputFilter,
}

#[async_trait]
impl<I: Input> Input for FilteredInput<I> {
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        if !self.filter.check(ctx, &message).await {
            return None;
        }
        self.input.handle_message(ctx, message).await
    }
}

/// A group of inputs evaluated in declaration order, stopping at the first match.
pub struct CombinedInput {
    inputs: MultiInput,
    filter: Option<InputFilter>,
}

#[bon]
impl CombinedInput {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(field)] inputs: Vec<Box<dyn Input>>,
        filter: Option<InputFilter>,
    ) -> Self {
        let inputs = inputs
            .into_iter()
            .fold(MultiInput::new(), |group, input| group.input_boxed(input));
        Self {
            inputs,
            filter,
        }
    }
}

impl<S> CombinedInputBuilder<S>
where
    S: combined_input_builder::State,
{
    pub fn input(mut self, input: impl Input) -> Self {
        self.inputs.push(Box::new(input));
        self
    }
}

#[async_trait]
impl Input for CombinedInput {
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        if let Some(filter) = &self.filter {
            if !filter.check(ctx, &message).await {
                return None;
            }
        }
        self.inputs.handle_message(ctx, message).await
    }
}

pub(crate) struct MultiInput {
    inputs: Vec<Box<dyn Input>>,
}

impl MultiInput {
    #[inline]
    #[must_use]
    pub(crate) const fn new() -> Self {
        Self {
            inputs: Vec::new(),
        }
    }

    #[must_use]
    pub(crate) fn input_boxed(mut self, input: Box<dyn Input>) -> Self {
        self.inputs.push(input);
        self
    }
}

#[async_trait]
impl Input for MultiInput {
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        for input in &self.inputs {
            if let Some(action) = input.handle_message(ctx, message.clone()).await {
                return Some(action);
            }
        }
        None
    }
}
