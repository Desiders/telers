use serde_json::{json, Value};

use super::{
    sync_scroll, sync_scrolls, CurrentPage, FirstPage, LastPage, NextPage, NumberedPager,
    OnPageChanged, PageDirection, PrevPage, ScrollingGroup, StubScroll, StubScrollPages,
    SwitchPage,
};
use crate::{
    entities::{Context, DataMap, RenderContext},
    widgets::{Button, ButtonAction, InlineKeyboard, Keyboard},
};

async fn two_pages(_render_ctx: RenderContext) -> usize {
    2
}

async fn four_pages(_render_ctx: RenderContext) -> usize {
    4
}

async fn catalog_pages(render_ctx: RenderContext) -> usize {
    let items = render_ctx
        .data
        .get("items")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    usize::try_from(items.div_ceil(3)).unwrap()
}

async fn record_page_change(change: super::PageChange) -> ButtonAction {
    ButtonAction::chain([
        ButtonAction::set_dialog_value("page_widget", change.widget_id.to_string()),
        ButtonAction::set_dialog_value("page_from", change.old_page),
        ButtonAction::set_dialog_value("page_to", change.new_page),
    ])
}

fn build_inner_keyboard(count: usize) -> InlineKeyboard {
    let mut kbds = InlineKeyboard::builder();
    for i in 0..count {
        kbds = kbds.row([Button::action(
            format!("btn_{i}"),
            format!("Item {i}"),
            ButtonAction::noop(),
        )]);
    }
    kbds.build()
}

#[tokio::test]
async fn scrolling_group_shows_first_page_by_default() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 4);
    assert_eq!(&*rows[0][0].text, "Item 0");
    assert_eq!(&*rows[1][0].text, "Item 1");
    assert_eq!(&*rows[2][0].text, "Item 2");
    assert_eq!(rows[3].len(), 5);
    assert_eq!(&*rows[3][0].text, "1");
    assert_eq!(&*rows[3][2].text, "1");
    assert_eq!(&*rows[3][4].text, "4");
}

#[tokio::test]
async fn scrolling_group_shows_correct_page_from_widget_data() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 4);
    assert_eq!(&*rows[0][0].text, "Item 3");
    assert_eq!(&*rows[1][0].text, "Item 4");
    assert_eq!(&*rows[2][0].text, "Item 5");
    assert_eq!(&*rows[3][2].text, "2");
}

#[tokio::test]
async fn scrolling_group_last_page_shows_remaining_items() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(3));
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(&*rows[0][0].text, "Item 9");
}

#[tokio::test]
async fn scrolling_group_pager_callback_sets_widget_value() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager").height(3).kbd(kbd).build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:pager:2", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pager" && value == &json!(2)
    ));
}

#[tokio::test]
async fn scrolling_group_delegates_inner_callbacks() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(4);
    let pager = ScrollingGroup::builder("pager").height(2).kbd(kbd).build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:btn_1", ctx.id))
        .await
        .unwrap();

    assert!(matches!(action, ButtonAction::Noop));
}

#[tokio::test]
async fn scrolling_group_hides_pager_on_single_page() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(2);
    let pager = ScrollingGroup::builder("pager")
        .height(5)
        .hide_on_single_page(true)
        .kbd(kbd)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 2);
}

#[tokio::test]
async fn scrolling_group_hide_pager_flag_suppresses_navigation() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager")
        .height(3)
        .hide_pager(true)
        .kbd(kbd)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 3);
}

#[tokio::test]
async fn scrolling_group_clamps_page_beyond_max() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(99));
    let kbd = build_inner_keyboard(5);
    let pager = ScrollingGroup::builder("pager").height(2).kbd(kbd).build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "Item 4");
}

#[tokio::test]
async fn scrolling_group_width_groups_buttons_into_fixed_grid() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(5);
    let pager = ScrollingGroup::builder("pager")
        .width(2)
        .height(2)
        .kbd(kbd)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let pager_row = &rows[2];

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].len(), 2);
    assert_eq!(rows[1].len(), 2);
    assert_eq!(&*rows[0][0].text, "Item 0");
    assert_eq!(&*rows[0][1].text, "Item 1");
    assert_eq!(&*rows[1][0].text, "Item 2");
    assert_eq!(&*rows[1][1].text, "Item 3");
    assert_eq!(pager_row.len(), 5);
    assert_eq!(&*pager_row[0].text, "1");
    assert_eq!(&*pager_row[1].text, "<");
    assert_eq!(&*pager_row[2].text, "1");
    assert_eq!(&*pager_row[3].text, ">");
    assert_eq!(&*pager_row[4].text, "2");
}

#[tokio::test]
async fn scrolling_group_pads_last_page_grid_with_fillers() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let kbd = build_inner_keyboard(5);
    let pager = ScrollingGroup::builder("pager")
        .width(2)
        .height(2)
        .filler_text(".".into())
        .kbd(kbd)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();
    let pager_row = &rows[2];

    assert_eq!(rows.len(), 3);
    assert_eq!(&*rows[0][0].text, "Item 4");
    assert_eq!(&*rows[0][1].text, ".");
    assert_eq!(&*rows[1][0].text, ".");
    assert_eq!(&*rows[1][1].text, ".");
    assert_eq!(
        rows[0][1].callback_data.as_deref(),
        Some(format!("td:{}:pager:1", ctx.id).as_str())
    );
    assert_eq!(&*pager_row[0].text, "1");
    assert_eq!(&*pager_row[1].text, "<");
    assert_eq!(&*pager_row[2].text, "2");
    assert_eq!(&*pager_row[3].text, ">");
    assert_eq!(&*pager_row[4].text, "2");
}

#[tokio::test]
async fn switch_page_renders_directional_button() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let pager = SwitchPage::builder("pager")
        .direction(PageDirection::Next)
        .page_count_getter(four_pages)
        .label_renderer(|_target, _current, _data| ">")
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, ">");
    assert_eq!(
        rows[0][0].callback_data.as_deref(),
        Some(format!("td:{}:pager:2", ctx.id).as_str())
    );
}

#[tokio::test]
async fn numbered_pager_renders_current_page_distinctly() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let pager = NumberedPager::builder("pager")
        .page_count_getter(four_pages)
        .page_renderer(|page, _data| (page + 1).to_string())
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .length(3)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(&*rows[0][0].text, "1");
    assert_eq!(&*rows[0][1].text, "[2]");
    assert_eq!(&*rows[0][2].text, "3");
    assert_eq!(&*rows[1][0].text, "4");
}

#[tokio::test]
async fn numbered_pager_callback_sets_page() {
    let ctx = Context::new("", "state", Value::Null);
    let pager = NumberedPager::builder("pager")
        .page_count_getter(two_pages)
        .page_renderer(|page, _data| (page + 1).to_string())
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:pager:1", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pager" && value == &json!(1)
    ));
}

#[tokio::test]
async fn scrolling_group_sync_scroll_updates_other_widget_page() {
    let ctx = Context::new("", "state", Value::Null);
    let kbd = build_inner_keyboard(10);
    let pager = ScrollingGroup::builder("pager")
        .height(3)
        .on_page_changed(sync_scroll("list"))
        .kbd(kbd)
        .build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:pager:2", ctx.id))
        .await
        .unwrap();

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chain action");
    };
    assert_eq!(actions.len(), 2);
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pager" && value == &json!(2)
    ));
    assert!(matches!(
        actions[1],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "list" && value == &json!(2)
    ));
}

#[tokio::test]
async fn numbered_pager_sync_scrolls_updates_multiple_widgets() {
    let ctx = Context::new("", "state", Value::Null);
    let pager = NumberedPager::builder("pager")
        .page_count_getter(four_pages)
        .page_renderer(|page, _data| (page + 1).to_string())
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .on_page_changed(sync_scrolls(["list", "grid"]))
        .build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:pager:3", ctx.id))
        .await
        .unwrap();

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chain action");
    };
    assert_eq!(actions.len(), 2);
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pager" && value == &json!(3)
    ));
    assert!(matches!(actions[1], ButtonAction::Chain(_)));

    let ButtonAction::Chain(sync_actions) = &actions[1] else {
        panic!("expected nested sync action chain");
    };
    assert_eq!(sync_actions.len(), 2);
    assert!(matches!(
        sync_actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "list" && value == &json!(3)
    ));
    assert!(matches!(
        sync_actions[1],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "grid" && value == &json!(3)
    ));
}

#[tokio::test]
async fn on_page_changed_can_use_widget_id_and_previous_page() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let pager = NumberedPager::builder("pager")
        .page_count_getter(four_pages)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .on_page_changed(OnPageChanged::new(record_page_change))
        .build();

    let action = pager
        .handle_callback_for_test(&ctx, &format!("td:{}:pager:3", ctx.id))
        .await
        .unwrap();

    let ButtonAction::Chain(actions) = action else {
        panic!("expected chain action");
    };
    assert_eq!(actions.len(), 2);
    assert!(matches!(
        actions[0],
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "pager" && value == &json!(3)
    ));

    let ButtonAction::Chain(detail_actions) = &actions[1] else {
        panic!("expected nested detail chain");
    };
    assert_eq!(detail_actions.len(), 3);
    assert!(matches!(
        detail_actions[0],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "page_widget" && value == "pager"
    ));
    assert!(matches!(
        detail_actions[1],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "page_from" && value == &json!(1)
    ));
    assert!(matches!(
        detail_actions[2],
        ButtonAction::SetDialogValue { ref key, ref value }
            if key.as_ref() == "page_to" && value == &json!(3)
    ));
}

#[tokio::test]
async fn convenience_pager_wrappers_render_expected_targets() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let data = DataMap::new();

    let first = FirstPage::builder("pager")
        .page_count_getter(four_pages)
        .build();
    let prev = PrevPage::builder("pager")
        .page_count_getter(four_pages)
        .build();
    let current = CurrentPage::builder("pager")
        .page_count_getter(four_pages)
        .build();
    let next = NextPage::builder("pager")
        .page_count_getter(four_pages)
        .build();
    let last = LastPage::builder("pager")
        .page_count_getter(four_pages)
        .build();

    let first_rows = first.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let prev_rows = prev.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let current_rows = current.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let next_rows = next.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let last_rows = last.render_keyboard_for_test(&ctx, &data).await.unwrap();

    assert_eq!(&*first_rows.inline_keyboard().unwrap()[0][0].text, "<<");
    assert_eq!(&*prev_rows.inline_keyboard().unwrap()[0][0].text, "<");
    assert_eq!(&*current_rows.inline_keyboard().unwrap()[0][0].text, "2");
    assert_eq!(&*next_rows.inline_keyboard().unwrap()[0][0].text, ">");
    assert_eq!(&*last_rows.inline_keyboard().unwrap()[0][0].text, ">>");

    assert_eq!(
        first_rows.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref(),
        Some(format!("td:{}:pager:0", ctx.id).as_str())
    );
    assert_eq!(
        prev_rows.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref(),
        Some(format!("td:{}:pager:0", ctx.id).as_str())
    );
    assert_eq!(
        current_rows.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref(),
        Some(format!("td:{}:pager:1", ctx.id).as_str())
    );
    assert_eq!(
        next_rows.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref(),
        Some(format!("td:{}:pager:2", ctx.id).as_str())
    );
    assert_eq!(
        last_rows.inline_keyboard().unwrap()[0][0]
            .callback_data
            .as_deref(),
        Some(format!("td:{}:pager:3", ctx.id).as_str())
    );
}

#[tokio::test]
async fn convenience_pager_wrappers_allow_label_override() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("pager".into(), json!(1));
    let data = DataMap::new();

    let first = FirstPage::builder("pager")
        .page_count_getter(four_pages)
        .label("First".into())
        .build();
    let prev = PrevPage::builder("pager")
        .page_count_getter(four_pages)
        .label("Back".into())
        .build();
    let next = NextPage::builder("pager")
        .page_count_getter(four_pages)
        .label("More".into())
        .build();
    let last = LastPage::builder("pager")
        .page_count_getter(four_pages)
        .label("Final".into())
        .build();

    assert_eq!(
        &*first
            .render_keyboard_for_test(&ctx, &data)
            .await
            .unwrap()
            .inline_keyboard()
            .unwrap()[0][0]
            .text,
        "First"
    );
    assert_eq!(
        &*prev
            .render_keyboard_for_test(&ctx, &data)
            .await
            .unwrap()
            .inline_keyboard()
            .unwrap()[0][0]
            .text,
        "Back"
    );
    assert_eq!(
        &*next
            .render_keyboard_for_test(&ctx, &data)
            .await
            .unwrap()
            .inline_keyboard()
            .unwrap()[0][0]
            .text,
        "More"
    );
    assert_eq!(
        &*last
            .render_keyboard_for_test(&ctx, &data)
            .await
            .unwrap()
            .inline_keyboard()
            .unwrap()[0][0]
            .text,
        "Final"
    );
}

#[tokio::test]
async fn stub_scroll_binds_numbered_pager_to_fixed_page_count() {
    let mut ctx = Context::new("", "state", Value::Null);
    ctx.widget_data.insert("catalog_page".into(), json!(2));
    let stub = StubScroll::builder("catalog_page").pages(4_usize).build();
    let pager = NumberedPager::builder(stub)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .length(4)
        .build();

    let markup = pager
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(&*rows[0][0].text, "1");
    assert_eq!(&*rows[0][1].text, "2");
    assert_eq!(&*rows[0][2].text, "[3]");
    assert_eq!(&*rows[0][3].text, "4");
}

#[tokio::test]
async fn stub_scroll_reads_page_count_from_data_field() {
    let ctx = Context::new("", "state", Value::Null);
    let mut data = DataMap::new();
    data.insert("page_count".into(), json!(3));
    let stub = StubScroll::builder("catalog_page")
        .pages("page_count")
        .build();
    let pager = NumberedPager::builder(stub)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .build();

    let markup = pager.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows[0].len(), 3);
    assert_eq!(&*rows[0][0].text, "[1]");
    assert_eq!(&*rows[0][2].text, "3");
}

#[tokio::test]
async fn stub_scroll_accepts_dynamic_page_getter() {
    let ctx = Context::new("", "state", Value::Null);
    let mut data = DataMap::new();
    data.insert("items".into(), json!(7));
    let stub = StubScroll::builder("catalog_page")
        .pages(StubScrollPages::getter(catalog_pages))
        .build();
    let pager = NumberedPager::builder(stub)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .build();

    let markup = pager.render_keyboard_for_test(&ctx, &data).await.unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows[0].len(), 3);
}

#[tokio::test]
async fn stub_scroll_handles_page_callbacks_without_rendering_markup() {
    let ctx = Context::new("", "state", Value::Null);
    let stub = StubScroll::builder("catalog_page").pages(4_usize).build();

    assert!(stub
        .render_keyboard_for_test(&ctx, &DataMap::new())
        .await
        .is_none());

    let action = stub
        .handle_callback_for_test(&ctx, &format!("td:{}:catalog_page:2", ctx.id))
        .await
        .unwrap();

    assert!(matches!(
        action,
        ButtonAction::SetWidgetValue { ref key, ref value }
            if key.as_ref() == "catalog_page" && value == &json!(2)
    ));
}
