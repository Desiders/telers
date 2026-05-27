//! `Calendar` date selection and the `TimeSelect` grid.

use serde_json::Value;
use telers_dialog::{
    entities::DataMap,
    widgets::{
        fn_text, keyboard, text, Button, ButtonAction, Calendar, InlineKeyboard, TimeSelect,
    },
    window, Dialog,
};

use crate::common::main_menu_button;

pub const STATE: &str = "calendar_menu";

fn back_nav() -> InlineKeyboard {
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
                text("Calendar & time\n\nPick a demo."),
                keyboard(
                    InlineKeyboard::builder()
                        .row([Button::switch_to("c_date", "📅 Calendar", "calendar_date")])
                        .row([Button::switch_to(
                            "c_time",
                            "🕒 Time select",
                            "calendar_time",
                        )])
                        .row([main_menu_button()])
                        .build(),
                ),
            ],
        ),
        window(
            "calendar_date",
            [
                text("Calendar\n\nTap a day; the selection is stored in dialog data.\n"),
                fn_text(|data: &DataMap| {
                    data.get("selected_date")
                        .and_then(Value::as_str)
                        .map_or_else(
                            || "Selected date: none yet.".to_owned(),
                            |date| format!("Selected date: {date}"),
                        )
                }),
                keyboard(
                    Calendar::builder("reservation_calendar")
                        .on_click(|_click, selected_date| async move {
                            ButtonAction::set_dialog_value(
                                "selected_date",
                                selected_date.to_string(),
                            )
                        })
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
        window(
            "calendar_time",
            [
                text("Time select\n\nThe selected hour and minute stay highlighted."),
                keyboard(
                    TimeSelect::builder("digest_time")
                        .hour_header("Hour".into())
                        .minute_header("Minute".into())
                        .minute_precision(15)
                        .minute_width(4)
                        .build(),
                ),
                keyboard(back_nav()),
            ],
        ),
    ])
}
