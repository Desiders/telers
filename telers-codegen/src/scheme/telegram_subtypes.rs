use crate::parser::tagged_info;

use super::types::TelegramTypeName;

use std::collections::HashMap;

pub type VariantTypeName = TelegramTypeName;
pub type TagValue = String;

/// # Variants
/// - `Tagged` - A tagged union type.
/// The tag is used to determine the type of the value.
/// The tag can be excluded from the serialized data. Usually it's `true`, because it's not necessary to include the tag in the serialized data.
/// The variants are a map of the tag value to the type of the value.
/// - `Untagged` - An untagged union type.
/// The tag is optional, and if it is present, it is used to possible to exclude the tag from the serialized data.
/// In some cases we can use `Untagged` even if the tag is present, for example, several types have the same tags.
/// The tag can be excluded from the serialized data. Usually it's `true`, because it's not necessary to include the tag in the serialized data.
#[derive(Debug)]
pub enum SubTypesKind {
    Tagged { tag: String, exclude: bool },
    Untagged { tag: Option<String>, exclude: bool },
}

#[derive(Debug)]
pub struct Type {
    pub r#type: TelegramTypeName,
    pub subtypes_kind: SubTypesKind,
    pub variants: HashMap<VariantTypeName, TagValue>,
}

#[derive(Debug)]
pub struct Types(pub Vec<Type>);

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Empty tag for tagged union type: {type_name}")]
    EmptyTagForTaggedUnionType { type_name: TelegramTypeName },
}

impl TryFrom<tagged_info::TaggedInfo> for Types {
    type Error = ErrorKind;

    fn try_from(tagged_info: tagged_info::TaggedInfo) -> Result<Self, Self::Error> {
        let mut types = vec![];

        for (type_name, type_info) in tagged_info.docs_tagged_types {
            let subtypes_kind = if type_info.is_untagged {
                SubTypesKind::Untagged {
                    tag: type_info.tag_field,
                    exclude: type_info.exclude_tag_field,
                }
            } else {
                match type_info.tag_field {
                    None => return Err(ErrorKind::EmptyTagForTaggedUnionType { type_name }),
                    Some(tag) => {
                        if tag.is_empty() {
                            return Err(ErrorKind::EmptyTagForTaggedUnionType { type_name });
                        }

                        SubTypesKind::Tagged {
                            tag,
                            exclude: type_info.exclude_tag_field,
                        }
                    }
                }
            };

            types.push(Type {
                r#type: type_name,
                subtypes_kind,
                variants: type_info.variants,
            });
        }

        Ok(Self(types))
    }
}
