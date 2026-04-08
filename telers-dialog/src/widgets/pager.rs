use std::fmt::Display;
use telers::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};
use tracing::debug;

use super::{
    kbd::{format_callback_data, parse_callback_data},
    ButtonAction, Keyboard,
};
use crate::entities::{Context, DataMap};

/// A scrolling group that wraps inner keyboard rows and adds pagination controls.
///
/// Mirrors `aiogram-dialog`'s `ScrollingGroup` semantics:
/// - `height` determines how many inner rows are shown per page.
/// - Page state is persisted in `widget_data[widget_id]` as the 0-indexed page number.
/// - A built-in pager row shows `[ 1 | < | current | > | last ]` navigation buttons.
/// - `hide_on_single_page` suppresses the pager when only one page exists.
/// - `hide_pager` suppresses the pager entirely.
pub struct ScrollingGroup<WidgetId, Inner> {
    id: WidgetId,
    inner: Inner,
    height: usize,
    hide_on_single_page: bool,
    hide_pager: bool,
}

impl<WidgetId, Inner> ScrollingGroup<WidgetId, Inner>
where
    WidgetId: Display,
    Inner: Keyboard,
{
    #[must_use]
    pub fn new(id: WidgetId, inner: Inner, height: usize) -> Self {
        Self {
            id,
            inner,
            height,
            hide_on_single_page: false,
            hide_pager: false,
        }
    }

    #[must_use]
    pub fn hide_on_single_page(mut self, hide: bool) -> Self {
        self.hide_on_single_page = hide;
        self
    }

    #[must_use]
    pub fn hide_pager(mut self, hide: bool) -> Self {
        self.hide_pager = hide;
        self
    }
}

/// Read the current page from widget_data. Defaults to 0.
fn read_page(ctx: &Context, widget_id: &str) -> usize {
    ctx.widget_value_as::<usize>(widget_id).unwrap_or(0)
}

impl<WidgetId, Inner> Keyboard for ScrollingGroup<WidgetId, Inner>
where
    WidgetId: Display + Send + Sync + 'static,
    Inner: Keyboard,
{
    fn render_keyboard(&self, ctx: &Context, data: &DataMap) -> Option<ReplyMarkup> {
        let inner_markup = self.inner.render_keyboard(ctx, data)?;
        let all_rows = inner_markup.inline_keyboard()?;

        if all_rows.is_empty() || self.height == 0 {
            return None;
        }

        let total_rows = all_rows.len();
        let pages = total_rows / self.height + usize::from(total_rows % self.height != 0);
        let widget_id = self.id.to_string();
        let current_page = read_page(ctx, &widget_id).min(pages.saturating_sub(1));

        let start = current_page * self.height;
        let end = (start + self.height).min(total_rows);
        let mut rows: Vec<Box<[InlineKeyboardButton]>> =
            all_rows[start..end].iter().map(|r| r.to_vec().into_boxed_slice()).collect();

        if !self.hide_pager && !(self.hide_on_single_page && pages <= 1) && pages > 0 {
            let last_page = pages - 1;
            let prev_page = current_page.saturating_sub(1);
            let next_page = (current_page + 1).min(last_page);
            rows.push(
                vec![
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
                    InlineKeyboardButton::new((current_page + 1).to_string())
                        .callback_data(format_callback_data(
                            ctx,
                            &self.id,
                            Some(&current_page.to_string()),
                        )),
                    InlineKeyboardButton::new(">").callback_data(format_callback_data(
                        ctx,
                        &self.id,
                        Some(&next_page.to_string()),
                    )),
                    InlineKeyboardButton::new((last_page + 1).to_string())
                        .callback_data(format_callback_data(
                            ctx,
                            &self.id,
                            Some(&last_page.to_string()),
                        )),
                ]
                .into_boxed_slice(),
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
                serde_json::Value::Number(page.into()),
            ));
        }

        // Delegate to inner keyboard for non-pager callbacks
        self.inner.handle_callback(ctx, callback_data)
    }
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
        let mut kb = InlineKeyboard::new();
        for i in 0..count {
            kb = kb.row([Button::action(
                format!("btn_{i}"),
                format!("Item {i}"),
                ButtonAction::noop(),
            )]);
        }
        kb
    }

    #[test]
    fn scrolling_group_shows_first_page_by_default() {
        let ctx = Context::new("", "state", Value::Null);
        let inner = build_inner_keyboard(10);
        let pager = ScrollingGroup::new("pager", inner, 3);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

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
        let inner = build_inner_keyboard(10);
        let pager = ScrollingGroup::new("pager", inner, 3);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

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
        let inner = build_inner_keyboard(10);
        let pager = ScrollingGroup::new("pager", inner, 3);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

        // last page has only 1 row + 1 pager
        assert_eq!(rows.len(), 2);
        assert_eq!(&*rows[0][0].text, "Item 9");
    }

    #[test]
    fn scrolling_group_pager_callback_sets_widget_value() {
        let ctx = Context::new("", "state", Value::Null);
        let inner = build_inner_keyboard(10);
        let pager = ScrollingGroup::new("pager", inner, 3);

        let action = pager
            .handle_callback(&ctx, &format!("td:{}:pager:2", ctx.id))
            .expect("action");

        assert!(matches!(
            action,
            ButtonAction::SetWidgetValue { ref key, ref value }
                if key.as_ref() == "pager" && value == &json!(2)
        ));
    }

    #[test]
    fn scrolling_group_delegates_inner_callbacks() {
        let ctx = Context::new("", "state", Value::Null);
        let inner = build_inner_keyboard(4);
        let pager = ScrollingGroup::new("pager", inner, 2);

        // Inner button callback should be delegated
        let action = pager
            .handle_callback(&ctx, &format!("td:{}:btn_1", ctx.id))
            .expect("inner action");

        assert!(matches!(action, ButtonAction::Noop));
    }

    #[test]
    fn scrolling_group_hides_pager_on_single_page() {
        let ctx = Context::new("", "state", Value::Null);
        let inner = build_inner_keyboard(2);
        let pager = ScrollingGroup::new("pager", inner, 5).hide_on_single_page(true);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

        // Just 2 content rows, no pager
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn scrolling_group_hide_pager_flag_suppresses_navigation() {
        let ctx = Context::new("", "state", Value::Null);
        let inner = build_inner_keyboard(10);
        let pager = ScrollingGroup::new("pager", inner, 3).hide_pager(true);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

        // 3 content rows only, no pager
        assert_eq!(rows.len(), 3);
    }

    #[test]
    fn scrolling_group_clamps_page_beyond_max() {
        let mut ctx = Context::new("", "state", Value::Null);
        ctx.widget_data.insert("pager".into(), json!(99)); // way past last page
        let inner = build_inner_keyboard(5);
        let pager = ScrollingGroup::new("pager", inner, 2);

        let markup = pager
            .render_keyboard(&ctx, &DataMap::new())
            .expect("keyboard");
        let rows = markup.inline_keyboard().expect("inline rows");

        // Should clamp to last page (page 2, showing item 4)
        assert_eq!(&*rows[0][0].text, "Item 4");
    }
}
