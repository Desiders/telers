//! Several stateful widgets combined in a single window.

use telers_dialog::{
    widgets::{format_text, keyboard, ButtonAction, ButtonAppearance, ButtonStyle, Checkbox, Counter, Group, InlineKeyboard,
        Multiselect, OnStateChanged, Radio, StateChange},
    window, Dialog,
};

use crate::common::{main_menu_button, FRUITS};

pub const STATE: &str = "multiwidget_main";

async fn record_change(change: StateChange) -> ButtonAction {
    ButtonAction::set_dialog_value(
        "last_widget_change", format!("{}: {:?}", change.widget_id, change.new_value),
    )
}

pub fn dialog() -> impl Dialog {
    telers_dialog::dialog([window(
        STATE,
        [
            format_text(
                "Combined widgets\n\nA checkbox, a radio, a multiselect, and a counter all live \
                 in one window. Each keeps its own value in `widget_data`.\n\n\
                 Last state-change hook: {last_widget_change}",
            ),
            keyboard(
                Checkbox::builder("mw_checkbox")
                    .checked_text("✓ Subscribe to updates")
                    .unchecked_text("□ Subscribe to updates")
                    .checked_appearance(ButtonAppearance::builder().style(ButtonStyle::Success).build())
                    .unchecked_appearance(ButtonAppearance::builder().style(ButtonStyle::Primary).build())
                    .on_state_changed(OnStateChanged::new(record_change))
                    .build(),
            ),
            keyboard(
                Group::builder(
                    Radio::builder("mw_radio")
                        .items_getter(|_data| FRUITS)
                        .checked_renderer(|item, _data| format!("🔘 {}", item.name))
                        .unchecked_renderer(|item, _data| format!("⚪️ {}", item.name))
                        .id_getter(|item| item.id)
                        .on_state_changed(OnStateChanged::new(record_change))
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
                        .on_state_changed(OnStateChanged::new(record_change))
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
                    .minus_appearance(ButtonAppearance::builder().style(ButtonStyle::Danger).build())
                    .plus_appearance(ButtonAppearance::builder().style(ButtonStyle::Success).build())
                    .value_appearance(ButtonAppearance::builder().style(ButtonStyle::Primary).build())
                    .on_state_changed(OnStateChanged::new(record_change))
                    .build(),
            ),
            keyboard(InlineKeyboard::builder().row([main_menu_button()]).build()),
        ],
    )])
}
