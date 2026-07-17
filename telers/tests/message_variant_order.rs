use telers::types::Message;

/// Telegram sets both fields on these messages ("for backward compatibility, the X field
/// will also be set"), so the specific variant must win over the generic one in the
/// untagged `Message` enum: venue over location, animation over document, live photo
/// over photo.
fn message_json(content: &str) -> String {
    format!(
        r#"{{
            "message_id": 1,
            "date": 1784219380,
            "chat": {{"id": -1, "title": "chat", "type": "supergroup"}},
            {content}
        }}"#
    )
}

#[test]
fn venue_message_with_location_parses_as_venue() {
    let json = message_json(
        r#""venue": {
            "location": {"latitude": 1.0, "longitude": 2.0},
            "title": "Cafe", "address": "Street 1"
        },
        "location": {"latitude": 1.0, "longitude": 2.0}"#,
    );
    let message: Message = serde_json::from_str(&json).unwrap();
    assert!(matches!(message, Message::Venue(_)), "got {message:?}");
}

#[test]
fn animation_message_with_document_parses_as_animation() {
    let json = message_json(
        r#""animation": {
            "file_id": "a", "file_unique_id": "a", "width": 1, "height": 1, "duration": 1
        },
        "document": {"file_id": "a", "file_unique_id": "a"}"#,
    );
    let message: Message = serde_json::from_str(&json).unwrap();
    assert!(matches!(message, Message::Animation(_)), "got {message:?}");
}

#[test]
fn live_photo_message_with_photo_parses_as_live_photo() {
    let json = message_json(
        r#""live_photo": {
            "file_id": "l", "file_unique_id": "l", "width": 1, "height": 1, "duration": 1
        },
        "photo": [{"file_id": "p", "file_unique_id": "p", "width": 1, "height": 1}]"#,
    );
    let message: Message = serde_json::from_str(&json).unwrap();
    assert!(matches!(message, Message::LivePhoto(_)), "got {message:?}");
}
