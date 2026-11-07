#![allow(clippy::too_many_lines)]

use super::{read_map_field, read_seq_field, FieldKey, FieldLookup, Metadata, SequenceExpected};
use serde::{
    de::{Error, IgnoredAny, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

pub(crate) struct CommonFields {
    pub(crate) user: Box<crate::types::User>,
}

#[derive(Default)]
#[allow(clippy::option_option)]
struct CommonSlots {
    user: Option<Box<crate::types::User>>,
}

impl FieldLookup for CommonSlots {
    const COUNT: usize = 1;

    #[inline]
    fn field(value: &str) -> Option<usize> {
        match value {
            "user" => Some(0),
            _ => None,
        }
    }

    #[inline]
    fn field_bytes(value: &[u8]) -> Option<usize> {
        match value {
            b"user" => Some(0),
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
            0 => read_map_field(&mut self.user, map, "user"),
            _ => unreachable!(),
        }
    }
}

const COMMON_FIELD_NAMES: [&str; 1] = ["user"];

struct CommonVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for CommonVisitor {
    type Value = CommonFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        Ok(CommonFields {
            user: common_user,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        Ok(CommonFields {
            user: common_user,
        })
    }
}

pub(crate) fn deserialize_common<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<CommonFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        CommonVisitor {
            metadata,
        },
    )
}

const fn content_field_names(variant_fields: &[&'static str; 1]) -> [&'static str; 2] {
    ["user", variant_fields[0]]
}

pub(crate) struct ContentFields<TContent> {
    pub(crate) common: CommonFields,
    pub(crate) content: TContent,
}

struct ContentVisitor<TContent> {
    metadata: &'static Metadata,
    marker: PhantomData<ContentFields<TContent>>,
}

impl<'de, TContent> Visitor<'de> for ContentVisitor<TContent>
where
    TContent: Deserialize<'de>,
{
    type Value = ContentFields<TContent>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_content = read_seq_field(&mut seq, 1, &expected)?;
        Ok(ContentFields {
            common: CommonFields {
                user: common_user,
            },
            content: variant_content,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_content: Option<TContent> = None;
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
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_content = variant_content
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[0]))?;
        Ok(ContentFields {
            common: CommonFields {
                user: common_user,
            },
            content: variant_content,
        })
    }
}

pub(crate) fn deserialize_content<'de, D, TContent>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<ContentFields<TContent>, D::Error>
where
    D: Deserializer<'de>,
    TContent: Deserialize<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        ContentVisitor {
            metadata,
            marker: PhantomData,
        },
    )
}

const fn invoice_payment_field_names(variant_fields: &[&'static str; 3]) -> [&'static str; 4] {
    [
        "user",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
    ]
}

pub(crate) struct InvoicePaymentFields {
    pub(crate) common: CommonFields,
    pub(crate) affiliate: Option<crate::types::AffiliateInfo>,
    pub(crate) invoice_payload: Option<Box<str>>,
    pub(crate) subscription_period: Option<i64>,
}

struct InvoicePaymentVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for InvoicePaymentVisitor {
    type Value = InvoicePaymentFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_affiliate = read_seq_field(&mut seq, 1, &expected)?;
        let variant_invoice_payload = read_seq_field(&mut seq, 2, &expected)?;
        let variant_subscription_period = read_seq_field(&mut seq, 3, &expected)?;
        Ok(InvoicePaymentFields {
            common: CommonFields {
                user: common_user,
            },
            affiliate: variant_affiliate,
            invoice_payload: variant_invoice_payload,
            subscription_period: variant_subscription_period,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_affiliate: Option<Option<crate::types::AffiliateInfo>> = None;
        let mut variant_invoice_payload: Option<Option<Box<str>>> = None;
        let mut variant_subscription_period: Option<Option<i64>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_affiliate,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_invoice_payload,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_subscription_period,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_affiliate = variant_affiliate.unwrap_or_default();
        let variant_invoice_payload = variant_invoice_payload.unwrap_or_default();
        let variant_subscription_period = variant_subscription_period.unwrap_or_default();
        Ok(InvoicePaymentFields {
            common: CommonFields {
                user: common_user,
            },
            affiliate: variant_affiliate,
            invoice_payload: variant_invoice_payload,
            subscription_period: variant_subscription_period,
        })
    }
}

pub(crate) fn deserialize_invoice_payment<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<InvoicePaymentFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        InvoicePaymentVisitor {
            metadata,
        },
    )
}

const fn paid_media_payment_field_names(variant_fields: &[&'static str; 3]) -> [&'static str; 4] {
    [
        "user",
        variant_fields[0],
        variant_fields[1],
        variant_fields[2],
    ]
}

pub(crate) struct PaidMediaPaymentFields {
    pub(crate) common: CommonFields,
    pub(crate) affiliate: Option<crate::types::AffiliateInfo>,
    pub(crate) paid_media: Box<[crate::types::PaidMedia]>,
    pub(crate) paid_media_payload: Option<Box<str>>,
}

struct PaidMediaPaymentVisitor {
    metadata: &'static Metadata,
}

impl<'de> Visitor<'de> for PaidMediaPaymentVisitor {
    type Value = PaidMediaPaymentFields;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(self.metadata.name)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let expected = SequenceExpected(self.metadata);
        let common_user = read_seq_field(&mut seq, 0, &expected)?;
        let variant_affiliate = read_seq_field(&mut seq, 1, &expected)?;
        let variant_paid_media = read_seq_field(&mut seq, 2, &expected)?;
        let variant_paid_media_payload = read_seq_field(&mut seq, 3, &expected)?;
        Ok(PaidMediaPaymentFields {
            common: CommonFields {
                user: common_user,
            },
            affiliate: variant_affiliate,
            paid_media: variant_paid_media,
            paid_media_payload: variant_paid_media_payload,
        })
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut shared = CommonSlots::default();
        let mut variant_affiliate: Option<Option<crate::types::AffiliateInfo>> = None;
        let mut variant_paid_media: Option<Box<[crate::types::PaidMedia]>> = None;
        let mut variant_paid_media_payload: Option<Option<Box<str>>> = None;
        while let Some(field) = map.next_key_seed(FieldKey::<CommonSlots>::new(self.metadata))? {
            match field {
                index if index < CommonSlots::COUNT => shared.read_field(index, &mut map)?,
                1 => {
                    read_map_field(
                        &mut variant_affiliate,
                        &mut map,
                        self.metadata.variant_fields[0],
                    )?;
                }
                2 => {
                    read_map_field(
                        &mut variant_paid_media,
                        &mut map,
                        self.metadata.variant_fields[1],
                    )?;
                }
                3 => {
                    read_map_field(
                        &mut variant_paid_media_payload,
                        &mut map,
                        self.metadata.variant_fields[2],
                    )?;
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        let common_user = shared.user.ok_or_else(|| A::Error::missing_field("user"))?;
        let variant_affiliate = variant_affiliate.unwrap_or_default();
        let variant_paid_media = variant_paid_media
            .ok_or_else(|| A::Error::missing_field(self.metadata.variant_fields[1]))?;
        let variant_paid_media_payload = variant_paid_media_payload.unwrap_or_default();
        Ok(PaidMediaPaymentFields {
            common: CommonFields {
                user: common_user,
            },
            affiliate: variant_affiliate,
            paid_media: variant_paid_media,
            paid_media_payload: variant_paid_media_payload,
        })
    }
}

pub(crate) fn deserialize_paid_media_payment<'de, D>(
    deserializer: D,
    metadata: &'static Metadata,
) -> Result<PaidMediaPaymentFields, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_struct(
        metadata.name,
        metadata.fields,
        PaidMediaPaymentVisitor {
            metadata,
        },
    )
}

pub(crate) static BUSINESS_ACCOUNT_TRANSFER: Metadata = Metadata {
    name: "TransactionPartnerUserBusinessAccountTransfer",
    fields: &COMMON_FIELD_NAMES,
    variant_fields: &[],
};

pub(crate) static GIFT_PURCHASE: Metadata = Metadata {
    name: "TransactionPartnerUserGiftPurchase",
    fields: &content_field_names(&["gift"]),
    variant_fields: &["gift"],
};

pub(crate) static PREMIUM_PURCHASE: Metadata = Metadata {
    name: "TransactionPartnerUserPremiumPurchase",
    fields: &content_field_names(&["premium_subscription_duration"]),
    variant_fields: &["premium_subscription_duration"],
};

pub(crate) static INVOICE_PAYMENT: Metadata = Metadata {
    name: "TransactionPartnerUserInvoicePayment",
    fields: &invoice_payment_field_names(&["affiliate", "invoice_payload", "subscription_period"]),
    variant_fields: &["affiliate", "invoice_payload", "subscription_period"],
};

pub(crate) static PAID_MEDIA_PAYMENT: Metadata = Metadata {
    name: "TransactionPartnerUserPaidMediaPayment",
    fields: &paid_media_payment_field_names(&["affiliate", "paid_media", "paid_media_payload"]),
    variant_fields: &["affiliate", "paid_media", "paid_media_payload"],
};
