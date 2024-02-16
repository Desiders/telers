use crate::parser::api;

/// [`TelegramTypeName`] is a string that is used to identify a type, for example `Chat` or `User`.
pub type TelegramTypeName = String;
pub type FieldName = String;

#[derive(Debug, PartialEq, Eq)]
pub enum IntegerKind {
    Int64,
    Int32,
    Int16,
    Float32,
}

/// # Variants
/// - `Any` - Any boolean value
/// - `True` - Only possible value is `true`
#[derive(Debug, PartialEq, Eq)]
pub enum BooleanKind {
    Any,
    True,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKindInField {
    String,
    Integer(IntegerKind),
    Boolean(BooleanKind),
    Telegram(TelegramTypeName),
    Array(Box<TypeKindInField>),
}

#[derive(Debug)]
pub struct Field {
    pub name: FieldName,
    pub required: bool,
    pub description: String,
    pub r#type: TypeKindInField,
}

impl From<api::Field> for Field {
    fn from(raw_field: api::Field) -> Self {
        let r#type = api::identify_field_type(&raw_field);

        Self {
            name: raw_field.name,
            required: raw_field.required,
            description: raw_field.description,
            r#type,
        }
    }
}

#[derive(Debug)]
pub struct Type {
    pub name: TelegramTypeName,
    pub href: String,
    pub description: Vec<String>,
    pub fields: Vec<Field>,
    pub subtypes: Vec<TelegramTypeName>,
    pub subtype_of: Vec<TelegramTypeName>,
}

impl From<api::Type> for Type {
    fn from(raw_type: api::Type) -> Self {
        let api::Type {
            name,
            href,
            description,
            fields: raw_fields,
            subtypes,
            subtype_of,
        } = raw_type;

        let mut fields = vec![];

        for raw_field in raw_fields {
            fields.push(Field::from(raw_field));
        }

        Self {
            name,
            href,
            description,
            fields,
            subtypes,
            subtype_of,
        }
    }
}
