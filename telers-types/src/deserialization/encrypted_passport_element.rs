#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserializer,
};
use std::fmt;

pub(crate) struct CommonFields {
    pub(crate) hash: Box<str>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    hash: Option<Box<str>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "hash" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"hash" => Some(0),
            _ => None,
        }
    }
}

impl CommonSlots {
    fn read_field<'de, A: MapAccess<'de>>(
        &mut self,
        index: usize,
        map: &mut A,
    ) -> Result<(), A::Error> {
        match index {
            0 => read_map_field(&mut self.hash, map, "hash"),
            _ => unreachable!(),
        }
    }
}

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    [variant_fields[0], "hash"]
}

pub(crate) struct ContentFields {
    pub(crate) common: CommonFields,
    pub(crate) content: Box<str>,
}

struct ContentVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for ContentVisitor {
    type Value = ContentFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_content = read_seq_field(&mut seq, 0, &expected)?;
        let common_hash = read_seq_field(&mut seq, 1, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                hash: common_hash,
            },
            content: variant_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_content: Option<Box<str>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_content,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let common_hash = shared.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
        Ok(ContentFields {
            common: CommonFields {
                hash: common_hash,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize_content<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ContentFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ContentVisitor {
            metadata,
        },
    )
}

const fn files_field_names(variant_fields: &[&'static str; 2]) -> [&'static str; 3] {
    [variant_fields[0], variant_fields[1], "hash"]
}

pub(crate) struct FilesFields {
    pub(crate) common: CommonFields,
    pub(crate) files: Box<[crate::types::PassportFile]>,
    pub(crate) translation: Option<Box<[crate::types::PassportFile]>>,
}

struct FilesVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for FilesVisitor {
    type Value = FilesFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_files = read_seq_field(&mut seq, 0, &expected)?;
        let variant_translation = read_seq_field(&mut seq, 1, &expected)?;
        let common_hash = read_seq_field(&mut seq, 2, &expected)?;
        Ok(FilesFields {
            common: CommonFields {
                hash: common_hash,
            },
            files: variant_files,
            translation: variant_translation,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_files: Option<Box<[crate::types::PassportFile]>> = None;
        let mut variant_translation: Option<Option<Box<[crate::types::PassportFile]>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_files,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_translation,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_files = variant_files
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_translation = variant_translation.unwrap_or_default();
        let common_hash = shared.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
        Ok(FilesFields {
            common: CommonFields {
                hash: common_hash,
            },
            files: variant_files,
            translation: variant_translation,
        })
    }
}

pub(crate) fn deserialize_files<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<FilesFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        FilesVisitor {
            metadata,
        },
    )
}

const fn driver_license_or_identity_card_field_names(
    variant_fields: &[&'static str; 5],
) -> [&'static str; 6] {
    [
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        variant_fields[4],
        "hash",
    ]
}

pub(crate) struct DriverLicenseOrIdentityCardFields {
    pub(crate) common: CommonFields,
    pub(crate) data: Box<str>,
    pub(crate) front_side: crate::types::PassportFile,
    pub(crate) reverse_side: crate::types::PassportFile,
    pub(crate) selfie: Option<crate::types::PassportFile>,
    pub(crate) translation: Option<Box<[crate::types::PassportFile]>>,
}

struct DriverLicenseOrIdentityCardVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for DriverLicenseOrIdentityCardVisitor {
    type Value = DriverLicenseOrIdentityCardFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_data = read_seq_field(&mut seq, 0, &expected)?;
        let variant_front_side = read_seq_field(&mut seq, 1, &expected)?;
        let variant_reverse_side = read_seq_field(&mut seq, 2, &expected)?;
        let variant_selfie = read_seq_field(&mut seq, 3, &expected)?;
        let variant_translation = read_seq_field(&mut seq, 4, &expected)?;
        let common_hash = read_seq_field(&mut seq, 5, &expected)?;
        Ok(DriverLicenseOrIdentityCardFields {
            common: CommonFields {
                hash: common_hash,
            },
            data: variant_data,
            front_side: variant_front_side,
            reverse_side: variant_reverse_side,
            selfie: variant_selfie,
            translation: variant_translation,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_data: Option<Box<str>> = None;
        let mut variant_front_side: Option<crate::types::PassportFile> = None;
        let mut variant_reverse_side: Option<crate::types::PassportFile> = None;
        let mut variant_selfie: Option<Option<crate::types::PassportFile>> = None;
        let mut variant_translation: Option<Option<Box<[crate::types::PassportFile]>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(&mut variant_data, &mut map, self.metadata.variant_fields[0])?;
                }
                2 => {
                    read_map_field(
                        &mut variant_front_side,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_reverse_side,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_selfie,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                5 => {
                    read_map_field(
                        &mut variant_translation,
                        &mut map,
                        self.metadata.variant_fields[4],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_data =
            variant_data.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_front_side = variant_front_side
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_reverse_side = variant_reverse_side
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[2]))?;
        let variant_selfie = variant_selfie.unwrap_or_default();
        let variant_translation = variant_translation.unwrap_or_default();
        let common_hash = shared.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
        Ok(DriverLicenseOrIdentityCardFields {
            common: CommonFields {
                hash: common_hash,
            },
            data: variant_data,
            front_side: variant_front_side,
            reverse_side: variant_reverse_side,
            selfie: variant_selfie,
            translation: variant_translation,
        })
    }
}

pub(crate) fn deserialize_driver_license_or_identity_card<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<DriverLicenseOrIdentityCardFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        DriverLicenseOrIdentityCardVisitor {
            metadata,
        },
    )
}

const fn internal_passport_or_passport_field_names(
    variant_fields: &[&'static str; 4],
) -> [&'static str; 5] {
    [
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
        variant_fields[3],
        "hash",
    ]
}

pub(crate) struct InternalPassportOrPassportFields {
    pub(crate) common: CommonFields,
    pub(crate) data: Box<str>,
    pub(crate) front_side: crate::types::PassportFile,
    pub(crate) selfie: Option<crate::types::PassportFile>,
    pub(crate) translation: Option<Box<[crate::types::PassportFile]>>,
}

struct InternalPassportOrPassportVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for InternalPassportOrPassportVisitor {
    type Value = InternalPassportOrPassportFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let variant_data = read_seq_field(&mut seq, 0, &expected)?;
        let variant_front_side = read_seq_field(&mut seq, 1, &expected)?;
        let variant_selfie = read_seq_field(&mut seq, 2, &expected)?;
        let variant_translation = read_seq_field(&mut seq, 3, &expected)?;
        let common_hash = read_seq_field(&mut seq, 4, &expected)?;
        Ok(InternalPassportOrPassportFields {
            common: CommonFields {
                hash: common_hash,
            },
            data: variant_data,
            front_side: variant_front_side,
            selfie: variant_selfie,
            translation: variant_translation,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_data: Option<Box<str>> = None;
        let mut variant_front_side: Option<crate::types::PassportFile> = None;
        let mut variant_selfie: Option<Option<crate::types::PassportFile>> = None;
        let mut variant_translation: Option<Option<Box<[crate::types::PassportFile]>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(&mut variant_data, &mut map, self.metadata.variant_fields[0])?;
                }
                2 => {
                    read_map_field(
                        &mut variant_front_side,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_selfie,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                4 => {
                    read_map_field(
                        &mut variant_translation,
                        &mut map,
                        self.metadata.variant_fields[3],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let variant_data =
            variant_data.ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        let variant_front_side = variant_front_side
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_selfie = variant_selfie.unwrap_or_default();
        let variant_translation = variant_translation.unwrap_or_default();
        let common_hash = shared.hash.ok_or_else(|| A::Error::missing_field("hash"))?;
        Ok(InternalPassportOrPassportFields {
            common: CommonFields {
                hash: common_hash,
            },
            data: variant_data,
            front_side: variant_front_side,
            selfie: variant_selfie,
            translation: variant_translation,
        })
    }
}

pub(crate) fn deserialize_internal_passport_or_passport<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<InternalPassportOrPassportFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        InternalPassportOrPassportVisitor {
            metadata,
        },
    )
}

pub(crate) static ADDRESS: Metadata = Metadata {
    name: "EncryptedPassportElementAddress",
    fields: &content_field_names(&["data"]),
    variant_fields: &["data"],
};

pub(crate) static EMAIL: Metadata = Metadata {
    name: "EncryptedPassportElementEmail",
    fields: &content_field_names(&["email"]),
    variant_fields: &["email"],
};

pub(crate) static PERSONAL_DETAILS: Metadata = Metadata {
    name: "EncryptedPassportElementPersonalDetails",
    fields: &content_field_names(&["data"]),
    variant_fields: &["data"],
};

pub(crate) static PHONE_NUMBER: Metadata = Metadata {
    name: "EncryptedPassportElementPhoneNumber",
    fields: &content_field_names(&["phone_number"]),
    variant_fields: &["phone_number"],
};

pub(crate) static BANK_STATEMENT: Metadata = Metadata {
    name: "EncryptedPassportElementBankStatement",
    fields: &files_field_names(&["files", "translation"]),
    variant_fields: &["files", "translation"],
};

pub(crate) static PASSPORT_REGISTRATION: Metadata = Metadata {
    name: "EncryptedPassportElementPassportRegistration",
    fields: &files_field_names(&["files", "translation"]),
    variant_fields: &["files", "translation"],
};

pub(crate) static RENTAL_AGREEMENT: Metadata = Metadata {
    name: "EncryptedPassportElementRentalAgreement",
    fields: &files_field_names(&["files", "translation"]),
    variant_fields: &["files", "translation"],
};

pub(crate) static TEMPORARY_REGISTRATION: Metadata = Metadata {
    name: "EncryptedPassportElementTemporaryRegistration",
    fields: &files_field_names(&["files", "translation"]),
    variant_fields: &["files", "translation"],
};

pub(crate) static UTILITY_BILL: Metadata = Metadata {
    name: "EncryptedPassportElementUtilityBill",
    fields: &files_field_names(&["files", "translation"]),
    variant_fields: &["files", "translation"],
};

pub(crate) static DRIVER_LICENSE: Metadata = Metadata {
    name: "EncryptedPassportElementDriverLicense",
    fields: &driver_license_or_identity_card_field_names(&[
        "data",
        "front_side",
        "reverse_side",
        "selfie",
        "translation",
    ]),
    variant_fields: &[
        "data",
        "front_side",
        "reverse_side",
        "selfie",
        "translation",
    ],
};

pub(crate) static IDENTITY_CARD: Metadata = Metadata {
    name: "EncryptedPassportElementIdentityCard",
    fields: &driver_license_or_identity_card_field_names(&[
        "data",
        "front_side",
        "reverse_side",
        "selfie",
        "translation",
    ]),
    variant_fields: &[
        "data",
        "front_side",
        "reverse_side",
        "selfie",
        "translation",
    ],
};

pub(crate) static INTERNAL_PASSPORT: Metadata = Metadata {
    name: "EncryptedPassportElementInternalPassport",
    fields: &internal_passport_or_passport_field_names(&[
        "data",
        "front_side",
        "selfie",
        "translation",
    ]),
    variant_fields: &["data", "front_side", "selfie", "translation"],
};

pub(crate) static PASSPORT: Metadata = Metadata {
    name: "EncryptedPassportElementPassport",
    fields: &internal_passport_or_passport_field_names(&[
        "data",
        "front_side",
        "selfie",
        "translation",
    ]),
    variant_fields: &["data", "front_side", "selfie", "translation"],
};
