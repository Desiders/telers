use telers::types::Message;

use crate::{entities::Context, widgets::ButtonAction};

pub trait Input: Send + Sync + 'static {
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction>;
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
    fn handle_message(&self, ctx: &Context, message: Message) -> Option<ButtonAction> {
        self.inputs
            .iter()
            .find_map(|input| input.handle_message(ctx, message.clone()))
    }
}
