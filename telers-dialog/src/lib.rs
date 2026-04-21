//! `telers-dialog` is a Rust-native dialog framework for `telers`.
//!
//! The crate exposes a small set of dialog primitives:
//! - [`DialogRegistry`] for registering dialogs by state
//! - [`DialogManager`] for runtime navigation and rendering
//! - [`dialog`] and [`window`] builders for dialog structure
//! - [`widgets`] for text, keyboard, input, and link-preview composition

mod dialog;
pub mod entities;
mod errors;
mod future;
mod manager;
mod message_manager;
mod registry;
mod setup;
pub mod widgets;
mod window;

pub use async_trait::async_trait;
pub use dialog::{dialog, Dialog, IntoDialog};
pub use entities::{
    AccessSettings, Data, DefaultAccessValidator, LaunchMode, ShowMode, StackAccessValidator,
    StartMode, DEFAULT_STACK_ID,
};
pub use errors::DialogError;
pub use manager::DialogManager;
pub use message_manager::MessageManager;
pub use registry::DialogRegistry;
pub use setup::{DialogContextMiddleware, DialogManagerMiddleware, DialogObserverExt};
pub use window::{window, IntoWindow, Window};
