use telers_codegen::parser::api::{NormalizedSchema, Schema, SubtypeKind, TypeKindInField};

fn schema() -> NormalizedSchema {
    Schema::parse_from_json(include_str!("../schemas/api.json"))
        .unwrap()
        .normalize()
}

#[test]
fn new_message_payloads_have_distinct_required_variants() {
    let mut schema = schema();
    schema.split_message_types();

    for (variant, field_name) in [
        ("Widget", "widget"),
        ("PaidMessagesWaived", "paid_messages_waived"),
        ("TonWalletTransfer", "ton_wallet_transfer"),
    ] {
        let type_name = format!("Message{variant}");
        assert!(
            schema.types["Message"]
                .subtypes
                .iter()
                .any(|subtype| { subtype.variant == variant && subtype.ty_name == type_name })
        );

        for subtype in &schema.types["Message"].subtypes {
            let field = schema.types[&subtype.ty_name]
                .fields
                .iter()
                .find(|field| field.name == field_name);
            if subtype.ty_name == type_name {
                assert!(field.unwrap().required, "{type_name}.{field_name}");
            } else {
                assert!(
                    field.is_none(),
                    "{field_name} leaked into {}",
                    subtype.ty_name
                );
            }
        }
    }
}

#[test]
fn bank_card_and_ton_entities_are_known_tagged_variants() {
    let mut schema = schema();
    schema.split_message_entity_types();

    for variant in ["BankCardNumber", "TonAddress"] {
        let type_name = format!("MessageEntity{variant}");
        assert!(
            schema.types["MessageEntity"]
                .subtypes
                .iter()
                .any(|subtype| { subtype.variant == variant && subtype.ty_name == type_name })
        );
        let ty = &schema.types[&type_name];
        for name in ["offset", "length"] {
            assert!(
                ty.fields
                    .iter()
                    .any(|field| field.name == name && field.required)
            );
        }
        assert!(matches!(
            &ty.subtype_kind,
            Some(SubtypeKind::Tagged { tag_field, .. }) if tag_field == "type"
        ));
    }
}

#[test]
fn new_tagged_types_keep_their_discriminators_and_widget_query_type_is_an_object() {
    let schema = schema();
    for (name, variants) in [
        (
            "WidgetQueryType",
            &["Data", "SignData", "SentTransaction"][..],
        ),
        ("ChatShareTheme", &["Emoji", "UniqueGift"][..]),
        ("EmojiType", &["Emoji", "CustomEmoji"][..]),
    ] {
        let ty = &schema.types[name];
        assert!(matches!(
            &ty.subtype_kind,
            Some(SubtypeKind::Tagged { tag_field, .. }) if tag_field == "type"
        ));
        assert_eq!(ty.subtypes.len(), variants.len());
        for variant in variants {
            assert!(
                ty.subtypes
                    .iter()
                    .any(|subtype| subtype.variant == *variant)
            );
        }
    }

    for name in ["WidgetQuery", "SharedWidgetQuery"] {
        let ty = &schema.types[name];
        assert!(ty.subtype_kind.is_none());
        let field = ty.fields.iter().find(|field| field.name == "type").unwrap();
        assert!(field.required);
        assert_eq!(
            field.r#type,
            TypeKindInField::Telegram("WidgetQueryType".to_owned())
        );
    }
}

#[test]
fn wallet_card_and_share_theme_follow_chat_applicability() {
    let mut schema = schema();
    schema.split_chat_types();

    for (name, has_wallet, has_theme) in [
        ("ChatFullInfoPrivate", true, true),
        ("ChatFullInfoGroup", false, false),
        ("ChatFullInfoSupergroup", false, true),
        ("ChatFullInfoChannel", false, true),
    ] {
        let fields = &schema.types[name].fields;
        assert_eq!(
            fields.iter().any(|field| field.name == "ton_wallet_card"),
            has_wallet,
            "{name}"
        );
        assert_eq!(
            fields.iter().any(|field| field.name == "share_theme"),
            has_theme,
            "{name}"
        );
    }
}
