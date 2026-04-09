//! Inline keyboard widgets and helpers.
//!
//! This module contains the public keyboard surface used by windows:
//! - primitive buttons such as [`Button`]
//! - navigation and mutation actions via [`ButtonAction`]
//! - collection widgets such as [`Select`], [`Radio`], and [`Multiselect`]
//! - paging helpers such as [`ScrollingGroup`] and [`NumberedPager`]

mod action;
mod base;
mod button;
mod callback;
mod group;
mod inline_keyboard;
mod pager;
mod select;
mod stateful_select;

pub(crate) use base::MultiKeyboard;
pub(crate) use callback::{format_callback_data, parse_callback_data};
pub(crate) use select::render_button_row;

pub use action::ButtonAction;
pub use base::Keyboard;
pub use button::Button;
pub use group::Group;
pub use inline_keyboard::InlineKeyboard;
pub use pager::{
    sync_scroll, sync_scrolls, CurrentPage, FirstPage, LastPage, NextPage, NumberedPager,
    OnPageChanged, PageChange, PageDirection, PrevPage, ScrollingGroup, SwitchPage,
};
pub use select::Select;
pub use stateful_select::{Multiselect, Radio, Toggle};

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{Button, ButtonAction, Group, InlineKeyboard, Keyboard, Select};
    use crate::entities::{Context, DataMap, StartMode};

    #[test]
    fn inline_keyboard_renders_callback_data_with_intent() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::new().row([Button::action("go", "Go", ButtonAction::Next)]);

        let markup = keyboard
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows| rows.first())
            .and_then(|row| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:go", ctx.id));
    }

    #[test]
    fn inline_keyboard_ignores_foreign_intent_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::new().row([Button::action(
            "go",
            "Go",
            ButtonAction::Start {
                state: "next".into(),
                data: Value::Null,
                mode: StartMode::Normal,
            },
        )]);

        assert!(keyboard.handle_callback(&ctx, "td:another:go").is_none());
    }

    #[test]
    fn select_renders_and_resolves_string_payloads() {
        let ctx = Context::new("", "state", Value::Null);

        let select = Select::builder("fruit")
            .items_getter(|_data| ["red:apple", "pear"])
            .item_renderer(|item, _data| item.to_owned())
            .id_getter(|item| item)
            .action(|value| ButtonAction::set_dialog_value("fruit", value))
            .build();

        let markup = select
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows| rows.first())
            .and_then(|row| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:fruit:red:apple", ctx.id));

        let action = select
            .handle_callback(&ctx, callback_data)
            .expect("select action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "fruit" && value == "red:apple"
        ));
    }

    #[test]
    fn select_allows_static_footer_buttons() {
        let ctx = Context::new("", "state", Value::Null);

        let select = Select::builder("fruit")
            .items_getter(|_data| ["pear"])
            .item_renderer(|item, _data| item.to_owned())
            .id_getter(|item| item)
            .action(|value| ButtonAction::set_dialog_value("fruit", value))
            .footer_push(Button::done("done", "Done"))
            .build();

        let action = select
            .handle_callback(&ctx, &format!("td:{}:done", ctx.id))
            .expect("footer action");

        assert!(matches!(action, ButtonAction::Done));
    }

    #[test]
    fn group_chunks_inline_keyboard_rows() {
        let ctx = Context::new("", "state", Value::Null);
        let grouped = Group::new(
            InlineKeyboard::new().row([
                Button::action("a", "A", ButtonAction::noop()),
                Button::action("b", "B", ButtonAction::noop()),
                Button::action("c", "C", ButtonAction::noop()),
            ]),
            2,
        );

        let markup = grouped
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 1);
    }
}
