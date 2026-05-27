//! Text widgets: static text, `FormatText`, `FnText`, and `ListText`.

use serde_json::Value;
use telers_dialog::{
    entities::DataMap,
    widgets::{
        fn_text, format_text, keyboard, text, Button, ButtonAction, InlineKeyboard, ListText,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "text_intro";

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([
        window(
            STATE,
            [
                text(
                    "Text widgets\n\nThe preview screen combines static text, formatted dialog \
                     data, computed text, and a rendered list into one message.",
                ),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::action(
                            "show_preview",
                            "Open preview",
                            ButtonAction::chain([
                                ButtonAction::extend_dialog_data([
                                    ("cafe_name", "North Roast"),
                                    ("campaign", "Weekend Espresso Sale"),
                                    ("week", "April 8-14"),
                                ]),
                                ButtonAction::next(),
                            ]),
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "text_preview",
            [
                text("Broadcast preview\n"),
                format_text("Cafe: {cafe_name}\nCampaign: {campaign}\nWeek: {week}\n"),
                fn_text(|data: &DataMap| {
                    let cafe = data
                        .get("cafe_name")
                        .and_then(Value::as_str)
                        .unwrap_or("the cafe");
                    format!("Computed line: {cafe} is running three weekend offers.\n")
                }),
                text(
                    ListText::builder()
                        .items_getter(|_data| {
                            [
                                "Espresso beans at 15% off",
                                "Saturday cupping at 12:00",
                                "Reusable cup reward for takeout",
                            ]
                        })
                        .item_renderer(|&item, _data| format!("- {item}"))
                        .build(),
                ),
                text("\n[Text] `text`, `FormatText`, `FnText`, and `ListText` in one window."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::back("back", "Back")])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
    ])
}
