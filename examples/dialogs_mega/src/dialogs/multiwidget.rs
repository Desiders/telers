//! Several stateful widgets combined in a single window.

use telers_dialog::{
    widgets::{keyboard, text, Checkbox, Counter, Group, InlineKeyboard, Multiselect, Radio},
    window, Dialog,
};

use crate::common::{main_menu_button, FRUITS};

pub const STATE: &str = "multiwidget_main";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            text(
                "Combined widgets\n\nA checkbox, a radio, a multiselect, and a counter all live \
                 in one window. Each keeps its own value in `widget_data`.",
            ),
            keyboard(
                Checkbox::builder("mw_checkbox")
                    .checked_text("✓ Subscribe to updates")
                    .unchecked_text("□ Subscribe to updates")
                    .build(),
            ),
            keyboard(
                Group::builder(
                    Radio::builder("mw_radio")
                        .items_getter(|_data| FRUITS)
                        .checked_renderer(|item, _data| format!("🔘 {}", item.name))
                        .unchecked_renderer(|item, _data| format!("⚪️ {}", item.name))
                        .id_getter(|item| item.id)
                        .build(),
                )
                .items_per_row(2)
                .build(),
            ),
            keyboard(
                Group::builder(
                    Multiselect::builder("mw_multi")
                        .items_getter(|_data| FRUITS)
                        .checked_renderer(|item, _data| format!("✓ {}", item.name))
                        .unchecked_renderer(|item, _data| item.name.to_owned())
                        .id_getter(|item| item.id)
                        .build(),
                )
                .items_per_row(2)
                .build(),
            ),
            keyboard(
                Counter::builder("mw_counter")
                    .default(0.0)
                    .min(0.0)
                    .max(10.0)
                    .build(),
            ),
            keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
        ],
    )])
}
