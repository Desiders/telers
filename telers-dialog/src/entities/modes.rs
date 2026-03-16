/// Modes of launching new dialog.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchMode {
    /// Dialogs have no limitations themselves
    #[default]
    Standard,
    /// Dialogs will be always a root dialog in stack.
    ///
    /// Starting such dialogs will automatically reset stack.
    Root,
    /// Dialogs can be only a single dialog in stack.
    ///
    /// Starting such dialogs will automatically reset stack.
    /// Starting other dialogs on top of them is forbidden.
    Exclusive,
    /// Dialogs will not be repeated on top of stack.
    ///
    /// Starting the same dialog right on top of it will just replace it.
    SingleTop,
}

/// Modes of show dialog message when new update handled
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShowMode {
    /// Uses [`Self::Send`] mode when new message from user handled or [`Self::Edit`] mode when any other updated handled.
    #[default]
    Auto,
    /// Edit dialog message
    Edit,
    /// Send new dialog message
    Send,
    /// Delete and send new dialog message.
    /// # Warning
    /// Telegram's restrictions will prevent the deletion of the message when more than 2 days has elapsed.
    DeleteAndSend,
    /// Will not update and rerender the dialog message
    NoUpdate,
}

/// Modes of starting a new dialog
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartMode {
    /// This mode continues from the current state without resetting or creating a new stack.
    #[default]
    Normal,
    /// This mode clears the existing stack and starts fresh.
    /// It is used when the existing stack needs to be discarded and a new operation stack is required.
    ResetStack,
    /// This mode initiates a new stack while retaining the old one,
    /// useful when a new sequence of operations is to be started alongside the current one.
    NewStack,
}
