use crate::{
    generator::{
        helpers::{format_attr_description, sanitize_field_name},
        types::{helper_field_accessor_expr, helper_method_return_type},
    },
    parser::api::{
        NormalizedField, NormalizedMethod, NormalizedSchema, NormalizedType, TypeKindInField,
    },
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeSet;

/// Field of a method that is filled from the message instead of being a parameter of the shortcut
#[derive(Clone, Copy)]
enum Fill {
    ChatId,
    FromChatId,
    MessageId,
    MessageThreadId,
    BusinessConnectionId,
    ReplyParameters,
    EphemeralMessageParameters,
}

impl Fill {
    #[must_use]
    const fn field_name(self) -> &'static str {
        match self {
            Self::ChatId => "chat_id",
            Self::FromChatId => "from_chat_id",
            Self::MessageId => "message_id",
            Self::MessageThreadId => "message_thread_id",
            Self::BusinessConnectionId => "business_connection_id",
            Self::ReplyParameters => "reply_parameters",
            Self::EphemeralMessageParameters => "ephemeral_message_parameters",
        }
    }
}

const ANSWER_FILLS: &[Fill] = &[
    Fill::ChatId,
    Fill::MessageThreadId,
    Fill::BusinessConnectionId,
];
const REPLY_FILLS: &[Fill] = &[
    Fill::ChatId,
    Fill::MessageThreadId,
    Fill::BusinessConnectionId,
    Fill::ReplyParameters,
    Fill::EphemeralMessageParameters,
];
const THIS_MESSAGE_FILLS: &[Fill] = &[Fill::ChatId, Fill::MessageId, Fill::BusinessConnectionId];

/// Send methods with `answer_*` and `reply_*` shortcuts, the same as in `aiogram`
const SEND_METHODS: &[(&str, &str)] = &[
    ("", "sendMessage"),
    ("_rich", "sendRichMessage"),
    ("_animation", "sendAnimation"),
    ("_audio", "sendAudio"),
    ("_contact", "sendContact"),
    ("_document", "sendDocument"),
    ("_game", "sendGame"),
    ("_invoice", "sendInvoice"),
    ("_location", "sendLocation"),
    ("_media_group", "sendMediaGroup"),
    ("_photo", "sendPhoto"),
    ("_poll", "sendPoll"),
    ("_dice", "sendDice"),
    ("_sticker", "sendSticker"),
    ("_venue", "sendVenue"),
    ("_video", "sendVideo"),
    ("_video_note", "sendVideoNote"),
    ("_voice", "sendVoice"),
    ("_paid_media", "sendPaidMedia"),
];

/// Shortcuts for the methods that take this message, the same as in `aiogram`
const MESSAGE_SHORTCUTS: &[(&str, &str, &[Fill])] = &[
    (
        "copy_to",
        "copyMessage",
        &[Fill::FromChatId, Fill::MessageId],
    ),
    (
        "forward",
        "forwardMessage",
        &[Fill::FromChatId, Fill::MessageId],
    ),
    ("edit_text", "editMessageText", THIS_MESSAGE_FILLS),
    ("edit_caption", "editMessageCaption", THIS_MESSAGE_FILLS),
    ("edit_media", "editMessageMedia", THIS_MESSAGE_FILLS),
    (
        "edit_reply_markup",
        "editMessageReplyMarkup",
        THIS_MESSAGE_FILLS,
    ),
    (
        "edit_live_location",
        "editMessageLiveLocation",
        THIS_MESSAGE_FILLS,
    ),
    (
        "stop_live_location",
        "stopMessageLiveLocation",
        THIS_MESSAGE_FILLS,
    ),
    ("delete", "deleteMessage", THIS_MESSAGE_FILLS),
    ("pin", "pinChatMessage", THIS_MESSAGE_FILLS),
    ("unpin", "unpinChatMessage", THIS_MESSAGE_FILLS),
    ("react", "setMessageReaction", THIS_MESSAGE_FILLS),
];

/// Fields of the message the shortcuts are built from, the required methods of the trait
const SHORTCUT_FIELDS: &[&str] = &[
    "chat",
    "message_id",
    "message_thread_id",
    "is_topic_message",
    "business_connection_id",
    "ephemeral_message_id",
    "from",
];

/// Value of a fill
struct FillValue {
    tokens: TokenStream,
    /// Whether the value is an `Option`, so it's set with the `_option` builder
    is_option: bool,
}

/// Fields of the message and the way to read them:
/// `Message` reads them by its helper accessors, its subtypes by fields
struct FieldAccess<'a> {
    fields: &'a [NormalizedField],
    by_accessors: bool,
}

impl FieldAccess<'_> {
    #[must_use]
    fn field(&self, name: &str) -> &NormalizedField {
        self.fields
            .iter()
            .find(|field| field.name == name)
            .unwrap_or_else(|| panic!("`{name}` field must exist in message"))
    }

    /// Expression to read the field, the same value the helper accessor of `Message` returns
    #[must_use]
    fn expr(&self, name: &str) -> TokenStream {
        if self.by_accessors {
            let ident = sanitize_field_name(name);
            quote! { self.#ident() }
        } else {
            helper_field_accessor_expr(&format_ident!("self"), self.field(name))
        }
    }

    #[must_use]
    fn field_value(&self, name: &str) -> FillValue {
        FillValue {
            tokens: self.expr(name),
            is_option: !self.field(name).required,
        }
    }

    #[must_use]
    fn value(&self, fill: Fill) -> FillValue {
        match fill {
            Fill::ChatId | Fill::FromChatId => {
                let chat = self.field_value("chat");
                let tokens = chat.tokens;
                FillValue {
                    tokens: quote! { #tokens.id() },
                    ..chat
                }
            }
            Fill::MessageId => self.field_value("message_id"),
            // Only a message in a forum topic is answered in its thread, like `aiogram` does
            Fill::MessageThreadId => {
                let message_thread_id = self.field_value("message_thread_id");
                let tokens = message_thread_id.tokens;
                let is_topic_message = self.expr("is_topic_message");
                FillValue {
                    tokens: quote! { #tokens.filter(|_| #is_topic_message == Some(true)) },
                    ..message_thread_id
                }
            }
            Fill::BusinessConnectionId => self.field_value("business_connection_id"),
            Fill::ReplyParameters => FillValue {
                tokens: quote! { self.as_reply_parameters() },
                is_option: false,
            },
            Fill::EphemeralMessageParameters => FillValue {
                tokens: quote! { self.as_ephemeral_message_parameters() },
                is_option: true,
            },
        }
    }
}

/// Shortcut of the trait: a method of the Telegram Bot API with the fields filled from the message
struct Shortcut<'a> {
    name: String,
    method: &'a NormalizedMethod,
    fills: &'static [Fill],
    doc: String,
}

#[must_use]
fn shortcuts(schema: &NormalizedSchema) -> Vec<Shortcut<'_>> {
    let method = |name: &str| {
        schema
            .methods
            .get(name)
            .unwrap_or_else(|| panic!("`{name}` method must exist in schema"))
    };

    SEND_METHODS
        .iter()
        .flat_map(|(suffix, name)| {
            [
                Shortcut {
                    name: format!("answer{suffix}"),
                    method: method(name),
                    fills: ANSWER_FILLS,
                    doc: format!(
                        "Creates `{}` to the chat of this message.",
                        method(name).name
                    ),
                },
                Shortcut {
                    name: format!("reply{suffix}"),
                    method: method(name),
                    fills: REPLY_FILLS,
                    doc: format!("Creates `{}` to reply to this message.", method(name).name),
                },
            ]
        })
        .chain(
            MESSAGE_SHORTCUTS
                .iter()
                .map(|(shortcut_name, name, fills)| Shortcut {
                    name: (*shortcut_name).to_owned(),
                    method: method(name),
                    fills,
                    doc: format!("Creates `{}` for this message.", method(name).name),
                }),
        )
        .collect()
}

/// Tokenizes a shortcut: the required fields of the method that aren't filled from the message
/// are the parameters, the filled optional fields are set with builders
#[must_use]
fn tokenize_shortcut(shortcut: &Shortcut<'_>, access: &FieldAccess<'_>) -> TokenStream {
    let method = shortcut.method;
    let method_name = format_ident!("{}", method.name);
    let fn_name = format_ident!("{}", shortcut.name);
    let fill_of = |field: &NormalizedField| {
        shortcut
            .fills
            .iter()
            .copied()
            .find(|fill| fill.field_name() == field.name)
    };

    let params: Vec<&NormalizedField> = method
        .fields
        .iter()
        .filter(|field| field.required && fill_of(field).is_none())
        .collect();
    let generics: Vec<_> = params
        .iter()
        .enumerate()
        .flat_map(|(i, field)| {
            let ty = &field.r#type;
            let t = format_ident!("T{i}");
            if let TypeKindInField::Array(inner) = ty {
                let t_item = format_ident!("T{i}Item");
                vec![
                    quote! { #t_item: Into<#inner> },
                    quote! { #t: IntoIterator<Item = #t_item> },
                ]
            } else {
                vec![quote! { #t: Into<#ty> }]
            }
        })
        .collect();
    let args: Vec<_> = params
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let name = sanitize_field_name(&field.name);
            let t = format_ident!("T{i}");
            quote! { #name: #t }
        })
        .collect();
    let new_args = method
        .fields
        .iter()
        .filter(|field| field.required)
        .map(|field| {
            if let Some(fill) = fill_of(field) {
                access.value(fill).tokens
            } else {
                let name = sanitize_field_name(&field.name);
                quote! { #name }
            }
        });
    let builders = method
        .fields
        .iter()
        .filter(|field| !field.required)
        .filter_map(|field| {
            let value = access.value(fill_of(field)?);
            let builder = if value.is_option {
                format_ident!("{}_option", field.name)
            } else {
                format_ident!("{}", field.name)
            };
            let tokens = value.tokens;
            Some(quote! { .#builder(#tokens) })
        });
    let generics = if generics.is_empty() {
        quote! {}
    } else {
        quote! { <#( #generics ),*> }
    };
    let doc = format_attr_description(&shortcut.doc);

    quote! {
        #[doc = #doc]
        #[must_use]
        fn #fn_name #generics (&self, #( #args ),*) -> #method_name {
            #method_name::new(#( #new_args ),*) #( #builders )*
        }
    }
}

/// Tokenizes the helpers that build the reply parameters of a shortcut
#[must_use]
fn tokenize_reply_helpers(access: &FieldAccess<'_>) -> TokenStream {
    let ephemeral_message_id = access.expr("ephemeral_message_id");
    let message_id = access.expr("message_id");
    let chat = access.expr("chat");
    let from = access.expr("from");

    quote! {
        /// Creates [`ReplyParameters`] to reply to this message.
        /// # Notes
        /// An ephemeral message is addressed by `ephemeral_message_id`,
        /// because its `message_id` is always 0 and `chat_id` isn't supported for it.
        #[must_use]
        fn as_reply_parameters(&self) -> ReplyParameters {
            match #ephemeral_message_id {
                Some(ephemeral_message_id) => {
                    ReplyParameters::new().ephemeral_message_id(ephemeral_message_id)
                }
                None => ReplyParameters::new().message_id(#message_id).chat_id(#chat.id()),
            }
        }
        /// Creates [`EphemeralMessageParameters`] to reply to this message if it's ephemeral,
        /// because a reply to an ephemeral message must be an ephemeral message too.
        /// # Returns
        /// `None` if the message isn't ephemeral or has no sender to address the reply to
        #[must_use]
        fn as_ephemeral_message_parameters(&self) -> Option<EphemeralMessageParameters> {
            #ephemeral_message_id?;
            #from.map(|from| EphemeralMessageParameters::new(from.id))
        }
    }
}

/// Fields of the message the renderers are built from, the required methods of the trait
const RENDERER_FIELDS: &[&str] = &["text", "entities", "caption", "caption_entities"];

/// Field of any subtype of the message, to describe a field that not every subtype has
#[must_use]
fn subtype_field<'a>(
    schema: &'a NormalizedSchema,
    type_quote: &NormalizedType,
    name: &str,
) -> &'a NormalizedField {
    type_quote
        .subtypes
        .iter()
        .find_map(|subtype| {
            schema
                .types
                .get(&subtype.ty_name)
                .expect("Message subtype must exist in schema")
                .fields
                .iter()
                .find(|field| field.name == name)
        })
        .unwrap_or_else(|| panic!("`{name}` field must exist in a subtype of message"))
}

/// Tokenizes the accessors of the renderer fields for a subtype:
/// a subtype without the field returns `None`, a required field is wrapped in `Some`
#[must_use]
fn tokenize_subtype_renderer_accessors(
    schema: &NormalizedSchema,
    type_quote: &NormalizedType,
    fields: &[NormalizedField],
) -> TokenStream {
    let access = FieldAccess {
        fields,
        by_accessors: false,
    };

    RENDERER_FIELDS
        .iter()
        .map(|name| {
            let ident = sanitize_field_name(name);
            let return_ty =
                helper_method_return_type(&subtype_field(schema, type_quote, name).r#type, false);
            let value = match fields.iter().find(|field| field.name == *name) {
                Some(field) if field.required => {
                    let value = access.expr(name);
                    quote! { Some(#value) }
                }
                Some(_) => access.expr(name),
                None => quote! { None },
            };
            quote! {
                fn #ident(&self) -> #return_ty {
                    #value
                }
            }
        })
        .collect()
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn tokenize_message_to_methods(schema: &NormalizedSchema) -> TokenStream {
    let type_quote = schema
        .types
        .get("Message")
        .expect("Message type must exist in schema");
    let type_name = format_ident!("{}", type_quote.name);
    let subtype_names = type_quote
        .subtypes
        .iter()
        .map(|subtype| format_ident!("{}", subtype.ty_name))
        .collect::<Vec<_>>();

    // The subtypes share the fields the shortcuts read, so any of them describes the fields
    let fields = &schema
        .types
        .get(&type_quote.subtypes[0].ty_name)
        .expect("Message subtype must exist in schema")
        .fields;
    // The shortcuts read the fields by the required methods of the trait,
    // `Message` implements them by its helper accessors, the subtypes by fields
    let trait_access = FieldAccess {
        fields,
        by_accessors: true,
    };
    let subtype_access = FieldAccess {
        fields,
        by_accessors: false,
    };

    let required_methods = SHORTCUT_FIELDS.iter().map(|name| {
        let field = trait_access.field(name);
        let ident = sanitize_field_name(name);
        let return_ty = helper_method_return_type(&field.r#type, field.required);
        let doc = format_attr_description(&format!("Helper method for field `{name}`."));
        quote! {
            #[doc = #doc]
            #[must_use]
            fn #ident(&self) -> #return_ty;
        }
    });
    let type_accessors = SHORTCUT_FIELDS.iter().map(|name| {
        let field = trait_access.field(name);
        let ident = sanitize_field_name(name);
        let return_ty = helper_method_return_type(&field.r#type, field.required);
        let value = trait_access.expr(name);
        quote! {
            fn #ident(&self) -> #return_ty {
                #value
            }
        }
    });
    // One token stream, because it's repeated as is for every subtype
    let subtype_accessors = SHORTCUT_FIELDS
        .iter()
        .map(|name| {
            let field = subtype_access.field(name);
            let ident = sanitize_field_name(name);
            let return_ty = helper_method_return_type(&field.r#type, field.required);
            let value = subtype_access.expr(name);
            quote! {
                fn #ident(&self) -> #return_ty {
                    #value
                }
            }
        })
        .collect::<TokenStream>();

    let shortcuts = shortcuts(schema);
    let method_names = shortcuts
        .iter()
        .map(|shortcut| shortcut.method.name.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|name| format_ident!("{name}"))
        .collect::<Vec<_>>();
    let provided_methods = shortcuts
        .iter()
        .map(|shortcut| tokenize_shortcut(shortcut, &trait_access))
        .collect::<Vec<_>>();
    let reply_helpers = tokenize_reply_helpers(&trait_access);
    let renderer_required_methods = RENDERER_FIELDS.iter().map(|name| {
        let ident = sanitize_field_name(name);
        let return_ty =
            helper_method_return_type(&subtype_field(schema, type_quote, name).r#type, false);
        let doc = format_attr_description(&format!("Helper method for field `{name}`."));
        quote! {
            #[doc = #doc]
            #[must_use]
            fn #ident(&self) -> #return_ty;
        }
    });
    let type_renderer_accessors = RENDERER_FIELDS.iter().map(|name| {
        let ident = sanitize_field_name(name);
        let return_ty =
            helper_method_return_type(&subtype_field(schema, type_quote, name).r#type, false);
        quote! {
            fn #ident(&self) -> #return_ty {
                self.#ident()
            }
        }
    });
    let subtype_renderer_accessors = type_quote
        .subtypes
        .iter()
        .map(|subtype| {
            let fields = &schema
                .types
                .get(&subtype.ty_name)
                .expect("Message subtype must exist in schema")
                .fields;
            tokenize_subtype_renderer_accessors(schema, type_quote, fields)
        })
        .collect::<Vec<_>>();

    quote! {
        use crate::types::{EphemeralMessageParameters, ReplyParameters, #type_name, #( #subtype_names ),*};
        use crate::methods::{#( #method_names ),*};
        use crate::utils::text::Renderer;

        /// Shortcuts that create methods for the message with its fields filled,
        /// for example [`MessageShortcuts::answer`] creates [`SendMessage`] to the chat of the message.
        /// Optional fields of the methods are set with their builders, as usual.
        ///
        /// It's implemented for [`Message`] and its subtypes.
        /// The required methods are the fields of the message the shortcuts are built from.
        /// # Notes
        /// The trait must be in scope to call the shortcuts: `use telers::types::MessageShortcuts as _;`
        pub trait MessageShortcuts {
            #( #required_methods )*
            #( #provided_methods )*
            #reply_helpers
        }

        impl MessageShortcuts for #type_name {
            #( #type_accessors )*
        }

        #(
            impl MessageShortcuts for #subtype_names {
                #subtype_accessors
            }
        )*

        /// Renderers of the text and the caption of the message with their entities
        /// as HTML or `MarkdownV2` strings, see [`Renderer`].
        ///
        /// It's implemented for [`Message`] and its subtypes.
        /// The required methods are the fields of the message the renderers are built from.
        /// # Notes
        /// The trait must be in scope to call the renderers: `use telers::types::MessageRenderers as _;`
        pub trait MessageRenderers {
            #( #renderer_required_methods )*
            /// Renders the message text and its entities as an HTML string, if the message has text.
            #[must_use]
            fn html_text(&self) -> Option<String> {
                self.text().map(|text| {
                    Renderer::new(text, self.entities().unwrap_or(&[])).as_html()
                })
            }
            /// Renders the message text and its entities as a `MarkdownV2` string, if the message has text.
            #[must_use]
            fn markdown_text(&self) -> Option<String> {
                self.text().map(|text| {
                    Renderer::new(text, self.entities().unwrap_or(&[])).as_markdown()
                })
            }
            /// Renders the message caption and its entities as an HTML string, if the message has a caption.
            #[must_use]
            fn html_caption(&self) -> Option<String> {
                self.caption().map(|caption| {
                    Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_html()
                })
            }
            /// Renders the message caption and its entities as a `MarkdownV2` string, if the message has a caption.
            #[must_use]
            fn markdown_caption(&self) -> Option<String> {
                self.caption().map(|caption| {
                    Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_markdown()
                })
            }
        }

        impl MessageRenderers for #type_name {
            #( #type_renderer_accessors )*
        }

        #(
            impl MessageRenderers for #subtype_names {
                #subtype_renderer_accessors
            }
        )*
    }
}
