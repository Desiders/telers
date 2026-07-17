//! Tests for media widgets.

use serde_json::{json, Value};

use super::{
    DynamicMedia, Media, MediaAttachment, MediaContentType, MediaId, MediaScroll, MultiMedia,
    StaticMedia,
};
use crate::entities::{Context, DataMap};

#[tokio::test]
async fn static_media_renders_photo_url() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = StaticMedia::builder(MediaContentType::Photo)
        .url("https://example.com/image.jpg")
        .build();
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/image.jpg")
    );
    assert!(attachment.file_id.is_none());
}

#[tokio::test]
async fn static_media_renders_video_with_caption() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = StaticMedia::builder(MediaContentType::Video)
        .url("https://example.com/video.mp4")
        .caption("My video")
        .parse_mode("HTML")
        .build();

    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Video);
    assert_eq!(attachment.caption.as_deref(), Some("My video"));
    assert_eq!(attachment.parse_mode.as_deref(), Some("HTML"));
}

#[tokio::test]
async fn static_media_returns_none_without_source() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = StaticMedia::builder(MediaContentType::Photo).build();
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn dynamic_media_reads_url_from_field() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("image_url".into(), json!("https://example.com/dynamic.jpg"));

    let media = DynamicMedia::from_url_field(MediaContentType::Photo, "image_url");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/dynamic.jpg")
    );
}

#[tokio::test]
async fn dynamic_media_returns_none_when_field_missing() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = DynamicMedia::from_url_field(MediaContentType::Photo, "missing_field");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn dynamic_media_from_field_parses_known_content_type() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert(
        "media".into(),
        json!({ "content_type": "video", "url": "https://example.com/v.mp4" }),
    );

    let media = DynamicMedia::from_field("media");
    let attachment = media.render_media_for_test(&ctx, &data).await.unwrap();

    assert_eq!(attachment.content_type, MediaContentType::Video);
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/v.mp4"));
}

#[tokio::test]
async fn dynamic_media_from_field_skips_unknown_content_type() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert(
        "media".into(),
        json!({ "content_type": "vidoe", "url": "https://example.com/v.mp4" }),
    );

    let media = DynamicMedia::from_field("media");

    assert!(media.render_media_for_test(&ctx, &data).await.is_none());
}

#[tokio::test]
async fn dynamic_media_with_custom_selector() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("video_id".into(), json!("ABC123"));

    let media = DynamicMedia::builder(|data: &DataMap| {
        data.get("video_id").and_then(Value::as_str).map(|id| {
            MediaAttachment::builder(MediaContentType::Video)
                .file_id(MediaId::new(id.to_string()))
                .build()
        })
    })
    .build();

    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Video);
    assert!(attachment.file_id.is_some());
    assert_eq!(attachment.get_file_id(), Some("ABC123"));
}

#[tokio::test]
async fn media_scroll_renders_current_page() {
    let mut ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert(
        "images".into(),
        json!([
            "https://example.com/1.jpg",
            "https://example.com/2.jpg",
            "https://example.com/3.jpg"
        ]),
    );

    ctx.widget_data.insert("gallery".into(), json!(1));

    let media = MediaScroll::from_url_array_field("gallery", MediaContentType::Photo, "images");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/2.jpg"));
}

#[tokio::test]
async fn media_scroll_handles_empty_items() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("images".into(), json!([]));

    let media = MediaScroll::from_url_array_field("gallery", MediaContentType::Photo, "images");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn media_scroll_clamps_page_to_bounds() {
    let mut ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("images".into(), json!(["https://example.com/only.jpg"]));

    ctx.widget_data.insert("gallery".into(), json!(10));

    let media = MediaScroll::from_url_array_field("gallery", MediaContentType::Photo, "images");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/only.jpg")
    );
}

#[tokio::test]
async fn media_attachment_to_input_file_from_id() {
    let attachment = MediaAttachment::builder(MediaContentType::Photo)
        .file_id(MediaId::new("file_abc123"))
        .build();
    let input_file = attachment.to_input_file();

    assert!(input_file.is_some());
}

#[tokio::test]
async fn media_attachment_to_input_file_from_url() {
    let attachment = MediaAttachment::builder(MediaContentType::Photo)
        .url("https://example.com/image.jpg")
        .build();
    let input_file = attachment.to_input_file();

    assert!(input_file.is_some());
}

#[tokio::test]
async fn media_attachment_builder_methods() {
    let attachment = MediaAttachment::builder(MediaContentType::Video)
        .url("https://example.com/video.mp4")
        .caption("Test video")
        .parse_mode("HTML")
        .width(1920)
        .height(1080)
        .duration(120)
        .supports_streaming(true)
        .has_spoiler(false)
        .build();

    assert_eq!(attachment.content_type, MediaContentType::Video);
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/video.mp4")
    );
    assert_eq!(attachment.caption.as_deref(), Some("Test video"));
    assert_eq!(attachment.parse_mode.as_deref(), Some("HTML"));
    assert_eq!(attachment.width, Some(1920));
    assert_eq!(attachment.height, Some(1080));
    assert_eq!(attachment.duration, Some(120));
    assert_eq!(attachment.supports_streaming, Some(true));
    assert_eq!(attachment.has_spoiler, Some(false));
}

#[tokio::test]
async fn dynamic_media_bon_builder_renders_attachment() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("image_url".into(), json!("https://example.com/builder.jpg"));

    let media = DynamicMedia::builder(|data: &DataMap| {
        data.get("image_url")
            .and_then(|value| value.as_str())
            .map(|url| {
                MediaAttachment::builder(MediaContentType::Photo)
                    .url(url.to_owned())
                    .build()
            })
    })
    .build();

    let attachment = media.render_media_for_test(&ctx, &data).await.unwrap();

    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/builder.jpg")
    );
}

#[tokio::test]
async fn media_scroll_bon_builder_renders_current_page() {
    let mut ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("images".into(), json!(["one", "two", "three"]));
    ctx.widget_data.insert("gallery".into(), json!(2));

    let media = MediaScroll::builder("gallery")
        .items_getter(|data: &DataMap| {
            data.get("images")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                .collect::<Vec<_>>()
        })
        .item_renderer(|file_id: &String, _data: &DataMap| {
            MediaAttachment::builder(MediaContentType::Photo)
                .file_id(MediaId::new(file_id.clone()))
                .build()
        })
        .build();

    let attachment = media.render_media_for_test(&ctx, &data).await.unwrap();

    assert_eq!(attachment.get_file_id(), Some("three"));
}

#[tokio::test]
async fn multi_media_bon_builder_returns_first_rendered_attachment() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = MultiMedia::builder()
        .media(DynamicMedia::from_field("missing"))
        .media(
            StaticMedia::builder(MediaContentType::Photo)
                .url("https://example.com/fallback.jpg")
                .build(),
        )
        .build();

    let attachment = media.render_media_for_test(&ctx, &data).await.unwrap();

    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(
        attachment.url.as_deref(),
        Some("https://example.com/fallback.jpg")
    );
}
