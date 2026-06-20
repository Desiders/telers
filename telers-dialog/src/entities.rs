//! Shared dialog runtime entities.
//!
//! This module exports the persistent state and runtime context types used
//! throughout the dialog manager, renderer, and middleware layers.

mod access;
mod context;
mod events;
mod messages;
mod modes;
mod render;
mod result;
mod stack;
mod update_event;

pub use access::{AccessSettings, DefaultAccessValidator, StackAccessValidator};
pub use context::{generate_id, Context, Data, DataMap};
pub use events::{
    chat_event_from_update, ChatEvent, EventContext, CHAT_EVENT_KEY, EVENT_CONTEXT_KEY,
};
pub use messages::{NewMessage, OldMessage};
pub use modes::{LaunchMode, ShowMode, StartMode};
pub use render::RenderContext;
pub use result::ResultContext;
pub use stack::{Stack, DEFAULT_STACK_ID};
pub use update_event::{
    DialogAction, DialogStartEvent, DialogSwitchEvent, DialogUpdateEvent, DIALOG_EVENT_NAME,
};
