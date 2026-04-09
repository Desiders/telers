use serde::{Deserialize, Serialize};

use super::{ChatEvent, Context, EventContext, Stack};

/// Access rules stored on a stack or dialog context.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccessSettings {
    /// Allowed user ids for non-private chats. Empty means unrestricted.
    pub user_ids: Vec<i64>,
    /// Application-specific access payload available to custom validators.
    pub custom: Option<serde_json::Value>,
}

/// Strategy object used to decide whether an event may interact with a stack.
pub trait StackAccessValidator: Send + Sync {
    /// Return `true` when the current event is allowed to access the stack.
    #[must_use]
    fn is_allowed(
        &self,
        stack: &Stack,
        context: Option<&Context>,
        event: &ChatEvent,
        event_ctx: &EventContext,
    ) -> bool;
}

/// Default access validator matching the built-in [`AccessSettings`] semantics.
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
