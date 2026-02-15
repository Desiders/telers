use quote::format_ident;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, mem};
use syn::{punctuated::Punctuated, Path, PathSegment};

use crate::generator::helpers::snake_to_upper_camel;

/// [`TelegramTypeName`] is a string that is used to identify a type, for example `Chat` or `User`.
pub type TelegramTypeName = String;
pub type FieldName = String;
pub type RawType = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub types: Vec<RawType>,
}

impl Field {
    /// # Panis
    /// * If the field has multiple types, but it's not a known special case.
    pub fn identify_field_type(&self) -> TypeKindInField {
        let types = self.types.as_slice();

        if multi_type_is_input_file(types) {
            return TypeKindInField::InputFile;
        }

        if multi_type_is_chat_id(types) {
            return TypeKindInField::ChatId;
        }

        if multi_type_is_reply_markup(types, &self.name) {
            return TypeKindInField::Telegram("ReplyMarkup".to_owned());
        }

        if types.len() > 1 {
            unimplemented!("Unknown case for multi types");
        }

        let r#type = types.first().unwrap();

        if is_array_of(r#type) {
            let inner_type = Field {
                name: self.name.clone(),
                required: self.required,
                description: self.description.clone(),
                types: vec![r#type.replacen("Array of ", "", 1)],
            }
            .identify_field_type();

            return TypeKindInField::Array(Box::new(inner_type));
        }

        if is_string(r#type) {
            return TypeKindInField::String;
        }

        if let Some(integer_kind) = get_if_integer(r#type, &self.description) {
            return TypeKindInField::Integer(integer_kind);
        }

        if let Some(boolean_kind) = get_if_boolean(r#type) {
            return TypeKindInField::Boolean(boolean_kind);
        }

        TypeKindInField::Telegram(r#type.to_owned())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    #[serde(default)]
    pub fields: Vec<Field>,
    #[serde(default)]
    pub subtypes: Vec<TelegramTypeName>,
    #[serde(default)]
    pub subtype_of: Vec<TelegramTypeName>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Schema {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub types: HashMap<TelegramTypeName, Type>,
}

impl Schema {
    pub fn parse_from_jsom(content: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(content)
    }

    fn detect_subtype_kind(&self, subtypes: &[TelegramTypeName]) -> Option<SubtypeKind> {
        if subtypes.is_empty() {
            return None;
        }

        let Some(first_ty) = subtypes.first().and_then(|name| self.types.get(name)) else {
            return Some(SubtypeKind::Untagged);
        };

        let candidates: Vec<&str> = first_ty
            .fields
            .iter()
            .filter(|val| {
                val.required
                    && val.types == ["String"]
                    && (val.description.contains("always") || val.description.contains("must be"))
            })
            .map(|val| val.name.as_str())
            .collect();

        for candidate in candidates {
            let all_have_it = subtypes.iter().skip(1).all(|subtype_name| {
                let Some(subtype) = self.types.get(subtype_name) else {
                    return false;
                };
                subtype.fields.iter().any(|val| {
                    val.name == candidate
                        && val.required
                        && val.types == ["String"]
                        && (val.description.contains("always")
                            || val.description.contains("must be"))
                })
            });

            if all_have_it {
                return Some(SubtypeKind::Tagged {
                    tag_field: candidate.to_owned(),
                });
            }
        }

        Some(SubtypeKind::Untagged)
    }

    pub fn normalize(self) -> NormalizedSchema {
        let mut subtype_kinds = HashMap::new();
        for (name, ty) in &self.types {
            if let Some(subtype_kind) = self.detect_subtype_kind(&ty.subtypes) {
                subtype_kinds.insert(name.clone(), subtype_kind.clone());
                for subtype in ty.subtypes.clone() {
                    subtype_kinds.insert(subtype, subtype_kind.clone());
                }
            }
        }

        let mut normalized_types = HashMap::new();
        for (name, ty) in self.types {
            let mut fields = vec![];
            for field in ty.fields {
                let field_type = field.identify_field_type();
                let is_recursive = match field_type {
                    TypeKindInField::Telegram(ref ty) => *ty == name,
                    _ => false,
                };
                let is_boxed = match field_type {
                    TypeKindInField::Telegram(ref name) => match name.as_str() {
                        "Message"
                        | "MaybeInaccessibleMessage"
                        | "User"
                        | "Chat"
                        | "PhotoSize"
                        | "Animation"
                        | "Document"
                        | "Sticker"
                        | "Video"
                        | "Audio"
                        | "Venue"
                        | "VideoNote"
                        | "Voice"
                        | "Poll"
                        | "Invoice"
                        | "Location"
                        | "Contact"
                        | "Game"
                        | "Gift"
                        | "UniqueGift"
                        | "GiftInfo"
                        | "UniqueGiftInfo"
                        | "GiveawayWinners"
                        | "Giveaway"
                        | "ExternalReplyInfo"
                        | "ShippingAddress"
                        | "SuccessfulPayment" => true,
                        _ => false,
                    },
                    _ => false,
                };

                fields.push(NormalizedField {
                    name: field.name,
                    required: field.required,
                    description: field.description,
                    r#type: field_type,
                    is_recursive,
                    is_boxed,
                });
            }
            let subtype_kind = subtype_kinds.remove(&name);
            let subtypes = ty
                .subtypes
                .into_iter()
                .map(|name| NormalizedSubtypeVariant {
                    variant: name.clone(),
                    name,
                })
                .collect();
            let ty = NormalizedType {
                name: ty.name,
                href: ty.href,
                description: ty.description,
                fields,
                subtype_kind,
                subtypes,
                subtype_of: ty.subtype_of,
            };
            normalized_types.insert(name, ty);
        }

        let schema = NormalizedSchema {
            version: self.version,
            release_date: self.release_date,
            changelog: self.changelog,
            types: normalized_types,
        };
        schema
    }

    pub fn is_telegram_type(&self, raw_type: &RawType) -> bool {
        self.types.contains_key(raw_type)
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedField {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub r#type: TypeKindInField,
    pub is_recursive: bool,
    pub is_boxed: bool,
}

#[derive(Debug, Clone)]
pub enum SubtypeKind {
    Tagged { tag_field: String },
    Untagged,
}

#[derive(Debug)]
pub struct NormalizedSubtypeVariant {
    pub variant: TelegramTypeName,
    pub name: TelegramTypeName,
}

#[derive(Debug)]
pub struct NormalizedType {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    pub fields: Vec<NormalizedField>,
    pub subtype_kind: Option<SubtypeKind>,
    pub subtypes: Vec<NormalizedSubtypeVariant>,
    pub subtype_of: Vec<TelegramTypeName>,
}

impl NormalizedType {
    pub fn get_paths(&self) -> Vec<Path> {
        let mut paths = vec![];
        for field in &self.fields {
            if let Some(path) = field.r#type.get_path() {
                paths.push(path);
            }
        }
        for subtype in &self.subtypes {
            let mut segments = Punctuated::new();
            segments.push(PathSegment::from(format_ident!("{}", subtype.name)));
            let path = Path {
                leading_colon: None,
                segments,
            };
            paths.push(path);
        }
        paths
    }

    pub fn get_paths_count(&self) -> usize {
        self.get_paths().len()
    }
}

#[derive(Debug, Default)]
pub struct NormalizedSchema {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub types: HashMap<TelegramTypeName, NormalizedType>,
}

impl NormalizedSchema {
    pub fn split_message_types(&mut self) {
        let mut message = self
            .types
            .remove("Message")
            .expect("Message doesn't exist in schema");
        let content_fields = [
            "text",
            "animation",
            "audio",
            "document",
            "paid_media",
            "photo",
            "sticker",
            "story",
            "video",
            "video_note",
            "voice",
            "checklist",
            "contact",
            "dice",
            "game",
            "poll",
            "venue",
            "location",
        ];
        let service_fields = [
            "new_chat_members",
            "left_chat_member",
            "chat_owner_left",
            "chat_owner_changed",
            "new_chat_title",
            "new_chat_photo",
            "delete_chat_photo",
            "group_chat_created",
            "supergroup_chat_created",
            "channel_chat_created",
            "message_auto_delete_timer_changed",
            "migrate_to_chat_id",
            "migrate_from_chat_id",
            "pinned_message",
            "invoice",
            "successful_payment",
            "refunded_payment",
            "users_shared",
            "chat_shared",
            "gift",
            "unique_gift",
            "gift_upgrade_sent",
            "connected_website",
            "write_access_allowed",
            "passport_data",
            "proximity_alert_triggered",
            "boost_added",
            "chat_background_set",
            "checklist_tasks_done",
            "checklist_tasks_added",
            "direct_message_price_changed",
            "forum_topic_created",
            "forum_topic_edited",
            "forum_topic_closed",
            "forum_topic_reopened",
            "general_forum_topic_hidden",
            "general_forum_topic_unhidden",
            "giveaway_created",
            "giveaway",
            "giveaway_winners",
            "giveaway_completed",
            "paid_message_price_changed",
            "suggested_post_approved",
            "suggested_post_approval_failed",
            "suggested_post_declined",
            "suggested_post_paid",
            "suggested_post_refunded",
            "video_chat_scheduled",
            "video_chat_started",
            "video_chat_ended",
            "video_chat_participants_invited",
            "web_app_data",
        ];

        let mut common_fields = vec![];
        let mut content_fields_map = HashMap::new();
        let mut service_fields_map = HashMap::new();

        for field in mem::take(&mut message.fields) {
            let field_name = field.name.as_str();
            if content_fields.contains(&field_name) {
                content_fields_map.insert(field_name.to_owned(), field);
            } else if service_fields.contains(&field_name) {
                service_fields_map.insert(field_name.to_owned(), field);
            } else {
                common_fields.push(field);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for (field_name, mut field) in content_fields_map {
            let variant_name = snake_to_upper_camel(&field_name);
            let type_name = format!("{}{variant_name}", message.name);
            let description = vec![
                field.description.clone(),
                "# Notes".to_owned(),
                format!(
                    "This object represents a message from original message field `{field_name}`."
                ),
            ];

            field.required = true;

            let mut fields = vec![];
            fields.extend(common_fields.clone());
            fields.push(field);
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: message.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                subtype_of: vec![message.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        for (field_name, mut field) in service_fields_map {
            let variant_name = snake_to_upper_camel(&field_name);
            let type_name = format!("{}{variant_name}", message.name);
            let description = vec![
                field.description.clone(),
                "# Notes".to_owned(),
                format!("This object represents a service message from original message field `{field_name}`."),
            ];

            field.required = true;

            let mut fields = vec![];
            fields.extend(common_fields.clone());
            fields.push(field);
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: message.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                subtype_of: vec![message.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        message
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            message.description.push(format!("- {name}"));
        }
        message.subtype_kind = Some(SubtypeKind::Untagged);
        message.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(message.name.clone(), message);
        self.types.extend(types);
    }

    pub fn split_chat_types(&mut self) {
        self.split_chat_type("Chat");
        self.split_chat_type("ChatFullInfo");
    }

    pub fn split_chat_type(&mut self, type_name: &str) {
        let mut chat = self
            .types
            .remove(type_name)
            .expect("Chat type doesn't exist in schema");

        let chat_types = ["private", "group", "supergroup", "channel"];

        let mut common_fields = vec![];
        let mut type_fields_map = HashMap::new();

        for field in mem::take(&mut chat.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("private") {
                applicable.push("private");
            }
            if desc.contains("group") && !desc.contains("supergroup") {
                applicable.push("group");
            }
            if desc.contains("supergroup") {
                applicable.push("supergroup");
            }
            if desc.contains("channel") {
                applicable.push("channel");
            }
            if applicable.is_empty() {
                applicable.extend(chat_types);
            }

            let is_optional = desc.contains("if available");
            for chat_type in &applicable {
                let mut field = field.clone();
                if is_optional {
                    field.required = false;
                } else {
                    field.required = true;
                }
                type_fields_map
                    .entry(chat_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field);
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for chat_type in chat_types {
            let variant_name = snake_to_upper_camel(chat_type);
            let type_name = format!("{}{variant_name}", chat.name);
            let description = vec![
                format!("This object represents a {chat_type} chat."),
                "# Notes".to_owned(),
                format!("This object represents a chat from original chat type `{chat_type}`."),
            ];

            let mut fields = vec![];
            fields.extend(common_fields.clone());
            if let Some(specific) = type_fields_map.get(chat_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: chat.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Tagged {
                    tag_field: "type".to_owned(),
                }),
                subtypes: vec![],
                subtype_of: vec![chat.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        chat.description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            chat.description.push(format!("- {name}"));
        }
        chat.subtype_kind = Some(SubtypeKind::Tagged {
            tag_field: "type".to_owned(),
        });
        chat.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(chat.name.clone(), chat);
        self.types.extend(types);
    }

    pub fn split_sticker_types(&mut self) {
        let mut sticker = self
            .types
            .remove("Sticker")
            .expect("Sticker doesn't exist in schema");

        let sticker_types = ["regular", "mask", "custom_emoji"];

        let mut common_fields = vec![];
        let mut type_fields_map = HashMap::new();

        for field in mem::take(&mut sticker.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("regular") {
                applicable.push("regular");
            }
            if desc.contains("mask") {
                applicable.push("mask");
            }
            if desc.contains("custom_emoji") {
                applicable.push("custom_emoji");
            }
            if applicable.is_empty() {
                applicable.extend(sticker_types);
            }

            for sticker_type in &applicable {
                type_fields_map
                    .entry(sticker_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field.clone());
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for sticker_type in sticker_types {
            let variant_name = snake_to_upper_camel(sticker_type);
            let type_name = format!("{}{variant_name}", sticker.name);
            let description = vec![
                format!("This object represents a {} sticker.", sticker_type),
                "# Notes".to_owned(),
                format!(
                    "This object represents a sticker from original sticker type `{sticker_type}`."
                ),
            ];

            let mut fields = vec![];
            fields.extend(common_fields.clone());
            if let Some(specific) = type_fields_map.get(sticker_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: sticker.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Tagged {
                    tag_field: "type".to_owned(),
                }),
                subtypes: vec![],
                subtype_of: vec![sticker.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        sticker
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            sticker.description.push(format!("- {name}"));
        }
        sticker.subtype_kind = Some(SubtypeKind::Tagged {
            tag_field: "type".to_owned(),
        });
        sticker.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(sticker.name.clone(), sticker);
        self.types.extend(types);
    }

    pub fn split_poll_types(&mut self) {
        let mut poll = self
            .types
            .remove("Poll")
            .expect("Poll doesn't exist in schema");

        let poll_types = ["regular", "quiz"];

        let mut common_fields = vec![];
        let mut type_fields_map = HashMap::new();

        for field in mem::take(&mut poll.fields) {
            if field.name == "type" {
                common_fields.push(field);
                continue;
            }

            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("regular") {
                applicable.push("regular");
            }
            if desc.contains("quiz") {
                applicable.push("quiz");
            }
            if applicable.is_empty() {
                applicable.extend(poll_types);
            }

            for poll_type in &applicable {
                type_fields_map
                    .entry(poll_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field.clone());
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for poll_type in poll_types {
            let variant_name = snake_to_upper_camel(poll_type);
            let type_name = format!("{}{variant_name}", poll.name);
            let description = vec![
                format!("This object represents a {} poll.", poll_type),
                "# Notes".to_owned(),
                format!("This object represents a poll from original poll type `{poll_type}`."),
            ];

            let mut fields = vec![];
            fields.extend(common_fields.clone());
            if let Some(specific) = type_fields_map.get(poll_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: poll.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Tagged {
                    tag_field: "type".to_owned(),
                }),
                subtypes: vec![],
                subtype_of: vec![poll.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        poll.description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            poll.description.push(format!("- {name}"));
        }
        poll.subtype_kind = Some(SubtypeKind::Tagged {
            tag_field: "type".to_owned(),
        });
        poll.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(poll.name.clone(), poll);
        self.types.extend(types);
    }

    pub fn split_giveaway_types(&mut self) {
        let mut giveaway = self
            .types
            .remove("Giveaway")
            .expect("Giveaway doesn't exist in schema");

        let giveaway_types = ["star", "premium"];

        let mut type_fields_map = HashMap::new();

        for mut field in mem::take(&mut giveaway.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("telegram star") || field.name == "prize_star_count" {
                field.required = true;
                applicable.push("star");
            }
            if desc.contains("telegram premium") || field.name == "premium_subscription_month_count"
            {
                field.required = true;
                applicable.push("premium");
            }
            if applicable.is_empty() {
                applicable.extend(giveaway_types);
            }

            for giveaway_type in &applicable {
                type_fields_map
                    .entry(giveaway_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field.clone());
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for giveaway_type in giveaway_types {
            let variant_name = snake_to_upper_camel(giveaway_type);
            let type_name = format!("{}{variant_name}", giveaway.name);
            let description = vec![
                format!("This object represents a {giveaway_type} giveaway."),
                "# Notes".to_owned(),
                format!("This object represents a giveaway from original giveaway type `{giveaway_type}`."),
            ];

            let mut fields = vec![];
            if let Some(specific) = type_fields_map.get(giveaway_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: giveaway.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                subtype_of: vec![giveaway.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        giveaway
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            giveaway.description.push(format!("- {name}"));
        }
        giveaway.subtype_kind = Some(SubtypeKind::Untagged);
        giveaway.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(giveaway.name.clone(), giveaway);
        self.types.extend(types);
    }

    pub fn split_giveaway_winners_types(&mut self) {
        let mut winners = self
            .types
            .remove("GiveawayWinners")
            .expect("GiveawayWinners doesn't exist in schema");

        let winners_types = ["star", "premium"];

        let mut type_fields_map = HashMap::new();

        for mut field in mem::take(&mut winners.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("telegram star") || field.name == "prize_star_count" {
                field.required = true;
                applicable.push("star");
            }
            if desc.contains("telegram premium") || field.name == "premium_subscription_month_count"
            {
                field.required = true;
                applicable.push("premium");
            }
            if applicable.is_empty() {
                applicable.extend(winners_types);
            }

            for winners_type in &applicable {
                type_fields_map
                    .entry(winners_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field.clone());
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for winners_type in winners_types {
            let variant_name = snake_to_upper_camel(winners_type);
            let type_name = format!("{}{variant_name}", winners.name);
            let description = vec![
                format!("This object represents a {winners_type} giveaway winners."),
                "# Notes".to_owned(),
                format!(
                    "This object represents giveaway winners from original type `{winners_type}`."
                ),
            ];

            let mut fields = vec![];
            if let Some(specific) = type_fields_map.get(winners_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: winners.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                subtype_of: vec![winners.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        winners
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            winners.description.push(format!("- {name}"));
        }
        winners.subtype_kind = Some(SubtypeKind::Untagged);
        winners.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(winners.name.clone(), winners);
        self.types.extend(types);
    }

    pub fn split_star_transaction_types(&mut self) {
        let mut transaction = self
            .types
            .remove("StarTransaction")
            .expect("StarTransaction doesn't exist in schema");

        let transaction_types = ["incoming", "outgoing"];

        let mut type_fields_map = HashMap::new();

        for mut field in mem::take(&mut transaction.fields) {
            let desc = field.description.to_lowercase();
            let mut applicable = vec![];

            if desc.contains("source of an incoming transaction") || field.name == "source" {
                field.required = true;
                applicable.push("incoming");
            }
            if desc.contains("receiver of an outgoing transaction") || field.name == "receiver" {
                field.required = true;
                applicable.push("outgoing");
            }
            if applicable.is_empty() {
                applicable.extend(transaction_types);
            }

            for transaction_type in &applicable {
                type_fields_map
                    .entry(transaction_type.to_owned())
                    .or_insert_with(Vec::new)
                    .push(field.clone());
            }
        }

        let mut types = HashMap::new();
        let mut subtypes = vec![];

        for transaction_type in transaction_types {
            let variant_name = snake_to_upper_camel(transaction_type);
            let type_name = format!("{}{variant_name}", transaction.name);
            let description = vec![
                format!("This object represents an {} Star transaction.", transaction_type),
                "# Notes".to_owned(),
                format!("This object represents a Star transaction from original type `{transaction_type}`."),
            ];

            let mut fields = vec![];
            if let Some(specific) = type_fields_map.get(transaction_type) {
                fields.extend(specific.clone());
            }
            fields.sort_by(|a, b| match (a.required, b.required) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });

            let subtype = NormalizedType {
                name: type_name.clone(),
                href: transaction.href.clone(),
                description,
                fields,
                subtype_kind: Some(SubtypeKind::Untagged),
                subtypes: vec![],
                subtype_of: vec![transaction.name.clone()],
            };

            subtypes.push((variant_name, type_name.clone()));
            types.insert(type_name, subtype);
        }

        transaction
            .description
            .push("Currently, it can be one of".to_owned());
        for (_, name) in &subtypes {
            transaction.description.push(format!("- {name}"));
        }
        transaction.subtype_kind = Some(SubtypeKind::Untagged);
        transaction.subtypes = subtypes
            .into_iter()
            .map(|(variant, name)| NormalizedSubtypeVariant { variant, name })
            .collect();

        self.types.insert(transaction.name.clone(), transaction);
        self.types.extend(types);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerKind {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
}

/// # Variants
/// - `Any` - Any boolean value
/// - `True` - Only possible value is `true`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanKind {
    Any,
    True,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKindInField {
    String,
    InputFile,
    ChatId,
    Integer(IntegerKind),
    Boolean(BooleanKind),
    Telegram(TelegramTypeName),
    Array(Box<TypeKindInField>),
}

impl TypeKindInField {
    pub fn get_path(&self) -> Option<Path> {
        match self {
            TypeKindInField::InputFile => {
                Some(syn::parse_str("super::InputFile").expect("incorrect path"))
            }
            TypeKindInField::ChatId => {
                Some(syn::parse_str("super::ChatIdKind").expect("incorrect path"))
            }
            TypeKindInField::Telegram(name) => {
                Some(syn::parse_str(&format!("super::{name}")).expect("incorrect path"))
            }
            TypeKindInField::Array(kind) => kind.get_path(),
            _ => None,
        }
    }

    pub fn is_copy(&self) -> bool {
        matches!(
            self,
            TypeKindInField::Integer(_) | TypeKindInField::Boolean(_)
        )
    }
}

pub fn is_string(raw_type: &RawType) -> bool {
    raw_type == "String"
}

pub fn get_if_integer(raw_type: &RawType, description: &str) -> Option<IntegerKind> {
    match raw_type.as_str() {
        "Integer" => {
            if let Some((min, max)) = extract_range(description) {
                if min < 0 {
                    if max <= i8::MAX as i64 {
                        Some(IntegerKind::Int8)
                    } else if max <= i16::MAX as i64 {
                        Some(IntegerKind::Int16)
                    } else if max <= i32::MAX as i64 {
                        Some(IntegerKind::Int32)
                    } else {
                        Some(IntegerKind::Int64)
                    }
                } else {
                    if max <= u8::MAX as i64 {
                        Some(IntegerKind::UInt8)
                    } else if max <= u16::MAX as i64 {
                        Some(IntegerKind::UInt16)
                    } else if max <= u32::MAX as i64 {
                        Some(IntegerKind::UInt32)
                    } else {
                        Some(IntegerKind::UInt64)
                    }
                }
            } else {
                Some(IntegerKind::Int64)
            }
        }
        "Float" => Some(IntegerKind::Float64),
        _ => None,
    }
}

fn extract_range(description: &str) -> Option<(i64, i64)> {
    let doc = description.to_lowercase();
    let patterns = [
        // from -999 to 999, between -999 and 999, 1-100, 1 to 100, etc.
        r"(?:from|between|must be)?\s*([-]?\d+)\s*(?:-|to|and)\s*([-]?\d+)",
    ];
    for pattern in patterns {
        let re = Regex::new(pattern).ok()?;
        if let Some(caps) = re.captures(&doc) {
            let min: i64 = caps[1].parse().ok()?;
            let max: i64 = caps[2].parse().ok()?;

            if min <= max {
                return Some((min, max));
            }
        }
    }
    None
}

/// # Notes
/// Currently use only [`BooleanKind::Any`] and [`BooleanKind::True`].
/// Type like `False` is not used in the Telegram API.
pub fn get_if_boolean(raw_type: &RawType) -> Option<BooleanKind> {
    match raw_type.as_str() {
        "Boolean" => Some(BooleanKind::Any),
        "True" => Some(BooleanKind::True),
        _ => None,
    }
}

/// # Notes
/// All arrays in the Telegram API are starts with `Array of` prefix.
pub fn is_array_of(raw_type: &RawType) -> bool {
    raw_type.starts_with("Array of")
}

/// If the type is an array with `Integer` and `String` then it's just `InputFile`,
/// because all possible `String` files reresentations are wrapped in `InputFile`.
/// # Notes
/// This function is a special case for `InputFile` type.
pub fn multi_type_is_input_file(types: &[RawType]) -> bool {
    if types.len() == 1 && types.contains(&"InputFile".to_owned()) {
        return true;
    }

    if types.len() != 2 {
        return false;
    }

    types.contains(&"InputFile".to_owned()) && types.contains(&"String".to_owned())
}

/// If the type is an array with `Integer` and `String` then it's just `ChatId`.
/// # Notes
/// `ChatId` is a helper type that can be represented as `Integer` or `String`.
///
/// This function is a special case for `ChatId` type.
pub fn multi_type_is_chat_id(types: &[RawType]) -> bool {
    if types.len() != 2 {
        return false;
    }

    types.contains(&"Integer".to_owned()) && types.contains(&"String".to_owned())
}

/// If the type is an array with `InlineKeyboardMarkup` and `ReplyKeyboardMarkup`, etc., then it's just `ReplyMarkup`.
/// If it's array with one type, then it's not `ReplyMarkup`.
/// # Notes
/// This function is a special case for `ReplyMarkup` type.
/// # Warnings
/// Here not checks that types are markup types, because if `name` is `reply_markup` then it's a markup type: single or multi.
pub fn multi_type_is_reply_markup(types: &[RawType], name: &str) -> bool {
    if types.len() == 1 {
        return false;
    }

    name == "reply_markup"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_to_schema() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_jsom(content).unwrap();

        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.release_date, "2021-01-01");
        assert_eq!(schema.changelog, "Initial release");
        assert_eq!(schema.types.len(), 1);
    }

    #[test]
    fn test_is_telegram_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_jsom(content).unwrap();

        assert!(schema.is_telegram_type(&"Type1".to_owned()));
        assert!(!schema.is_telegram_type(&"Type2".to_owned()));
    }

    #[test]
    fn test_is_string() {
        assert!(is_string(&"String".to_owned()));
        assert!(!is_string(&"Integer".to_owned()));
    }

    #[test]
    fn test_get_if_integer() {
        assert_eq!(
            get_if_integer(&"Integer".to_owned(), ""),
            Some(IntegerKind::Int64)
        );
        assert_eq!(
            get_if_integer(&"Float".to_owned(), ""),
            Some(IntegerKind::Float32)
        );
        assert_eq!(get_if_integer(&"String".to_owned(), ""), None);
    }

    #[test]
    fn test_get_if_boolean() {
        assert_eq!(
            get_if_boolean(&"Boolean".to_owned()),
            Some(BooleanKind::Any)
        );
        assert_eq!(get_if_boolean(&"True".to_owned()), Some(BooleanKind::True));
        assert_eq!(get_if_boolean(&"String".to_owned()), None);
    }

    #[test]
    fn test_is_array_of() {
        assert!(is_array_of(&"Array of String".to_owned()));
        assert!(is_array_of(&"Array of Array of String".to_owned()));
        assert!(!is_array_of(&"String".to_owned()));
    }

    #[test]
    fn test_multi_type_is_input_file() {
        assert!(multi_type_is_input_file(&["InputFile".to_owned()]));
        assert!(multi_type_is_input_file(&[
            "InputFile".to_owned(),
            "String".to_owned(),
        ]));
        assert!(multi_type_is_input_file(&[
            "String".to_owned(),
            "InputFile".to_owned(),
        ]));
        assert!(!multi_type_is_input_file(&["String".to_owned()]));
        assert!(!multi_type_is_input_file(&[
            "String".to_owned(),
            "Integer".to_owned(),
        ]));
    }

    #[test]
    fn test_multi_type_is_chat_id() {
        assert!(!multi_type_is_chat_id(&["Integer".to_owned()]));
        assert!(!multi_type_is_chat_id(&["String".to_owned()]));
        assert!(multi_type_is_chat_id(&[
            "String".to_owned(),
            "Integer".to_owned(),
        ]));
        assert!(multi_type_is_chat_id(&[
            "Integer".to_owned(),
            "String".to_owned(),
        ]));
        assert!(!multi_type_is_chat_id(&["InputFile".to_owned()]));
        assert!(!multi_type_is_chat_id(&[
            "InputFile".to_owned(),
            "String".to_owned(),
        ]));
    }

    #[test]
    fn test_multi_type_is_reply_markup() {
        assert!(!multi_type_is_reply_markup(
            &["String".to_owned()],
            "reply_markup"
        ));
        assert!(!multi_type_is_reply_markup(
            &["InlineKeyboardMarkup".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &["Markup1".to_owned(), "Markup2".to_owned()],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "InlineKeyboardMarkup".to_owned(),
                "ReplyKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
        assert!(multi_type_is_reply_markup(
            &[
                "ReplyKeyboardMarkup".to_owned(),
                "InlineKeyboardMarkup".to_owned(),
            ],
            "reply_markup"
        ));
    }

    #[test]
    fn test_identify_field_type() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Type1": {
                        "name": "Type1",
                        "href": "https://example.com",
                        "description": ["Type1 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["Integer"]
                            },
                            {
                                "name": "field3",
                                "required": true,
                                "description": "Field3 description",
                                "types": ["Boolean"]
                            },
                            {
                                "name": "field4",
                                "required": true,
                                "description": "Field4 description",
                                "types": ["Float"]
                            },
                            {
                                "name": "field5",
                                "required": true,
                                "description": "Field5 description",
                                "types": ["Array of String"]
                            },
                            {
                                "name": "field6",
                                "required": true,
                                "description": "Field6 description",
                                "types": ["Array of Array of String"]
                            },
                            {
                                "name": "field7",
                                "required": true,
                                "description": "Field7 description",
                                "types": ["Array of Integer"]
                            },
                            {
                                "name": "field8",
                                "required": true,
                                "description": "Field8 description",
                                "types": ["Array of Float"]
                            },
                            {
                                "name": "field9",
                                "required": true,
                                "description": "Field9 description",
                                "types": ["True"]
                            },
                            {
                                "name": "field10",
                                "required": true,
                                "description": "Field10 description",
                                "types": ["Type1"]
                            }
                        ]
                    },
                    "Type2": {
                        "name": "Type2",
                        "href": "https://example.com",
                        "description": ["Type2 description"],
                        "fields": [
                            {
                                "name": "field1",
                                "required": true,
                                "description": "Field1 description",
                                "types": ["InputFile", "String"]
                            },
                            {
                                "name": "field2",
                                "required": true,
                                "description": "Field2 description",
                                "types": ["InputFile"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["InlineKeyboardMarkup", "ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "reply_markup",
                                "required": true,
                                "description": "Field description",
                                "types": ["ReplyKeyboardMarkup"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer", "String"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["String", "Integer"]
                            },
                            {
                                "name": "chat_id",
                                "required": true,
                                "description": "Field description",
                                "types": ["Integer"]
                            }
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_jsom(content).unwrap();

        let fields = schema.types.get("Type1").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::String
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64)
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::Any)
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Float32)
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::String))
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Array(Box::new(
                TypeKindInField::String
            ))))
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Int64))),
        );
        assert_eq!(
            fields.get(7).unwrap().identify_field_type(),
            TypeKindInField::Array(Box::new(TypeKindInField::Integer(IntegerKind::Float32))),
        );
        assert_eq!(
            fields.get(8).unwrap().identify_field_type(),
            TypeKindInField::Boolean(BooleanKind::True),
        );
        assert_eq!(
            fields.get(9).unwrap().identify_field_type(),
            TypeKindInField::Telegram("Type1".to_owned()),
        );

        let fields = schema.types.get("Type2").unwrap().fields.as_slice();

        assert_eq!(
            fields.get(0).unwrap().identify_field_type(),
            TypeKindInField::InputFile,
        );
        assert_eq!(
            fields.get(1).unwrap().identify_field_type(),
            TypeKindInField::InputFile
        );
        assert_eq!(
            fields.get(2).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyMarkup".to_owned())
        );
        assert_eq!(
            fields.get(3).unwrap().identify_field_type(),
            TypeKindInField::Telegram("ReplyKeyboardMarkup".to_owned()),
        );
        assert_eq!(
            fields.get(4).unwrap().identify_field_type(),
            TypeKindInField::ChatId,
        );
        assert_eq!(
            fields.get(5).unwrap().identify_field_type(),
            TypeKindInField::ChatId,
        );
        assert_eq!(
            fields.get(6).unwrap().identify_field_type(),
            TypeKindInField::Integer(IntegerKind::Int64),
        );
    }

    #[test]
    fn test_split_message_types_creates_new_types() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Message": {
                        "name": "Message",
                        "href": "https://core.telegram.org/bots/api#message",
                        "description": [
                            "This object represents a message."
                        ],
                        "fields": [
                            {
                            "name": "message_id",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent"
                            },
                            {
                            "name": "message_thread_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only"
                            },
                            {
                            "name": "direct_messages_topic",
                            "types": [
                                "DirectMessagesTopic"
                            ],
                            "required": false,
                            "description": "Optional. Information about the direct messages chat topic that contains the message"
                            },
                            {
                            "name": "from",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats"
                            },
                            {
                            "name": "sender_chat",
                            "types": [
                                "Chat"
                            ],
                            "required": false,
                            "description": "Optional. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats."
                            },
                            {
                            "name": "sender_boost_count",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. If the sender of the message boosted the chat, the number of boosts added by the user"
                            },
                            {
                            "name": "sender_business_bot",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account."
                            },
                            {
                            "name": "date",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Date the message was sent in Unix time. It is always a positive number, representing a valid date."
                            },
                            {
                            "name": "business_connection_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier."
                            },
                            {
                            "name": "chat",
                            "types": [
                                "Chat"
                            ],
                            "required": true,
                            "description": "Chat the message belongs to"
                            },
                            {
                            "name": "forward_origin",
                            "types": [
                                "MessageOrigin"
                            ],
                            "required": false,
                            "description": "Optional. Information about the original message for forwarded messages"
                            },
                            {
                            "name": "is_topic_message",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is sent to a topic in a forum supergroup or a private chat with the bot"
                            },
                            {
                            "name": "is_automatic_forward",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is a channel post that was automatically forwarded to the connected discussion group"
                            },
                            {
                            "name": "reply_to_message",
                            "types": [
                                "Message"
                            ],
                            "required": false,
                            "description": "Optional. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply."
                            },
                            {
                            "name": "external_reply",
                            "types": [
                                "ExternalReplyInfo"
                            ],
                            "required": false,
                            "description": "Optional. Information about the message that is being replied to, which may come from another chat or forum topic"
                            },
                            {
                            "name": "quote",
                            "types": [
                                "TextQuote"
                            ],
                            "required": false,
                            "description": "Optional. For replies that quote part of the original message, the quoted part of the message"
                            },
                            {
                            "name": "reply_to_story",
                            "types": [
                                "Story"
                            ],
                            "required": false,
                            "description": "Optional. For replies to a story, the original story"
                            },
                            {
                            "name": "reply_to_checklist_task_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Identifier of the specific checklist task that is being replied to"
                            },
                            {
                            "name": "via_bot",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. Bot through which the message was sent"
                            },
                            {
                            "name": "edit_date",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. Date the message was last edited in Unix time"
                            },
                            {
                            "name": "has_protected_content",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message can't be forwarded"
                            },
                            {
                            "name": "is_from_offline",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message"
                            },
                            {
                            "name": "is_paid_post",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited."
                            },
                            {
                            "name": "media_group_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. The unique identifier of a media message group this message belongs to"
                            },
                            {
                            "name": "author_signature",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator"
                            },
                            {
                            "name": "paid_star_count",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The number of Telegram Stars that were paid by the sender of the message to send it"
                            },
                            {
                            "name": "text",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. For text messages, the actual UTF-8 text of the message"
                            },
                            {
                            "name": "entities",
                            "types": [
                                "Array of MessageEntity"
                            ],
                            "required": false,
                            "description": "Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text"
                            },
                            {
                            "name": "link_preview_options",
                            "types": [
                                "LinkPreviewOptions"
                            ],
                            "required": false,
                            "description": "Optional. Options used for link preview generation for the message, if it is a text message and link preview options were changed"
                            },
                            {
                            "name": "suggested_post_info",
                            "types": [
                                "SuggestedPostInfo"
                            ],
                            "required": false,
                            "description": "Optional. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited."
                            },
                            {
                            "name": "effect_id",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Unique identifier of the message effect added to the message"
                            },
                            {
                            "name": "animation",
                            "types": [
                                "Animation"
                            ],
                            "required": false,
                            "description": "Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set"
                            },
                            {
                            "name": "audio",
                            "types": [
                                "Audio"
                            ],
                            "required": false,
                            "description": "Optional. Message is an audio file, information about the file"
                            },
                            {
                            "name": "document",
                            "types": [
                                "Document"
                            ],
                            "required": false,
                            "description": "Optional. Message is a general file, information about the file"
                            },
                            {
                            "name": "paid_media",
                            "types": [
                                "PaidMediaInfo"
                            ],
                            "required": false,
                            "description": "Optional. Message contains paid media; information about the paid media"
                            },
                            {
                            "name": "photo",
                            "types": [
                                "Array of PhotoSize"
                            ],
                            "required": false,
                            "description": "Optional. Message is a photo, available sizes of the photo"
                            },
                            {
                            "name": "sticker",
                            "types": [
                                "Sticker"
                            ],
                            "required": false,
                            "description": "Optional. Message is a sticker, information about the sticker"
                            },
                            {
                            "name": "story",
                            "types": [
                                "Story"
                            ],
                            "required": false,
                            "description": "Optional. Message is a forwarded story"
                            },
                            {
                            "name": "video",
                            "types": [
                                "Video"
                            ],
                            "required": false,
                            "description": "Optional. Message is a video, information about the video"
                            },
                            {
                            "name": "video_note",
                            "types": [
                                "VideoNote"
                            ],
                            "required": false,
                            "description": "Optional. Message is a video note, information about the video message"
                            },
                            {
                            "name": "voice",
                            "types": [
                                "Voice"
                            ],
                            "required": false,
                            "description": "Optional. Message is a voice message, information about the file"
                            },
                            {
                            "name": "caption",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Caption for the animation, audio, document, paid media, photo, video or voice"
                            },
                            {
                            "name": "caption_entities",
                            "types": [
                                "Array of MessageEntity"
                            ],
                            "required": false,
                            "description": "Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption"
                            },
                            {
                            "name": "show_caption_above_media",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the caption must be shown above the message media"
                            },
                            {
                            "name": "has_media_spoiler",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the message media is covered by a spoiler animation"
                            },
                            {
                            "name": "checklist",
                            "types": [
                                "Checklist"
                            ],
                            "required": false,
                            "description": "Optional. Message is a checklist"
                            },
                            {
                            "name": "contact",
                            "types": [
                                "Contact"
                            ],
                            "required": false,
                            "description": "Optional. Message is a shared contact, information about the contact"
                            },
                            {
                            "name": "dice",
                            "types": [
                                "Dice"
                            ],
                            "required": false,
                            "description": "Optional. Message is a dice with random value"
                            },
                            {
                            "name": "game",
                            "types": [
                                "Game"
                            ],
                            "required": false,
                            "description": "Optional. Message is a game, information about the game. More about games: https://core.telegram.org/bots/api#games"
                            },
                            {
                            "name": "poll",
                            "types": [
                                "Poll"
                            ],
                            "required": false,
                            "description": "Optional. Message is a native poll, information about the poll"
                            },
                            {
                            "name": "venue",
                            "types": [
                                "Venue"
                            ],
                            "required": false,
                            "description": "Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set"
                            },
                            {
                            "name": "location",
                            "types": [
                                "Location"
                            ],
                            "required": false,
                            "description": "Optional. Message is a shared location, information about the location"
                            },
                            {
                            "name": "new_chat_members",
                            "types": [
                                "Array of User"
                            ],
                            "required": false,
                            "description": "Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)"
                            },
                            {
                            "name": "left_chat_member",
                            "types": [
                                "User"
                            ],
                            "required": false,
                            "description": "Optional. A member was removed from the group, information about them (this member may be the bot itself)"
                            },
                            {
                            "name": "chat_owner_left",
                            "types": [
                                "ChatOwnerLeft"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat owner has left"
                            },
                            {
                            "name": "chat_owner_changed",
                            "types": [
                                "ChatOwnerChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat owner has changed"
                            },
                            {
                            "name": "new_chat_title",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. A chat title was changed to this value"
                            },
                            {
                            "name": "new_chat_photo",
                            "types": [
                                "Array of PhotoSize"
                            ],
                            "required": false,
                            "description": "Optional. A chat photo was change to this value"
                            },
                            {
                            "name": "delete_chat_photo",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the chat photo was deleted"
                            },
                            {
                            "name": "group_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the group has been created"
                            },
                            {
                            "name": "supergroup_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup."
                            },
                            {
                            "name": "channel_chat_created",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel."
                            },
                            {
                            "name": "message_auto_delete_timer_changed",
                            "types": [
                                "MessageAutoDeleteTimerChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: auto-delete timer settings changed in the chat"
                            },
                            {
                            "name": "migrate_to_chat_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "migrate_from_chat_id",
                            "types": [
                                "Integer"
                            ],
                            "required": false,
                            "description": "Optional. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "pinned_message",
                            "types": [
                                "MaybeInaccessibleMessage"
                            ],
                            "required": false,
                            "description": "Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply."
                            },
                            {
                            "name": "invoice",
                            "types": [
                                "Invoice"
                            ],
                            "required": false,
                            "description": "Optional. Message is an invoice for a payment, information about the invoice. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "successful_payment",
                            "types": [
                                "SuccessfulPayment"
                            ],
                            "required": false,
                            "description": "Optional. Message is a service message about a successful payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "refunded_payment",
                            "types": [
                                "RefundedPayment"
                            ],
                            "required": false,
                            "description": "Optional. Message is a service message about a refunded payment, information about the payment. More about payments: https://core.telegram.org/bots/api#payments"
                            },
                            {
                            "name": "users_shared",
                            "types": [
                                "UsersShared"
                            ],
                            "required": false,
                            "description": "Optional. Service message: users were shared with the bot"
                            },
                            {
                            "name": "chat_shared",
                            "types": [
                                "ChatShared"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a chat was shared with the bot"
                            },
                            {
                            "name": "gift",
                            "types": [
                                "GiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a regular gift was sent or received"
                            },
                            {
                            "name": "unique_gift",
                            "types": [
                                "UniqueGiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a unique gift was sent or received"
                            },
                            {
                            "name": "gift_upgrade_sent",
                            "types": [
                                "GiftInfo"
                            ],
                            "required": false,
                            "description": "Optional. Service message: upgrade of a gift was purchased after the gift was sent"
                            },
                            {
                            "name": "connected_website",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. The domain name of the website on which the user has logged in. More about Telegram Login: https://core.telegram.org/widgets/login"
                            },
                            {
                            "name": "write_access_allowed",
                            "types": [
                                "WriteAccessAllowed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess"
                            },
                            {
                            "name": "passport_data",
                            "types": [
                                "PassportData"
                            ],
                            "required": false,
                            "description": "Optional. Telegram Passport data"
                            },
                            {
                            "name": "proximity_alert_triggered",
                            "types": [
                                "ProximityAlertTriggered"
                            ],
                            "required": false,
                            "description": "Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location."
                            },
                            {
                            "name": "boost_added",
                            "types": [
                                "ChatBoostAdded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: user boosted the chat"
                            },
                            {
                            "name": "chat_background_set",
                            "types": [
                                "ChatBackground"
                            ],
                            "required": false,
                            "description": "Optional. Service message: chat background set"
                            },
                            {
                            "name": "checklist_tasks_done",
                            "types": [
                                "ChecklistTasksDone"
                            ],
                            "required": false,
                            "description": "Optional. Service message: some tasks in a checklist were marked as done or not done"
                            },
                            {
                            "name": "checklist_tasks_added",
                            "types": [
                                "ChecklistTasksAdded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: tasks were added to a checklist"
                            },
                            {
                            "name": "direct_message_price_changed",
                            "types": [
                                "DirectMessagePriceChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed"
                            },
                            {
                            "name": "forum_topic_created",
                            "types": [
                                "ForumTopicCreated"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic created"
                            },
                            {
                            "name": "forum_topic_edited",
                            "types": [
                                "ForumTopicEdited"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic edited"
                            },
                            {
                            "name": "forum_topic_closed",
                            "types": [
                                "ForumTopicClosed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic closed"
                            },
                            {
                            "name": "forum_topic_reopened",
                            "types": [
                                "ForumTopicReopened"
                            ],
                            "required": false,
                            "description": "Optional. Service message: forum topic reopened"
                            },
                            {
                            "name": "general_forum_topic_hidden",
                            "types": [
                                "GeneralForumTopicHidden"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the 'General' forum topic hidden"
                            },
                            {
                            "name": "general_forum_topic_unhidden",
                            "types": [
                                "GeneralForumTopicUnhidden"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the 'General' forum topic unhidden"
                            },
                            {
                            "name": "giveaway_created",
                            "types": [
                                "GiveawayCreated"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a scheduled giveaway was created"
                            },
                            {
                            "name": "giveaway",
                            "types": [
                                "Giveaway"
                            ],
                            "required": false,
                            "description": "Optional. The message is a scheduled giveaway message"
                            },
                            {
                            "name": "giveaway_winners",
                            "types": [
                                "GiveawayWinners"
                            ],
                            "required": false,
                            "description": "Optional. A giveaway with public winners was completed"
                            },
                            {
                            "name": "giveaway_completed",
                            "types": [
                                "GiveawayCompleted"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a giveaway without public winners was completed"
                            },
                            {
                            "name": "paid_message_price_changed",
                            "types": [
                                "PaidMessagePriceChanged"
                            ],
                            "required": false,
                            "description": "Optional. Service message: the price for paid messages has changed in the chat"
                            },
                            {
                            "name": "suggested_post_approved",
                            "types": [
                                "SuggestedPostApproved"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a suggested post was approved"
                            },
                            {
                            "name": "suggested_post_approval_failed",
                            "types": [
                                "SuggestedPostApprovalFailed"
                            ],
                            "required": false,
                            "description": "Optional. Service message: approval of a suggested post has failed"
                            },
                            {
                            "name": "suggested_post_declined",
                            "types": [
                                "SuggestedPostDeclined"
                            ],
                            "required": false,
                            "description": "Optional. Service message: a suggested post was declined"
                            },
                            {
                            "name": "suggested_post_paid",
                            "types": [
                                "SuggestedPostPaid"
                            ],
                            "required": false,
                            "description": "Optional. Service message: payment for a suggested post was received"
                            },
                            {
                            "name": "suggested_post_refunded",
                            "types": [
                                "SuggestedPostRefunded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: payment for a suggested post was refunded"
                            },
                            {
                            "name": "video_chat_scheduled",
                            "types": [
                                "VideoChatScheduled"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat scheduled"
                            },
                            {
                            "name": "video_chat_started",
                            "types": [
                                "VideoChatStarted"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat started"
                            },
                            {
                            "name": "video_chat_ended",
                            "types": [
                                "VideoChatEnded"
                            ],
                            "required": false,
                            "description": "Optional. Service message: video chat ended"
                            },
                            {
                            "name": "video_chat_participants_invited",
                            "types": [
                                "VideoChatParticipantsInvited"
                            ],
                            "required": false,
                            "description": "Optional. Service message: new participants invited to a video chat"
                            },
                            {
                            "name": "web_app_data",
                            "types": [
                                "WebAppData"
                            ],
                            "required": false,
                            "description": "Optional. Service message: data sent by a Web App"
                            },
                            {
                            "name": "reply_markup",
                            "types": [
                                "InlineKeyboardMarkup"
                            ],
                            "required": false,
                            "description": "Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons."
                            }
                        ],
                        "subtype_of": [
                            "MaybeInaccessibleMessage"
                        ]
                    }
                }
            }
        "#;

        let schema = Schema::parse_from_jsom(content).unwrap();
        let mut normalized = schema.normalize();

        normalized.split_message_types();

        let message = normalized.types.get("Message").unwrap();
        assert!(message.fields.is_empty());
        assert!(message.subtype_kind.is_some());
        assert_eq!(message.subtypes.len(), 70);

        let expected_major_types = [
            "MessageText",
            "MessagePhoto",
            "MessageVideo",
            "MessageAnimation",
            "MessageAudio",
            "MessageDocument",
            "MessageVoice",
            "MessageSticker",
            "MessagePoll",
            "MessageLocation",
            "MessageVenue",
            "MessageContact",
            "MessageGame",
            "MessageDice",
            "MessageInvoice",
            "MessageSuccessfulPayment",
            "MessageNewChatMembers",
            "MessageLeftChatMember",
            "MessagePinnedMessage",
        ];

        for type_name in expected_major_types.iter() {
            assert!(
                normalized.types.contains_key(*type_name),
                "Major type {} should exist",
                type_name
            );
        }

        for subtype in &message.subtypes {
            let type_name = &subtype.name;
            let ty = normalized.types.get(type_name).unwrap();

            assert!(
                normalized.types.contains_key(type_name),
                "Subtype {type_name} should exist"
            );
            assert_eq!(
                ty.subtype_of,
                vec!["Message"],
                "{type_name} should be subtype of Message",
            );
            assert!(!ty.fields.is_empty(), "{} should have fields", type_name);
        }

        let content_type_main_fields = [
            ("MessageText", "text"),
            ("MessagePhoto", "photo"),
            ("MessageVideo", "video"),
            ("MessageAnimation", "animation"),
            ("MessageAudio", "audio"),
            ("MessageDocument", "document"),
            ("MessageVoice", "voice"),
            ("MessageSticker", "sticker"),
            ("MessagePoll", "poll"),
            ("MessageLocation", "location"),
            ("MessageVenue", "venue"),
            ("MessageContact", "contact"),
            ("MessageGame", "game"),
            ("MessageDice", "dice"),
            ("MessageInvoice", "invoice"),
            ("MessageSuccessfulPayment", "successful_payment"),
        ];

        for (type_name, main_field) in content_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let service_type_main_fields = [
            ("MessageNewChatMembers", "new_chat_members"),
            ("MessageLeftChatMember", "left_chat_member"),
            ("MessagePinnedMessage", "pinned_message"),
        ];

        for (type_name, main_field) in service_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let all_main_fields: Vec<&str> = content_type_main_fields.iter().map(|(_, f)| *f).collect();
        for (type_name, main_field) in content_type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for other_field in all_main_fields.iter() {
                    if *other_field != *main_field {
                        assert!(
                            !ty.fields.iter().any(|f| f.name == *other_field),
                            "{type_name} should not contain {other_field} field",
                        );
                    }
                }
            }
        }

        let common_fields = ["message_id", "date", "chat", "from"];
        let types_to_check = ["MessageText", "MessagePhoto", "MessageNewChatMembers"];

        for type_name in types_to_check.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for field_name in common_fields.iter() {
                    assert!(
                        ty.fields.iter().any(|f| f.name == *field_name),
                        "{type_name} should have common field {field_name}",
                    );
                }
            }
        }
    }

    #[test]
    fn test_split_chat_types_creates_new_types() {
        let content = r#"
            {
                "version": "1.0",
                "release_date": "2021-01-01",
                "changelog": "Initial release",
                "types": {
                    "Chat": {
                        "name": "Chat",
                        "href": "https://core.telegram.org/bots/api#chat",
                        "description": [
                            "This object represents a chat."
                        ],
                        "fields": [
                            {
                            "name": "id",
                            "types": [
                                "Integer"
                            ],
                            "required": true,
                            "description": "Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier."
                            },
                            {
                            "name": "type",
                            "types": [
                                "String"
                            ],
                            "required": true,
                            "description": "Type of the chat, can be either \"private\", \"group\", \"supergroup\" or \"channel\""
                            },
                            {
                            "name": "title",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Title, for supergroups, channels and group chats"
                            },
                            {
                            "name": "username",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Username, for private chats, supergroups and channels if available"
                            },
                            {
                            "name": "first_name",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. First name of the other party in a private chat"
                            },
                            {
                            "name": "last_name",
                            "types": [
                                "String"
                            ],
                            "required": false,
                            "description": "Optional. Last name of the other party in a private chat"
                            },
                            {
                            "name": "is_forum",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the supergroup chat is a forum (has topics enabled)"
                            },
                            {
                            "name": "is_direct_messages",
                            "types": [
                                "Boolean"
                            ],
                            "required": false,
                            "description": "Optional. True, if the chat is the direct messages chat of a channel"
                            }
                        ]
                        }
                }
            }
        "#;

        let schema = Schema::parse_from_jsom(content).unwrap();
        let mut normalized = schema.normalize();

        normalized.split_chat_type("Chat");

        let chat = normalized.types.get("Chat").unwrap();
        assert!(chat.fields.is_empty());
        assert!(chat.subtype_kind.is_some());
        assert_eq!(chat.subtypes.len(), 4);

        let expected_types = ["ChatPrivate", "ChatGroup", "ChatSupergroup", "ChatChannel"];

        for type_name in expected_types.iter() {
            assert!(
                normalized.types.contains_key(*type_name),
                "Type {} should exist",
                type_name
            );
        }

        for subtype in &chat.subtypes {
            let type_name = &subtype.name;
            let ty = normalized.types.get(type_name).unwrap();

            assert!(
                normalized.types.contains_key(type_name),
                "Subtype {type_name} should exist"
            );
            assert_eq!(
                ty.subtype_of,
                vec!["Chat"],
                "{type_name} should be subtype of Chat",
            );
            assert!(!ty.fields.is_empty(), "{} should have fields", type_name);
        }

        let type_main_fields = [
            ("ChatPrivate", "type", "private"),
            ("ChatGroup", "type", "group"),
            ("ChatSupergroup", "type", "supergroup"),
            ("ChatChannel", "type", "channel"),
        ];

        for (type_name, main_field, _) in type_main_fields.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                let field = ty.fields.iter().find(|f| f.name == *main_field);

                assert!(
                    field.is_some(),
                    "{type_name} should have {main_field} field",
                );
                assert!(
                    field.unwrap().required,
                    "{main_field} field should be required in {type_name}",
                );
            }
        }

        let common_fields = ["id", "type"];
        let types_to_check = ["ChatPrivate", "ChatGroup", "ChatSupergroup", "ChatChannel"];

        for type_name in types_to_check.iter() {
            if let Some(ty) = normalized.types.get(*type_name) {
                for field_name in common_fields.iter() {
                    assert!(
                        ty.fields.iter().any(|f| f.name == *field_name),
                        "{type_name} should have common field {field_name}",
                    );
                }
            }
        }
    }
}
