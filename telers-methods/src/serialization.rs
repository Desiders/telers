//! Serialization traits and JSON operations shared with Telegram data types.
pub use telers_types::serialization::*;

const _: () = assert!(
    telers_types::serialization::USES_DESER == cfg!(feature = "deser"),
    "telers-types uses Deser: enable the `deser` feature on telers-methods as well"
);
