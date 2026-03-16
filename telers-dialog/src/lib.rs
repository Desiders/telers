pub mod dialog;
pub mod entities;
pub mod errors;
pub mod manager;
pub mod message_manager;
pub mod registry;
pub mod setup;
pub mod widgets;
pub mod window;

pub use dialog::{Dialog, DialogImpl, IntoDialog};
pub use entities::{AccessSettings, Data, LaunchMode, ShowMode, StartMode, DEFAULT_STACK_ID};
pub use errors::DialogError;
pub use manager::DialogManager;
pub use message_manager::MessageManager;
pub use registry::DialogRegistry;
pub use setup::{
    DialogContextMiddleware, DialogManagerMiddleware, DialogObserverExt, Dialogs,
};
pub use widgets::{
    Button, ButtonAction, FnText, FormatText, InlineKeyboard, Keyboard, MultiText, Text,
};
pub use window::{IntoWindow, Window, WindowImpl};
