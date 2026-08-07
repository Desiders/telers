//! `Calendar` date selection (default and customized) and the `TimeSelect` grid.

use serde_json::Value;
use telers_dialog::{
    entities::DataMap,
    widgets::{
        fn_text, keyboard, text, Button, ButtonAction, Calendar, CalendarAppearance,
        CalendarButtonKind, InlineKeyboard, TimeSelect,
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

/// Shared readout of the date stored by either calendar window.
fn selected_date_text(data: &DataMap) -> String {
    data.get("selected_date")
        .and_then(Value::as_str)
        .map_or_else(
            || "Selected date: none yet.".to_owned(),
            |date| format!("Selected date: {date}"),
        )
}

/// Label override used by the customized calendar.
///
/// A `text_renderer` fully replaces the built-in labels, so it must return a
/// string for every [`CalendarButtonKind`]. `CalendarDate` is a `time::Date`
/// alias, so its `day`/`month`/`year` accessors are available here directly.
fn custom_calendar_label(kind: CalendarButtonKind) -> String {
    match kind {
        CalendarButtonKind::Empty => " ".to_owned(),
        CalendarButtonKind::Weekday { weekday } => weekday
            .to_string()
            .chars()
            .take(2)
            .collect::<String>()
            .to_uppercase(),
        CalendarButtonKind::DaysHeader { month } => {
            format!("✦ {} {} ✦", month.month(), month.year())
        }
        CalendarButtonKind::Day { date, is_today } => {
            if is_today {
                format!("·{:02}·", date.day())
            } else {
                format!("{:02}", date.day())
            }
        }
        CalendarButtonKind::DaysPrevMonth { .. } => "◀ Prev".to_owned(),
        CalendarButtonKind::DaysNextMonth { .. } => "Next ▶".to_owned(),
        CalendarButtonKind::DaysZoom { .. } | CalendarButtonKind::MonthsZoom { .. } => {
            "🔍".to_owned()
        }
        CalendarButtonKind::MonthsHeader { year } => format!("✦ {year} ✦"),
        CalendarButtonKind::Month { month, is_current } => {
            if is_current {
                format!("·{}·", month.month())
            } else {
                month.month().to_string()
            }
        }
        CalendarButtonKind::MonthsPrevYear { .. } | CalendarButtonKind::YearsPrevPage { .. } => {
            "◀".to_owned()
        }
        CalendarButtonKind::MonthsNextYear { .. } | CalendarButtonKind::YearsNextPage { .. } => {
            "▶".to_owned()
        }
        CalendarButtonKind::Year { year, is_current } => {
            if is_current {
                format!("·{year}·")
            } else {
                format!("{year}")
            }
        }
    }
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
                            "c_custom",
                            "🎨 Custom calendar",
                            "calendar_custom",
                        )])
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
                text(
                    "Calendar\n\nDefault appearance. Tap a day; the selection is stored in dialog \
                     data.\n",
                ),
                fn_text(selected_date_text),
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
            "calendar_custom",
            [
                text(
                    "Custom calendar\n\nThe labels are customized with \
                     `CalendarAppearance::text_renderer` — emoji headers, dotted today/selection \
                     markers, and custom navigation.\n",
                ),
                fn_text(selected_date_text),
                keyboard(
                    Calendar::builder("custom_calendar")
                        .appearance(
                            CalendarAppearance::builder()
                                .text_renderer(
                                    |kind, _ctx| async move { custom_calendar_label(kind) },
                                )
                                .build(),
                        )
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
