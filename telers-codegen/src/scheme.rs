use crate::parser::{api, tagged_info};

pub mod telegram_subtypes;
pub mod types;

#[derive(Debug)]
pub struct Scheme {
    pub version: String,
    pub release_date: String,
    pub changelog: String,
    pub types: Vec<types::Type>,
    pub telegram_subtypes: telegram_subtypes::Types,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Telegram subtypes error: {0}")]
    Subtypes(#[from] telegram_subtypes::ErrorKind),
}

impl TryFrom<(api::Scheme, tagged_info::TaggedInfo)> for Scheme {
    type Error = ErrorKind;

    fn try_from(
        (
            api::Scheme {
                version,
                release_date,
                changelog,
                types: raw_types,
            },
            tagged_info,
        ): (api::Scheme, tagged_info::TaggedInfo),
    ) -> Result<Self, Self::Error> {
        let mut types = vec![];

        for (_, r#type) in raw_types {
            types.push(types::Type::from(r#type));
        }

        let telegram_subtypes = telegram_subtypes::Types::try_from(tagged_info)?;

        Ok(Self {
            version,
            release_date,
            changelog,
            types,
            telegram_subtypes,
        })
    }
}
