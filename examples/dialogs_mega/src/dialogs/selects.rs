//! Selection widgets: `Select`, `Radio`, `Multiselect`, and `Toggle`.

use telers_dialog::{
    widgets::{
        format_text, keyboard, text, Button, ButtonAction, Group, InlineKeyboard, Multiselect,
        Radio, Select, Toggle,
    },
    window, Dialog,
};

use crate::common::{main_menu_button, FRUITS};

pub const STATE: &str = "selects_menu";

fn back_nav() -> InlineKeyboard {
    InlineKeyboard::builder()
        .row([Button::switch_to("back", "Back", STATE)])
        .row([main_menu_button()])
        .build()
}

#[allow(clippy::too_many_lines)]
pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text("Selection widgets\n\nChoose a widget to try."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("s_select", "Select", "selects_select")])
                        .row([Button::switch_to("s_radio", "Radio", "selects_radio")])
                        .row([Button::switch_to("s_multi", "Multiselect", "selects_multi")])
                        .row([Button::switch_to("s_toggle", "Toggle", "selects_toggle")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "selects_select",
            [
                format_text(
                    "Select\n\nTap an item to store it in dialog data; the line below \
                     updates.\nSelected: {selected_fruit}",
                ),
                keyboard(
                    Group::builder(
                        Select::builder("fruit_select")
                            .items_getter(|_data| FRUITS)
                            .item_renderer(|item, _data| format!("{} {}", item.emoji, item.name))
                            .id_getter(|item| item.id)
                            .action(|value| async move {
                                ButtonAction::set_dialog_value("selected_fruit", value)
                            })
                            .build(),
                    )
                    .items_per_row(1)
                    .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "selects_radio",
            [
                text("Radio\n\nExactly one item stays selected."),
                keyboard(
                    Group::builder(
                        Radio::builder("fruit_radio")
                            .items_getter(|_data| FRUITS)
                            .checked_renderer(|item, _data| {
                                format!("🔘 {} {}", item.emoji, item.name)
                            })
                            .unchecked_renderer(|item, _data| {
                                format!("⚪️ {} {}", item.emoji, item.name)
                            })
                            .id_getter(|item| item.id)
                            .build(),
                    )
                    .items_per_row(1)
                    .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "selects_multi",
            [
                text("Multiselect\n\nToggle between one and three items."),
                keyboard(
                    Group::builder(
                        Multiselect::builder("fruit_multi")
                            .items_getter(|_data| FRUITS)
                            .checked_renderer(|item, _data| {
                                format!("✓ {} {}", item.emoji, item.name)
                            })
                            .unchecked_renderer(|item, _data| {
                                format!("{} {}", item.emoji, item.name)
                            })
                            .id_getter(|item| item.id)
                            .min_selected(1)
                            .max_selected(3)
                            .build(),
                    )
                    .items_per_row(1)
                    .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "selects_toggle",
            [
                text("Toggle\n\nOne button cycles through the items."),
                keyboard(
                    Toggle::builder("fruit_toggle")
                        .items_getter(|_data| FRUITS)
                        .item_renderer(|item, _data| format!("{} {}", item.emoji, item.name))
                        .id_getter(|item| item.id)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
    ])
}
