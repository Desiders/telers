#![cfg(feature = "deser")]

use telers::{
    methods::{GetMe, GetUpdates, TelegramMethod},
    types::{Chat, RichText, User},
};

#[test]
fn default_client_response_uses_deser_and_rejects_trailing_input() {
    let input = r#"{"ok":true,"result":{"id":1,"is_bot":true,"first_name":"bot"}}"#;
    let response = GetMe::build_response(input).unwrap();
    assert_eq!(response.result.unwrap().first_name.as_ref(), "bot");
    assert!(GetMe::build_response(&format!("{input} null")).is_err());
}

#[test]
fn duplicate_known_fields_are_rejected() {
    let input = r#"{"id":1,"is_bot":true,"first_name":"one","first_name":"two"}"#;
    assert!(deser_json::from_str::<User>(input).is_err());
}

#[test]
fn tagged_unknown_payloads_keep_their_public_json_values() {
    let input = r#"{"type":"future_chat","id":42,"future":{"items":[1,true,"x"]}}"#;
    let chat: Chat = deser_json::from_str(input).unwrap();
    let Chat::Unknown(ref unknown) = chat else {
        panic!("expected unknown chat")
    };
    assert_eq!(
        deser_json::to_string(&unknown.extra["future"]).unwrap(),
        r#"{"items":[1,true,"x"]}"#
    );
    let serialized = deser_json::to_string(&chat).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
        serde_json::from_str::<serde_json::Value>(input).unwrap()
    );
}

#[test]
fn tagged_enum_with_scalar_and_sequence_fallbacks_roundtrips() {
    let input = r#"["hello",{"type":"bold","text":"world"}]"#;
    let text: RichText = deser_json::from_str(input).unwrap();
    assert_eq!(deser_json::to_string(&text).unwrap(), input);
}

#[test]
fn recursive_rich_text_parses_beyond_serdes_default_depth_limit() {
    let input = format!("{}\"text\"{}", "[".repeat(256), "]".repeat(256));
    let text: RichText = deser_json::from_str(&input).unwrap();
    assert_eq!(deser_json::to_string(&text).unwrap(), input);
}

#[test]
fn polling_preserves_unknown_updates() {
    let input = r#"{"ok":true,"result":[{"update_id":7,"future_update":{"foo":true}}]}"#;
    let response = GetUpdates::build_response(input).unwrap();
    let updates = response.result.unwrap();
    assert_eq!(updates.len(), 1);
    let telers::Either::Right(ref unparsed) = updates[0] else {
        panic!("expected unparsed update")
    };
    assert_eq!(unparsed.update_id, 7);
}

#[test]
fn unknown_payloads_preserve_deep_values_and_large_integers() {
    let future = format!(
        "{}340282366920938463463374607431768211455{}",
        "[".repeat(256),
        "]".repeat(256)
    );
    let input = format!(r#"{{"type":"future_chat","id":42,"future":{future}}}"#);
    let chat: Chat = telers::serialization::from_str(&input).unwrap();
    let Chat::Unknown(unknown) = &chat else {
        panic!("expected unknown chat")
    };
    assert_eq!(
        telers::serialization::to_string(&unknown.extra["future"]).unwrap(),
        future
    );
    let serialized = telers::serialization::to_string(&chat).unwrap();
    let parsed: Chat = telers::serialization::from_str(&serialized).unwrap();
    assert_eq!(
        telers::serialization::to_string(&parsed).unwrap(),
        serialized
    );
}

#[test]
fn unknown_payloads_reject_duplicate_fields() {
    let input = r#"{"type":"future_chat","id":42,"future":1,"future":2}"#;
    assert!(telers::serialization::from_str::<Chat>(input).is_err());
}

// These types intentionally derive only Deser. Enabling both Cargo features must
// still require exclusively the selected backend's traits.
#[derive(Debug, PartialEq, deser::Serialize, deser::Deserialize)]
struct NativeData {
    answer: u128,
}

struct NativeMethod;
impl TelegramMethod for NativeMethod {
    type Method = NativeData;
    type Return = NativeData;

    fn build_request<Client>(
        self,
        _bot: &telers::Bot<Client>,
    ) -> telers::methods::Request<Self::Method> {
        telers::methods::Request::new(
            "nativeMethod",
            NativeData {
                answer: u128::MAX,
            },
            None,
        )
    }
}

#[test]
fn custom_methods_accept_types_that_only_implement_deser() {
    let response = NativeMethod::build_response(
        r#"{"ok":true,"result":{"answer":340282366920938463463374607431768211455}}"#,
    )
    .unwrap();
    assert_eq!(response.result.unwrap().answer, u128::MAX);
    let request = NativeMethod.build_request(&telers::Bot::<telers::client::Reqwest>::default());
    assert_eq!(
        telers::serialization::to_string(&request.data).unwrap(),
        r#"{"answer":340282366920938463463374607431768211455}"#
    );
}

#[cfg(feature = "memory-storage")]
#[tokio::test]
async fn fsm_storage_accepts_types_that_only_implement_deser() {
    use telers::fsm::storage::{Memory, Storage, StorageKey};
    let storage = Memory::default();
    let key = StorageKey::new(1, 2, 3, None, None);
    storage
        .set_value(
            &key,
            "native",
            NativeData {
                answer: u128::MAX,
            },
        )
        .await
        .unwrap();
    let value: NativeData = storage.get_value(&key, "native").await.unwrap().unwrap();
    assert_eq!(value.answer, u128::MAX);
}
