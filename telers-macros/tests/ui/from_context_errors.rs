//! Every error branch of `#[derive(FromContext)]`
#![allow(dead_code)]

use telers_macros::FromContext;

#[derive(Clone)]
struct Wrapper;

#[derive(Clone)]
struct Source;

fn main() {}

// 1. Missing `#[context(...)]` attribute
#[derive(Clone, FromContext)]
struct E01;

// 2. `#[context]` without arguments
#[derive(Clone, FromContext)]
#[context]
struct E02;

// 3. `#[context()]` without `key`
#[derive(Clone, FromContext)]
#[context()]
struct E03;

// 4. Missing `key` with the other arguments set
#[derive(Clone, FromContext)]
#[context(into = Wrapper, description = "no key")]
struct E04;

// 5. Repeated `#[context(...)]` attribute
#[derive(Clone, FromContext)]
#[context(key = "a")]
#[context(key = "b")]
struct E05;

// 6. Duplicate `key`
#[derive(Clone, FromContext)]
#[context(key = "a", key = "b")]
struct E06;

// 7. Duplicate `into`
#[derive(Clone, FromContext)]
#[context(key = "a", into = Wrapper, into = Wrapper)]
struct E07;

// 8. Duplicate `from`
#[derive(Clone, FromContext)]
#[context(key = "a", from = Source, from = Source)]
struct E08;

// 9. Duplicate `description`
#[derive(Clone, FromContext)]
#[context(key = "a", description = "a", description = "b")]
struct E09;

// 10. `into` and `from` at the same time
#[derive(Clone, FromContext)]
#[context(key = "a", into = Wrapper, from = Source)]
struct E10;

// 11. Unknown argument
#[derive(Clone, FromContext)]
#[context(key = "a", unknown = "b")]
struct E11;

// 12. `key` without a value
#[derive(Clone, FromContext)]
#[context(key)]
struct E12;

// 13. `key` is a number instead of a string literal
#[derive(Clone, FromContext)]
#[context(key = 1)]
struct E13;

// 14. `key` is an identifier instead of a string literal
#[derive(Clone, FromContext)]
#[context(key = a)]
struct E14;

// 15. `description` is not a string literal
#[derive(Clone, FromContext)]
#[context(key = "a", description = 1)]
struct E15;

// 16. Missing comma between the arguments
#[derive(Clone, FromContext)]
#[context(key = "a" into = Wrapper)]
struct E16;

// 17. Double comma between the arguments
#[derive(Clone, FromContext)]
#[context(key = "a",, into = Wrapper)]
struct E17;

// 18. Union instead of a struct or an enum
#[derive(FromContext)]
#[context(key = "a")]
union E18 {
    field: i32,
}
