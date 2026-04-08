use bon::bon;
use serde_json::Value;
use std::fmt::Display;
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    kbd::{format_callback_data, parse_callback_data},
    ButtonAction, Keyboard,
};
use crate::entities::{Context, DataMap};

pub struct ScrollingGroup<WidgetId, Kbd> {
    id: WidgetId,
    kbd: Kbd,
    height: usize,
    hide_on_single_page: bool,
    hide_pager: bool,
}

#[bon]
impl<WidgetId, Kbd> ScrollingGroup<WidgetId, Kbd> {
    #[builder]
    #[must_use]
    pub fn new(
        #[builder(start_fn)] id: WidgetId,
        kbd: Kbd,
        height: usize,
        #[builder(default = false)] hide_on_single_page: bool,
        #[builder(default = false)] hide_pager: bool,
    ) -> Self
    where
        WidgetId: Display,
        Kbd: Keyboard,
    {
        Self {
            id,
            kbd,
            height,
            hide_on_single_page,
            hide_pager,
        }
    }
}

impl<WidgetId, Kbd> Keyboard for ScrollingGroup<WidgetId, Kbd>
where
    WidgetId: Display + Send + Sync + 'static,
    Kbd: Keyboard,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let inner_markup = self.kbd.render_keyboard(ctx, data)?;
        let rows = inner_markup.inline_keyboard()?;

        if rows.is_empty() || self.height == 0 {
            return None;
        }

        let total_rows = rows.len();
        let pages_count = total_rows / self.height + usize::from(total_rows % self.height != 0);
        let widget_id = self.id.to_string();
        let current_page = read_page(ctx, &widget_id).min(pages_count.saturating_sub(1));

        let start = current_page * self.height;
        let end = (start + self.height).min(total_rows);
        let mut rows = rows[start..end]
            .iter()
            .map(|val| val.to_vec().into_boxed_slice())
            .collect::<Vec<_>>();

        if !(self.hide_pager || self.hide_on_single_page && pages_count <= 1) {
            let last_page = pages_count - 1;
            let prev_page = current_page.saturating_sub(1);
            let next_page = (current_page + 1).min(last_page);
            rows.push(
                [
                    InlineKeyboardButton::new("1").callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some("0"),
                    )),
                    InlineKeyboardButton::new("<").callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&prev_page.to_string()),
                    )),
                    InlineKeyboardButton::new((current_page + 1).to_string()).callback_data(
                        format_callback_data(ctx, &self.id, Some(&current_page.to_string())),
                    ),
                    InlineKeyboardButton::new(">").callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&next_page.to_string()),
                    )),
                    InlineKeyboardButton::new(pages_count.to_string()).callback_data(
                        format_callback_data(ctx, &self.id, Some(&last_page.to_string())),
                    ),
                ]
                .into(),
            );
        }

        Some(InlineKeyboardMarkup::new(rows).into())
    }

    fn handle_callback(&self, ctx: &Context, callback_data: &str) -> Option<ButtonAction> {
        let parsed = parse_callback_data(ctx, callback_data)?;
        let widget_id = self.id.to_string();

        if parsed.target_id == widget_id {
            let page: usize = parsed.payload?.parse().ok()?;
            debug!(
                context_id = %ctx.id,
                widget_id = %self.id,
                page,
                "Resolved pager navigation callback"
            );
            return Some(ButtonAction::set_widget_value(
                widget_id,
                Value::Number(page.into()),
            ));
        }

        // Delegate to inner keyboard for non-pager callbacks
        self.kbd.handle_callback(ctx, callback_data)
    }
}

#[inline]
#[must_use]
fn read_page(ctx: &Context, widget_id: &str) -> usize {
    ctx.widget_value_as::<usize>(widget_id).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::ScrollingGroup;
    use crate::{
        entities::{Context, DataMap},
        widgets::{Button, ButtonAction, InlineKeyboard, Keyboard},
    };

    fn build_inner_keyboard(count: usize) -> InlineKeyboard {
        let mut kbds = InlineKeyboard::new();
        for i in 0..count {
            kbds = kbds.row([Button::action(
                format!("btn_{i}"),
                format!("Item {i}"),
                ButtonAction::noop(),
            )]);
        }
        kbds
    }

    #[test]
    fn scrolling_group_shows_first_page_by_default() {
        let ctx = Context::new("", "state", Value::Null);
        let kbd = build_inner_keyboard(10);
        let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // 3 content rows + 1 pager row = 4
        assert_eq!(rows.len(), 4);
        assert_eq!(&*rows[0][0].text, "Item 0");
        assert_eq!(&*rows[1][0].text, "Item 1");
        assert_eq!(&*rows[2][0].text, "Item 2");

        // Pager row
        assert_eq!(rows[3].len(), 5);
        assert_eq!(&*rows[3][0].text, "1"); // first page
        assert_eq!(&*rows[3][2].text, "1"); // current page (1-indexed)
        assert_eq!(&*rows[3][4].text, "4"); // last page (10/3 = 4 pages)
    }

    #[test]
    fn scrolling_group_shows_correct_page_from_widget_data() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("pager".into(), json!(1)); // page index 1 (second page)
        let kbd = build_inner_keyboard(10);
        let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // 3 content rows + 1 pager row
        assert_eq!(rows.len(), 4);
        assert_eq!(&*rows[0][0].text, "Item 3");
        assert_eq!(&*rows[1][0].text, "Item 4");
        assert_eq!(&*rows[2][0].text, "Item 5");
        assert_eq!(&*rows[3][2].text, "2"); // current page display (1-indexed)
    }

    #[test]
    fn scrolling_group_last_page_shows_remaining_items() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("pager".into(), json!(3)); // page 3 (last page of 10 items, height 3)
        let kbd = build_inner_keyboard(10);
        let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // last page has only 1 row + 1 pager
        assert_eq!(rows.len(), 2);
        assert_eq!(&*rows[0][0].text, "Item 9");
    }

    #[test]
    fn scrolling_group_pager_callback_sets_widget_value() {
        let ctx = Context::new("", "state", Value::Null);
        let kbd = build_inner_keyboard(10);
        let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

        let action = pager
            .handle_callback(&ctx, &format!("td:{}:pager:2", ctx.id))
            .unwrap();

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "pager" && value == &json!(2)
        ));
    }

    #[test]
    fn scrolling_group_delegates_inner_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let kbd = build_inner_keyboard(4);
        let pager = ScrollingGroup::builder("pager").height(2).kbd(kbd).build();

        // Inner button callback should be delegated
        let action = pager
            .handle_callback(&ctx, &format!("td:{}:btn_1", ctx.id))
            .unwrap();

        assert!(matches!(action, ButtonAction::Noop));
    }

    #[test]
    fn scrolling_group_hides_pager_on_single_page() {
        let ctx = Context::new("", "state", Value::Null);
        let kbd = build_inner_keyboard(2);
        let pager = ScrollingGroup::builder("pager")
            .height(5)
            .hide_on_single_page(true)
            .kbd(kbd)
            .build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // Just 2 content rows, no pager
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn scrolling_group_hide_pager_flag_suppresses_navigation() {
        let ctx = Context::new("", "state", Value::Null);
        let kbd = build_inner_keyboard(10);
        let pager = ScrollingGroup::builder("pager")
            .height(3)
            .hide_pager(true)
            .kbd(kbd)
            .build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // 3 content rows only, no pager
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn scrolling_group_clamps_page_beyond_max() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("pager".into(), json!(99)); // way past last page
        let kbd = build_inner_keyboard(5);
        let pager = ScrollingGroup::builder("pager").height(2).kbd(kbd).build();

        let markup = pager.render_keyboard(&ctx, &DataMap::new()).unwrap();
        let rows = markup.inline_keyboard().unwrap();

        // Should clamp to last page (page 2, showing item 4)
        assert_eq!(&*rows[0][0].text, "Item 4");
    }
}
