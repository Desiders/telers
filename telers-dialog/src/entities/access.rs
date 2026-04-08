use serde::{Deserialize, Serialize};

use super::{ChatEvent, Context, EventContext, Stack};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccessSettings {
    pub user_ids: Vec<i64>,
    pub custom: Option<serde_json::Value>,
}

pub trait StackAccessValidator: Send + Sync {
    #[must_use]
    fn is_allowed(
        &self,
        stack: &Stack,
        context: Option<&Context>,
        event: &ChatEvent,
        event_ctx: &EventContext,
    ) -> bool;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultAccessValidator;

impl StackAccessValidator for DefaultAccessValidator {
    fn is_allowed(
        &self,
        stack: &Stack,
        context: Option<&Context>,
        _event: &ChatEvent,
        event_ctx: &EventContext,
    ) -> bool {
        let access_settings = context
            .and_then(|ctx| ctx.access_settings.as_ref())
            .or(stack.access_settings.as_ref());

        let Some(settings) = access_settings else {
            return true;
        };
        if matches!(event_ctx.chat, telers::types::Chat::Private(_)) {
            return true;
        }
        if settings.user_ids.is_empty() {
            return true;
        }
        settings.user_ids.contains(&event_ctx.user.id)
    }
}
