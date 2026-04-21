use telers::types::Message;

use crate::{entities::Context, widgets::ButtonAction};
use async_trait::async_trait;

#[async_trait]
pub trait Input: Send + Sync + 'static {
    async fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;

    #[cfg(test)]
    async fn handle_message_for_test(
        &self,
        ctx: &Context,
        message: Message,
    ) -> Option<ButtonAction> {
        self.handle_message(ctx, message).await
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
