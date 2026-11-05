use telers_methods::{
    serialization,
    types::{ChatIdKind, InputFile, InputMedia, InputMediaPhoto, InputMediaVideo, Message},
    DeleteMessage, EditMessageText, Either, GetMe, GetUpdates, SendMediaGroup, SendMessage,
    SendPhoto, TelegramMethod as _,
};

#[test]
fn requests_use_shared_types_and_serialize_without_a_client() {
    let chat_id: telers_types::types::ChatIdKind = ChatIdKind::Id(-42);
    let request = SendMessage::new(chat_id, "hello")
        .disable_notification(true)
        .build_request();

    assert_eq!(request.method_name, "sendMessage");
    assert!(request.files.is_none());
    let actual: serialization::Value =
        serialization::from_str(&serialization::to_string(&request.data).unwrap()).unwrap();
    let expected: serialization::Value =
        serialization::from_str(r#"{"chat_id":-42,"text":"hello","disable_notification":true}"#)
            .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn nested_uploads_keep_their_parts_and_attach_references() {
    let video = InputFile::buffered_with_name(&b"video bytes"[..], "video.mp4");
    let thumbnail = InputFile::fs_with_name("unopened-thumbnail.jpg", "thumbnail.jpg");
    let video_reference = video.str_to_file().to_owned();
    let thumbnail_reference = thumbnail.str_to_file().to_owned();
    let media = [
        InputMedia::Video(
            InputMediaVideo::new(video)
                .thumbnail(thumbnail)
                .cover(InputFile::url("https://example.com/cover.jpg")),
        ),
        InputMedia::Photo(InputMediaPhoto::new(InputFile::id("existing-photo"))),
    ];

    let request = SendMediaGroup::new(42, media).build_request();
    assert_eq!(request.method_name, "sendMediaGroup");
    let files = request.files.unwrap();
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].str_to_file(), video_reference);
    assert_eq!(files[1].str_to_file(), thumbnail_reference);
    let InputFile::Buffered(buffer) = files[0].clone() else {
        panic!("video contents must remain a buffered upload");
    };
    assert_eq!(buffer.file_name(), Some("video.mp4"));
    assert_eq!(&buffer.into_bytes()[..], b"video bytes");
    let InputFile::FS(file) = &files[1] else {
        panic!("the thumbnail must remain a file descriptor");
    };
    assert_eq!(file.file_name(), Some("thumbnail.jpg"));

    let InputMedia::Video(video) = &request.data.media[0] else {
        panic!("the first media item must remain a video");
    };
    let InputFile::Buffered(reference) = video.media.clone() else {
        panic!("the prepared video must keep its upload descriptor");
    };
    assert!(reference.into_bytes().is_empty());
    assert_eq!(video.media.str_to_file(), video_reference);
    assert_eq!(
        video.thumbnail.as_ref().unwrap().str_to_file(),
        thumbnail_reference
    );
    assert_eq!(
        video.cover.as_ref().unwrap().str_to_file(),
        "https://example.com/cover.jpg"
    );

    let json = serialization::to_string(&request.data).unwrap();
    assert!(json.contains(&video_reference));
    assert!(json.contains(&thumbnail_reference));
    assert!(json.contains("existing-photo"));
}

#[test]
fn remote_references_do_not_create_upload_parts() {
    for file in [
        InputFile::id("existing-photo"),
        InputFile::url("https://example.com/photo.jpg"),
    ] {
        let reference = file.str_to_file().to_owned();
        let request = SendPhoto::new(42, file).build_request();
        assert!(request.files.unwrap().is_empty());
        assert_eq!(request.data.photo.str_to_file(), reference);
    }
}

#[test]
fn response_envelopes_keep_results_and_telegram_error_parameters() {
    let response =
        GetMe::build_response(r#"{"ok":true,"result":{"id":42,"is_bot":true,"first_name":"Bot"}}"#)
            .unwrap();
    assert!(response.ok);
    let user: telers_types::types::User = response.result.unwrap();
    assert_eq!(user.id, 42);

    let response = DeleteMessage::build_response(
        r#"{"ok":false,"description":"Too Many Requests","error_code":429,"parameters":{"retry_after":3}}"#,
    )
    .unwrap();
    assert!(!response.ok);
    assert!(response.result.is_none());
    assert_eq!(response.error_code, Some(429));
    assert_eq!(response.description.as_deref(), Some("Too Many Requests"));
    assert_eq!(response.parameters.unwrap().retry_after, Some(3));
}

#[test]
fn multiple_result_kinds_and_unparsed_updates_share_either() {
    let response = EditMessageText::build_response(r#"{"ok":true,"result":true}"#).unwrap();
    assert!(matches!(response.result, Some(Either::Right(true))));

    let response = EditMessageText::build_response(
        r#"{"ok":true,"result":{"message_id":7,"date":1,"chat":{"id":42,"type":"private"},"text":"edited"}}"#,
    )
    .unwrap();
    let Some(Either::Left(message)) = response.result else {
        panic!("a message response must use the message branch");
    };
    let message: Message = message;
    assert_eq!(message.message_id(), 7);

    let response = GetUpdates::build_response(
        r#"{"ok":true,"result":[{"update_id":9,"message":"malformed known update"}]}"#,
    )
    .unwrap();
    let updates = response.result.unwrap();
    let Either::Right(update) = &updates[0] else {
        panic!("an update with malformed known fields must remain inspectable");
    };
    assert_eq!(update.update_id, 9);
    assert!(update.extra.contains_key("message"));
}

#[test]
fn malformed_responses_report_a_parse_error() {
    assert!(GetMe::build_response("{").is_err());
    assert!(DeleteMessage::build_response(r#"{"ok":true,"result":"wrong type"}"#).is_err());
}
