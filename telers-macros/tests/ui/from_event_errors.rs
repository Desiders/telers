//! Every error branch of `#[derive(FromEvent)]`
#![allow(dead_code)]

use telers_macros::FromEvent;

fn main() {}

// 1. Missing `#[event(...)]` attribute
#[derive(FromEvent)]
struct E01;

// 2. `#[event]` without arguments
#[derive(FromEvent)]
#[event]
struct E02;

// 3. `#[event()]` without `from` or `try_from`
#[derive(FromEvent)]
#[event()]
struct E03;

// 4. Only `description`, no conversion
#[derive(FromEvent)]
#[event(description = "no conversion")]
struct E04;

// 5. `from` and `try_from` at the same time
#[derive(FromEvent)]
#[event(from = Update, try_from = Update)]
struct E05;

// 6. `error` with `from`
#[derive(FromEvent)]
#[event(from = Update, error = Infallible)]
struct E06;

// 7. Unknown type in `from`
#[derive(FromEvent)]
#[event(from = Message)]
struct E07;

// 8. Unknown type in `try_from`
#[derive(FromEvent)]
#[event(try_from = Message)]
struct E08;

// 9. Path instead of the bare `Update`
#[derive(FromEvent)]
#[event(from = telers::types::Update)]
struct E09;

// 10. Unknown argument
#[derive(FromEvent)]
#[event(from = Update, unknown = "b")]
struct E10;

// 11. Duplicate `from`
#[derive(FromEvent)]
#[event(from = Update, from = Update)]
struct E11;

// 12. Duplicate `try_from`
#[derive(FromEvent)]
#[event(try_from = Update, try_from = Update)]
struct E12;

// 13. Duplicate `error`
#[derive(FromEvent)]
#[event(try_from = Update, error = Infallible, error = Infallible)]
struct E13;

// 14. Duplicate `description`
#[derive(FromEvent)]
#[event(from = Update, description = "a", description = "b")]
struct E14;

// 15. `from` without a value
#[derive(FromEvent)]
#[event(from)]
struct E15;

// 16. `from` is a string literal instead of a type
#[derive(FromEvent)]
#[event(from = "Update")]
struct E16;

// 17. `description` is not a string literal
#[derive(FromEvent)]
#[event(from = Update, description = 1)]
struct E17;

// 18. Missing comma between the arguments
#[derive(FromEvent)]
#[event(try_from = Update error = Infallible)]
struct E18;

// 19. Union instead of a struct or an enum
#[derive(FromEvent)]
#[event(from = Update)]
union E19 {
    field: i32,
}
