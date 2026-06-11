use serde::{de::DeserializeOwned, Serialize};
use telers::types::*;
fn must_parse<T: DeserializeOwned>(type_name: &str, value: &serde_json::Value) -> T {
    serde_json::from_value(value.clone()).unwrap_or_else(|err| {
        panic!("failed to deserialize {type_name} from JSON: {err}; json: {value}")
    })
}
fn must_to_value<T: Serialize>(type_name: &str, value: &T) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or_else(|err| {
        panic!("failed to convert {type_name} to JSON value after deserialize: {err}")
    })
}
fn must_roundtrip<T: Serialize + DeserializeOwned>(type_name: &str, value: &T) {
    let serialized = serde_json::to_string(value)
        .unwrap_or_else(|err| panic!("failed to serialize {type_name} after deserialize: {err}"));
    serde_json::from_str::<T>(&serialized).unwrap_or_else(|err| {
        panic!("failed roundtrip deserialize {type_name} from JSON: {err}; json: {serialized}")
    });
}
fn assert_json_subset(actual: &serde_json::Value, expected: &serde_json::Value) {
    match (actual, expected) {
        (serde_json::Value::Object(actual_obj), serde_json::Value::Object(expected_obj)) => {
            for (key, actual_val) in actual_obj {
                let expected_val = expected_obj.get(key).unwrap_or_else(|| {
                    panic!("missing key in expected json: {key}");
                });
                assert_json_subset(actual_val, expected_val);
            }
        }
        (serde_json::Value::Array(actual_arr), serde_json::Value::Array(expected_arr)) => {
            assert_eq!(
                actual_arr.len(),
                expected_arr.len(),
                "array length mismatch: actual={}, expected={}",
                actual_arr.len(),
                expected_arr.len()
            );
            for (actual_item, expected_item) in actual_arr.iter().zip(expected_arr.iter()) {
                assert_json_subset(actual_item, expected_item);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}
#[test]
fn test_accepted_gift_types_serialize_deserialize() {
    let value = serde_json::json!(
        { "unlimited_gifts" : true, "limited_gifts" : true, "unique_gifts" : true,
        "premium_subscription" : true, "gifts_from_channels" : true }
    );
    let parsed: AcceptedGiftTypes = must_parse(stringify!(AcceptedGiftTypes), &value);
    let parsed_value = must_to_value(stringify!(AcceptedGiftTypes), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(AcceptedGiftTypes), &parsed);
}
#[test]
fn test_affiliate_info_serialize_deserialize() {
    let value = serde_json::json!({ "commission_per_mille" : 1, "amount" : 1 });
    let parsed: AffiliateInfo = must_parse(stringify!(AffiliateInfo), &value);
    let parsed_value = must_to_value(stringify!(AffiliateInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(AffiliateInfo), &parsed);
}
#[test]
fn test_animation_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "duration" : 1 }
    );
    let parsed: Animation = must_parse(stringify!(Animation), &value);
    let parsed_value = must_to_value(stringify!(Animation), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Animation), &parsed);
}
#[test]
fn test_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "duration" : 1 }
    );
    let parsed: Audio = must_parse(stringify!(Audio), &value);
    let parsed_value = must_to_value(stringify!(Audio), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Audio), &parsed);
}
#[test]
fn test_background_fill_solid_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "solid", "color" : 1 });
    let parsed: BackgroundFill = must_parse(stringify!(BackgroundFill), &value);
    assert!(
        matches!(&parsed, BackgroundFill::Solid(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundFill),
        stringify!(Solid),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundFill), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundFill), &parsed);
}
#[test]
fn test_background_fill_gradient_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "gradient", "top_color" : 1, "bottom_color" : 1, "rotation_angle" : 1
        }
    );
    let parsed: BackgroundFill = must_parse(stringify!(BackgroundFill), &value);
    assert!(
        matches!(&parsed, BackgroundFill::Gradient(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundFill),
        stringify!(Gradient),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundFill), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundFill), &parsed);
}
#[test]
fn test_background_fill_freeform_gradient_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "freeform_gradient", "colors" : [1] });
    let parsed: BackgroundFill = must_parse(stringify!(BackgroundFill), &value);
    assert!(
        matches!(&parsed, BackgroundFill::FreeformGradient(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundFill),
        stringify!(FreeformGradient),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundFill), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundFill), &parsed);
}
#[test]
fn test_background_type_fill_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "fill", "fill" : { "type" : "solid", "color" : 1 },
        "dark_theme_dimming" : 1 }
    );
    let parsed: BackgroundType = must_parse(stringify!(BackgroundType), &value);
    assert!(
        matches!(&parsed, BackgroundType::Fill(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundType),
        stringify!(Fill),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundType), &parsed);
}
#[test]
fn test_background_type_wallpaper_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "wallpaper", "document" : { "file_id" : "used", "file_unique_id" :
        "test" }, "dark_theme_dimming" : 1 }
    );
    let parsed: BackgroundType = must_parse(stringify!(BackgroundType), &value);
    assert!(
        matches!(&parsed, BackgroundType::Wallpaper(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundType),
        stringify!(Wallpaper),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundType), &parsed);
}
#[test]
fn test_background_type_pattern_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "pattern", "document" : { "file_id" : "used", "file_unique_id" :
        "test" }, "fill" : { "type" : "solid", "color" : 1 }, "intensity" : 1 }
    );
    let parsed: BackgroundType = must_parse(stringify!(BackgroundType), &value);
    assert!(
        matches!(&parsed, BackgroundType::Pattern(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundType),
        stringify!(Pattern),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundType), &parsed);
}
#[test]
fn test_background_type_chat_theme_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "chat_theme", "theme_name" : "test" });
    let parsed: BackgroundType = must_parse(stringify!(BackgroundType), &value);
    assert!(
        matches!(&parsed, BackgroundType::ChatTheme(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BackgroundType),
        stringify!(ChatTheme),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BackgroundType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BackgroundType), &parsed);
}
#[test]
fn test_birthdate_serialize_deserialize() {
    let value = serde_json::json!({ "day" : 1, "month" : 1 });
    let parsed: Birthdate = must_parse(stringify!(Birthdate), &value);
    let parsed_value = must_to_value(stringify!(Birthdate), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Birthdate), &parsed);
}
#[test]
fn test_bot_access_settings_serialize_deserialize() {
    let value = serde_json::json!({ "is_access_restricted" : true });
    let parsed: BotAccessSettings = must_parse(stringify!(BotAccessSettings), &value);
    let parsed_value = must_to_value(stringify!(BotAccessSettings), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotAccessSettings), &parsed);
}
#[test]
fn test_bot_command_serialize_deserialize() {
    let value = serde_json::json!({ "command" : "test", "description" : "test" });
    let parsed: BotCommand = must_parse(stringify!(BotCommand), &value);
    let parsed_value = must_to_value(stringify!(BotCommand), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommand), &parsed);
}
#[test]
fn test_bot_command_scope_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "default" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_all_chat_administrators_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "all_chat_administrators" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::AllChatAdministrators(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(AllChatAdministrators),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_all_group_chats_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "all_group_chats" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::AllGroupChats(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(AllGroupChats),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_all_private_chats_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "all_private_chats" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::AllPrivateChats(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(AllPrivateChats),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_chat_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "chat", "chat_id" : "123" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::Chat(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(Chat),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_chat_administrators_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "chat_administrators", "chat_id" : "123" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::ChatAdministrators(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(ChatAdministrators),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_chat_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "chat_member", "chat_id" : "123", "user_id" : 1 }
    );
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::ChatMember(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(ChatMember),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_command_scope_default_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "default" });
    let parsed: BotCommandScope = must_parse(stringify!(BotCommandScope), &value);
    assert!(
        matches!(&parsed, BotCommandScope::Default(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(BotCommandScope),
        stringify!(Default),
        parsed
    );
    let parsed_value = must_to_value(stringify!(BotCommandScope), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotCommandScope), &parsed);
}
#[test]
fn test_bot_description_serialize_deserialize() {
    let value = serde_json::json!({ "description" : "test" });
    let parsed: BotDescription = must_parse(stringify!(BotDescription), &value);
    let parsed_value = must_to_value(stringify!(BotDescription), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotDescription), &parsed);
}
#[test]
fn test_bot_name_serialize_deserialize() {
    let value = serde_json::json!({ "name" : "test" });
    let parsed: BotName = must_parse(stringify!(BotName), &value);
    let parsed_value = must_to_value(stringify!(BotName), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotName), &parsed);
}
#[test]
fn test_bot_short_description_serialize_deserialize() {
    let value = serde_json::json!({ "short_description" : "test" });
    let parsed: BotShortDescription = must_parse(stringify!(BotShortDescription), &value);
    let parsed_value = must_to_value(stringify!(BotShortDescription), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BotShortDescription), &parsed);
}
#[test]
fn test_business_bot_rights_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: BusinessBotRights = must_parse(stringify!(BusinessBotRights), &value);
    let parsed_value = must_to_value(stringify!(BusinessBotRights), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessBotRights), &parsed);
}
#[test]
fn test_business_connection_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "user_chat_id" : 1, "date" : 1, "is_enabled" : true }
    );
    let parsed: BusinessConnection = must_parse(stringify!(BusinessConnection), &value);
    let parsed_value = must_to_value(stringify!(BusinessConnection), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessConnection), &parsed);
}
#[test]
fn test_business_intro_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: BusinessIntro = must_parse(stringify!(BusinessIntro), &value);
    let parsed_value = must_to_value(stringify!(BusinessIntro), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessIntro), &parsed);
}
#[test]
fn test_business_location_serialize_deserialize() {
    let value = serde_json::json!({ "address" : "test" });
    let parsed: BusinessLocation = must_parse(stringify!(BusinessLocation), &value);
    let parsed_value = must_to_value(stringify!(BusinessLocation), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessLocation), &parsed);
}
#[test]
fn test_business_messages_deleted_serialize_deserialize() {
    let value = serde_json::json!(
        { "business_connection_id" : "test", "chat" : { "type" : "private", "id" : 1 },
        "message_ids" : [1] }
    );
    let parsed: BusinessMessagesDeleted = must_parse(stringify!(BusinessMessagesDeleted), &value);
    let parsed_value = must_to_value(stringify!(BusinessMessagesDeleted), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessMessagesDeleted), &parsed);
}
#[test]
fn test_business_opening_hours_serialize_deserialize() {
    let value = serde_json::json!(
        { "time_zone_name" : "test", "opening_hours" : [{ "opening_minute" : 1,
        "closing_minute" : 1 }] }
    );
    let parsed: BusinessOpeningHours = must_parse(stringify!(BusinessOpeningHours), &value);
    let parsed_value = must_to_value(stringify!(BusinessOpeningHours), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessOpeningHours), &parsed);
}
#[test]
fn test_business_opening_hours_interval_serialize_deserialize() {
    let value = serde_json::json!({ "opening_minute" : 1, "closing_minute" : 1 });
    let parsed: BusinessOpeningHoursInterval =
        must_parse(stringify!(BusinessOpeningHoursInterval), &value);
    let parsed_value = must_to_value(stringify!(BusinessOpeningHoursInterval), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(BusinessOpeningHoursInterval), &parsed);
}
#[test]
fn test_callback_game_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: CallbackGame = must_parse(stringify!(CallbackGame), &value);
    let parsed_value = must_to_value(stringify!(CallbackGame), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(CallbackGame), &parsed);
}
#[test]
fn test_callback_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "chat_instance" : "test" }
    );
    let parsed: CallbackQuery = must_parse(stringify!(CallbackQuery), &value);
    let parsed_value = must_to_value(stringify!(CallbackQuery), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(CallbackQuery), &parsed);
}
#[test]
fn test_chat_channel_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "channel", "id" : 1 });
    let parsed: Chat = must_parse(stringify!(Chat), &value);
    assert!(
        matches!(&parsed, Chat::Channel(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Chat),
        stringify!(Channel),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Chat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Chat), &parsed);
}
#[test]
fn test_chat_group_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "group", "id" : 1 });
    let parsed: Chat = must_parse(stringify!(Chat), &value);
    assert!(
        matches!(&parsed, Chat::Group(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Chat),
        stringify!(Group),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Chat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Chat), &parsed);
}
#[test]
fn test_chat_private_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "private", "id" : 1 });
    let parsed: Chat = must_parse(stringify!(Chat), &value);
    assert!(
        matches!(&parsed, Chat::Private(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Chat),
        stringify!(Private),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Chat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Chat), &parsed);
}
#[test]
fn test_chat_supergroup_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "supergroup", "id" : 1 });
    let parsed: Chat = must_parse(stringify!(Chat), &value);
    assert!(
        matches!(&parsed, Chat::Supergroup(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Chat),
        stringify!(Supergroup),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Chat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Chat), &parsed);
}
#[test]
fn test_chat_administrator_rights_serialize_deserialize() {
    let value = serde_json::json!(
        { "is_anonymous" : true, "can_manage_chat" : true, "can_delete_messages" : true,
        "can_manage_video_chats" : true, "can_restrict_members" : true,
        "can_promote_members" : true, "can_change_info" : true, "can_invite_users" :
        true, "can_post_stories" : true, "can_edit_stories" : true, "can_delete_stories"
        : true }
    );
    let parsed: ChatAdministratorRights = must_parse(stringify!(ChatAdministratorRights), &value);
    let parsed_value = must_to_value(stringify!(ChatAdministratorRights), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatAdministratorRights), &parsed);
}
#[test]
fn test_chat_background_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : { "type" : "fill", "fill" : { "type" : "solid", "color" : 1 },
        "dark_theme_dimming" : 1 } }
    );
    let parsed: ChatBackground = must_parse(stringify!(ChatBackground), &value);
    let parsed_value = must_to_value(stringify!(ChatBackground), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBackground), &parsed);
}
#[test]
fn test_chat_boost_serialize_deserialize() {
    let value = serde_json::json!(
        { "boost_id" : "test", "add_date" : 1, "expiration_date" : 1, "source" : {
        "source" : "premium", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test"
        } } }
    );
    let parsed: ChatBoost = must_parse(stringify!(ChatBoost), &value);
    let parsed_value = must_to_value(stringify!(ChatBoost), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoost), &parsed);
}
#[test]
fn test_chat_boost_added_serialize_deserialize() {
    let value = serde_json::json!({ "boost_count" : 1 });
    let parsed: ChatBoostAdded = must_parse(stringify!(ChatBoostAdded), &value);
    let parsed_value = must_to_value(stringify!(ChatBoostAdded), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostAdded), &parsed);
}
#[test]
fn test_chat_boost_removed_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "boost_id" : "test", "remove_date" :
        1, "source" : { "source" : "premium", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } } }
    );
    let parsed: ChatBoostRemoved = must_parse(stringify!(ChatBoostRemoved), &value);
    let parsed_value = must_to_value(stringify!(ChatBoostRemoved), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostRemoved), &parsed);
}
#[test]
fn test_chat_boost_source_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "premium", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } }
    );
    let parsed: ChatBoostSource = must_parse(stringify!(ChatBoostSource), &value);
    let parsed_value = must_to_value(stringify!(ChatBoostSource), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostSource), &parsed);
}
#[test]
fn test_chat_boost_source_gift_code_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "gift_code", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } }
    );
    let parsed: ChatBoostSource = must_parse(stringify!(ChatBoostSource), &value);
    assert!(
        matches!(&parsed, ChatBoostSource::GiftCode(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatBoostSource),
        stringify!(GiftCode),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatBoostSource), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostSource), &parsed);
}
#[test]
fn test_chat_boost_source_giveaway_serialize_deserialize() {
    let value = serde_json::json!({ "source" : "giveaway", "giveaway_message_id" : 1 });
    let parsed: ChatBoostSource = must_parse(stringify!(ChatBoostSource), &value);
    assert!(
        matches!(&parsed, ChatBoostSource::Giveaway(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatBoostSource),
        stringify!(Giveaway),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatBoostSource), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostSource), &parsed);
}
#[test]
fn test_chat_boost_source_premium_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "premium", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } }
    );
    let parsed: ChatBoostSource = must_parse(stringify!(ChatBoostSource), &value);
    assert!(
        matches!(&parsed, ChatBoostSource::Premium(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatBoostSource),
        stringify!(Premium),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatBoostSource), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostSource), &parsed);
}
#[test]
fn test_chat_boost_updated_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "boost" : { "boost_id" : "test",
        "add_date" : 1, "expiration_date" : 1, "source" : { "source" : "premium", "user"
        : { "id" : 1, "is_bot" : true, "first_name" : "test" } } } }
    );
    let parsed: ChatBoostUpdated = must_parse(stringify!(ChatBoostUpdated), &value);
    let parsed_value = must_to_value(stringify!(ChatBoostUpdated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatBoostUpdated), &parsed);
}
#[test]
fn test_chat_full_info_channel_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "channel", "id" : 1, "accent_color_id" : 1, "max_reaction_count" : 1 }
    );
    let parsed: ChatFullInfo = must_parse(stringify!(ChatFullInfo), &value);
    assert!(
        matches!(&parsed, ChatFullInfo::Channel(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatFullInfo),
        stringify!(Channel),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatFullInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatFullInfo), &parsed);
}
#[test]
fn test_chat_full_info_group_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "group", "id" : 1, "accent_color_id" : 1, "max_reaction_count" : 1 }
    );
    let parsed: ChatFullInfo = must_parse(stringify!(ChatFullInfo), &value);
    assert!(
        matches!(&parsed, ChatFullInfo::Group(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatFullInfo),
        stringify!(Group),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatFullInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatFullInfo), &parsed);
}
#[test]
fn test_chat_full_info_private_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "private", "id" : 1, "accent_color_id" : 1, "max_reaction_count" : 1,
        "accepted_gift_types" : { "unlimited_gifts" : true, "limited_gifts" : true,
        "unique_gifts" : true, "premium_subscription" : true, "gifts_from_channels" :
        true } }
    );
    let parsed: ChatFullInfo = must_parse(stringify!(ChatFullInfo), &value);
    assert!(
        matches!(&parsed, ChatFullInfo::Private(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatFullInfo),
        stringify!(Private),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatFullInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatFullInfo), &parsed);
}
#[test]
fn test_chat_full_info_supergroup_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "supergroup", "id" : 1, "accent_color_id" : 1, "max_reaction_count" :
        1 }
    );
    let parsed: ChatFullInfo = must_parse(stringify!(ChatFullInfo), &value);
    assert!(
        matches!(&parsed, ChatFullInfo::Supergroup(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatFullInfo),
        stringify!(Supergroup),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatFullInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatFullInfo), &parsed);
}
#[test]
fn test_chat_invite_link_serialize_deserialize() {
    let value = serde_json::json!(
        { "invite_link" : "test", "creator" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "creates_join_request" : true, "is_primary" : true, "is_revoked" : true
        }
    );
    let parsed: ChatInviteLink = must_parse(stringify!(ChatInviteLink), &value);
    let parsed_value = must_to_value(stringify!(ChatInviteLink), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatInviteLink), &parsed);
}
#[test]
fn test_chat_join_request_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "from" : { "id" : 1, "is_bot" :
        true, "first_name" : "test" }, "user_chat_id" : 1, "date" : 1 }
    );
    let parsed: ChatJoinRequest = must_parse(stringify!(ChatJoinRequest), &value);
    let parsed_value = must_to_value(stringify!(ChatJoinRequest), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatJoinRequest), &parsed);
}
#[test]
fn test_chat_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "location" : { "latitude" : 1.25, "longitude" : 1.25 }, "address" : "test" }
    );
    let parsed: ChatLocation = must_parse(stringify!(ChatLocation), &value);
    let parsed_value = must_to_value(stringify!(ChatLocation), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatLocation), &parsed);
}
#[test]
fn test_chat_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "creator", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "is_anonymous" : true }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_administrator_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "administrator", "user" : { "id" : 1, "is_bot" : true, "first_name"
        : "test" }, "can_be_edited" : true, "is_anonymous" : true, "can_manage_chat" :
        true, "can_delete_messages" : true, "can_manage_video_chats" : true,
        "can_restrict_members" : true, "can_promote_members" : true, "can_change_info" :
        true, "can_invite_users" : true, "can_post_stories" : true, "can_edit_stories" :
        true, "can_delete_stories" : true }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Administrator(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Administrator),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_banned_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "kicked", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "until_date" : 1 }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Kicked(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Kicked),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_left_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "left", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test"
        } }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Left(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Left),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "member", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Member(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Member),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_owner_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "creator", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "is_anonymous" : true }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Creator(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Creator),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_restricted_serialize_deserialize() {
    let value = serde_json::json!(
        { "status" : "restricted", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "is_member" : true, "can_send_messages" : true, "can_send_audios" :
        true, "can_send_documents" : true, "can_send_photos" : true, "can_send_videos" :
        true, "can_send_video_notes" : true, "can_send_voice_notes" : true,
        "can_send_polls" : true, "can_send_other_messages" : true,
        "can_add_web_page_previews" : true, "can_react_to_messages" : true,
        "can_edit_tag" : true, "can_change_info" : true, "can_invite_users" : true,
        "can_pin_messages" : true, "can_manage_topics" : true, "until_date" : 1 }
    );
    let parsed: ChatMember = must_parse(stringify!(ChatMember), &value);
    assert!(
        matches!(&parsed, ChatMember::Restricted(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ChatMember),
        stringify!(Restricted),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ChatMember), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMember), &parsed);
}
#[test]
fn test_chat_member_updated_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "from" : { "id" : 1, "is_bot" :
        true, "first_name" : "test" }, "date" : 1, "old_chat_member" : { "status" :
        "creator", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "is_anonymous" : true }, "new_chat_member" : { "status" : "creator", "user" : {
        "id" : 1, "is_bot" : true, "first_name" : "test" }, "is_anonymous" : true } }
    );
    let parsed: ChatMemberUpdated = must_parse(stringify!(ChatMemberUpdated), &value);
    let parsed_value = must_to_value(stringify!(ChatMemberUpdated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatMemberUpdated), &parsed);
}
#[test]
fn test_chat_owner_changed_serialize_deserialize() {
    let value = serde_json::json!(
        { "new_owner" : { "id" : 1, "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: ChatOwnerChanged = must_parse(stringify!(ChatOwnerChanged), &value);
    let parsed_value = must_to_value(stringify!(ChatOwnerChanged), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatOwnerChanged), &parsed);
}
#[test]
fn test_chat_owner_left_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ChatOwnerLeft = must_parse(stringify!(ChatOwnerLeft), &value);
    let parsed_value = must_to_value(stringify!(ChatOwnerLeft), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatOwnerLeft), &parsed);
}
#[test]
fn test_chat_permissions_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ChatPermissions = must_parse(stringify!(ChatPermissions), &value);
    let parsed_value = must_to_value(stringify!(ChatPermissions), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatPermissions), &parsed);
}
#[test]
fn test_chat_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "small_file_id" : "used", "small_file_unique_id" : "test", "big_file_id" :
        "used", "big_file_unique_id" : "test" }
    );
    let parsed: ChatPhoto = must_parse(stringify!(ChatPhoto), &value);
    let parsed_value = must_to_value(stringify!(ChatPhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatPhoto), &parsed);
}
#[test]
fn test_chat_shared_serialize_deserialize() {
    let value = serde_json::json!({ "request_id" : 1, "chat_id" : 1 });
    let parsed: ChatShared = must_parse(stringify!(ChatShared), &value);
    let parsed_value = must_to_value(stringify!(ChatShared), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChatShared), &parsed);
}
#[test]
fn test_checklist_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "tasks" : [{ "id" : 1, "text" : "test" }] }
    );
    let parsed: Checklist = must_parse(stringify!(Checklist), &value);
    let parsed_value = must_to_value(stringify!(Checklist), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Checklist), &parsed);
}
#[test]
fn test_checklist_task_serialize_deserialize() {
    let value = serde_json::json!({ "id" : 1, "text" : "test" });
    let parsed: ChecklistTask = must_parse(stringify!(ChecklistTask), &value);
    let parsed_value = must_to_value(stringify!(ChecklistTask), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChecklistTask), &parsed);
}
#[test]
fn test_checklist_tasks_added_serialize_deserialize() {
    let value = serde_json::json!({ "tasks" : [{ "id" : 1, "text" : "test" }] });
    let parsed: ChecklistTasksAdded = must_parse(stringify!(ChecklistTasksAdded), &value);
    let parsed_value = must_to_value(stringify!(ChecklistTasksAdded), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChecklistTasksAdded), &parsed);
}
#[test]
fn test_checklist_tasks_done_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ChecklistTasksDone = must_parse(stringify!(ChecklistTasksDone), &value);
    let parsed_value = must_to_value(stringify!(ChecklistTasksDone), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChecklistTasksDone), &parsed);
}
#[test]
fn test_chosen_inline_result_serialize_deserialize() {
    let value = serde_json::json!(
        { "result_id" : "test", "from" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "query" : "test" }
    );
    let parsed: ChosenInlineResult = must_parse(stringify!(ChosenInlineResult), &value);
    let parsed_value = must_to_value(stringify!(ChosenInlineResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ChosenInlineResult), &parsed);
}
#[test]
fn test_contact_serialize_deserialize() {
    let value = serde_json::json!({ "phone_number" : "test", "first_name" : "test" });
    let parsed: Contact = must_parse(stringify!(Contact), &value);
    let parsed_value = must_to_value(stringify!(Contact), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Contact), &parsed);
}
#[test]
fn test_copy_text_button_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test" });
    let parsed: CopyTextButton = must_parse(stringify!(CopyTextButton), &value);
    let parsed_value = must_to_value(stringify!(CopyTextButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(CopyTextButton), &parsed);
}
#[test]
fn test_dice_serialize_deserialize() {
    let value = serde_json::json!({ "emoji" : "test", "value" : 1 });
    let parsed: Dice = must_parse(stringify!(Dice), &value);
    let parsed_value = must_to_value(stringify!(Dice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Dice), &parsed);
}
#[test]
fn test_direct_message_price_changed_serialize_deserialize() {
    let value = serde_json::json!({ "are_direct_messages_enabled" : true });
    let parsed: DirectMessagePriceChanged =
        must_parse(stringify!(DirectMessagePriceChanged), &value);
    let parsed_value = must_to_value(stringify!(DirectMessagePriceChanged), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(DirectMessagePriceChanged), &parsed);
}
#[test]
fn test_direct_messages_topic_serialize_deserialize() {
    let value = serde_json::json!({ "topic_id" : 1 });
    let parsed: DirectMessagesTopic = must_parse(stringify!(DirectMessagesTopic), &value);
    let parsed_value = must_to_value(stringify!(DirectMessagesTopic), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(DirectMessagesTopic), &parsed);
}
#[test]
fn test_document_serialize_deserialize() {
    let value = serde_json::json!({ "file_id" : "used", "file_unique_id" : "test" });
    let parsed: Document = must_parse(stringify!(Document), &value);
    let parsed_value = must_to_value(stringify!(Document), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Document), &parsed);
}
#[test]
fn test_encrypted_credentials_serialize_deserialize() {
    let value = serde_json::json!(
        { "data" : "test", "hash" : "test", "secret" : "test" }
    );
    let parsed: EncryptedCredentials = must_parse(stringify!(EncryptedCredentials), &value);
    let parsed_value = must_to_value(stringify!(EncryptedCredentials), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedCredentials), &parsed);
}
#[test]
fn test_encrypted_passport_element_address_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "address", "data" : "decrypted", "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::Address(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(Address),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_bank_statement_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "bank_statement", "files" : [{ "file_id" : "used", "file_unique_id" :
        "test", "file_size" : 1, "file_date" : 1 }], "translation" : [{ "file_id" :
        "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "hash" :
        "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::BankStatement(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(BankStatement),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_driver_license_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "driver_license", "data" : "decrypted", "front_side" : { "file_id" :
        "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 },
        "reverse_side" : { "file_id" : "used", "file_unique_id" : "test", "file_size" :
        1, "file_date" : 1 }, "selfie" : { "file_id" : "used", "file_unique_id" : "test",
        "file_size" : 1, "file_date" : 1 }, "translation" : [{ "file_id" : "used",
        "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::DriverLicense(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(DriverLicense),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_email_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "email", "email" : "test", "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::Email(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(Email),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_identity_card_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "identity_card", "data" : "decrypted", "front_side" : { "file_id" :
        "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 },
        "reverse_side" : { "file_id" : "used", "file_unique_id" : "test", "file_size" :
        1, "file_date" : 1 }, "selfie" : { "file_id" : "used", "file_unique_id" : "test",
        "file_size" : 1, "file_date" : 1 }, "translation" : [{ "file_id" : "used",
        "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::IdentityCard(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(IdentityCard),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_internal_passport_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "internal_passport", "data" : "decrypted", "front_side" : { "file_id"
        : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }, "selfie"
        : { "file_id" : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" :
        1 }, "translation" : [{ "file_id" : "used", "file_unique_id" : "test",
        "file_size" : 1, "file_date" : 1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::InternalPassport(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(InternalPassport),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_passport_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "passport", "data" : "decrypted", "front_side" : { "file_id" : "used",
        "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }, "selfie" : {
        "file_id" : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1
        }, "translation" : [{ "file_id" : "used", "file_unique_id" : "test", "file_size"
        : 1, "file_date" : 1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::Passport(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(Passport),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_passport_registration_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "passport_registration", "files" : [{ "file_id" : "used",
        "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "translation" :
        [{ "file_id" : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" :
        1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::PassportRegistration(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(PassportRegistration),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_personal_details_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "personal_details", "data" : "decrypted", "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::PersonalDetails(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(PersonalDetails),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_phone_number_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "phone_number", "phone_number" : "test", "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::PhoneNumber(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(PhoneNumber),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_rental_agreement_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "rental_agreement", "files" : [{ "file_id" : "used", "file_unique_id"
        : "test", "file_size" : 1, "file_date" : 1 }], "translation" : [{ "file_id" :
        "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "hash" :
        "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::RentalAgreement(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(RentalAgreement),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_temporary_registration_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "temporary_registration", "files" : [{ "file_id" : "used",
        "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "translation" :
        [{ "file_id" : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" :
        1 }], "hash" : "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::TemporaryRegistration(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(TemporaryRegistration),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_encrypted_passport_element_utility_bill_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "utility_bill", "files" : [{ "file_id" : "used", "file_unique_id" :
        "test", "file_size" : 1, "file_date" : 1 }], "translation" : [{ "file_id" :
        "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1 }], "hash" :
        "test" }
    );
    let parsed: EncryptedPassportElement = must_parse(stringify!(EncryptedPassportElement), &value);
    assert!(
        matches!(&parsed, EncryptedPassportElement::UtilityBill(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(EncryptedPassportElement),
        stringify!(UtilityBill),
        parsed
    );
    let parsed_value = must_to_value(stringify!(EncryptedPassportElement), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(EncryptedPassportElement), &parsed);
}
#[test]
fn test_external_reply_info_animation_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Animation(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Animation),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "audio" : { "file_id" : "used",
        "file_unique_id" : "test", "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_checklist_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "checklist" : { "title" : "test", "tasks" : [{
        "id" : 1, "text" : "test" }] } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Checklist(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Checklist),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_contact_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "contact" : { "phone_number" : "test",
        "first_name" : "test" } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Contact(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Contact),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_dice_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "dice" : { "emoji" : "test", "value" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Dice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Dice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_document_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "document" : { "file_id" : "used",
        "file_unique_id" : "test" } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Document(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Document),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_game_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "game" : { "title" : "test", "description" :
        "test", "photo" : [{ "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1 }] } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Game(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Game),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_giveaway_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "giveaway" : { "chats" : [{ "type" :
        "private", "id" : 1 }], "winners_selection_date" : 1, "winner_count" : 1,
        "premium_subscription_month_count" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Giveaway(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Giveaway),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_giveaway_winners_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "giveaway_winners" : { "chat" : { "type" :
        "private", "id" : 1 }, "giveaway_message_id" : 1, "winners_selection_date" : 1,
        "winner_count" : 1, "winners" : [{ "id" : 1, "is_bot" : true, "first_name" :
        "test" }], "premium_subscription_month_count" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::GiveawayWinners(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(GiveawayWinners),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_invoice_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "invoice" : { "title" : "test", "description"
        : "test", "start_parameter" : "used", "currency" : "test", "total_amount" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Invoice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Invoice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_live_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "live_photo" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "location" : { "latitude" : 1.25, "longitude"
        : 1.25 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "photo" : [{ "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1 }] }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_poll_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "poll" : { "type" : "regular", "id" : "test",
        "question" : "test", "options" : [{ "persistent_id" : "test", "text" : "test",
        "voter_count" : 1 }], "total_voter_count" : 1, "is_closed" : true, "is_anonymous"
        : true, "allows_multiple_answers" : true, "allows_revoting" : true,
        "members_only" : true } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Poll(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Poll),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_sticker_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "sticker" : { "type" : "regular", "file_id" :
        "used", "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" :
        true, "is_video" : true } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Sticker(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Sticker),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_story_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "story" : { "chat" : { "type" : "private",
        "id" : 1 }, "id" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Story(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Story),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "venue" : { "location" : { "latitude" : 1.25,
        "longitude" : 1.25 }, "title" : "test", "address" : "test" } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Venue(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Venue),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "video" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_video_note_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "video_note" : { "file_id" : "used",
        "file_unique_id" : "test", "length" : 1, "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::VideoNote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(VideoNote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_external_reply_info_voice_serialize_deserialize() {
    let value = serde_json::json!(
        { "origin" : { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot"
        : true, "first_name" : "test" } }, "voice" : { "file_id" : "used",
        "file_unique_id" : "test", "duration" : 1 } }
    );
    let parsed: ExternalReplyInfo = must_parse(stringify!(ExternalReplyInfo), &value);
    assert!(
        matches!(&parsed, ExternalReplyInfo::Voice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ExternalReplyInfo),
        stringify!(Voice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ExternalReplyInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ExternalReplyInfo), &parsed);
}
#[test]
fn test_file_serialize_deserialize() {
    let value = serde_json::json!({ "file_id" : "used", "file_unique_id" : "test" });
    let parsed: File = must_parse(stringify!(File), &value);
    let parsed_value = must_to_value(stringify!(File), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(File), &parsed);
}
#[test]
fn test_force_reply_serialize_deserialize() {
    let value = serde_json::json!({ "force_reply" : true });
    let parsed: ReplyMarkup = must_parse(stringify!(ReplyMarkup), &value);
    assert!(
        matches!(&parsed, ReplyMarkup::ForceReply(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReplyMarkup),
        stringify!(ForceReply),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReplyMarkup), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReplyMarkup), &parsed);
}
#[test]
fn test_forum_topic_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_thread_id" : 1, "name" : "test", "icon_color" : 1 }
    );
    let parsed: ForumTopic = must_parse(stringify!(ForumTopic), &value);
    let parsed_value = must_to_value(stringify!(ForumTopic), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ForumTopic), &parsed);
}
#[test]
fn test_forum_topic_closed_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ForumTopicClosed = must_parse(stringify!(ForumTopicClosed), &value);
    let parsed_value = must_to_value(stringify!(ForumTopicClosed), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ForumTopicClosed), &parsed);
}
#[test]
fn test_forum_topic_created_serialize_deserialize() {
    let value = serde_json::json!({ "name" : "test", "icon_color" : 1 });
    let parsed: ForumTopicCreated = must_parse(stringify!(ForumTopicCreated), &value);
    let parsed_value = must_to_value(stringify!(ForumTopicCreated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ForumTopicCreated), &parsed);
}
#[test]
fn test_forum_topic_edited_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ForumTopicEdited = must_parse(stringify!(ForumTopicEdited), &value);
    let parsed_value = must_to_value(stringify!(ForumTopicEdited), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ForumTopicEdited), &parsed);
}
#[test]
fn test_forum_topic_reopened_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ForumTopicReopened = must_parse(stringify!(ForumTopicReopened), &value);
    let parsed_value = must_to_value(stringify!(ForumTopicReopened), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ForumTopicReopened), &parsed);
}
#[test]
fn test_game_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "description" : "test", "photo" : [{ "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1 }] }
    );
    let parsed: Game = must_parse(stringify!(Game), &value);
    let parsed_value = must_to_value(stringify!(Game), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Game), &parsed);
}
#[test]
fn test_game_high_score_serialize_deserialize() {
    let value = serde_json::json!(
        { "position" : 1, "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "score" : 1 }
    );
    let parsed: GameHighScore = must_parse(stringify!(GameHighScore), &value);
    let parsed_value = must_to_value(stringify!(GameHighScore), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GameHighScore), &parsed);
}
#[test]
fn test_general_forum_topic_hidden_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: GeneralForumTopicHidden = must_parse(stringify!(GeneralForumTopicHidden), &value);
    let parsed_value = must_to_value(stringify!(GeneralForumTopicHidden), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GeneralForumTopicHidden), &parsed);
}
#[test]
fn test_general_forum_topic_unhidden_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: GeneralForumTopicUnhidden =
        must_parse(stringify!(GeneralForumTopicUnhidden), &value);
    let parsed_value = must_to_value(stringify!(GeneralForumTopicUnhidden), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GeneralForumTopicUnhidden), &parsed);
}
#[test]
fn test_gift_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "star_count" : 1 }
    );
    let parsed: Gift = must_parse(stringify!(Gift), &value);
    let parsed_value = must_to_value(stringify!(Gift), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Gift), &parsed);
}
#[test]
fn test_gift_background_serialize_deserialize() {
    let value = serde_json::json!(
        { "center_color" : 1, "edge_color" : 1, "text_color" : 1 }
    );
    let parsed: GiftBackground = must_parse(stringify!(GiftBackground), &value);
    let parsed_value = must_to_value(stringify!(GiftBackground), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiftBackground), &parsed);
}
#[test]
fn test_gift_info_serialize_deserialize() {
    let value = serde_json::json!(
        { "gift" : { "id" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "star_count" : 1 } }
    );
    let parsed: GiftInfo = must_parse(stringify!(GiftInfo), &value);
    let parsed_value = must_to_value(stringify!(GiftInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiftInfo), &parsed);
}
#[test]
fn test_gifts_serialize_deserialize() {
    let value = serde_json::json!(
        { "gifts" : [{ "id" : "test", "sticker" : { "type" : "regular", "file_id" :
        "used", "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" :
        true, "is_video" : true }, "star_count" : 1 }] }
    );
    let parsed: Gifts = must_parse(stringify!(Gifts), &value);
    let parsed_value = must_to_value(stringify!(Gifts), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Gifts), &parsed);
}
#[test]
fn test_giveaway_premium_serialize_deserialize() {
    let value = serde_json::json!(
        { "chats" : [{ "type" : "private", "id" : 1 }], "winners_selection_date" : 1,
        "winner_count" : 1, "premium_subscription_month_count" : 1 }
    );
    let parsed: Giveaway = must_parse(stringify!(Giveaway), &value);
    assert!(
        matches!(&parsed, Giveaway::Premium(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Giveaway),
        stringify!(Premium),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Giveaway), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Giveaway), &parsed);
}
#[test]
fn test_giveaway_star_serialize_deserialize() {
    let value = serde_json::json!(
        { "chats" : [{ "type" : "private", "id" : 1 }], "winners_selection_date" : 1,
        "winner_count" : 1, "prize_star_count" : 1 }
    );
    let parsed: Giveaway = must_parse(stringify!(Giveaway), &value);
    assert!(
        matches!(&parsed, Giveaway::Star(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Giveaway),
        stringify!(Star),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Giveaway), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Giveaway), &parsed);
}
#[test]
fn test_giveaway_completed_serialize_deserialize() {
    let value = serde_json::json!({ "winner_count" : 1 });
    let parsed: GiveawayCompleted = must_parse(stringify!(GiveawayCompleted), &value);
    let parsed_value = must_to_value(stringify!(GiveawayCompleted), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiveawayCompleted), &parsed);
}
#[test]
fn test_giveaway_created_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: GiveawayCreated = must_parse(stringify!(GiveawayCreated), &value);
    let parsed_value = must_to_value(stringify!(GiveawayCreated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiveawayCreated), &parsed);
}
#[test]
fn test_giveaway_winners_premium_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "giveaway_message_id" : 1,
        "winners_selection_date" : 1, "winner_count" : 1, "winners" : [{ "id" : 1,
        "is_bot" : true, "first_name" : "test" }], "premium_subscription_month_count" : 1
        }
    );
    let parsed: GiveawayWinners = must_parse(stringify!(GiveawayWinners), &value);
    assert!(
        matches!(&parsed, GiveawayWinners::Premium(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(GiveawayWinners),
        stringify!(Premium),
        parsed
    );
    let parsed_value = must_to_value(stringify!(GiveawayWinners), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiveawayWinners), &parsed);
}
#[test]
fn test_giveaway_winners_star_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "giveaway_message_id" : 1,
        "winners_selection_date" : 1, "winner_count" : 1, "winners" : [{ "id" : 1,
        "is_bot" : true, "first_name" : "test" }], "prize_star_count" : 1 }
    );
    let parsed: GiveawayWinners = must_parse(stringify!(GiveawayWinners), &value);
    assert!(
        matches!(&parsed, GiveawayWinners::Star(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(GiveawayWinners),
        stringify!(Star),
        parsed
    );
    let parsed_value = must_to_value(stringify!(GiveawayWinners), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(GiveawayWinners), &parsed);
}
#[test]
fn test_inaccessible_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "message_id" : 1, "date" : 1 }
    );
    let parsed: MaybeInaccessibleMessage = must_parse(stringify!(MaybeInaccessibleMessage), &value);
    assert!(
        matches!(&parsed, MaybeInaccessibleMessage::InaccessibleMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MaybeInaccessibleMessage),
        stringify!(InaccessibleMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MaybeInaccessibleMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MaybeInaccessibleMessage), &parsed);
}
#[test]
fn test_inline_keyboard_button_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test" });
    let parsed: InlineKeyboardButton = must_parse(stringify!(InlineKeyboardButton), &value);
    let parsed_value = must_to_value(stringify!(InlineKeyboardButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineKeyboardButton), &parsed);
}
#[test]
fn test_inline_keyboard_markup_serialize_deserialize() {
    let value = serde_json::json!({ "inline_keyboard" : [[{ "text" : "test" }]] });
    let parsed: ReplyMarkup = must_parse(stringify!(ReplyMarkup), &value);
    assert!(
        matches!(&parsed, ReplyMarkup::InlineKeyboardMarkup(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReplyMarkup),
        stringify!(InlineKeyboardMarkup),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReplyMarkup), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReplyMarkup), &parsed);
}
#[test]
fn test_inline_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "query" : "test", "offset" : "controlled" }
    );
    let parsed: InlineQuery = must_parse(stringify!(InlineQuery), &value);
    let parsed_value = must_to_value(stringify!(InlineQuery), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQuery), &parsed);
}
#[test]
fn test_inline_query_result_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "audio", "id" : "test", "audio_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_article_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "article", "id" : "test", "title" : "test", "input_message_content" :
        { "title" : "test", "description" : "test", "payload" : "test", "currency" :
        "test", "prices" : [{ "label" : "test", "amount" : 1 }] } }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Article(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Article),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "audio", "id" : "test", "audio_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResultAudio = must_parse(stringify!(InlineQueryResultAudio), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultAudio), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultAudio), &parsed);
}
#[test]
fn test_inline_query_result_audio_kind_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "audio", "id" : "test", "audio_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_cached_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "audio", "id" : "test", "audio_file_id" : "test" }
    );
    let parsed: InlineQueryResultCachedAudio =
        must_parse(stringify!(InlineQueryResultCachedAudio), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedAudio), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedAudio), &parsed);
}
#[test]
fn test_inline_query_result_cached_document_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "document", "id" : "test", "title" : "test", "document_file_id" :
        "test" }
    );
    let parsed: InlineQueryResultCachedDocument =
        must_parse(stringify!(InlineQueryResultCachedDocument), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedDocument), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedDocument), &parsed);
}
#[test]
fn test_inline_query_result_cached_gif_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "gif", "id" : "test", "gif_file_id" : "test" }
    );
    let parsed: InlineQueryResultCachedGif =
        must_parse(stringify!(InlineQueryResultCachedGif), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedGif), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedGif), &parsed);
}
#[test]
fn test_inline_query_result_cached_mpeg4_gif_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mpeg4_gif", "id" : "test", "mpeg4_file_id" : "test" }
    );
    let parsed: InlineQueryResultCachedMpeg4Gif =
        must_parse(stringify!(InlineQueryResultCachedMpeg4Gif), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedMpeg4Gif), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedMpeg4Gif), &parsed);
}
#[test]
fn test_inline_query_result_cached_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "photo", "id" : "test", "photo_file_id" : "test" }
    );
    let parsed: InlineQueryResultCachedPhoto =
        must_parse(stringify!(InlineQueryResultCachedPhoto), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedPhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedPhoto), &parsed);
}
#[test]
fn test_inline_query_result_cached_sticker_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "sticker", "id" : "test", "sticker_file_id" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Sticker(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Sticker),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_cached_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "video", "id" : "test", "video_file_id" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResultCachedVideo =
        must_parse(stringify!(InlineQueryResultCachedVideo), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedVideo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedVideo), &parsed);
}
#[test]
fn test_inline_query_result_cached_voice_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "voice", "id" : "test", "voice_file_id" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResultCachedVoice =
        must_parse(stringify!(InlineQueryResultCachedVoice), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultCachedVoice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultCachedVoice), &parsed);
}
#[test]
fn test_inline_query_result_contact_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "contact", "id" : "test", "phone_number" : "test", "first_name" :
        "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Contact(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Contact),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_document_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "document", "id" : "test", "title" : "test", "document_url" : "test",
        "mime_type" : "test" }
    );
    let parsed: InlineQueryResultDocument =
        must_parse(stringify!(InlineQueryResultDocument), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultDocument), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultDocument), &parsed);
}
#[test]
fn test_inline_query_result_document_kind_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "document", "id" : "test", "title" : "test", "document_url" : "test",
        "mime_type" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Document(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Document),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_game_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "game", "id" : "test", "game_short_name" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Game(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Game),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_gif_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "gif", "id" : "test", "gif_url" : "test", "thumbnail_url" : "test" }
    );
    let parsed: InlineQueryResultGif = must_parse(stringify!(InlineQueryResultGif), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultGif), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultGif), &parsed);
}
#[test]
fn test_inline_query_result_gif_kind_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "gif", "id" : "test", "gif_url" : "test", "thumbnail_url" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Gif(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Gif),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "location", "id" : "test", "latitude" : 1.25, "longitude" : 1.25,
        "title" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_mpeg4_gif_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mpeg4_gif", "id" : "test", "mpeg4_url" : "test", "thumbnail_url" :
        "test" }
    );
    let parsed: InlineQueryResultMpeg4Gif =
        must_parse(stringify!(InlineQueryResultMpeg4Gif), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultMpeg4Gif), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultMpeg4Gif), &parsed);
}
#[test]
fn test_inline_query_result_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "photo", "id" : "test", "photo_url" : "in", "thumbnail_url" : "test" }
    );
    let parsed: InlineQueryResultPhoto = must_parse(stringify!(InlineQueryResultPhoto), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultPhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultPhoto), &parsed);
}
#[test]
fn test_inline_query_result_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "venue", "id" : "test", "latitude" : 1.25, "longitude" : 1.25, "title"
        : "test", "address" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Venue(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Venue),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "video", "id" : "test", "video_url" : "test", "mime_type" : "test",
        "thumbnail_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResultVideo = must_parse(stringify!(InlineQueryResultVideo), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultVideo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultVideo), &parsed);
}
#[test]
fn test_inline_query_result_video_kind_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "video", "id" : "test", "video_url" : "test", "mime_type" : "test",
        "thumbnail_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_result_voice_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "voice", "id" : "test", "voice_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResultVoice = must_parse(stringify!(InlineQueryResultVoice), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultVoice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultVoice), &parsed);
}
#[test]
fn test_inline_query_result_voice_kind_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "voice", "id" : "test", "voice_url" : "test", "title" : "test" }
    );
    let parsed: InlineQueryResult = must_parse(stringify!(InlineQueryResult), &value);
    assert!(
        matches!(&parsed, InlineQueryResult::Voice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InlineQueryResult),
        stringify!(Voice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InlineQueryResult), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResult), &parsed);
}
#[test]
fn test_inline_query_results_button_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test" });
    let parsed: InlineQueryResultsButton = must_parse(stringify!(InlineQueryResultsButton), &value);
    let parsed_value = must_to_value(stringify!(InlineQueryResultsButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InlineQueryResultsButton), &parsed);
}
#[test]
fn test_input_checklist_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "tasks" : [{ "id" : 1, "text" : "test" }] }
    );
    let parsed: InputChecklist = must_parse(stringify!(InputChecklist), &value);
    let parsed_value = must_to_value(stringify!(InputChecklist), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputChecklist), &parsed);
}
#[test]
fn test_input_checklist_task_serialize_deserialize() {
    let value = serde_json::json!({ "id" : 1, "text" : "test" });
    let parsed: InputChecklistTask = must_parse(stringify!(InputChecklistTask), &value);
    let parsed_value = must_to_value(stringify!(InputChecklistTask), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputChecklistTask), &parsed);
}
#[test]
fn test_input_contact_message_content_serialize_deserialize() {
    let value = serde_json::json!({ "phone_number" : "test", "first_name" : "test" });
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputContactMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputContactMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_invoice_message_content_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "description" : "test", "payload" : "test", "currency" :
        "test", "prices" : [{ "label" : "test", "amount" : 1 }] }
    );
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputInvoiceMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputInvoiceMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_location_message_content_serialize_deserialize() {
    let value = serde_json::json!({ "latitude" : 1.25, "longitude" : 1.25 });
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputLocationMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputLocationMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_media_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "animation", "media" : "" });
    let parsed: InputMedia = must_parse(stringify!(InputMedia), &value);
    let parsed_value = must_to_value(stringify!(InputMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMedia), &parsed);
}
#[test]
fn test_input_media_animation_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "animation", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Animation(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Animation),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_audio_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "audio", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_document_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "document", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Document(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Document),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_link_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "link", "url" : "test" });
    let parsed: InputPollOptionMedia = must_parse(stringify!(InputPollOptionMedia), &value);
    assert!(
        matches!(&parsed, InputPollOptionMedia::Link(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollOptionMedia),
        stringify!(Link),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollOptionMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollOptionMedia), &parsed);
}
#[test]
fn test_input_media_live_photo_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "live_photo", "media" : "", "photo" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "location", "latitude" : 1.25, "longitude" : 1.25 }
    );
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_photo_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "photo", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_sticker_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "sticker", "media" : "" });
    let parsed: InputPollOptionMedia = must_parse(stringify!(InputPollOptionMedia), &value);
    assert!(
        matches!(&parsed, InputPollOptionMedia::Sticker(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollOptionMedia),
        stringify!(Sticker),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollOptionMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollOptionMedia), &parsed);
}
#[test]
fn test_input_media_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "venue", "latitude" : 1.25, "longitude" : 1.25, "title" : "test",
        "address" : "test" }
    );
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Venue(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Venue),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_media_video_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "video", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    assert!(
        matches!(&parsed, InputPollMedia::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPollMedia),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_message_content_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "description" : "test", "payload" : "test", "currency" :
        "test", "prices" : [{ "label" : "test", "amount" : 1 }] }
    );
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_paid_media_live_photo_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "live_photo", "media" : "", "photo" : "" });
    let parsed: InputPaidMedia = must_parse(stringify!(InputPaidMedia), &value);
    assert!(
        matches!(&parsed, InputPaidMedia::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPaidMedia),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPaidMedia), &parsed);
}
#[test]
fn test_input_paid_media_photo_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "photo", "media" : "" });
    let parsed: InputPaidMedia = must_parse(stringify!(InputPaidMedia), &value);
    assert!(
        matches!(&parsed, InputPaidMedia::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPaidMedia),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPaidMedia), &parsed);
}
#[test]
fn test_input_paid_media_video_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "video", "media" : "" });
    let parsed: InputPaidMedia = must_parse(stringify!(InputPaidMedia), &value);
    assert!(
        matches!(&parsed, InputPaidMedia::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputPaidMedia),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputPaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPaidMedia), &parsed);
}
#[test]
fn test_input_poll_media_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "animation", "media" : "" });
    let parsed: InputPollMedia = must_parse(stringify!(InputPollMedia), &value);
    let parsed_value = must_to_value(stringify!(InputPollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollMedia), &parsed);
}
#[test]
fn test_input_poll_option_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test" });
    let parsed: InputPollOption = must_parse(stringify!(InputPollOption), &value);
    let parsed_value = must_to_value(stringify!(InputPollOption), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollOption), &parsed);
}
#[test]
fn test_input_poll_option_media_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "animation", "media" : "" });
    let parsed: InputPollOptionMedia = must_parse(stringify!(InputPollOptionMedia), &value);
    let parsed_value = must_to_value(stringify!(InputPollOptionMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputPollOptionMedia), &parsed);
}
#[test]
fn test_input_profile_photo_static_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "static", "photo" : "" });
    let parsed: InputProfilePhoto = must_parse(stringify!(InputProfilePhoto), &value);
    assert!(
        matches!(&parsed, InputProfilePhoto::Static(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputProfilePhoto),
        stringify!(Static),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputProfilePhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputProfilePhoto), &parsed);
}
#[test]
fn test_input_profile_photo_animated_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "animated", "animation" : "" });
    let parsed: InputProfilePhoto = must_parse(stringify!(InputProfilePhoto), &value);
    assert!(
        matches!(&parsed, InputProfilePhoto::Animated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputProfilePhoto),
        stringify!(Animated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputProfilePhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputProfilePhoto), &parsed);
}
#[test]
fn test_input_rich_message_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: InputRichMessage = must_parse(stringify!(InputRichMessage), &value);
    let parsed_value = must_to_value(stringify!(InputRichMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputRichMessage), &parsed);
}
#[test]
fn test_input_rich_message_content_serialize_deserialize() {
    let value = serde_json::json!({ "rich_message" : {} });
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputRichMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputRichMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_sticker_serialize_deserialize() {
    let value = serde_json::json!(
        { "sticker" : "", "format" : "static", "emoji_list" : ["test"] }
    );
    let parsed: InputSticker = must_parse(stringify!(InputSticker), &value);
    let parsed_value = must_to_value(stringify!(InputSticker), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputSticker), &parsed);
}
#[test]
fn test_input_story_content_photo_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "photo", "photo" : "" });
    let parsed: InputStoryContent = must_parse(stringify!(InputStoryContent), &value);
    assert!(
        matches!(&parsed, InputStoryContent::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputStoryContent),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputStoryContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputStoryContent), &parsed);
}
#[test]
fn test_input_story_content_video_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "video", "video" : "" });
    let parsed: InputStoryContent = must_parse(stringify!(InputStoryContent), &value);
    assert!(
        matches!(&parsed, InputStoryContent::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputStoryContent),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputStoryContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputStoryContent), &parsed);
}
#[test]
fn test_input_text_message_content_serialize_deserialize() {
    let value = serde_json::json!({ "message_text" : "test" });
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputTextMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputTextMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_input_venue_message_content_serialize_deserialize() {
    let value = serde_json::json!(
        { "latitude" : 1.25, "longitude" : 1.25, "title" : "test", "address" : "test" }
    );
    let parsed: InputMessageContent = must_parse(stringify!(InputMessageContent), &value);
    assert!(
        matches!(&parsed, InputMessageContent::InputVenueMessageContent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(InputMessageContent),
        stringify!(InputVenueMessageContent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(InputMessageContent), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(InputMessageContent), &parsed);
}
#[test]
fn test_invoice_serialize_deserialize() {
    let value = serde_json::json!(
        { "title" : "test", "description" : "test", "start_parameter" : "used",
        "currency" : "test", "total_amount" : 1 }
    );
    let parsed: Invoice = must_parse(stringify!(Invoice), &value);
    let parsed_value = must_to_value(stringify!(Invoice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Invoice), &parsed);
}
#[test]
fn test_keyboard_button_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "the" });
    let parsed: KeyboardButton = must_parse(stringify!(KeyboardButton), &value);
    let parsed_value = must_to_value(stringify!(KeyboardButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(KeyboardButton), &parsed);
}
#[test]
fn test_keyboard_button_poll_type_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: KeyboardButtonPollType = must_parse(stringify!(KeyboardButtonPollType), &value);
    let parsed_value = must_to_value(stringify!(KeyboardButtonPollType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(KeyboardButtonPollType), &parsed);
}
#[test]
fn test_keyboard_button_request_chat_serialize_deserialize() {
    let value = serde_json::json!({ "request_id" : 1, "chat_is_channel" : true });
    let parsed: KeyboardButtonRequestChat =
        must_parse(stringify!(KeyboardButtonRequestChat), &value);
    let parsed_value = must_to_value(stringify!(KeyboardButtonRequestChat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(KeyboardButtonRequestChat), &parsed);
}
#[test]
fn test_keyboard_button_request_managed_bot_serialize_deserialize() {
    let value = serde_json::json!({ "request_id" : 1 });
    let parsed: KeyboardButtonRequestManagedBot =
        must_parse(stringify!(KeyboardButtonRequestManagedBot), &value);
    let parsed_value = must_to_value(stringify!(KeyboardButtonRequestManagedBot), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(KeyboardButtonRequestManagedBot), &parsed);
}
#[test]
fn test_keyboard_button_request_users_serialize_deserialize() {
    let value = serde_json::json!({ "request_id" : 1 });
    let parsed: KeyboardButtonRequestUsers =
        must_parse(stringify!(KeyboardButtonRequestUsers), &value);
    let parsed_value = must_to_value(stringify!(KeyboardButtonRequestUsers), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(KeyboardButtonRequestUsers), &parsed);
}
#[test]
fn test_labeled_price_serialize_deserialize() {
    let value = serde_json::json!({ "label" : "test", "amount" : 1 });
    let parsed: LabeledPrice = must_parse(stringify!(LabeledPrice), &value);
    let parsed_value = must_to_value(stringify!(LabeledPrice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(LabeledPrice), &parsed);
}
#[test]
fn test_link_serialize_deserialize() {
    let value = serde_json::json!({ "url" : "test" });
    let parsed: Link = must_parse(stringify!(Link), &value);
    let parsed_value = must_to_value(stringify!(Link), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Link), &parsed);
}
#[test]
fn test_link_preview_options_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: LinkPreviewOptions = must_parse(stringify!(LinkPreviewOptions), &value);
    let parsed_value = must_to_value(stringify!(LinkPreviewOptions), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(LinkPreviewOptions), &parsed);
}
#[test]
fn test_live_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "duration" : 1 }
    );
    let parsed: LivePhoto = must_parse(stringify!(LivePhoto), &value);
    let parsed_value = must_to_value(stringify!(LivePhoto), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(LivePhoto), &parsed);
}
#[test]
fn test_location_serialize_deserialize() {
    let value = serde_json::json!({ "latitude" : 1.25, "longitude" : 1.25 });
    let parsed: Location = must_parse(stringify!(Location), &value);
    let parsed_value = must_to_value(stringify!(Location), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Location), &parsed);
}
#[test]
fn test_location_address_serialize_deserialize() {
    let value = serde_json::json!({ "country_code" : "test" });
    let parsed: LocationAddress = must_parse(stringify!(LocationAddress), &value);
    let parsed_value = must_to_value(stringify!(LocationAddress), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(LocationAddress), &parsed);
}
#[test]
fn test_login_url_serialize_deserialize() {
    let value = serde_json::json!({ "url" : "check" });
    let parsed: LoginUrl = must_parse(stringify!(LoginUrl), &value);
    let parsed_value = must_to_value(stringify!(LoginUrl), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(LoginUrl), &parsed);
}
#[test]
fn test_managed_bot_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "bot" : { "id" : 1, "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: ManagedBotCreated = must_parse(stringify!(ManagedBotCreated), &value);
    let parsed_value = must_to_value(stringify!(ManagedBotCreated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ManagedBotCreated), &parsed);
}
#[test]
fn test_managed_bot_updated_serialize_deserialize() {
    let value = serde_json::json!(
        { "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" }, "bot" : { "id" :
        1, "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: ManagedBotUpdated = must_parse(stringify!(ManagedBotUpdated), &value);
    let parsed_value = must_to_value(stringify!(ManagedBotUpdated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ManagedBotUpdated), &parsed);
}
#[test]
fn test_mask_position_serialize_deserialize() {
    let value = serde_json::json!(
        { "point" : "forehead", "x_shift" : 1.25, "y_shift" : 1.25, "scale" : 1.25 }
    );
    let parsed: MaskPosition = must_parse(stringify!(MaskPosition), &value);
    let parsed_value = must_to_value(stringify!(MaskPosition), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MaskPosition), &parsed);
}
#[test]
fn test_maybe_inaccessible_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "message_id" : 1, "date" : 1 }
    );
    let parsed: MaybeInaccessibleMessage = must_parse(stringify!(MaybeInaccessibleMessage), &value);
    let parsed_value = must_to_value(stringify!(MaybeInaccessibleMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MaybeInaccessibleMessage), &parsed);
}
#[test]
fn test_menu_button_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "commands" });
    let parsed: MenuButton = must_parse(stringify!(MenuButton), &value);
    let parsed_value = must_to_value(stringify!(MenuButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MenuButton), &parsed);
}
#[test]
fn test_menu_button_commands_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "commands" });
    let parsed: MenuButton = must_parse(stringify!(MenuButton), &value);
    assert!(
        matches!(&parsed, MenuButton::Commands(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MenuButton),
        stringify!(Commands),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MenuButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MenuButton), &parsed);
}
#[test]
fn test_menu_button_default_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "default" });
    let parsed: MenuButton = must_parse(stringify!(MenuButton), &value);
    assert!(
        matches!(&parsed, MenuButton::Default(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MenuButton),
        stringify!(Default),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MenuButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MenuButton), &parsed);
}
#[test]
fn test_menu_button_web_app_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "web_app", "text" : "test", "web_app" : { "url" : "test" } }
    );
    let parsed: MenuButton = must_parse(stringify!(MenuButton), &value);
    assert!(
        matches!(&parsed, MenuButton::WebApp(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MenuButton),
        stringify!(WebApp),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MenuButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MenuButton), &parsed);
}
#[test]
fn test_message_animation_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "animation" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Animation(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Animation),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "audio" : { "file_id" : "used", "file_unique_id" : "test", "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_boost_added_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "boost_added" : { "boost_count" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::BoostAdded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(BoostAdded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_channel_chat_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "channel_chat_created" : true }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChannelChatCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChannelChatCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_chat_background_set_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "chat_background_set" : { "type" : { "type" : "fill", "fill" : { "type" :
        "solid", "color" : 1 }, "dark_theme_dimming" : 1 } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChatBackgroundSet(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChatBackgroundSet),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_chat_owner_changed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "chat_owner_changed" : { "new_owner" : { "id" : 1, "is_bot" : true, "first_name"
        : "test" } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChatOwnerChanged(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChatOwnerChanged),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_chat_owner_left_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "chat_owner_left" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChatOwnerLeft(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChatOwnerLeft),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_chat_shared_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "chat_shared" : { "request_id" : 1, "chat_id" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChatShared(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChatShared),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_checklist_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "checklist" : { "title" : "test", "tasks" : [{ "id" : 1, "text" : "test" }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Checklist(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Checklist),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_checklist_tasks_added_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "checklist_tasks_added" : { "tasks" : [{ "id" : 1, "text" : "test" }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChecklistTasksAdded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChecklistTasksAdded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_checklist_tasks_done_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "checklist_tasks_done" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ChecklistTasksDone(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ChecklistTasksDone),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_connected_website_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "connected_website" : "test" }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ConnectedWebsite(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ConnectedWebsite),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_contact_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "contact" : { "phone_number" : "test", "first_name" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Contact(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Contact),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_delete_chat_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "delete_chat_photo" : true }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::DeleteChatPhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(DeleteChatPhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_dice_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 }, "dice"
        : { "emoji" : "test", "value" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Dice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Dice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_direct_message_price_changed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "direct_message_price_changed" : { "are_direct_messages_enabled" : true } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::DirectMessagePriceChanged(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(DirectMessagePriceChanged),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_document_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "document" : { "file_id" : "used", "file_unique_id" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Document(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Document),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_forum_topic_closed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "forum_topic_closed" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ForumTopicClosed(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ForumTopicClosed),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_forum_topic_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "forum_topic_created" : { "name" : "test", "icon_color" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ForumTopicCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ForumTopicCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_forum_topic_edited_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "forum_topic_edited" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ForumTopicEdited(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ForumTopicEdited),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_forum_topic_reopened_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "forum_topic_reopened" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ForumTopicReopened(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ForumTopicReopened),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_game_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 }, "game"
        : { "title" : "test", "description" : "test", "photo" : [{ "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1 }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Game(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Game),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_general_forum_topic_hidden_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "general_forum_topic_hidden" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GeneralForumTopicHidden(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GeneralForumTopicHidden),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_general_forum_topic_unhidden_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "general_forum_topic_unhidden" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GeneralForumTopicUnhidden(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GeneralForumTopicUnhidden),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_gift_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 }, "gift"
        : { "gift" : { "id" : "test", "sticker" : { "type" : "regular", "file_id" :
        "used", "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" :
        true, "is_video" : true }, "star_count" : 1 } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Gift(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Gift),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_gift_upgrade_sent_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "gift_upgrade_sent" : { "gift" : { "id" : "test", "sticker" : { "type" :
        "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" :
        1, "is_animated" : true, "is_video" : true }, "star_count" : 1 } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GiftUpgradeSent(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GiftUpgradeSent),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_giveaway_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "giveaway" : { "chats" : [{ "type" : "private", "id" : 1 }],
        "winners_selection_date" : 1, "winner_count" : 1,
        "premium_subscription_month_count" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Giveaway(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Giveaway),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_giveaway_completed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "giveaway_completed" : { "winner_count" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GiveawayCompleted(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GiveawayCompleted),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_giveaway_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "giveaway_created" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GiveawayCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GiveawayCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_giveaway_winners_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "giveaway_winners" : { "chat" : { "type" : "private", "id" : 1 },
        "giveaway_message_id" : 1, "winners_selection_date" : 1, "winner_count" : 1,
        "winners" : [{ "id" : 1, "is_bot" : true, "first_name" : "test" }],
        "premium_subscription_month_count" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GiveawayWinners(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GiveawayWinners),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_group_chat_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "group_chat_created" : true }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::GroupChatCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(GroupChatCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_invoice_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "invoice" : { "title" : "test", "description" : "test", "start_parameter" :
        "used", "currency" : "test", "total_amount" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Invoice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Invoice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_left_chat_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "left_chat_member" : { "id" : 1, "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::LeftChatMember(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(LeftChatMember),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_live_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "live_photo" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "location" : { "latitude" : 1.25, "longitude" : 1.25 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_managed_bot_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "managed_bot_created" : { "bot" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ManagedBotCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ManagedBotCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_message_auto_delete_timer_changed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "message_auto_delete_timer_changed" : { "message_auto_delete_time" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::MessageAutoDeleteTimerChanged(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(MessageAutoDeleteTimerChanged),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_migrate_from_chat_id_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "migrate_from_chat_id" : 1 }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::MigrateFromChatId(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(MigrateFromChatId),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_migrate_to_chat_id_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "migrate_to_chat_id" : 1 }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::MigrateToChatId(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(MigrateToChatId),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_new_chat_members_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "new_chat_members" : [{ "id" : 1, "is_bot" : true, "first_name" : "test" }] }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::NewChatMembers(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(NewChatMembers),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_new_chat_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "new_chat_photo" : [{ "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1 }] }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::NewChatPhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(NewChatPhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_new_chat_title_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "new_chat_title" : "test" }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::NewChatTitle(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(NewChatTitle),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_paid_media_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "paid_media" : { "star_count" : 1, "paid_media" : [{ "type" : "live_photo",
        "live_photo" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PaidMedia(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PaidMedia),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_paid_message_price_changed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "paid_message_price_changed" : { "paid_message_star_count" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PaidMessagePriceChanged(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PaidMessagePriceChanged),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_passport_data_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "passport_data" : { "data" : [{ "type" : "personal_details", "data" :
        "decrypted", "hash" : "test" }], "credentials" : { "data" : "test", "hash" :
        "test", "secret" : "test" } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PassportData(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PassportData),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "photo" : [{ "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height"
        : 1 }] }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_pinned_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "pinned_message" : { "chat" : { "type" : "private", "id" : 1 }, "message_id" : 1,
        "date" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PinnedMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PinnedMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_poll_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 }, "poll"
        : { "type" : "regular", "id" : "test", "question" : "test", "options" : [{
        "persistent_id" : "test", "text" : "test", "voter_count" : 1 }],
        "total_voter_count" : 1, "is_closed" : true, "is_anonymous" : true,
        "allows_multiple_answers" : true, "allows_revoting" : true, "members_only" : true
        } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Poll(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Poll),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_poll_option_added_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "poll_option_added" : { "option_persistent_id" : "test", "option_text" : "test" }
        }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PollOptionAdded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PollOptionAdded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_poll_option_deleted_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "poll_option_deleted" : { "option_persistent_id" : "test", "option_text" : "test"
        } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::PollOptionDeleted(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(PollOptionDeleted),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_proximity_alert_triggered_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "proximity_alert_triggered" : { "traveler" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" }, "watcher" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" }, "distance" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::ProximityAlertTriggered(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(ProximityAlertTriggered),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_refunded_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "refunded_payment" : { "currency" : "xtr", "total_amount" : 1, "invoice_payload"
        : "test", "telegram_payment_charge_id" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::RefundedPayment(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(RefundedPayment),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_sticker_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "sticker" : { "type" : "regular", "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1, "is_animated" : true, "is_video" : true } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Sticker(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Sticker),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_story_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "story" : { "chat" : { "type" : "private", "id" : 1 }, "id" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Story(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Story),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_successful_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "successful_payment" : { "currency" : "test", "total_amount" : 1,
        "invoice_payload" : "test", "telegram_payment_charge_id" : "test",
        "provider_payment_charge_id" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuccessfulPayment(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuccessfulPayment),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_suggested_post_approval_failed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "suggested_post_approval_failed" : { "price" : { "currency" : "xtr", "amount" : 1
        } } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuggestedPostApprovalFailed(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuggestedPostApprovalFailed),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_suggested_post_approved_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "suggested_post_approved" : { "send_date" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuggestedPostApproved(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuggestedPostApproved),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_suggested_post_declined_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "suggested_post_declined" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuggestedPostDeclined(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuggestedPostDeclined),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_suggested_post_paid_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "suggested_post_paid" : { "currency" : "xtr" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuggestedPostPaid(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuggestedPostPaid),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_suggested_post_refunded_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "suggested_post_refunded" : { "reason" : "post_deleted" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SuggestedPostRefunded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SuggestedPostRefunded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_supergroup_chat_created_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "supergroup_chat_created" : true }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::SupergroupChatCreated(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(SupergroupChatCreated),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_text_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 }, "text"
        : "test" }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Text(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Text),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_unique_gift_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "unique_gift" : { "gift" : { "gift_id" : "test", "base_name" : "test", "name" :
        "used", "number" : 1, "model" : { "name" : "test", "sticker" : { "type" :
        "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" :
        1, "is_animated" : true, "is_video" : true }, "rarity_per_mille" : 1 }, "symbol"
        : { "name" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "rarity_per_mille" : 1 }, "backdrop" : { "name" : "test",
        "colors" : { "center_color" : 1, "edge_color" : 1, "symbol_color" : 1,
        "text_color" : 1 }, "rarity_per_mille" : 1 } }, "origin" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::UniqueGift(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(UniqueGift),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_users_shared_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "users_shared" : { "request_id" : 1, "users" : [{ "user_id" : 1 }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::UsersShared(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(UsersShared),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "venue" : { "location" : { "latitude" : 1.25, "longitude" : 1.25 }, "title" :
        "test", "address" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Venue(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Venue),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height"
        : 1, "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_chat_ended_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video_chat_ended" : { "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::VideoChatEnded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(VideoChatEnded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_chat_participants_invited_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video_chat_participants_invited" : { "users" : [{ "id" : 1, "is_bot" : true,
        "first_name" : "test" }] } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::VideoChatParticipantsInvited(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(VideoChatParticipantsInvited),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_chat_scheduled_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video_chat_scheduled" : { "start_date" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::VideoChatScheduled(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(VideoChatScheduled),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_chat_started_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video_chat_started" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::VideoChatStarted(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(VideoChatStarted),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_video_note_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "video_note" : { "file_id" : "used", "file_unique_id" : "test", "length" : 1,
        "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::VideoNote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(VideoNote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_voice_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "voice" : { "file_id" : "used", "file_unique_id" : "test", "duration" : 1 } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::Voice(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(Voice),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_web_app_data_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "web_app_data" : { "data" : "test", "button_text" : "test" } }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::WebAppData(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(WebAppData),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_write_access_allowed_serialize_deserialize() {
    let value = serde_json::json!(
        { "message_id" : 1, "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "write_access_allowed" : {} }
    );
    let parsed: Message = must_parse(stringify!(Message), &value);
    assert!(
        matches!(&parsed, Message::WriteAccessAllowed(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Message),
        stringify!(WriteAccessAllowed),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Message), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Message), &parsed);
}
#[test]
fn test_message_auto_delete_timer_changed_serialize_deserialize() {
    let value = serde_json::json!({ "message_auto_delete_time" : 1 });
    let parsed: MessageAutoDeleteTimerChanged =
        must_parse(stringify!(MessageAutoDeleteTimerChanged), &value);
    let parsed_value = must_to_value(stringify!(MessageAutoDeleteTimerChanged), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageAutoDeleteTimerChanged), &parsed);
}
#[test]
fn test_message_entity_blockquote_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "blockquote", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Blockquote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Blockquote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_bold_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "bold", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Bold(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Bold),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_bot_command_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "bot_command", "offset" : 1, "length" : 1 }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::BotCommand(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(BotCommand),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_cashtag_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "cashtag", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Cashtag(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Cashtag),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_code_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "code", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Code(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Code),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_custom_emoji_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "custom_emoji", "offset" : 1, "length" : 1, "custom_emoji_id" : "test"
        }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::CustomEmoji(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(CustomEmoji),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_date_time_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "date_time", "offset" : 1, "length" : 1, "unix_time" : 1 }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::DateTime(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(DateTime),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_email_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "email", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Email(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Email),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_expandable_blockquote_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "expandable_blockquote", "offset" : 1, "length" : 1 }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::ExpandableBlockquote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(ExpandableBlockquote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_hashtag_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "hashtag", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Hashtag(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Hashtag),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_italic_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "italic", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Italic(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Italic),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_mention_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "mention", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Mention(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Mention),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_phone_number_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "phone_number", "offset" : 1, "length" : 1 }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::PhoneNumber(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(PhoneNumber),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_pre_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "pre", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Pre(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Pre),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_spoiler_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "spoiler", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Spoiler(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Spoiler),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_strikethrough_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "strikethrough", "offset" : 1, "length" : 1 }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Strikethrough(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Strikethrough),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_text_link_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "text_link", "offset" : 1, "length" : 1, "url" : "test" }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::TextLink(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(TextLink),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_text_mention_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "text_mention", "offset" : 1, "length" : 1, "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::TextMention(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(TextMention),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_underline_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "underline", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Underline(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Underline),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_entity_url_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "url", "offset" : 1, "length" : 1 });
    let parsed: MessageEntity = must_parse(stringify!(MessageEntity), &value);
    assert!(
        matches!(&parsed, MessageEntity::Url(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageEntity),
        stringify!(Url),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageEntity), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageEntity), &parsed);
}
#[test]
fn test_message_id_serialize_deserialize() {
    let value = serde_json::json!({ "message_id" : 1 });
    let parsed: MessageId = must_parse(stringify!(MessageId), &value);
    let parsed_value = must_to_value(stringify!(MessageId), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageId), &parsed);
}
#[test]
fn test_message_origin_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } }
    );
    let parsed: MessageOrigin = must_parse(stringify!(MessageOrigin), &value);
    let parsed_value = must_to_value(stringify!(MessageOrigin), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageOrigin), &parsed);
}
#[test]
fn test_message_origin_channel_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "channel", "date" : 1, "chat" : { "type" : "private", "id" : 1 },
        "message_id" : 1 }
    );
    let parsed: MessageOrigin = must_parse(stringify!(MessageOrigin), &value);
    assert!(
        matches!(&parsed, MessageOrigin::Channel(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageOrigin),
        stringify!(Channel),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageOrigin), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageOrigin), &parsed);
}
#[test]
fn test_message_origin_chat_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "chat", "date" : 1, "sender_chat" : { "type" : "private", "id" : 1 } }
    );
    let parsed: MessageOrigin = must_parse(stringify!(MessageOrigin), &value);
    assert!(
        matches!(&parsed, MessageOrigin::Chat(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageOrigin),
        stringify!(Chat),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageOrigin), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageOrigin), &parsed);
}
#[test]
fn test_message_origin_hidden_user_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "hidden_user", "date" : 1, "sender_user_name" : "test" }
    );
    let parsed: MessageOrigin = must_parse(stringify!(MessageOrigin), &value);
    assert!(
        matches!(&parsed, MessageOrigin::HiddenUser(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageOrigin),
        stringify!(HiddenUser),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageOrigin), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageOrigin), &parsed);
}
#[test]
fn test_message_origin_user_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "user", "date" : 1, "sender_user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } }
    );
    let parsed: MessageOrigin = must_parse(stringify!(MessageOrigin), &value);
    assert!(
        matches!(&parsed, MessageOrigin::User(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(MessageOrigin),
        stringify!(User),
        parsed
    );
    let parsed_value = must_to_value(stringify!(MessageOrigin), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageOrigin), &parsed);
}
#[test]
fn test_message_reaction_count_updated_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "message_id" : 1, "date" : 1,
        "reactions" : [{ "type" : { "type" : "emoji", "emoji" : "test" }, "total_count" :
        1 }] }
    );
    let parsed: MessageReactionCountUpdated =
        must_parse(stringify!(MessageReactionCountUpdated), &value);
    let parsed_value = must_to_value(stringify!(MessageReactionCountUpdated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageReactionCountUpdated), &parsed);
}
#[test]
fn test_message_reaction_updated_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "message_id" : 1, "date" : 1,
        "old_reaction" : [{ "type" : "emoji", "emoji" : "test" }], "new_reaction" : [{
        "type" : "emoji", "emoji" : "test" }] }
    );
    let parsed: MessageReactionUpdated = must_parse(stringify!(MessageReactionUpdated), &value);
    let parsed_value = must_to_value(stringify!(MessageReactionUpdated), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(MessageReactionUpdated), &parsed);
}
#[test]
fn test_order_info_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: OrderInfo = must_parse(stringify!(OrderInfo), &value);
    let parsed_value = must_to_value(stringify!(OrderInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(OrderInfo), &parsed);
}
#[test]
fn test_owned_gift_regular_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "regular", "gift" : { "id" : "test", "sticker" : { "type" : "regular",
        "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "is_animated" : true, "is_video" : true }, "star_count" : 1 }, "send_date" : 1 }
    );
    let parsed: OwnedGift = must_parse(stringify!(OwnedGift), &value);
    assert!(
        matches!(&parsed, OwnedGift::Regular(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(OwnedGift),
        stringify!(Regular),
        parsed
    );
    let parsed_value = must_to_value(stringify!(OwnedGift), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(OwnedGift), &parsed);
}
#[test]
fn test_owned_gift_unique_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "unique", "gift" : { "gift_id" : "test", "base_name" : "test", "name"
        : "used", "number" : 1, "model" : { "name" : "test", "sticker" : { "type" :
        "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" :
        1, "is_animated" : true, "is_video" : true }, "rarity_per_mille" : 1 }, "symbol"
        : { "name" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "rarity_per_mille" : 1 }, "backdrop" : { "name" : "test",
        "colors" : { "center_color" : 1, "edge_color" : 1, "symbol_color" : 1,
        "text_color" : 1 }, "rarity_per_mille" : 1 } }, "send_date" : 1 }
    );
    let parsed: OwnedGift = must_parse(stringify!(OwnedGift), &value);
    assert!(
        matches!(&parsed, OwnedGift::Unique(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(OwnedGift),
        stringify!(Unique),
        parsed
    );
    let parsed_value = must_to_value(stringify!(OwnedGift), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(OwnedGift), &parsed);
}
#[test]
fn test_owned_gifts_serialize_deserialize() {
    let value = serde_json::json!(
        { "total_count" : 1, "gifts" : [{ "type" : "regular", "gift" : { "id" : "test",
        "sticker" : { "type" : "regular", "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1, "is_animated" : true, "is_video" : true },
        "star_count" : 1 }, "send_date" : 1 }] }
    );
    let parsed: OwnedGifts = must_parse(stringify!(OwnedGifts), &value);
    let parsed_value = must_to_value(stringify!(OwnedGifts), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(OwnedGifts), &parsed);
}
#[test]
fn test_paid_media_live_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "live_photo", "live_photo" : { "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: PaidMedia = must_parse(stringify!(PaidMedia), &value);
    assert!(
        matches!(&parsed, PaidMedia::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PaidMedia),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMedia), &parsed);
}
#[test]
fn test_paid_media_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "photo", "photo" : [{ "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1 }] }
    );
    let parsed: PaidMedia = must_parse(stringify!(PaidMedia), &value);
    assert!(
        matches!(&parsed, PaidMedia::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PaidMedia),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMedia), &parsed);
}
#[test]
fn test_paid_media_preview_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "preview" });
    let parsed: PaidMedia = must_parse(stringify!(PaidMedia), &value);
    assert!(
        matches!(&parsed, PaidMedia::Preview(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PaidMedia),
        stringify!(Preview),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMedia), &parsed);
}
#[test]
fn test_paid_media_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "video", "video" : { "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: PaidMedia = must_parse(stringify!(PaidMedia), &value);
    assert!(
        matches!(&parsed, PaidMedia::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PaidMedia),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PaidMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMedia), &parsed);
}
#[test]
fn test_paid_media_info_serialize_deserialize() {
    let value = serde_json::json!(
        { "star_count" : 1, "paid_media" : [{ "type" : "live_photo", "live_photo" : {
        "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "duration" : 1 } }] }
    );
    let parsed: PaidMediaInfo = must_parse(stringify!(PaidMediaInfo), &value);
    let parsed_value = must_to_value(stringify!(PaidMediaInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMediaInfo), &parsed);
}
#[test]
fn test_paid_media_purchased_serialize_deserialize() {
    let value = serde_json::json!(
        { "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "paid_media_payload" : "test" }
    );
    let parsed: PaidMediaPurchased = must_parse(stringify!(PaidMediaPurchased), &value);
    let parsed_value = must_to_value(stringify!(PaidMediaPurchased), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMediaPurchased), &parsed);
}
#[test]
fn test_paid_message_price_changed_serialize_deserialize() {
    let value = serde_json::json!({ "paid_message_star_count" : 1 });
    let parsed: PaidMessagePriceChanged = must_parse(stringify!(PaidMessagePriceChanged), &value);
    let parsed_value = must_to_value(stringify!(PaidMessagePriceChanged), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PaidMessagePriceChanged), &parsed);
}
#[test]
fn test_passport_data_serialize_deserialize() {
    let value = serde_json::json!(
        { "data" : [{ "type" : "personal_details", "data" : "decrypted", "hash" : "test"
        }], "credentials" : { "data" : "test", "hash" : "test", "secret" : "test" } }
    );
    let parsed: PassportData = must_parse(stringify!(PassportData), &value);
    let parsed_value = must_to_value(stringify!(PassportData), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportData), &parsed);
}
#[test]
fn test_passport_element_error_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "data", "type" : "personal_details", "field_name" : "test",
        "data_hash" : "test", "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_data_field_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "data", "type" : "personal_details", "field_name" : "test",
        "data_hash" : "test", "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::Data(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(Data),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_file_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "file", "type" : "utility_bill", "file_hash" : "test", "message" :
        "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::File(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(File),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_files_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "files", "type" : "utility_bill", "file_hashes" : ["test"],
        "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::Files(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(Files),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_front_side_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "front_side", "type" : "passport", "file_hash" : "test", "message" :
        "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::FrontSide(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(FrontSide),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_reverse_side_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "reverse_side", "type" : "driver_license", "file_hash" : "test",
        "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::ReverseSide(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(ReverseSide),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_selfie_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "selfie", "type" : "passport", "file_hash" : "test", "message" :
        "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::Selfie(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(Selfie),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_translation_file_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "translation_file", "type" : "passport", "file_hash" : "test",
        "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::TranslationFile(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(TranslationFile),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_translation_files_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "translation_files", "type" : "passport", "file_hashes" : ["test"],
        "message" : "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::TranslationFiles(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(TranslationFiles),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_element_error_unspecified_serialize_deserialize() {
    let value = serde_json::json!(
        { "source" : "unspecified", "type" : "test", "element_hash" : "test", "message" :
        "test" }
    );
    let parsed: PassportElementError = must_parse(stringify!(PassportElementError), &value);
    assert!(
        matches!(&parsed, PassportElementError::Unspecified(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PassportElementError),
        stringify!(Unspecified),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PassportElementError), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportElementError), &parsed);
}
#[test]
fn test_passport_file_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "file_size" : 1, "file_date" : 1
        }
    );
    let parsed: PassportFile = must_parse(stringify!(PassportFile), &value);
    let parsed_value = must_to_value(stringify!(PassportFile), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PassportFile), &parsed);
}
#[test]
fn test_photo_size_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1 }
    );
    let parsed: PhotoSize = must_parse(stringify!(PhotoSize), &value);
    let parsed_value = must_to_value(stringify!(PhotoSize), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PhotoSize), &parsed);
}
#[test]
fn test_poll_quiz_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "quiz", "id" : "test", "question" : "test", "options" : [{
        "persistent_id" : "test", "text" : "test", "voter_count" : 1 }],
        "total_voter_count" : 1, "is_closed" : true, "is_anonymous" : true,
        "allows_multiple_answers" : true, "allows_revoting" : true, "members_only" : true
        }
    );
    let parsed: Poll = must_parse(stringify!(Poll), &value);
    assert!(
        matches!(&parsed, Poll::Quiz(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Poll),
        stringify!(Quiz),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Poll), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Poll), &parsed);
}
#[test]
fn test_poll_regular_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "regular", "id" : "test", "question" : "test", "options" : [{
        "persistent_id" : "test", "text" : "test", "voter_count" : 1 }],
        "total_voter_count" : 1, "is_closed" : true, "is_anonymous" : true,
        "allows_multiple_answers" : true, "allows_revoting" : true, "members_only" : true
        }
    );
    let parsed: Poll = must_parse(stringify!(Poll), &value);
    assert!(
        matches!(&parsed, Poll::Regular(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Poll),
        stringify!(Regular),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Poll), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Poll), &parsed);
}
#[test]
fn test_poll_answer_serialize_deserialize() {
    let value = serde_json::json!(
        { "poll_id" : "test", "option_ids" : [1], "option_persistent_ids" : ["test"] }
    );
    let parsed: PollAnswer = must_parse(stringify!(PollAnswer), &value);
    let parsed_value = must_to_value(stringify!(PollAnswer), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollAnswer), &parsed);
}
#[test]
fn test_poll_media_animation_serialize_deserialize() {
    let value = serde_json::json!(
        { "animation" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Animation(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Animation),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "audio" : { "file_id" : "used", "file_unique_id" : "test", "duration" : 1 } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_document_serialize_deserialize() {
    let value = serde_json::json!(
        { "document" : { "file_id" : "used", "file_unique_id" : "test" } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Document(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Document),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_link_serialize_deserialize() {
    let value = serde_json::json!({ "link" : { "url" : "test" } });
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Link(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Link),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_live_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "live_photo" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::LivePhoto(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(LivePhoto),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "location" : { "latitude" : 1.25, "longitude" : 1.25 } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "photo" : [{ "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1 }] }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_sticker_serialize_deserialize() {
    let value = serde_json::json!(
        { "sticker" : { "type" : "regular", "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "is_animated" : true, "is_video" : true } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Sticker(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Sticker),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "venue" : { "location" : { "latitude" : 1.25, "longitude" : 1.25 }, "title" :
        "test", "address" : "test" } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Venue(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Venue),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_media_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "video" : { "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "duration" : 1 } }
    );
    let parsed: PollMedia = must_parse(stringify!(PollMedia), &value);
    assert!(
        matches!(&parsed, PollMedia::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(PollMedia),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(PollMedia), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollMedia), &parsed);
}
#[test]
fn test_poll_option_serialize_deserialize() {
    let value = serde_json::json!(
        { "persistent_id" : "test", "text" : "test", "voter_count" : 1 }
    );
    let parsed: PollOption = must_parse(stringify!(PollOption), &value);
    let parsed_value = must_to_value(stringify!(PollOption), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollOption), &parsed);
}
#[test]
fn test_poll_option_added_serialize_deserialize() {
    let value = serde_json::json!(
        { "option_persistent_id" : "test", "option_text" : "test" }
    );
    let parsed: PollOptionAdded = must_parse(stringify!(PollOptionAdded), &value);
    let parsed_value = must_to_value(stringify!(PollOptionAdded), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollOptionAdded), &parsed);
}
#[test]
fn test_poll_option_deleted_serialize_deserialize() {
    let value = serde_json::json!(
        { "option_persistent_id" : "test", "option_text" : "test" }
    );
    let parsed: PollOptionDeleted = must_parse(stringify!(PollOptionDeleted), &value);
    let parsed_value = must_to_value(stringify!(PollOptionDeleted), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PollOptionDeleted), &parsed);
}
#[test]
fn test_pre_checkout_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "currency" : "test", "total_amount" : 1, "invoice_payload" : "test" }
    );
    let parsed: PreCheckoutQuery = must_parse(stringify!(PreCheckoutQuery), &value);
    let parsed_value = must_to_value(stringify!(PreCheckoutQuery), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PreCheckoutQuery), &parsed);
}
#[test]
fn test_prepared_inline_message_serialize_deserialize() {
    let value = serde_json::json!({ "id" : "test", "expiration_date" : 1 });
    let parsed: PreparedInlineMessage = must_parse(stringify!(PreparedInlineMessage), &value);
    let parsed_value = must_to_value(stringify!(PreparedInlineMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PreparedInlineMessage), &parsed);
}
#[test]
fn test_prepared_keyboard_button_serialize_deserialize() {
    let value = serde_json::json!({ "id" : "test" });
    let parsed: PreparedKeyboardButton = must_parse(stringify!(PreparedKeyboardButton), &value);
    let parsed_value = must_to_value(stringify!(PreparedKeyboardButton), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(PreparedKeyboardButton), &parsed);
}
#[test]
fn test_proximity_alert_triggered_serialize_deserialize() {
    let value = serde_json::json!(
        { "traveler" : { "id" : 1, "is_bot" : true, "first_name" : "test" }, "watcher" :
        { "id" : 1, "is_bot" : true, "first_name" : "test" }, "distance" : 1 }
    );
    let parsed: ProximityAlertTriggered = must_parse(stringify!(ProximityAlertTriggered), &value);
    let parsed_value = must_to_value(stringify!(ProximityAlertTriggered), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ProximityAlertTriggered), &parsed);
}
#[test]
fn test_reaction_count_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : { "type" : "emoji", "emoji" : "test" }, "total_count" : 1 }
    );
    let parsed: ReactionCount = must_parse(stringify!(ReactionCount), &value);
    let parsed_value = must_to_value(stringify!(ReactionCount), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReactionCount), &parsed);
}
#[test]
fn test_reaction_type_emoji_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "emoji", "emoji" : "test" });
    let parsed: ReactionType = must_parse(stringify!(ReactionType), &value);
    assert!(
        matches!(&parsed, ReactionType::Emoji(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReactionType),
        stringify!(Emoji),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReactionType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReactionType), &parsed);
}
#[test]
fn test_reaction_type_custom_emoji_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "custom_emoji", "custom_emoji_id" : "test" }
    );
    let parsed: ReactionType = must_parse(stringify!(ReactionType), &value);
    assert!(
        matches!(&parsed, ReactionType::CustomEmoji(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReactionType),
        stringify!(CustomEmoji),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReactionType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReactionType), &parsed);
}
#[test]
fn test_reaction_type_paid_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "paid" });
    let parsed: ReactionType = must_parse(stringify!(ReactionType), &value);
    assert!(
        matches!(&parsed, ReactionType::Paid(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReactionType),
        stringify!(Paid),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReactionType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReactionType), &parsed);
}
#[test]
fn test_refunded_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "currency" : "xtr", "total_amount" : 1, "invoice_payload" : "test",
        "telegram_payment_charge_id" : "test" }
    );
    let parsed: RefundedPayment = must_parse(stringify!(RefundedPayment), &value);
    let parsed_value = must_to_value(stringify!(RefundedPayment), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RefundedPayment), &parsed);
}
#[test]
fn test_reply_keyboard_markup_serialize_deserialize() {
    let value = serde_json::json!({ "keyboard" : [[{ "text" : "the" }]] });
    let parsed: ReplyMarkup = must_parse(stringify!(ReplyMarkup), &value);
    assert!(
        matches!(&parsed, ReplyMarkup::ReplyKeyboardMarkup(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReplyMarkup),
        stringify!(ReplyKeyboardMarkup),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReplyMarkup), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReplyMarkup), &parsed);
}
#[test]
fn test_reply_keyboard_remove_serialize_deserialize() {
    let value = serde_json::json!({ "remove_keyboard" : true });
    let parsed: ReplyMarkup = must_parse(stringify!(ReplyMarkup), &value);
    assert!(
        matches!(&parsed, ReplyMarkup::ReplyKeyboardRemove(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(ReplyMarkup),
        stringify!(ReplyKeyboardRemove),
        parsed
    );
    let parsed_value = must_to_value(stringify!(ReplyMarkup), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReplyMarkup), &parsed);
}
#[test]
fn test_reply_parameters_serialize_deserialize() {
    let value = serde_json::json!({ "message_id" : 1 });
    let parsed: ReplyParameters = must_parse(stringify!(ReplyParameters), &value);
    let parsed_value = must_to_value(stringify!(ReplyParameters), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ReplyParameters), &parsed);
}
#[test]
fn test_response_parameters_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: ResponseParameters = must_parse(stringify!(ResponseParameters), &value);
    let parsed_value = must_to_value(stringify!(ResponseParameters), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ResponseParameters), &parsed);
}
#[test]
fn test_revenue_withdrawal_state_pending_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "pending" });
    let parsed: RevenueWithdrawalState = must_parse(stringify!(RevenueWithdrawalState), &value);
    assert!(
        matches!(&parsed, RevenueWithdrawalState::Pending(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RevenueWithdrawalState),
        stringify!(Pending),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RevenueWithdrawalState), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RevenueWithdrawalState), &parsed);
}
#[test]
fn test_revenue_withdrawal_state_succeeded_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "succeeded", "date" : 1, "url" : "used" });
    let parsed: RevenueWithdrawalState = must_parse(stringify!(RevenueWithdrawalState), &value);
    assert!(
        matches!(&parsed, RevenueWithdrawalState::Succeeded(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RevenueWithdrawalState),
        stringify!(Succeeded),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RevenueWithdrawalState), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RevenueWithdrawalState), &parsed);
}
#[test]
fn test_revenue_withdrawal_state_failed_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "failed" });
    let parsed: RevenueWithdrawalState = must_parse(stringify!(RevenueWithdrawalState), &value);
    assert!(
        matches!(&parsed, RevenueWithdrawalState::Failed(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RevenueWithdrawalState),
        stringify!(Failed),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RevenueWithdrawalState), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RevenueWithdrawalState), &parsed);
}
#[test]
fn test_rich_block_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "paragraph", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_anchor_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "anchor", "name" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Anchor(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Anchor),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_animation_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "animation", "animation" : { "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Animation(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Animation),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_audio_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "audio", "audio" : { "file_id" : "used", "file_unique_id" : "test",
        "duration" : 1 } }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Audio(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Audio),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_block_quotation_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "blockquote", "blocks" : [{ "type" : "paragraph", "text" : "test" }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Blockquote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Blockquote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_caption_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test" });
    let parsed: RichBlockCaption = must_parse(stringify!(RichBlockCaption), &value);
    let parsed_value = must_to_value(stringify!(RichBlockCaption), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlockCaption), &parsed);
}
#[test]
fn test_rich_block_collage_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "collage", "blocks" : [{ "type" : "paragraph", "text" : "test" }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Collage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Collage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_details_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "details", "summary" : "test", "blocks" : [{ "type" : "paragraph",
        "text" : "test" }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Details(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Details),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_divider_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "divider" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Divider(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Divider),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_footer_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "footer", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Footer(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Footer),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_list_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "list", "items" : [{ "label" : "test", "blocks" : [{ "type" :
        "paragraph", "text" : "test" }] }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::List(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(List),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_list_item_serialize_deserialize() {
    let value = serde_json::json!(
        { "label" : "test", "blocks" : [{ "type" : "paragraph", "text" : "test" }] }
    );
    let parsed: RichBlockListItem = must_parse(stringify!(RichBlockListItem), &value);
    let parsed_value = must_to_value(stringify!(RichBlockListItem), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlockListItem), &parsed);
}
#[test]
fn test_rich_block_map_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "map", "location" : { "latitude" : 1.25, "longitude" : 1.25 }, "zoom"
        : 1, "width" : 1, "height" : 1 }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Map(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Map),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_mathematical_expression_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mathematical_expression", "expression" : "test" }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::MathematicalExpression(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(MathematicalExpression),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_paragraph_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "paragraph", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Paragraph(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Paragraph),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_photo_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "photo", "photo" : [{ "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1 }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Photo(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Photo),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_preformatted_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "pre", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Pre(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Pre),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_pull_quotation_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "pullquote", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Pullquote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Pullquote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_section_heading_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "heading", "text" : "test", "size" : 1 });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Heading(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Heading),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_slideshow_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "slideshow", "blocks" : [{ "type" : "paragraph", "text" : "test" }] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Slideshow(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Slideshow),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_table_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "table", "cells" : [[{ "align" : "left", "valign" : "top" }]] }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Table(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Table),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_table_cell_serialize_deserialize() {
    let value = serde_json::json!({ "align" : "left", "valign" : "top" });
    let parsed: RichBlockTableCell = must_parse(stringify!(RichBlockTableCell), &value);
    let parsed_value = must_to_value(stringify!(RichBlockTableCell), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlockTableCell), &parsed);
}
#[test]
fn test_rich_block_thinking_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "thinking", "text" : "test" });
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Thinking(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Thinking),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "video", "video" : { "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1, "duration" : 1 } }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::Video(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(Video),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_block_voice_note_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "voice_note", "voice_note" : { "file_id" : "used", "file_unique_id" :
        "test", "duration" : 1 } }
    );
    let parsed: RichBlock = must_parse(stringify!(RichBlock), &value);
    assert!(
        matches!(&parsed, RichBlock::VoiceNote(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichBlock),
        stringify!(VoiceNote),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichBlock), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichBlock), &parsed);
}
#[test]
fn test_rich_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "blocks" : [{ "type" : "paragraph", "text" : "test" }] }
    );
    let parsed: RichMessage = must_parse(stringify!(RichMessage), &value);
    let parsed_value = must_to_value(stringify!(RichMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichMessage), &parsed);
}
#[test]
fn test_rich_text_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "bold", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_anchor_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "anchor", "name" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Anchor(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Anchor),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_anchor_link_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "anchor_link", "text" : "test", "anchor_name" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::AnchorLink(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(AnchorLink),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_bank_card_number_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "bank_card_number", "text" : "test", "bank_card_number" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::BankCardNumber(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(BankCardNumber),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_bold_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "bold", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Bold(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Bold),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_bot_command_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "bot_command", "text" : "test", "bot_command" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::BotCommand(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(BotCommand),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_cashtag_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "cashtag", "text" : "test", "cashtag" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Cashtag(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Cashtag),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_code_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "code", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Code(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Code),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_custom_emoji_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "custom_emoji", "custom_emoji_id" : "test", "alternative_text" :
        "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::CustomEmoji(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(CustomEmoji),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_date_time_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "date_time", "text" : "test", "unix_time" : 1, "date_time_format" :
        "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::DateTime(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(DateTime),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_email_address_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "email_address", "text" : "test", "email_address" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::EmailAddress(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(EmailAddress),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_hashtag_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "hashtag", "text" : "test", "hashtag" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Hashtag(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Hashtag),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_italic_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "italic", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Italic(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Italic),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_marked_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "marked", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Marked(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Marked),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_mathematical_expression_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mathematical_expression", "expression" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::MathematicalExpression(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(MathematicalExpression),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_mention_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mention", "text" : "test", "username" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Mention(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Mention),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_phone_number_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "phone_number", "text" : "test", "phone_number" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::PhoneNumber(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(PhoneNumber),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_reference_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "reference", "text" : "test", "name" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Reference(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Reference),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_reference_link_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "reference_link", "text" : "test", "reference_name" : "test" }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::ReferenceLink(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(ReferenceLink),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_spoiler_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "spoiler", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Spoiler(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Spoiler),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_strikethrough_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "strikethrough", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Strikethrough(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Strikethrough),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_subscript_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "subscript", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Subscript(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Subscript),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_superscript_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "superscript", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Superscript(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Superscript),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_text_mention_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "text_mention", "text" : "test", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } }
    );
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::TextMention(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(TextMention),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_underline_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "underline", "text" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Underline(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Underline),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_rich_text_url_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "url", "text" : "test", "url" : "test" });
    let parsed: RichText = must_parse(stringify!(RichText), &value);
    assert!(
        matches!(&parsed, RichText::Url(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(RichText),
        stringify!(Url),
        parsed
    );
    let parsed_value = must_to_value(stringify!(RichText), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(RichText), &parsed);
}
#[test]
fn test_sent_guest_message_serialize_deserialize() {
    let value = serde_json::json!({ "inline_message_id" : "test" });
    let parsed: SentGuestMessage = must_parse(stringify!(SentGuestMessage), &value);
    let parsed_value = must_to_value(stringify!(SentGuestMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SentGuestMessage), &parsed);
}
#[test]
fn test_sent_web_app_message_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: SentWebAppMessage = must_parse(stringify!(SentWebAppMessage), &value);
    let parsed_value = must_to_value(stringify!(SentWebAppMessage), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SentWebAppMessage), &parsed);
}
#[test]
fn test_shared_user_serialize_deserialize() {
    let value = serde_json::json!({ "user_id" : 1 });
    let parsed: SharedUser = must_parse(stringify!(SharedUser), &value);
    let parsed_value = must_to_value(stringify!(SharedUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SharedUser), &parsed);
}
#[test]
fn test_shipping_address_serialize_deserialize() {
    let value = serde_json::json!(
        { "country_code" : "test", "state" : "test", "city" : "test", "street_line1" :
        "test", "street_line2" : "test", "post_code" : "test" }
    );
    let parsed: ShippingAddress = must_parse(stringify!(ShippingAddress), &value);
    let parsed_value = must_to_value(stringify!(ShippingAddress), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ShippingAddress), &parsed);
}
#[test]
fn test_shipping_option_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "title" : "test", "prices" : [{ "label" : "test", "amount" : 1
        }] }
    );
    let parsed: ShippingOption = must_parse(stringify!(ShippingOption), &value);
    let parsed_value = must_to_value(stringify!(ShippingOption), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ShippingOption), &parsed);
}
#[test]
fn test_shipping_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "invoice_payload" : "test", "shipping_address" : { "country_code" : "test",
        "state" : "test", "city" : "test", "street_line1" : "test", "street_line2" :
        "test", "post_code" : "test" } }
    );
    let parsed: ShippingQuery = must_parse(stringify!(ShippingQuery), &value);
    let parsed_value = must_to_value(stringify!(ShippingQuery), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(ShippingQuery), &parsed);
}
#[test]
fn test_star_amount_serialize_deserialize() {
    let value = serde_json::json!({ "amount" : 1 });
    let parsed: StarAmount = must_parse(stringify!(StarAmount), &value);
    let parsed_value = must_to_value(stringify!(StarAmount), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StarAmount), &parsed);
}
#[test]
fn test_star_transaction_incoming_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "amount" : 1, "date" : 1, "source" : { "transaction_type" :
        "invoice_payment", "type" : "user", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } } }
    );
    let parsed: StarTransaction = must_parse(stringify!(StarTransaction), &value);
    assert!(
        matches!(&parsed, StarTransaction::Incoming(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StarTransaction),
        stringify!(Incoming),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StarTransaction), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StarTransaction), &parsed);
}
#[test]
fn test_star_transaction_outgoing_serialize_deserialize() {
    let value = serde_json::json!(
        { "id" : "test", "amount" : 1, "date" : 1, "receiver" : { "transaction_type" :
        "invoice_payment", "type" : "user", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } } }
    );
    let parsed: StarTransaction = must_parse(stringify!(StarTransaction), &value);
    assert!(
        matches!(&parsed, StarTransaction::Outgoing(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StarTransaction),
        stringify!(Outgoing),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StarTransaction), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StarTransaction), &parsed);
}
#[test]
fn test_star_transactions_serialize_deserialize() {
    let value = serde_json::json!(
        { "transactions" : [{ "id" : "test", "amount" : 1, "date" : 1, "source" : {
        "transaction_type" : "invoice_payment", "type" : "user", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" } } }] }
    );
    let parsed: StarTransactions = must_parse(stringify!(StarTransactions), &value);
    let parsed_value = must_to_value(stringify!(StarTransactions), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StarTransactions), &parsed);
}
#[test]
fn test_sticker_custom_emoji_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "custom_emoji", "file_id" : "used", "file_unique_id" : "test", "width"
        : 1, "height" : 1, "is_animated" : true, "is_video" : true }
    );
    let parsed: Sticker = must_parse(stringify!(Sticker), &value);
    assert!(
        matches!(&parsed, Sticker::CustomEmoji(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Sticker),
        stringify!(CustomEmoji),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Sticker), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Sticker), &parsed);
}
#[test]
fn test_sticker_mask_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "mask", "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "is_animated" : true, "is_video" : true }
    );
    let parsed: Sticker = must_parse(stringify!(Sticker), &value);
    assert!(
        matches!(&parsed, Sticker::Mask(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Sticker),
        stringify!(Mask),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Sticker), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Sticker), &parsed);
}
#[test]
fn test_sticker_regular_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "is_animated" : true, "is_video" : true }
    );
    let parsed: Sticker = must_parse(stringify!(Sticker), &value);
    assert!(
        matches!(&parsed, Sticker::Regular(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Sticker),
        stringify!(Regular),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Sticker), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Sticker), &parsed);
}
#[test]
fn test_sticker_set_serialize_deserialize() {
    let value = serde_json::json!(
        { "name" : "test", "title" : "test", "sticker_type" : "regular", "stickers" : [{
        "type" : "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "is_animated" : true, "is_video" : true }] }
    );
    let parsed: StickerSet = must_parse(stringify!(StickerSet), &value);
    let parsed_value = must_to_value(stringify!(StickerSet), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StickerSet), &parsed);
}
#[test]
fn test_story_serialize_deserialize() {
    let value = serde_json::json!(
        { "chat" : { "type" : "private", "id" : 1 }, "id" : 1 }
    );
    let parsed: Story = must_parse(stringify!(Story), &value);
    let parsed_value = must_to_value(stringify!(Story), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Story), &parsed);
}
#[test]
fn test_story_area_serialize_deserialize() {
    let value = serde_json::json!(
        { "position" : { "x_percentage" : 1.25, "y_percentage" : 1.25, "width_percentage"
        : 1.25, "height_percentage" : 1.25, "rotation_angle" : 1.25,
        "corner_radius_percentage" : 1.25 }, "type" : { "type" : "location", "latitude" :
        1.25, "longitude" : 1.25 } }
    );
    let parsed: StoryArea = must_parse(stringify!(StoryArea), &value);
    let parsed_value = must_to_value(stringify!(StoryArea), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryArea), &parsed);
}
#[test]
fn test_story_area_position_serialize_deserialize() {
    let value = serde_json::json!(
        { "x_percentage" : 1.25, "y_percentage" : 1.25, "width_percentage" : 1.25,
        "height_percentage" : 1.25, "rotation_angle" : 1.25, "corner_radius_percentage" :
        1.25 }
    );
    let parsed: StoryAreaPosition = must_parse(stringify!(StoryAreaPosition), &value);
    let parsed_value = must_to_value(stringify!(StoryAreaPosition), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaPosition), &parsed);
}
#[test]
fn test_story_area_type_location_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "location", "latitude" : 1.25, "longitude" : 1.25 }
    );
    let parsed: StoryAreaType = must_parse(stringify!(StoryAreaType), &value);
    assert!(
        matches!(&parsed, StoryAreaType::Location(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StoryAreaType),
        stringify!(Location),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StoryAreaType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaType), &parsed);
}
#[test]
fn test_story_area_type_suggested_reaction_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "suggested_reaction", "reaction_type" : { "type" : "emoji", "emoji" :
        "test" } }
    );
    let parsed: StoryAreaType = must_parse(stringify!(StoryAreaType), &value);
    assert!(
        matches!(&parsed, StoryAreaType::SuggestedReaction(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StoryAreaType),
        stringify!(SuggestedReaction),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StoryAreaType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaType), &parsed);
}
#[test]
fn test_story_area_type_link_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "link", "url" : "test" });
    let parsed: StoryAreaType = must_parse(stringify!(StoryAreaType), &value);
    assert!(
        matches!(&parsed, StoryAreaType::Link(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StoryAreaType),
        stringify!(Link),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StoryAreaType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaType), &parsed);
}
#[test]
fn test_story_area_type_weather_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "weather", "temperature" : 1.25, "emoji" : "test", "background_color"
        : 1 }
    );
    let parsed: StoryAreaType = must_parse(stringify!(StoryAreaType), &value);
    assert!(
        matches!(&parsed, StoryAreaType::Weather(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StoryAreaType),
        stringify!(Weather),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StoryAreaType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaType), &parsed);
}
#[test]
fn test_story_area_type_unique_gift_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "unique_gift", "name" : "test" });
    let parsed: StoryAreaType = must_parse(stringify!(StoryAreaType), &value);
    assert!(
        matches!(&parsed, StoryAreaType::UniqueGift(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(StoryAreaType),
        stringify!(UniqueGift),
        parsed
    );
    let parsed_value = must_to_value(stringify!(StoryAreaType), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(StoryAreaType), &parsed);
}
#[test]
fn test_successful_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "currency" : "test", "total_amount" : 1, "invoice_payload" : "test",
        "telegram_payment_charge_id" : "test", "provider_payment_charge_id" : "test" }
    );
    let parsed: SuccessfulPayment = must_parse(stringify!(SuccessfulPayment), &value);
    let parsed_value = must_to_value(stringify!(SuccessfulPayment), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuccessfulPayment), &parsed);
}
#[test]
fn test_suggested_post_approval_failed_serialize_deserialize() {
    let value = serde_json::json!({ "price" : { "currency" : "xtr", "amount" : 1 } });
    let parsed: SuggestedPostApprovalFailed =
        must_parse(stringify!(SuggestedPostApprovalFailed), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostApprovalFailed), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostApprovalFailed), &parsed);
}
#[test]
fn test_suggested_post_approved_serialize_deserialize() {
    let value = serde_json::json!({ "send_date" : 1 });
    let parsed: SuggestedPostApproved = must_parse(stringify!(SuggestedPostApproved), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostApproved), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostApproved), &parsed);
}
#[test]
fn test_suggested_post_declined_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: SuggestedPostDeclined = must_parse(stringify!(SuggestedPostDeclined), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostDeclined), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostDeclined), &parsed);
}
#[test]
fn test_suggested_post_info_serialize_deserialize() {
    let value = serde_json::json!({ "state" : "pending" });
    let parsed: SuggestedPostInfo = must_parse(stringify!(SuggestedPostInfo), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostInfo), &parsed);
}
#[test]
fn test_suggested_post_paid_serialize_deserialize() {
    let value = serde_json::json!({ "currency" : "xtr" });
    let parsed: SuggestedPostPaid = must_parse(stringify!(SuggestedPostPaid), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostPaid), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostPaid), &parsed);
}
#[test]
fn test_suggested_post_parameters_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: SuggestedPostParameters = must_parse(stringify!(SuggestedPostParameters), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostParameters), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostParameters), &parsed);
}
#[test]
fn test_suggested_post_price_serialize_deserialize() {
    let value = serde_json::json!({ "currency" : "xtr", "amount" : 1 });
    let parsed: SuggestedPostPrice = must_parse(stringify!(SuggestedPostPrice), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostPrice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostPrice), &parsed);
}
#[test]
fn test_suggested_post_refunded_serialize_deserialize() {
    let value = serde_json::json!({ "reason" : "post_deleted" });
    let parsed: SuggestedPostRefunded = must_parse(stringify!(SuggestedPostRefunded), &value);
    let parsed_value = must_to_value(stringify!(SuggestedPostRefunded), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SuggestedPostRefunded), &parsed);
}
#[test]
fn test_switch_inline_query_chosen_chat_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: SwitchInlineQueryChosenChat =
        must_parse(stringify!(SwitchInlineQueryChosenChat), &value);
    let parsed_value = must_to_value(stringify!(SwitchInlineQueryChosenChat), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(SwitchInlineQueryChosenChat), &parsed);
}
#[test]
fn test_text_quote_serialize_deserialize() {
    let value = serde_json::json!({ "text" : "test", "position" : 1 });
    let parsed: TextQuote = must_parse(stringify!(TextQuote), &value);
    let parsed_value = must_to_value(stringify!(TextQuote), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TextQuote), &parsed);
}
#[test]
fn test_transaction_partner_user_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "invoice_payment", "type" : "user", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::User(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(User),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_chat_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "chat", "chat" : { "type" : "private", "id" : 1 } }
    );
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::Chat(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(Chat),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_affiliate_program_serialize_deserialize() {
    let value = serde_json::json!(
        { "type" : "affiliate_program", "commission_per_mille" : 1 }
    );
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::AffiliateProgram(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(AffiliateProgram),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_fragment_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "fragment" });
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::Fragment(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(Fragment),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_telegram_ads_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "telegram_ads" });
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::TelegramAds(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(TelegramAds),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_telegram_api_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "telegram_api", "request_count" : 1 });
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::TelegramApi(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(TelegramApi),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_other_serialize_deserialize() {
    let value = serde_json::json!({ "type" : "other" });
    let parsed: TransactionPartner = must_parse(stringify!(TransactionPartner), &value);
    assert!(
        matches!(&parsed, TransactionPartner::Other(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartner),
        stringify!(Other),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartner), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartner), &parsed);
}
#[test]
fn test_transaction_partner_user_business_account_transfer_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "business_account_transfer", "type" : "user", "user" : {
        "id" : 1, "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: TransactionPartnerUser = must_parse(stringify!(TransactionPartnerUser), &value);
    assert!(
        matches!(&parsed, TransactionPartnerUser::BusinessAccountTransfer(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartnerUser),
        stringify!(BusinessAccountTransfer),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartnerUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartnerUser), &parsed);
}
#[test]
fn test_transaction_partner_user_gift_purchase_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "gift_purchase", "type" : "user", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "gift" : { "id" : "test", "sticker" : {
        "type" : "regular", "file_id" : "used", "file_unique_id" : "test", "width" : 1,
        "height" : 1, "is_animated" : true, "is_video" : true }, "star_count" : 1 } }
    );
    let parsed: TransactionPartnerUser = must_parse(stringify!(TransactionPartnerUser), &value);
    assert!(
        matches!(&parsed, TransactionPartnerUser::GiftPurchase(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartnerUser),
        stringify!(GiftPurchase),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartnerUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartnerUser), &parsed);
}
#[test]
fn test_transaction_partner_user_invoice_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "invoice_payment", "type" : "user", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" } }
    );
    let parsed: TransactionPartnerUser = must_parse(stringify!(TransactionPartnerUser), &value);
    assert!(
        matches!(&parsed, TransactionPartnerUser::InvoicePayment(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartnerUser),
        stringify!(InvoicePayment),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartnerUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartnerUser), &parsed);
}
#[test]
fn test_transaction_partner_user_paid_media_payment_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "paid_media_payment", "type" : "user", "user" : { "id" :
        1, "is_bot" : true, "first_name" : "test" }, "paid_media" : [{ "type" :
        "live_photo", "live_photo" : { "file_id" : "used", "file_unique_id" : "test",
        "width" : 1, "height" : 1, "duration" : 1 } }] }
    );
    let parsed: TransactionPartnerUser = must_parse(stringify!(TransactionPartnerUser), &value);
    assert!(
        matches!(&parsed, TransactionPartnerUser::PaidMediaPayment(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartnerUser),
        stringify!(PaidMediaPayment),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartnerUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartnerUser), &parsed);
}
#[test]
fn test_transaction_partner_user_premium_purchase_serialize_deserialize() {
    let value = serde_json::json!(
        { "transaction_type" : "premium_purchase", "type" : "user", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "premium_subscription_duration" : 1 }
    );
    let parsed: TransactionPartnerUser = must_parse(stringify!(TransactionPartnerUser), &value);
    assert!(
        matches!(&parsed, TransactionPartnerUser::PremiumPurchase(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(TransactionPartnerUser),
        stringify!(PremiumPurchase),
        parsed
    );
    let parsed_value = must_to_value(stringify!(TransactionPartnerUser), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(TransactionPartnerUser), &parsed);
}
#[test]
fn test_unique_gift_serialize_deserialize() {
    let value = serde_json::json!(
        { "gift_id" : "test", "base_name" : "test", "name" : "used", "number" : 1,
        "model" : { "name" : "test", "sticker" : { "type" : "regular", "file_id" :
        "used", "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" :
        true, "is_video" : true }, "rarity_per_mille" : 1 }, "symbol" : { "name" :
        "test", "sticker" : { "type" : "regular", "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "is_animated" : true, "is_video" : true },
        "rarity_per_mille" : 1 }, "backdrop" : { "name" : "test", "colors" : {
        "center_color" : 1, "edge_color" : 1, "symbol_color" : 1, "text_color" : 1 },
        "rarity_per_mille" : 1 } }
    );
    let parsed: UniqueGift = must_parse(stringify!(UniqueGift), &value);
    let parsed_value = must_to_value(stringify!(UniqueGift), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGift), &parsed);
}
#[test]
fn test_unique_gift_backdrop_serialize_deserialize() {
    let value = serde_json::json!(
        { "name" : "test", "colors" : { "center_color" : 1, "edge_color" : 1,
        "symbol_color" : 1, "text_color" : 1 }, "rarity_per_mille" : 1 }
    );
    let parsed: UniqueGiftBackdrop = must_parse(stringify!(UniqueGiftBackdrop), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftBackdrop), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftBackdrop), &parsed);
}
#[test]
fn test_unique_gift_backdrop_colors_serialize_deserialize() {
    let value = serde_json::json!(
        { "center_color" : 1, "edge_color" : 1, "symbol_color" : 1, "text_color" : 1 }
    );
    let parsed: UniqueGiftBackdropColors = must_parse(stringify!(UniqueGiftBackdropColors), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftBackdropColors), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftBackdropColors), &parsed);
}
#[test]
fn test_unique_gift_colors_serialize_deserialize() {
    let value = serde_json::json!(
        { "model_custom_emoji_id" : "test", "symbol_custom_emoji_id" : "test",
        "light_theme_main_color" : 1, "light_theme_other_colors" : [1],
        "dark_theme_main_color" : 1, "dark_theme_other_colors" : [1] }
    );
    let parsed: UniqueGiftColors = must_parse(stringify!(UniqueGiftColors), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftColors), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftColors), &parsed);
}
#[test]
fn test_unique_gift_info_serialize_deserialize() {
    let value = serde_json::json!(
        { "gift" : { "gift_id" : "test", "base_name" : "test", "name" : "used", "number"
        : 1, "model" : { "name" : "test", "sticker" : { "type" : "regular", "file_id" :
        "used", "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" :
        true, "is_video" : true }, "rarity_per_mille" : 1 }, "symbol" : { "name" :
        "test", "sticker" : { "type" : "regular", "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "is_animated" : true, "is_video" : true },
        "rarity_per_mille" : 1 }, "backdrop" : { "name" : "test", "colors" : {
        "center_color" : 1, "edge_color" : 1, "symbol_color" : 1, "text_color" : 1 },
        "rarity_per_mille" : 1 } }, "origin" : "test" }
    );
    let parsed: UniqueGiftInfo = must_parse(stringify!(UniqueGiftInfo), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftInfo), &parsed);
}
#[test]
fn test_unique_gift_model_serialize_deserialize() {
    let value = serde_json::json!(
        { "name" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "rarity_per_mille" : 1 }
    );
    let parsed: UniqueGiftModel = must_parse(stringify!(UniqueGiftModel), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftModel), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftModel), &parsed);
}
#[test]
fn test_unique_gift_symbol_serialize_deserialize() {
    let value = serde_json::json!(
        { "name" : "test", "sticker" : { "type" : "regular", "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "is_animated" : true,
        "is_video" : true }, "rarity_per_mille" : 1 }
    );
    let parsed: UniqueGiftSymbol = must_parse(stringify!(UniqueGiftSymbol), &value);
    let parsed_value = must_to_value(stringify!(UniqueGiftSymbol), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UniqueGiftSymbol), &parsed);
}
#[test]
fn test_update_business_connection_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "business_connection" : { "id" : "test", "user" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "user_chat_id" : 1, "date" : 1,
        "is_enabled" : true } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::BusinessConnection(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(BusinessConnection),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_business_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "business_message" : { "message_id" : 1, "date" : 1, "chat" :
        { "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::BusinessMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(BusinessMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_callback_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "callback_query" : { "id" : "test", "from" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "chat_instance" : "test" } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::CallbackQuery(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(CallbackQuery),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_channel_post_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "channel_post" : { "message_id" : 1, "date" : 1, "chat" : {
        "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ChannelPost(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ChannelPost),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_chat_boost_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "chat_boost" : { "chat" : { "type" : "private", "id" : 1 },
        "boost" : { "boost_id" : "test", "add_date" : 1, "expiration_date" : 1, "source"
        : { "source" : "premium", "user" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } } } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ChatBoost(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ChatBoost),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_chat_join_request_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "chat_join_request" : { "chat" : { "type" : "private", "id" :
        1 }, "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "user_chat_id" : 1, "date" : 1 } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ChatJoinRequest(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ChatJoinRequest),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_chat_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "chat_member" : { "chat" : { "type" : "private", "id" : 1 },
        "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" }, "date" : 1,
        "old_chat_member" : { "status" : "creator", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" }, "is_anonymous" : true }, "new_chat_member" : { "status"
        : "creator", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "is_anonymous" : true } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ChatMember(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ChatMember),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_chosen_inline_result_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "chosen_inline_result" : { "result_id" : "test", "from" : {
        "id" : 1, "is_bot" : true, "first_name" : "test" }, "query" : "test" } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ChosenInlineResult(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ChosenInlineResult),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_deleted_business_messages_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "deleted_business_messages" : { "business_connection_id" :
        "test", "chat" : { "type" : "private", "id" : 1 }, "message_ids" : [1] } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::DeletedBusinessMessages(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(DeletedBusinessMessages),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_edited_business_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "edited_business_message" : { "message_id" : 1, "date" : 1,
        "chat" : { "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::EditedBusinessMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(EditedBusinessMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_edited_channel_post_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "edited_channel_post" : { "message_id" : 1, "date" : 1, "chat"
        : { "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::EditedChannelPost(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(EditedChannelPost),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_edited_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "edited_message" : { "message_id" : 1, "date" : 1, "chat" : {
        "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::EditedMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(EditedMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_guest_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "guest_message" : { "message_id" : 1, "date" : 1, "chat" : {
        "type" : "private", "id" : 1 }, "animation" : { "file_id" : "used",
        "file_unique_id" : "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::GuestMessage(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(GuestMessage),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_inline_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "inline_query" : { "id" : "test", "from" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "query" : "test", "offset" :
        "controlled" } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::InlineQuery(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(InlineQuery),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_managed_bot_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "managed_bot" : { "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" }, "bot" : { "id" : 1, "is_bot" : true, "first_name" :
        "test" } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ManagedBot(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ManagedBot),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_message_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "message" : { "message_id" : 1, "date" : 1, "chat" : { "type"
        : "private", "id" : 1 }, "animation" : { "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1, "duration" : 1 } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::Message(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(Message),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_message_reaction_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "message_reaction" : { "chat" : { "type" : "private", "id" : 1
        }, "message_id" : 1, "date" : 1, "old_reaction" : [{ "type" : "emoji", "emoji" :
        "test" }], "new_reaction" : [{ "type" : "emoji", "emoji" : "test" }] } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::MessageReaction(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(MessageReaction),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_message_reaction_count_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "message_reaction_count" : { "chat" : { "type" : "private",
        "id" : 1 }, "message_id" : 1, "date" : 1, "reactions" : [{ "type" : { "type" :
        "emoji", "emoji" : "test" }, "total_count" : 1 }] } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::MessageReactionCount(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(MessageReactionCount),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_my_chat_member_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "my_chat_member" : { "chat" : { "type" : "private", "id" : 1
        }, "from" : { "id" : 1, "is_bot" : true, "first_name" : "test" }, "date" : 1,
        "old_chat_member" : { "status" : "creator", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" }, "is_anonymous" : true }, "new_chat_member" : { "status"
        : "creator", "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" },
        "is_anonymous" : true } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::MyChatMember(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(MyChatMember),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_poll_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "poll" : { "type" : "regular", "id" : "test", "question" :
        "test", "options" : [{ "persistent_id" : "test", "text" : "test", "voter_count" :
        1 }], "total_voter_count" : 1, "is_closed" : true, "is_anonymous" : true,
        "allows_multiple_answers" : true, "allows_revoting" : true, "members_only" : true
        } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::Poll(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(Poll),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_poll_answer_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "poll_answer" : { "poll_id" : "test", "option_ids" : [1],
        "option_persistent_ids" : ["test"] } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::PollAnswer(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(PollAnswer),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_pre_checkout_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "pre_checkout_query" : { "id" : "test", "from" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "currency" : "test", "total_amount" :
        1, "invoice_payload" : "test" } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::PreCheckoutQuery(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(PreCheckoutQuery),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_purchased_paid_media_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "purchased_paid_media" : { "from" : { "id" : 1, "is_bot" :
        true, "first_name" : "test" }, "paid_media_payload" : "test" } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::PurchasedPaidMedia(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(PurchasedPaidMedia),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_removed_chat_boost_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "removed_chat_boost" : { "chat" : { "type" : "private", "id" :
        1 }, "boost_id" : "test", "remove_date" : 1, "source" : { "source" : "premium",
        "user" : { "id" : 1, "is_bot" : true, "first_name" : "test" } } } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::RemovedChatBoost(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(RemovedChatBoost),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_shipping_query_serialize_deserialize() {
    let value = serde_json::json!(
        { "update_id" : 1, "shipping_query" : { "id" : "test", "from" : { "id" : 1,
        "is_bot" : true, "first_name" : "test" }, "invoice_payload" : "test",
        "shipping_address" : { "country_code" : "test", "state" : "test", "city" :
        "test", "street_line1" : "test", "street_line2" : "test", "post_code" : "test" }
        } }
    );
    let parsed: Update = must_parse(stringify!(Update), &value);
    assert!(
        matches!(&parsed, Update::ShippingQuery(_)),
        "failed to deserialize {} into expected subtype {}; parsed={:?}",
        stringify!(Update),
        stringify!(ShippingQuery),
        parsed
    );
    let parsed_value = must_to_value(stringify!(Update), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Update), &parsed);
}
#[test]
fn test_update_unparsed_serialize_deserialize() {
    let value = serde_json::json!({ "update_id" : 1 });
    let parsed: UpdateUnparsed = must_parse(stringify!(UpdateUnparsed), &value);
    let parsed_value = must_to_value(stringify!(UpdateUnparsed), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UpdateUnparsed), &parsed);
}
#[test]
fn test_user_serialize_deserialize() {
    let value = serde_json::json!({ "id" : 1, "is_bot" : true, "first_name" : "test" });
    let parsed: User = must_parse(stringify!(User), &value);
    let parsed_value = must_to_value(stringify!(User), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(User), &parsed);
}
#[test]
fn test_user_chat_boosts_serialize_deserialize() {
    let value = serde_json::json!(
        { "boosts" : [{ "boost_id" : "test", "add_date" : 1, "expiration_date" : 1,
        "source" : { "source" : "premium", "user" : { "id" : 1, "is_bot" : true,
        "first_name" : "test" } } }] }
    );
    let parsed: UserChatBoosts = must_parse(stringify!(UserChatBoosts), &value);
    let parsed_value = must_to_value(stringify!(UserChatBoosts), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UserChatBoosts), &parsed);
}
#[test]
fn test_user_profile_audios_serialize_deserialize() {
    let value = serde_json::json!(
        { "total_count" : 1, "audios" : [{ "file_id" : "used", "file_unique_id" : "test",
        "duration" : 1 }] }
    );
    let parsed: UserProfileAudios = must_parse(stringify!(UserProfileAudios), &value);
    let parsed_value = must_to_value(stringify!(UserProfileAudios), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UserProfileAudios), &parsed);
}
#[test]
fn test_user_profile_photos_serialize_deserialize() {
    let value = serde_json::json!(
        { "total_count" : 1, "photos" : [[{ "file_id" : "used", "file_unique_id" :
        "test", "width" : 1, "height" : 1 }]] }
    );
    let parsed: UserProfilePhotos = must_parse(stringify!(UserProfilePhotos), &value);
    let parsed_value = must_to_value(stringify!(UserProfilePhotos), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UserProfilePhotos), &parsed);
}
#[test]
fn test_user_rating_serialize_deserialize() {
    let value = serde_json::json!(
        { "level" : 1, "rating" : 1, "current_level_rating" : 1 }
    );
    let parsed: UserRating = must_parse(stringify!(UserRating), &value);
    let parsed_value = must_to_value(stringify!(UserRating), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UserRating), &parsed);
}
#[test]
fn test_users_shared_serialize_deserialize() {
    let value = serde_json::json!({ "request_id" : 1, "users" : [{ "user_id" : 1 }] });
    let parsed: UsersShared = must_parse(stringify!(UsersShared), &value);
    let parsed_value = must_to_value(stringify!(UsersShared), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(UsersShared), &parsed);
}
#[test]
fn test_venue_serialize_deserialize() {
    let value = serde_json::json!(
        { "location" : { "latitude" : 1.25, "longitude" : 1.25 }, "title" : "test",
        "address" : "test" }
    );
    let parsed: Venue = must_parse(stringify!(Venue), &value);
    let parsed_value = must_to_value(stringify!(Venue), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Venue), &parsed);
}
#[test]
fn test_video_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "duration" : 1 }
    );
    let parsed: Video = must_parse(stringify!(Video), &value);
    let parsed_value = must_to_value(stringify!(Video), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Video), &parsed);
}
#[test]
fn test_video_chat_ended_serialize_deserialize() {
    let value = serde_json::json!({ "duration" : 1 });
    let parsed: VideoChatEnded = must_parse(stringify!(VideoChatEnded), &value);
    let parsed_value = must_to_value(stringify!(VideoChatEnded), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoChatEnded), &parsed);
}
#[test]
fn test_video_chat_participants_invited_serialize_deserialize() {
    let value = serde_json::json!(
        { "users" : [{ "id" : 1, "is_bot" : true, "first_name" : "test" }] }
    );
    let parsed: VideoChatParticipantsInvited =
        must_parse(stringify!(VideoChatParticipantsInvited), &value);
    let parsed_value = must_to_value(stringify!(VideoChatParticipantsInvited), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoChatParticipantsInvited), &parsed);
}
#[test]
fn test_video_chat_scheduled_serialize_deserialize() {
    let value = serde_json::json!({ "start_date" : 1 });
    let parsed: VideoChatScheduled = must_parse(stringify!(VideoChatScheduled), &value);
    let parsed_value = must_to_value(stringify!(VideoChatScheduled), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoChatScheduled), &parsed);
}
#[test]
fn test_video_chat_started_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: VideoChatStarted = must_parse(stringify!(VideoChatStarted), &value);
    let parsed_value = must_to_value(stringify!(VideoChatStarted), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoChatStarted), &parsed);
}
#[test]
fn test_video_note_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "length" : 1, "duration" : 1 }
    );
    let parsed: VideoNote = must_parse(stringify!(VideoNote), &value);
    let parsed_value = must_to_value(stringify!(VideoNote), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoNote), &parsed);
}
#[test]
fn test_video_quality_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "width" : 1, "height" : 1,
        "codec" : "h264" }
    );
    let parsed: VideoQuality = must_parse(stringify!(VideoQuality), &value);
    let parsed_value = must_to_value(stringify!(VideoQuality), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(VideoQuality), &parsed);
}
#[test]
fn test_voice_serialize_deserialize() {
    let value = serde_json::json!(
        { "file_id" : "used", "file_unique_id" : "test", "duration" : 1 }
    );
    let parsed: Voice = must_parse(stringify!(Voice), &value);
    let parsed_value = must_to_value(stringify!(Voice), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(Voice), &parsed);
}
#[test]
fn test_web_app_data_serialize_deserialize() {
    let value = serde_json::json!({ "data" : "test", "button_text" : "test" });
    let parsed: WebAppData = must_parse(stringify!(WebAppData), &value);
    let parsed_value = must_to_value(stringify!(WebAppData), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(WebAppData), &parsed);
}
#[test]
fn test_web_app_info_serialize_deserialize() {
    let value = serde_json::json!({ "url" : "test" });
    let parsed: WebAppInfo = must_parse(stringify!(WebAppInfo), &value);
    let parsed_value = must_to_value(stringify!(WebAppInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(WebAppInfo), &parsed);
}
#[test]
fn test_webhook_info_serialize_deserialize() {
    let value = serde_json::json!(
        { "url" : "test", "has_custom_certificate" : true, "pending_update_count" : 1 }
    );
    let parsed: WebhookInfo = must_parse(stringify!(WebhookInfo), &value);
    let parsed_value = must_to_value(stringify!(WebhookInfo), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(WebhookInfo), &parsed);
}
#[test]
fn test_write_access_allowed_serialize_deserialize() {
    let value = serde_json::json!({});
    let parsed: WriteAccessAllowed = must_parse(stringify!(WriteAccessAllowed), &value);
    let parsed_value = must_to_value(stringify!(WriteAccessAllowed), &parsed);
    assert_json_subset(&parsed_value, &value);
    must_roundtrip(stringify!(WriteAccessAllowed), &parsed);
}
