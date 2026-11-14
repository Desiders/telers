use serde_json::{json, Value};
use telers_types::{
    serialization::{self, DeserializeOwned, Serialize},
    types::{
        ChatShareTheme, EmojiType, Message, MessageEntity, RichText, TonNft, TonWalletCard, Update,
        WidgetQueryType,
    },
};

fn roundtrip<T: DeserializeOwned + Serialize>(value: &Value) -> T {
    let parsed: T = serialization::from_str(&value.to_string()).unwrap();
    let serialized = serialization::to_string(&parsed).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&serialized).unwrap(), *value);
    parsed
}

fn message_with(field: &str, content: Value) -> Value {
    let mut value = json!({
        "message_id": 42,
        "date": 1794614400,
        "chat": {"id": 12345, "type": "private", "first_name": "Alice"}
    });
    value[field] = content;
    value
}

#[test]
fn widget_message_is_a_known_variant_and_preserves_caption_and_widget_data() {
    let mut value = message_with(
        "widget",
        json!({
            "id": "shared-widget",
            "html": "<button>Open</button>",
            "width": 320,
            "height": 240,
            "public_data": "public",
            "chat_data": "private"
        }),
    );
    value["caption"] = json!("A widget");
    value["show_caption_above_media"] = json!(true);
    let message: Message = roundtrip(&value);
    assert!(matches!(message, Message::Widget(_)), "{message:?}");
}

#[test]
fn new_service_messages_do_not_fall_back_to_unknown() {
    let waived: Message = roundtrip(&message_with("paid_messages_waived", json!({})));
    assert!(
        matches!(waived, Message::PaidMessagesWaived(_)),
        "{waived:?}"
    );

    let transfer: Message = roundtrip(&message_with(
        "ton_wallet_transfer",
        json!({
            "transaction_id": "transaction-hash",
            "peer_address": "0:0123456789abcdef",
            "amount": "-1234567890123456789012345678901234567890",
            "comment": "Outgoing transfer"
        }),
    ));
    assert!(
        matches!(transfer, Message::TonWalletTransfer(_)),
        "{transfer:?}"
    );
}

#[test]
fn widget_query_updates_preserve_the_nested_type_discriminator() {
    let update: Update = roundtrip(&json!({
        "update_id": 100,
        "widget_query": {
            "id": "query",
            "from": {"id": 12345, "is_bot": false, "first_name": "Alice"},
            "chat_instance": "instance",
            "inline_message_id": "inline-message",
            "type": {
                "type": "data",
                "wallet_card": {
                    "address": "0:0123456789abcdef",
                    "amount": "18446744073709551616000",
                    "stickers": []
                }
            },
            "data": "open",
            "widget_id": "shared-widget"
        }
    }));
    let Update::WidgetQuery(update) = update else {
        panic!("widget query parsed into the wrong update variant");
    };
    assert!(matches!(
        update.widget_query.r#type,
        WidgetQueryType::Data(_)
    ));

    let update: Update = roundtrip(&json!({
        "update_id": 101,
        "shared_widget_query": {
            "id": "shared-query",
            "from": {"id": 12345, "is_bot": false, "first_name": "Alice"},
            "type": {
                "type": "sign_data",
                "wallet": {"address": "0:0123456789abcdef", "amount": "18446744073709551616000"},
                "sign_data": "{\"signature\":\"signed\"}"
            },
            "data": "signed",
            "widget_id": "shared-widget"
        }
    }));
    let Update::SharedWidgetQuery(update) = update else {
        panic!("shared widget query parsed into the wrong update variant");
    };
    assert!(matches!(
        update.shared_widget_query.r#type,
        WidgetQueryType::SignData(_)
    ));
}

#[test]
fn widget_transaction_and_unknown_query_types_roundtrip() {
    let transaction: WidgetQueryType = roundtrip(&json!({
        "type": "sent_transaction",
        "wallet": {"address": "0:0123456789abcdef", "amount": "999999999999999999999999999999"},
        "msg_hash": "transaction-hash"
    }));
    assert!(matches!(transaction, WidgetQueryType::SentTransaction(_)));

    let future: WidgetQueryType = roundtrip(&json!({
        "type": "future_query",
        "payload": {"enabled": true}
    }));
    assert!(matches!(future, WidgetQueryType::Unknown(_)));
}

#[test]
fn share_theme_preserves_regular_and_custom_emoji() {
    let regular: EmojiType = roundtrip(&json!({"type": "emoji", "emoji": "🌟"}));
    assert!(matches!(regular, EmojiType::Emoji(_)));
    let custom: EmojiType =
        roundtrip(&json!({"type": "custom_emoji", "custom_emoji_id": "1234567890123456789"}));
    assert!(matches!(custom, EmojiType::CustomEmoji(_)));

    let theme: ChatShareTheme = roundtrip(&json!({
        "type": "emoji",
        "emojis": [
            {"type": "emoji", "emoji": "🌟"},
            {"type": "custom_emoji", "custom_emoji_id": "1234567890123456789"}
        ],
        "background_backdrop_id": 1234567890123_i64
    }));
    assert!(matches!(theme, ChatShareTheme::Emoji(_)));
}

#[test]
fn new_entity_tags_and_recursive_ton_rich_text_are_recognized() {
    let bank_card: MessageEntity =
        roundtrip(&json!({"type": "bank_card_number", "offset": 0, "length": 19}));
    assert!(matches!(bank_card, MessageEntity::BankCardNumber(_)));
    let ton: MessageEntity = roundtrip(&json!({"type": "ton_address", "offset": 20, "length": 48}));
    assert!(matches!(ton, MessageEntity::TonAddress(_)));

    let rich: RichText = roundtrip(&json!({
        "type": "ton_address",
        "address": "0:0123456789abcdef",
        "text": ["Wallet ", {"type": "bold", "text": "TON"}]
    }));
    assert!(matches!(rich, RichText::TonAddress(_)));
}

#[test]
fn ton_balances_and_nft_indices_remain_decimal_strings() {
    let amount = "12345678901234567890123456789012345678901234567890";
    let card: TonWalletCard = roundtrip(&json!({
        "address": "0:0123456789abcdef", "amount": amount, "stickers": []
    }));
    assert_eq!(card.amount.as_ref(), amount);

    let nft: TonNft = roundtrip(&json!({
        "address": "0:fedcba9876543210",
        "index": amount,
        "name": "Collectible",
        "description": "A collectible NFT",
        "attributes": [{"trait_type": "Color", "value": "Blue"}]
    }));
    assert_eq!(nft.index.as_ref(), amount);
}
