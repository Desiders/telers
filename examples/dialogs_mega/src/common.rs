//! Shared constants and helpers used across the mega bot's feature dialogs.

use telers_dialog::widgets::Button;

/// State id of the root menu window.
///
/// Every feature dialog is started on top of this state and returns to it with
/// [`Button::done`] (see [`main_menu_button`]).
pub const MAIN_MENU_STATE: &str = "mega_main_menu";

/// A `☰ Main menu` button that closes the current feature dialog and pops back
/// to the root menu.
///
/// The id only has to be unique inside the window it is placed in, so the same
/// `"main_menu"` id is reused across windows.
#[must_use]
pub fn main_menu_button() -> Button {
    Button::done("main_menu", "☰ Main menu").primary()
}

/// A fruit item used by the selection and combined-widget dialogs.
#[derive(Clone, Copy)]
pub struct Fruit {
    pub id: &'static str,
    pub name: &'static str,
    pub emoji: &'static str,
}

/// The shared fruit catalog.
pub const FRUITS: &[Fruit] = &[
    Fruit {
        id: "apple",
        name: "Apple",
        emoji: "🍏",
    },
    Fruit {
        id: "banana",
        name: "Banana",
        emoji: "🍌",
    },
    Fruit {
        id: "orange",
        name: "Orange",
        emoji: "🍊",
    },
    Fruit {
        id: "pear",
        name: "Pear",
        emoji: "🍐",
    },
];
