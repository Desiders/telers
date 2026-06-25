use crate::parser::api::NormalizedType;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[must_use]
pub fn tokenize_message_to_methods(type_quote: &NormalizedType) -> TokenStream {
    let type_name = format_ident!("{}", type_quote.name);
    let subtype_names = type_quote
        .subtypes
        .iter()
        .map(|subtype| format_ident!("{}", subtype.ty_name))
        .collect::<Vec<_>>();

    quote! {
        use crate::types::{ChatIdKind, #type_name, #( #subtype_names ),*};
        use crate::methods::{CopyMessage, ForwardMessage, DeleteMessage};
        use crate::utils::text::Renderer;

        impl #type_name {
            /// Creates [`CopyMessage`] for this message.
            #[must_use]
            pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
                CopyMessage::new(chat_id, self.chat().id(), self.message_id())
            }
            /// Creates [`ForwardMessage`] for this message.
            #[must_use]
            pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
                ForwardMessage::new(chat_id, self.chat().id(), self.message_id())
            }
            /// Creates [`DeleteMessage`] for this message.
            #[must_use]
            pub fn delete_message(&self) -> DeleteMessage {
                DeleteMessage::new(self.chat().id(), self.message_id())
            }
            /// Renders the message text and its entities as an HTML string, if the message has text.
            #[must_use]
            pub fn html_text(&self) -> Option<String> {
                self.text().map(|text| {
                    Renderer::new(text, self.entities().unwrap_or(&[])).as_html()
                })
            }
            /// Renders the message text and its entities as a MarkdownV2 string, if the message has text.
            #[must_use]
            pub fn markdown_text(&self) -> Option<String> {
                self.text().map(|text| {
                    Renderer::new(text, self.entities().unwrap_or(&[])).as_markdown()
                })
            }
            /// Renders the message caption and its entities as an HTML string, if the message has a caption.
            #[must_use]
            pub fn html_caption(&self) -> Option<String> {
                self.caption().map(|caption| {
                    Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_html()
                })
            }
            /// Renders the message caption and its entities as a MarkdownV2 string, if the message has a caption.
            #[must_use]
            pub fn markdown_caption(&self) -> Option<String> {
                self.caption().map(|caption| {
                    Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_markdown()
                })
            }
        }

        #(
            impl #subtype_names {
                /// Creates [`CopyMessage`] for this message.
                #[must_use]
                pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
                    CopyMessage::new(chat_id, self.chat.id(), self.message_id)
                }
                /// Creates [`ForwardMessage`] for this message.
                #[must_use]
                pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
                    ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
                }
                /// Creates [`DeleteMessage`] for this message.
                #[must_use]
                pub fn delete_message(&self) -> DeleteMessage {
                    DeleteMessage::new(self.chat.id(), self.message_id)
                }
            }
        )*
    }
}
