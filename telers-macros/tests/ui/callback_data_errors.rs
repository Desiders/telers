//! Every error branch of `#[derive(CallbackData)]`
#![allow(dead_code)]

use telers_macros::CallbackData;

fn main() {}

// 1. Enum instead of a struct
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a")]
enum E01 {
    Variant,
}

// 2. Tuple struct
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a")]
struct E02(String);

// 3. Unit struct
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a")]
struct E03;

// 4. Struct without fields
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a")]
struct E04 {}

// 5. Missing `#[callback_data(...)]` attribute
#[derive(CallbackData, Clone)]
struct E05 {
    field: String,
}

// 6. `#[callback_data]` without arguments
#[derive(CallbackData, Clone)]
#[callback_data]
struct E06 {
    field: String,
}

// 7. `#[callback_data()]` without `prefix`
#[derive(CallbackData, Clone)]
#[callback_data()]
struct E07 {
    field: String,
}

// 8. Missing `prefix` with `separator` set
#[derive(CallbackData, Clone)]
#[callback_data(separator = '|')]
struct E08 {
    field: String,
}

// 9. Repeated `#[callback_data(...)]` attribute
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a")]
#[callback_data(separator = '|')]
struct E09 {
    field: String,
}

// 10. Duplicate `prefix`
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a", prefix = "b")]
struct E10 {
    field: String,
}

// 11. Duplicate `separator`
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a", separator = '|', separator = ';')]
struct E11 {
    field: String,
}

// 12. Unknown argument
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a", unknown = "b")]
struct E12 {
    field: String,
}

// 13. `prefix` without a value
#[derive(CallbackData, Clone)]
#[callback_data(prefix)]
struct E13 {
    field: String,
}

// 14. `prefix` is not a string literal
#[derive(CallbackData, Clone)]
#[callback_data(prefix = 1)]
struct E14 {
    field: String,
}

// 15. `separator` is a string literal instead of a char literal
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a", separator = ":")]
struct E15 {
    field: String,
}

// 16. `separator` is a number instead of a char literal
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a", separator = 1)]
struct E16 {
    field: String,
}

// 17. Missing comma between the arguments
#[derive(CallbackData, Clone)]
#[callback_data(prefix = "a" separator = '|')]
struct E17 {
    field: String,
}
