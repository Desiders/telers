use super::{Case, FormatText, ListText, MultiText, Progress, ScrollingText, Text};
use crate::{
    entities::{Context, DataMap},
    widgets::{Keyboard, NumberedPager},
};
use serde_json::json;

#[test]
fn format_text_replaces_known_keys() {
    let mut data = DataMap::new();
    data.insert("name".into(), "telers".into());

    let text = FormatText::new("hello {name}");

    assert_eq!(&*text.render_text(&data), "hello telers");
}

#[test]
fn format_text_keeps_unknown_keys_visible() {
    let text = FormatText::new("hello {name}");

    assert_eq!(&*text.render_text(&DataMap::new()), "hello {name}");
}

#[test]
fn multi_text_joins_items() {
    let text = MultiText::builder()
        .text("one")
        .text("two")
        .separator(" | ")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()), "one | two");
}

#[test]
fn list_text_renders_items_with_separator() {
    let text = ListText::builder()
        .items_getter(|_data| ["one", "two", "three"])
        .item_renderer(|&item, _data| format!("- {item}"))
        .separator(" | ")
        .build();

    assert_eq!(
        &*text.render_text(&DataMap::new()),
        "- one | - two | - three"
    );
}

#[test]
fn case_selects_matching_variant() {
    let mut data = DataMap::new();
    data.insert("status".into(), json!("paid"));

    let text = Case::new(|data: &DataMap| data.get("status").cloned())
        .when(Some(json!("draft")), "Draft order")
        .when(Some(json!("paid")), "Paid order")
        .default("Unknown");

    assert_eq!(&*text.render_text(&data), "Paid order");
}

#[test]
fn case_uses_default_when_key_missing() {
    let text = Case::new(|data: &DataMap| data.get("status").cloned())
        .when(Some(json!("draft")), "Draft order")
        .default("Unknown");

    assert_eq!(&*text.render_text(&DataMap::new()), "Unknown");
}

#[test]
fn progress_renders_bar_from_percentage_field() {
    let mut data = DataMap::new();
    data.insert("percent".into(), json!(35));

    let text = Progress::new("percent").width(10);

    assert_eq!(&*text.render_text(&data), "####------  35%");
}

#[test]
fn progress_clamps_and_supports_custom_symbols() {
    let mut data = DataMap::new();
    data.insert("percent".into(), json!(120));

    let text = Progress::new("percent").width(4).filled("=").empty(".");

    assert_eq!(&*text.render_text(&data), "==== 100%");
}

#[test]
fn scrolling_text_defaults_to_first_page_without_context() {
    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()), "abcd");
    assert_eq!(text.page_count(&DataMap::new()), 3);
}

#[test]
fn scrolling_text_uses_widget_page_from_context() {
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("article".into(), json!(2));

    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();

    assert_eq!(&*text.render_text_in_context(&ctx, &DataMap::new()), "ij");
    assert_eq!(text.page_count_in_context(&ctx, &DataMap::new()), 3);
}

#[test]
fn scrolling_text_slices_by_char_boundaries() {
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("article".into(), json!(1));

    let text = ScrollingText::builder("article")
        .text("ab😀cd")
        .page_size(3)
        .build();

    assert_eq!(&*text.render_text_in_context(&ctx, &DataMap::new()), "cd");
    assert_eq!(text.page_count(&DataMap::new()), 2);
}

#[test]
fn scrolling_text_can_drive_numbered_pager_via_scroll_trait() {
    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();
    let markup = NumberedPager::builder(text)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .length(5)
        .build()
        .render_keyboard(&Context::new("", "state", json!(null)), &DataMap::new())
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 3);
    assert_eq!(&*rows[0][0].text, "[1]");
    assert_eq!(&*rows[0][2].text, "3");
}
