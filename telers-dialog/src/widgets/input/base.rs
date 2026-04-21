use telers::types::Message;

use crate::{entities::Context, future::BoxFuture, widgets::ButtonAction};

pub trait Input: Send + Sync + 'static {
    fn handle_message<'a>(
        &'a self,
        ctx: &'a Context,
        message: Message,
    ) -> BoxFuture<'a, Option<ButtonAction>>;

    #[cfg(test)]
    fn handle_message_for_test<'a>(
        &'a self,
        ctx: &'a Context,
        message: Message,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        self.handle_message(ctx, message)
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

impl Input for MultiInput {
    fn handle_message<'a>(
        &'a self,
        ctx: &'a Context,
        message: Message,
    ) -> BoxFuture<'a, Option<ButtonAction>> {
        Box::pin(async move {
            for input in &self.inputs {
                if let Some(action) = input.handle_message(ctx, message.clone()).await {
                    return Some(action);
                }
            }
            None
        })
    }
}
