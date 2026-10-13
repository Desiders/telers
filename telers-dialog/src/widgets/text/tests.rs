use super::{Case, FirstText, FormatText, ListText, MultiText, Progress, ScrollingText, Text};
use crate::{
    entities::{Context, DataMap},
    widgets::{FnText, Keyboard, NumberedPager, WhenCondition, WhenContext},
};
use telers::serialization::json;

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
async fn multi_text_skips_empty_items_without_extra_separators() {
    let text = MultiText::builder()
        .text("")
        .text("one")
        .text("")
        .text("two")
        .text("")
        .separator(" | ")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "one | two");
    assert_eq!(
        &*text
            .render_text_in_context_for_test(
                &Context::new("", "state", json!(null)),
                &DataMap::new(),
            )
            .await,
        "one | two",
    );
}

#[tokio::test]
async fn conditional_text_reads_data_in_both_render_paths() {
    let text = "visible".when(WhenCondition::data_field("enabled"));
    let ctx = Context::new("", "state", json!(null));

    assert!(text.render_text(&DataMap::new()).await.is_empty());
    assert!(text
        .render_text_in_context_for_test(&ctx, &DataMap::new())
        .await
        .is_empty());

    let data = DataMap::from([("enabled".into(), json!(true))]);
    assert_eq!(&*text.render_text(&data).await, "visible");
    assert_eq!(
        &*text.render_text_in_context_for_test(&ctx, &data).await,
        "visible"
    );
}

#[tokio::test]
async fn conditional_text_reads_stored_widget_state() {
    let mut ctx = Context::new("", "state", json!(null));
    let text = "selected".when(WhenCondition::new(|ctx: WhenContext| async move {
        ctx.context
            .widget_value_as::<bool>("selected")
            .unwrap_or(false)
    }));

    assert!(text
        .render_text_in_context_for_test(&ctx, &DataMap::new())
        .await
        .is_empty());

    ctx.widget_data.insert("selected".into(), json!(true));
    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "selected"
    );
}

#[tokio::test]
async fn hidden_conditional_text_does_not_run_renderer() {
    let text = FnText::new(|_: &DataMap| -> String { panic!("hidden renderer called") })
        .when(WhenCondition::data_field("missing"));

    assert!(text.render_text(&DataMap::new()).await.is_empty());
    assert!(text
        .render_text_in_context_for_test(
            &Context::new("", "state", json!(null)),
            &DataMap::new(),
        )
        .await
        .is_empty());
}

#[tokio::test]
async fn first_text_returns_first_nonempty_output_and_stops_rendering() {
    let text = FirstText::builder()
        .text("")
        .text("first")
        .text(FnText::new(|_: &DataMap| -> String {
            panic!("fallback renderer called")
        }))
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "first");
    assert_eq!(
        &*text
            .render_text_in_context_for_test(
                &Context::new("", "state", json!(null)),
                &DataMap::new(),
            )
            .await,
        "first"
    );
}

#[tokio::test]
async fn first_text_preserves_context_for_conditional_and_scrolling_children() {
    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("article".into(), json!(1));
    let text = FirstText::builder()
        .text("hidden".when(WhenCondition::data_field("missing")))
        .text(
            ScrollingText::builder("article")
                .text("abcdef")
                .page_size(3)
                .build(),
        )
        .text("fallback")
        .build();

    assert_eq!(&*text.render_text(&DataMap::new()).await, "abc");
    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "def"
    );
}

#[tokio::test]
async fn first_text_and_multi_text_return_empty_when_all_children_are_empty() {
    assert!(FirstText::builder()
        .build()
        .render_text(&DataMap::new())
        .await
        .is_empty());
    assert!(FirstText::builder()
        .text("")
        .text("")
        .build()
        .render_text(&DataMap::new())
        .await
        .is_empty());
    assert!(MultiText::builder()
        .text("")
        .text("")
        .build()
        .render_text(&DataMap::new())
        .await
        .is_empty());
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

    assert_eq!(&*text.render_text(&DataMap::new()).await, "one | two");

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
async fn list_text_item_metadata_tracks_absolute_positions_and_clamps_page() {
    let text = ListText::builder()
        .id("catalog")
        .page_size(2)
        .items_getter(|_: &DataMap| ["a", "b", "c", "d", "e"])
        .item_renderer(|item: &&str, data: &DataMap| {
            format!(
                "{}:{}:{}/{}={}",
                data["pos"].as_u64().unwrap(),
                data["pos0"].as_u64().unwrap(),
                data["current_page1"].as_u64().unwrap(),
                data["pages"].as_u64().unwrap(),
                item,
            )
        })
        .separator("|")
        .build();

    let mut ctx = Context::new("", "state", json!(null));
    ctx.widget_data.insert("catalog".into(), json!(1));
    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "3:2:2/3=c|4:3:2/3=d"
    );

    ctx.widget_data.insert("catalog".into(), json!(99));
    assert_eq!(
        &*text
            .render_text_in_context_for_test(&ctx, &DataMap::new())
            .await,
        "5:4:3/3=e"
    );
}

#[tokio::test]
async fn list_text_item_metadata_preserves_original_data_and_skips_empty_items() {
    let data = DataMap::from([
        ("pos".into(), json!("original")),
        ("greeting".into(), json!("hello")),
    ]);
    let text = ListText::builder()
        .items_getter(|data: &DataMap| {
            assert_eq!(data["pos"].as_str(), Some("original"));
            ["a", "", "b"]
        })
        .item_renderer(|item: &&str, data: &DataMap| {
            assert_eq!(
                data["data"].get("pos").and_then(|value| value.as_str()),
                Some("original")
            );
            assert_eq!(data["greeting"].as_str(), Some("hello"));
            if item.is_empty() {
                String::new()
            } else {
                format!("{}:{item}", data["pos"].as_u64().unwrap())
            }
        })
        .separator("|")
        .build();

    assert_eq!(&*text.render_text(&data).await, "1:a|3:b");
    assert_eq!(data["pos"].as_str(), Some("original"));
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
