//! Tests for media widgets.

use serde_json::json;

use super::{DynamicMedia, Media, MediaAttachment, MediaContentType, MediaScroll, StaticMedia};
use crate::entities::{Context, DataMap};

#[tokio::test]
async fn static_media_renders_photo_url() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = StaticMedia::photo_url("https://example.com/image.jpg");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/image.jpg"));
    assert!(attachment.file_id.is_none());
}

#[tokio::test]
async fn static_media_renders_video_with_caption() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = StaticMedia::video_url("https://example.com/video.mp4")
        .with_caption("My video")
        .with_parse_mode("HTML");

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

    let media = StaticMedia::builder().build();
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn dynamic_media_reads_url_from_field() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("image_url".into(), json!("https://example.com/dynamic.jpg"));

    let media = DynamicMedia::photo_url_from_field("image_url");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    assert_eq!(attachment.content_type, MediaContentType::Photo);
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/dynamic.jpg"));
}

#[tokio::test]
async fn dynamic_media_returns_none_when_field_missing() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let data = DataMap::new();

    let media = DynamicMedia::photo_url_from_field("missing_field");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn dynamic_media_with_custom_selector() {
    let ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("video_id".into(), json!("ABC123"));

    let media = DynamicMedia::new(|data| {
        data.get("video_id")
            .and_then(|v| v.as_str())
            .map(|id| MediaAttachment::video_id(id.to_string()))
    });

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
        json!(["https://example.com/1.jpg", "https://example.com/2.jpg", "https://example.com/3.jpg"]),
    );

    // Set current page to 1 (second image)
    ctx.widget_data.insert("gallery".into(), json!(1));

    let media = MediaScroll::photo_urls_from_field("gallery", "images");
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

    let media = MediaScroll::photo_urls_from_field("gallery", "images");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_none());
}

#[tokio::test]
async fn media_scroll_clamps_page_to_bounds() {
    let mut ctx = Context::new("", "state", serde_json::Value::Null);
    let mut data = DataMap::new();
    data.insert("images".into(), json!(["https://example.com/only.jpg"]));

    // Set page beyond bounds
    ctx.widget_data.insert("gallery".into(), json!(10));

    let media = MediaScroll::photo_urls_from_field("gallery", "images");
    let attachment = media.render_media_for_test(&ctx, &data).await;

    assert!(attachment.is_some());
    let attachment = attachment.unwrap();
    // Should clamp to last item (index 0)
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/only.jpg"));
}

#[tokio::test]
async fn media_attachment_to_input_file_from_id() {
    let attachment = MediaAttachment::photo_id("file_abc123");
    let input_file = attachment.to_input_file();

    assert!(input_file.is_some());
}

#[tokio::test]
async fn media_attachment_to_input_file_from_url() {
    let attachment = MediaAttachment::photo_url("https://example.com/image.jpg");
    let input_file = attachment.to_input_file();

    assert!(input_file.is_some());
}

#[tokio::test]
async fn media_attachment_builder_methods() {
    let attachment = MediaAttachment::new(MediaContentType::Video)
        .url("https://example.com/video.mp4")
        .caption("Test video")
        .parse_mode("HTML")
        .width(1920)
        .height(1080)
        .duration(120)
        .supports_streaming(true)
        .has_spoiler(false);

    assert_eq!(attachment.content_type, MediaContentType::Video);
    assert_eq!(attachment.url.as_deref(), Some("https://example.com/video.mp4"));
    assert_eq!(attachment.caption.as_deref(), Some("Test video"));
    assert_eq!(attachment.parse_mode.as_deref(), Some("HTML"));
    assert_eq!(attachment.width, Some(1920));
    assert_eq!(attachment.height, Some(1080));
    assert_eq!(attachment.duration, Some(120));
    assert_eq!(attachment.supports_streaming, Some(true));
    assert_eq!(attachment.has_spoiler, Some(false));
}
