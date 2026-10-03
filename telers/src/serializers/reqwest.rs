//! Multipart encoding for Reqwest with the selected serialization backend.

use crate::serialization::{Error as JsonError, Serialize};
use reqwest::multipart::Form;
use std::{borrow::Cow, fmt::Debug};

#[cfg(feature = "deser")]
mod deser;
#[cfg(not(feature = "deser"))]
mod serde;

#[cfg(feature = "deser")]
use self::deser as backend;
#[cfg(not(feature = "deser"))]
use self::serde as backend;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("Cannot serialize a field, custom error: {0}")]
    Custom(Cow<'static, str>),
    #[error(transparent)]
    Json(#[from] JsonError),
}

impl Error {
    fn top_level(value: impl Debug) -> Self {
        Self::Custom(format!("Cannot serialize a top-level struct: {value:?}").into())
    }
}

pub(crate) struct MultipartSerializer;

impl MultipartSerializer {
    pub(crate) fn build(data: &impl Serialize) -> Result<Form, Error> {
        backend::serialize(data)
    }
}

#[cfg(test)]
mod tests {
    use super::MultipartSerializer;
    use crate::{
        methods::{SendMediaGroup, SendMessage},
        serialization::Serialize,
        types::{
            InlineKeyboardButton, InlineKeyboardMarkup, InputFile, InputMedia, InputMediaPhoto,
        },
    };
    use futures_util::TryStreamExt;

    #[derive(Serialize)]
    struct Field<T> {
        value: T,
    }

    #[derive(Serialize)]
    struct EscapedKey<'a> {
        #[cfg_attr(not(feature = "deser"), serde(rename = "quote\"\\\n\t\0🚀"))]
        #[cfg_attr(feature = "deser", deser(rename = "quote\"\\\n\t\0🚀"))]
        value: &'a str,
    }

    struct Binary<'a>(&'a [u8]);

    #[derive(Serialize)]
    struct Newtype<T>(T);

    #[derive(Serialize)]
    struct Empty {}

    #[cfg(not(feature = "deser"))]
    impl Serialize for Binary<'_> {
        fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_bytes(self.0)
        }
    }

    #[cfg(feature = "deser")]
    impl Serialize for Binary<'_> {
        fn serialize(
            &self,
            _state: &mut ::deser::State,
        ) -> Result<::deser::ser::Chunk<'_>, ::deser::Error> {
            Ok(::deser::ser::Chunk::Atom(::deser::Atom::Bytes(
                self.0.into(),
            )))
        }
    }

    async fn field_body(value: impl Serialize) -> Vec<u8> {
        let form = MultipartSerializer::build(&Field {
            value,
        })
        .unwrap();
        let boundary = form.boundary().to_owned();
        let chunks: Vec<_> = form.into_stream().try_collect().await.unwrap();
        let body: Vec<_> = chunks.into_iter().flatten().collect();
        let start = body
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
            .unwrap()
            + 4;
        body[start..]
            .strip_suffix(format!("\r\n--{boundary}--\r\n").as_bytes())
            .unwrap()
            .to_vec()
    }

    #[tokio::test]
    async fn multipart_preserves_every_unicode_scalar() {
        let text: String = (0..=0x10ffff).filter_map(char::from_u32).collect();
        assert_eq!(field_body(&text).await, text.as_bytes());

        let nested = EscapedKey {
            value: &text,
        };
        let expected = format!(
            "{{{}:{}}}",
            serde_json::to_string("quote\"\\\n\t\0🚀").unwrap(),
            serde_json::to_string(&text).unwrap(),
        );
        let actual = field_body(nested).await;
        assert_eq!(actual.len(), expected.len());
        assert!(actual == expected.as_bytes());
    }

    #[tokio::test]
    async fn multipart_preserves_scalar_float_format() {
        for value in [
            1.0_f64,
            -0.0,
            1e-20,
            1e20,
            f64::MIN_POSITIVE,
            f64::MAX,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ] {
            assert_eq!(field_body(value).await, value.to_string().as_bytes());
        }
        for value in [
            1.0_f32,
            -0.0,
            1e-20,
            1e20,
            f32::MIN_POSITIVE,
            f32::MAX,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ] {
            assert_eq!(field_body(value).await, value.to_string().as_bytes());
        }
    }

    #[tokio::test]
    async fn multipart_preserves_nested_float_format() {
        let values = [
            1.0_f64,
            -0.0,
            1e-20,
            1e20,
            f64::MIN_POSITIVE,
            f64::MAX,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ];
        assert_eq!(
            field_body(values.to_vec()).await,
            serde_json::to_vec(&values).unwrap()
        );
        let values = [
            1.0_f32,
            -0.0,
            1e-20,
            1e20,
            f32::MIN_POSITIVE,
            f32::MAX,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        assert_eq!(
            field_body(values.to_vec()).await,
            serde_json::to_vec(&values).unwrap()
        );
    }

    #[tokio::test]
    async fn multipart_preserves_every_byte() {
        let bytes: Vec<_> = (0..=255).collect();
        assert_eq!(field_body(Binary(&bytes)).await, bytes);
        assert_eq!(
            field_body(Field {
                value: Binary(&bytes)
            })
            .await,
            format!("{{\"value\":{}}}", serde_json::to_string(&bytes).unwrap()).as_bytes(),
        );
    }

    #[tokio::test]
    async fn multipart_preserves_large_integers() {
        for value in [i128::MIN, i128::MAX] {
            assert_eq!(field_body(value).await, value.to_string().as_bytes());
            assert_eq!(
                field_body(vec![value]).await,
                format!("[{value}]").as_bytes()
            );
        }
        assert_eq!(
            field_body(u128::MAX).await,
            u128::MAX.to_string().as_bytes()
        );
        assert_eq!(
            field_body(vec![u128::MAX]).await,
            format!("[{}]", u128::MAX).as_bytes()
        );
    }

    #[tokio::test]
    async fn multipart_preserves_compound_fields() {
        let text = "\"\\\r\n\t\0é🚀e\u{301}";
        let expected = serde_json::to_string(text).unwrap();
        let map = std::collections::BTreeMap::from([(text, text)]);
        assert_eq!(
            field_body(map).await,
            format!("{{{expected}:{expected}}}").as_bytes(),
        );
        assert_eq!(
            field_body((text, 42)).await,
            format!("[{expected},42]").as_bytes()
        );
        assert_eq!(field_body(Empty {}).await, b"{}");
        assert_eq!(field_body(Vec::<String>::new()).await, b"[]");
        assert_eq!(field_body(None::<String>).await, b"null");
        assert_eq!(field_body(()).await, b"null");
        assert_eq!(field_body(Some(text)).await, text.as_bytes());
        assert_eq!(field_body(Newtype(text)).await, text.as_bytes());
    }

    #[tokio::test]
    async fn multipart_accepts_maps_and_forwarded_requests() {
        let map = std::collections::BTreeMap::from([("value", "\"\\\r\n\t\0é🚀")]);
        let body = form_body(&map).await;
        assert!(body.contains("\r\n\r\n\"\\\r\n\t\0é🚀\r\n"));
        let body = form_body(&Newtype(Some(Field {
            value: "text",
        })))
        .await;
        assert!(body.contains("name=\"value\""));
        assert!(body.contains("\r\n\r\ntext\r\n"));
        assert!(
            MultipartSerializer::build(&std::collections::BTreeMap::from([(42, "text")])).is_err()
        );
        assert!(MultipartSerializer::build(&42).is_err());
    }

    async fn form_body(data: &impl Serialize) -> String {
        let chunks: Vec<_> = MultipartSerializer::build(data)
            .unwrap()
            .into_stream()
            .try_collect()
            .await
            .unwrap();
        String::from_utf8(chunks.into_iter().flatten().collect()).unwrap()
    }

    #[tokio::test]
    async fn multipart_preserves_scalars_json_and_omits_none() {
        let request = SendMessage::new(-123_i64, "quotes: \" and newline\n")
            .disable_notification(false)
            .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                "open",
            )
            .url("https://example.com")]]));
        let body = form_body(&request).await;
        assert!(body.contains("\r\n\r\nquotes: \" and newline\n\r\n"));
        assert!(body.contains("\r\n\r\nfalse\r\n"));
        assert!(body.contains("\r\n\r\n-123\r\n"));
        assert!(
            body.contains(r#"{"inline_keyboard":[[{"text":"open","url":"https://example.com"}]]}"#)
        );
        assert!(!body.contains("name=\"parse_mode\""));
    }

    #[tokio::test]
    async fn multipart_preserves_nested_files_and_empty_sequences() {
        let file = InputFile::buffered_with_name(&b"photo"[..], "photo.jpg");
        let reference = file.str_to_file().to_owned();
        let request = SendMediaGroup::new(123_i64, [InputMediaPhoto::new(file)]);
        let body = form_body(&request).await;
        assert!(body.contains(&format!(r#"[{{"type":"photo","media":"{reference}"}}]"#)));

        let request = SendMediaGroup::new(123_i64, Vec::<InputMedia>::new());
        let body = form_body(&request).await;
        assert!(body.contains("name=\"media\""));
        assert!(body.contains("\r\n\r\n[]\r\n"));

        let fields = std::collections::BTreeMap::from([("media", Vec::<InputMedia>::new())]);
        let body = form_body(&fields).await;
        assert!(body.contains("name=\"media\""));
        assert!(body.contains("\r\n\r\n[]\r\n"));
    }
}
