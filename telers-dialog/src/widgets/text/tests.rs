use super::{Case, FormatText, ListText, MultiText, Progress, ScrollingText, Text};
use crate::{
    entities::{Context, DataMap},
    widgets::{Keyboard, NumberedPager},
};
use serde_json::json;

#[tokio::test]
async fn format_text_replaces_known_keys() {
    let mut data = DataMap::new();
    data.insert("name".into(), "telers".into());

    let text = FormatText::new("hello {name}");

    assert_eq!(&*text.render_text(&data).await, "hello telers");
}

#[tokio::test]
async fn format_text_keeps_unknown_keys_visible() {
    let text = FormatText::new("hello {name}");

    assert_eq!(&*text.render_text(&DataMap::new()).await, "hello {name}");
}

#[tokio::test]
async fn multi_text_joins_items() {
    let text = MultiText::builder()
        .text("one")
        .text("two")
        .separator(" | ")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "one | two");
}

#[tokio::test]
async fn list_text_renders_items_with_separator() {
    let text = ListText::builder()
        .items_getter(|_data| ["one", "two", "three"])
        .item_renderer(|&item, _data| format!("- {item}"))
        .separator(" | ")
        .build();

    assert_eq!(
        &*text.render_text(&DataMap::new()).await,
        "- one | - two | - three"
    );
}

#[tokio::test]
async fn list_text_paginates_when_page_size_set() {
    let text = ListText::builder()
        .id("catalog")
        .page_size(2)
        .items_getter(|_data| ["one", "two", "three", "four", "five"])
        .item_renderer(|&item, _data| item.to_owned())
        .separator(" | ")
        .build();

    // Without context the widget renders the first page.
    assert_eq!(&*text.render_text(&DataMap::new()).await, "one | two");

    // The current page comes from `widget_data[id]`.
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("catalog".into(), json!(2));
    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "five"
    );
}

#[tokio::test]
async fn list_text_without_page_size_renders_every_item() {
    let text = ListText::builder()
        .items_getter(|_data| ["one", "two", "three"])
        .item_renderer(|&item, _data| item.to_owned())
        .separator(",")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "one,two,three");
}

#[tokio::test]
async fn list_text_can_drive_numbered_pager_via_scroll_trait() {
    let text = ListText::builder()
        .id("catalog")
        .page_size(2)
        .items_getter(|_data| ["one", "two", "three", "four", "five"])
        .item_renderer(|&item, _data| item.to_owned())
        .build();
    let markup = NumberedPager::builder(text)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .length(5)
        .build()
        .render_keyboard_for_test(&Context::new("", "state", json!(null)), &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    // 5 items / 2 per page = 3 pages.
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 3);
    assert_eq!(&*rows[0][0].text, "[1]");
    assert_eq!(&*rows[0][2].text, "3");
}

#[tokio::test]
async fn case_selects_matching_variant() {
    let mut data = DataMap::new();
    data.insert("status".into(), json!("paid"));

    let text = Case::builder(|data: &DataMap| data.get("status").cloned())
        .when(Some(json!("draft")), "Draft order")
        .when(Some(json!("paid")), "Paid order")
        .default("Unknown")
        .build();

    assert_eq!(&*text.render_text(&data).await, "Paid order");
}

#[tokio::test]
async fn case_uses_default_when_key_missing() {
    let text = Case::builder(|data: &DataMap| data.get("status").cloned())
        .when(Some(json!("draft")), "Draft order")
        .default("Unknown")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "Unknown");
}

#[tokio::test]
async fn progress_renders_bar_from_percentage_field() {
    let mut data = DataMap::new();
    data.insert("percent".into(), json!(35));

    let text = Progress::builder("percent").width(10).build();

    assert_eq!(&*text.render_text(&data).await, "####------  35%");
}

#[tokio::test]
async fn progress_clamps_and_supports_custom_symbols() {
    let mut data = DataMap::new();
    data.insert("percent".into(), json!(120));

    let text = Progress::builder("percent")
        .width(4)
        .filled("=")
        .empty(".")
        .build();

    assert_eq!(&*text.render_text(&data).await, "==== 100%");
}

#[tokio::test]
async fn scrolling_text_defaults_to_first_page_without_context() {
    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "abcd");
    assert_eq!(text.page_count(&DataMap::new()).await, 3);
}

#[tokio::test]
async fn scrolling_text_uses_widget_page_from_context() {
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("article".into(), json!(2));

    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();

    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "ij"
    );
    assert_eq!(
        text.page_count_in_context_for_test(&ctx, &DataMap::new())
            .await,
        3
    );
}

#[tokio::test]
async fn scrolling_text_slices_by_char_boundaries() {
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("article".into(), json!(1));

    let text = ScrollingText::builder("article")
        .text("ab😀cd")
        .page_size(3)
        .build();

    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "cd"
    );
    assert_eq!(text.page_count(&DataMap::new()).await, 2);
}

#[tokio::test]
async fn scrolling_text_can_drive_numbered_pager_via_scroll_trait() {
    let text = ScrollingText::builder("article")
        .text("abcdefghij")
        .page_size(4)
        .build();
    let markup = NumberedPager::builder(text)
        .page_renderer(|page, _data| format!("{}", page + 1))
        .current_page_renderer(|page, _data| format!("[{}]", page + 1))
        .length(5)
        .build()
        .render_keyboard_for_test(&Context::new("", "state", json!(null)), &DataMap::new())
        .await
        .unwrap();
    let rows = markup.inline_keyboard().unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 3);
    assert_eq!(&*rows[0][0].text, "[1]");
    assert_eq!(&*rows[0][2].text, "3");
}
