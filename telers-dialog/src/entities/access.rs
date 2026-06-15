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

#[cfg(test)]
mod tests {
    use super::{AccessSettings, DefaultAccessValidator, StackAccessValidator};
    use crate::entities::{ChatEvent, Context, EventContext, Stack};
    use serde_json::Value;
    use telers::{
        client::Reqwest,
        types::{ChatGroup, ChatPrivate, MessageText, User},
        Bot,
    };

    /// Build an event and event context for a group chat `G` with user id `N`.
    fn ev_group(group_id: i64, user_id: i64) -> (ChatEvent, EventContext<Reqwest>) {
        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatGroup::new(group_id), "/x")
                .from(User::new(user_id, false, "u"))
                .into(),
        );
        let event_ctx = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        (event, event_ctx)
    }

    /// Build an event and event context for a private chat `P` with user id `N`.
    fn ev_private(private_id: i64, user_id: i64) -> (ChatEvent, EventContext<Reqwest>) {
        let event = ChatEvent::Message(
            MessageText::new(1, 1, ChatPrivate::new(private_id), "/x")
                .from(User::new(user_id, false, "u"))
                .into(),
        );
        let event_ctx = EventContext::<Reqwest>::new(Bot::<Reqwest>::default(), event.clone());
        (event, event_ctx)
    }

    #[test]
    fn no_settings_allows() {
        let stack = Stack::new();
        let (event, event_ctx) = ev_group(100, 8);

        assert!(DefaultAccessValidator.is_allowed(&stack, None, &event, &event_ctx));
    }

    #[test]
    fn private_chat_always_allowed() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![7],
            custom: None,
        });
        let (event, event_ctx) = ev_private(10, 8);

        assert!(DefaultAccessValidator.is_allowed(&stack, None, &event, &event_ctx));
    }

    #[test]
    fn group_empty_user_ids_allowed() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![],
            custom: None,
        });
        let (event, event_ctx) = ev_group(100, 8);

        assert!(DefaultAccessValidator.is_allowed(&stack, None, &event, &event_ctx));
    }

    #[test]
    fn group_user_in_stack_user_ids_allowed() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![7],
            custom: None,
        });
        let (event, event_ctx) = ev_group(100, 7);

        assert!(DefaultAccessValidator.is_allowed(&stack, None, &event, &event_ctx));
    }

    #[test]
    fn group_user_not_in_stack_user_ids_denied() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![7],
            custom: None,
        });
        let (event, event_ctx) = ev_group(100, 8);

        assert!(!DefaultAccessValidator.is_allowed(&stack, None, &event, &event_ctx));
    }

    #[test]
    fn group_multi_user_allowlist() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![7, 8, 9],
            custom: None,
        });

        let (allowed_event, allowed_ctx) = ev_group(100, 8);
        assert!(DefaultAccessValidator.is_allowed(&stack, None, &allowed_event, &allowed_ctx));

        let (denied_event, denied_ctx) = ev_group(100, 10);
        assert!(!DefaultAccessValidator.is_allowed(&stack, None, &denied_event, &denied_ctx));
    }

    #[test]
    fn context_takes_priority_over_restrictive_stack() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![999],
            custom: None,
        });
        let mut ctx = Context::new("", "s", Value::Null);
        ctx.access_settings = Some(AccessSettings {
            user_ids: vec![7],
            custom: None,
        });
        let (event, event_ctx) = ev_group(100, 7);

        assert!(DefaultAccessValidator.is_allowed(&stack, Some(&ctx), &event, &event_ctx));
    }

    #[test]
    fn context_takes_priority_over_permissive_stack() {
        let mut stack = Stack::new();
        stack.access_settings = Some(AccessSettings {
            user_ids: vec![7],
            custom: None,
        });
        let mut ctx = Context::new("", "s", Value::Null);
        ctx.access_settings = Some(AccessSettings {
            user_ids: vec![999],
            custom: None,
        });
        let (event, event_ctx) = ev_group(100, 7);

        assert!(!DefaultAccessValidator.is_allowed(&stack, Some(&ctx), &event, &event_ctx));
    }
}
