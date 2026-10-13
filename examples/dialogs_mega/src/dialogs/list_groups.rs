//! Repeated keyboard rows with independent per-item state and item pagination.

use telers::serialization::{json, Value};
use telers_dialog::{
    widgets::{
        format_text, keyboard, text, Button, ButtonAction, Checkbox, ClickContext, FormatText,
        InlineKeyboard, Keyboard, KeyboardGroup, ListGroup, NumberedPager, Scroll,
    },
    window, Dialog,
};

use crate::common::{main_menu_button, FRUITS};

pub const STATE: &str = "list_groups";

fn fruit_rows() -> impl Keyboard + Scroll {
    ListGroup::builder("fruit_rows")
        .items_getter(|_data| {
            FRUITS.iter().map(|fruit| json!({
                "id": fruit.id, "name": fruit.name, "emoji": fruit.emoji,
            })).collect::<Vec<_>>()
        })
        .id_getter(|item: &Value| item["id"].as_str().unwrap_or_default().to_owned())
        .page_size(2)
        .kbd(
            KeyboardGroup::builder()
                .items_per_row(2)
                .keyboard(
                    Checkbox::builder("favorite")
                        .checked_text(FormatText::new("✓ {item.emoji} {item.name}"))
                        .unchecked_text(FormatText::new("□ {item.emoji} {item.name}"))
                        .build(),
                )
                .keyboard(
                    InlineKeyboard::builder()
                        .row([Button::on_click("pick", "Pick", |click: ClickContext| async move {
                            ButtonAction::set_dialog_value(
                                "last_item",
                                click.data.get("item_id").cloned().unwrap_or_default(),
                            )
                        })])
                        .build(),
                )
                .build(),
        )
        .build()
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            text("Repeated keyboard rows\n\nEach fruit keeps its own checkbox state. Change pages and return to see it persist."),
            format_text("Last picked: {last_item}"),
            keyboard(fruit_rows()),
            keyboard(NumberedPager::builder(fruit_rows())
                .page_renderer(|page, _data| (page + 1).to_string())
                .current_page_renderer(|page, _data| format!("[{}]", page + 1))
                .build()),
            keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
        ],
    )])
}
