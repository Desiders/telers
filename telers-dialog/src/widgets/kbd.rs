//! Inline keyboard widgets and helpers.
//!
//! This module contains the public keyboard surface used by windows:
//! - primitive buttons such as [`Button`]
//! - navigation and mutation actions via [`ButtonAction`]
//! - collection widgets such as [`Select`], [`Radio`], and [`Multiselect`]
//! - reply-keyboard request widgets such as [`RequestContact`]
//! - paging helpers such as [`ScrollingGroup`] and [`NumberedPager`]

mod action;
mod base;
mod button;
mod callback;
mod click;
mod group;
mod inline_keyboard;
mod pager;
mod request;
mod select;
mod stateful_select;
mod when;

pub(crate) use base::MultiKeyboard;
pub(crate) use callback::{format_callback_data, parse_callback_data};
pub(crate) use select::render_button_row;

pub use action::ButtonAction;
pub use base::Keyboard;
pub use button::Button;
pub use click::ClickContext;
pub use group::Group;
pub use inline_keyboard::InlineKeyboard;
pub use pager::{
    sync_scroll, sync_scrolls, BaseScroll, CurrentPage, FirstPage, LastPage, NextPage,
    NumberedPager, OnPageChanged, PageChange, PageDirection, PagerBinding, PrevPage, Scroll,
    ScrollingGroup, SwitchPage,
};
pub use request::{RequestContact, RequestLocation, RequestPoll};
pub use select::Select;
pub use stateful_select::{Checkbox, Counter, Multiselect, Radio, TimeSelect, Toggle};
pub use when::WhenCondition;

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use telers::types::{CopyTextButton, LoginUrl, SwitchInlineQueryChosenChat, WebAppInfo};

    use super::{
        Button, ButtonAction, Group, InlineKeyboard, Keyboard, MultiKeyboard, Select, WhenCondition,
    };
    use crate::entities::{Context, DataMap, StartMode};

    #[test]
    fn inline_keyboard_renders_callback_data_with_intent() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::builder()
            .row([Button::action("go", "Go", ButtonAction::Next)])
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
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
        let keyboard = InlineKeyboard::builder()
            .row([Button::action(
                "go",
                "Go",
                ButtonAction::Start {
                    state: "next".into(),
                    data: Value::Null,
                    mode: StartMode::Normal,
                },
            )])
            .build();

        assert!(keyboard
            .handle_callback_for_test(&ctx, "td:another:go")
            .is_none());
    }

    #[test]
    fn button_on_click_receives_click_context() {
        let ctx = Context::new("", "confirm_delete", Value::Null);
        let keyboard = InlineKeyboard::builder()
            .row([Button::on_click("confirm", "Confirm", |click| {
                ButtonAction::set_dialog_value("handled_state", click.context.state.clone())
            })])
            .build();

        let action = keyboard
            .handle_callback_for_test(&ctx, &format!("td:{}:confirm", ctx.id))
            .expect("button action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "handled_state" && value == "confirm_delete"
        ));
    }

    #[test]
    fn keyboard_when_condition_filters_render_and_callbacks() {
        let mut ctx = Context::new("", "state", Value::Null);
        let mut data = DataMap::new();
        let keyboard = MultiKeyboard::new().kbd_boxed(Box::new(
            InlineKeyboard::builder()
                .row([Button::action("go", "Go", ButtonAction::Next)])
                .when(WhenCondition::data_field("show"))
                .build(),
        ));

        assert!(keyboard.render_keyboard_for_test(&ctx, &data).is_none());
        assert!(keyboard
            .handle_callback_for_test(&ctx, &format!("td:{}:go", ctx.id))
            .is_none());

        data.insert("show".into(), Value::Bool(true));
        ctx.dialog_data = data.clone();

        assert!(keyboard.render_keyboard_for_test(&ctx, &data).is_some());
        assert!(matches!(
            keyboard.handle_callback_for_test(&ctx, &format!("td:{}:go", ctx.id)),
            Some(ButtonAction::Next)
        ));
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
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("keyboard");
        let callback_data = markup
            .inline_keyboard()
            .and_then(|rows| rows.first())
            .and_then(|row| row.first())
            .and_then(|button| button.callback_data.as_deref())
            .expect("callback data");

        assert_eq!(callback_data, format!("td:{}:fruit:red:apple", ctx.id));

        let action = select
            .handle_callback_for_test(&ctx, callback_data)
            .expect("select action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "fruit" && value == "red:apple"
        ));
    }

    #[test]
    fn select_action_receives_click_context() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.dialog_data
            .insert("prefix".into(), Value::String("chosen".into()));

        let select = Select::builder("fruit")
            .items_getter(|_data| ["apple"])
            .item_renderer(|item, _data| item.to_owned())
            .id_getter(|item| item)
            .on_click(|click, value| {
                let prefix = click
                    .context
                    .dialog_data
                    .get("prefix")
                    .and_then(Value::as_str)
                    .unwrap_or("missing");
                ButtonAction::set_dialog_value("fruit", format!("{prefix}:{value}"))
            })
            .build();

        let action = select
            .handle_callback_for_test(&ctx, &format!("td:{}:fruit:apple", ctx.id))
            .expect("select action");

        assert!(matches!(
            action,
            ButtonAction::SetDialogValue { ref key, ref value }
                if key.as_ref() == "fruit" && value == "chosen:apple"
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
            .handle_callback_for_test(&ctx, &format!("td:{}:done", ctx.id))
            .expect("footer action");

        assert!(matches!(action, ButtonAction::Done));
    }

    #[test]
    fn group_chunks_inline_keyboard_rows() {
        let ctx = Context::new("", "state", Value::Null);
        let grouped = Group::builder(
            InlineKeyboard::builder()
                .row([
                    Button::action("a", "A", ButtonAction::noop()),
                    Button::action("b", "B", ButtonAction::noop()),
                    Button::action("c", "C", ButtonAction::noop()),
                ])
                .build(),
        )
        .items_per_row(2)
        .build();

        let markup = grouped
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 1);
    }

    #[test]
    fn inline_keyboard_renders_non_callback_button_variants() {
        let ctx = Context::new("", "state", Value::Null);
        let keyboard = InlineKeyboard::builder()
            .row([Button::web_app(
                "Web",
                WebAppInfo::new("https://example.com/app"),
            )])
            .row([Button::login_url(
                "Login",
                LoginUrl::new("https://example.com/login"),
            )])
            .row([Button::switch_inline_query("Inline", "query")])
            .row([Button::switch_inline_query_current_chat("Here", "local")])
            .row([Button::switch_inline_query_chosen_chat(
                "Pick chat",
                SwitchInlineQueryChosenChat::new().query("pick"),
            )])
            .row([Button::copy_text("Copy", CopyTextButton::new("copied"))])
            .build();

        let markup = keyboard
            .render_keyboard_for_test(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline keyboard");

        assert_eq!(
            rows[0][0].web_app.as_ref().map(|v| v.url.as_ref()),
            Some("https://example.com/app")
        );
        assert_eq!(
            rows[1][0].login_url.as_ref().map(|v| v.url.as_ref()),
            Some("https://example.com/login")
        );
        assert_eq!(rows[2][0].switch_inline_query.as_deref(), Some("query"));
        assert_eq!(
            rows[3][0].switch_inline_query_current_chat.as_deref(),
            Some("local")
        );
        assert_eq!(
            rows[4][0]
                .switch_inline_query_chosen_chat
                .as_ref()
                .and_then(|v| v.query.as_deref()),
            Some("pick")
        );
        assert_eq!(
            rows[5][0].copy_text.as_ref().map(|v| v.text.as_ref()),
            Some("copied")
        );
    }
}
