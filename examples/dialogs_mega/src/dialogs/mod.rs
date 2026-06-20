//! Feature dialogs for the mega bot.
//!
//! Each submodule exposes a `dialog()` function that returns a ready-to-register
//! dialog, plus the state ids it owns. The [`main_menu`] dialog links to the
//! others by starting their entry state.

pub mod button_actions;
pub mod buttons;
pub mod calendar;
pub mod counter;
pub mod inputs;
pub mod layouts;
pub mod link_preview;
pub mod main_menu;
pub mod media;
pub mod multiwidget;
pub mod reply_kbd;
pub mod scrolls;
pub mod selects;
pub mod switch;
pub mod template;
pub mod text_widgets;
