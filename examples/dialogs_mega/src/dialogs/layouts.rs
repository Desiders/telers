//! Keyboard layouts: a `Select` arranged with different `items_per_row`.
//!
//! `telers-dialog` has no separate `Row`/`Column` widgets; the row width is a
//! `Group` setting, so this dialog shows the same select at width 4, 1, and 2.

use telers_dialog::{
    widgets::{keyboard, text, Button, ButtonAction, Group, InlineKeyboard, Keyboard, Select},
    window, Dialog,
};

use crate::common::{main_menu_button, FRUITS};

pub const STATE: &str = "layouts_menu";

/// Build a select whose chosen item is written to `dialog_data[layout_pick]`.
fn fruit_select(id: &'static str) -> impl Keyboard {
    Select::builder(id)
        .items_getter(|_data| FRUITS)
        .item_renderer(|item, _data| format!("{} {}", item.emoji, item.name))
        .id_getter(|item| item.id)
        .action(|value| async move { ButtonAction::set_dialog_value("layout_pick", value) })
        .build()
}

/// Back-to-menu controls shared by the three demo windows.
fn demo_nav() -> InlineKeyboard {
    InlineKeyboard::builder()
        .row([Button::switch_to("back", "Back", STATE)])
        .row([main_menu_button()])
        .build()
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text("Keyboard layouts\n\nSame select, different row widths."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("row", "↔️ Width 4 (row)", "layouts_row")])
                        .row([Button::switch_to(
                            "column",
                            "↕️ Width 1 (column)",
                            "layouts_column",
                        )])
                        .row([Button::switch_to(
                            "group",
                            "▦ Width 2 (group)",
                            "layouts_group",
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "layouts_row",
            [
                text("Width 4 — one wide row of items."),
                keyboard(
                    Group::builder(fruit_select("layout_row_select"))
                        .items_per_row(4)
                        .build(),
                ),
                keyboard(demo_nav()),
            ],
        ),
        window(
            "layouts_column",
            [
                text("Width 1 — one item per row (a column)."),
                keyboard(
                    Group::builder(fruit_select("layout_col_select"))
                        .items_per_row(1)
                        .build(),
                ),
                keyboard(demo_nav()),
            ],
        ),
        window(
            "layouts_group",
            [
                text("Width 2 — a 2-wide grid."),
                keyboard(
                    Group::builder(fruit_select("layout_grp_select"))
                        .items_per_row(2)
                        .build(),
                ),
                keyboard(demo_nav()),
            ],
        ),
    ])
}
