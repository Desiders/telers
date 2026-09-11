//! Every error branch of `#[derive(Command)]`
#![allow(dead_code)]

use telers_macros::Command;

fn main() {}

// 1. Struct instead of an enum
#[derive(Command)]
struct E01;

// 2. Union instead of an enum
#[derive(Command)]
union E02 {
    field: i32,
}

// 3. Generic enum
#[derive(Command)]
enum E03<T> {
    Start(T),
}

// 4. Enum with a lifetime
#[derive(Command)]
enum E04<'a> {
    Start(&'a str),
}

// 5. Unknown `rename_rule`
#[derive(Command)]
#[command(rename_rule = "kebab-case")]
enum E05 {
    Start,
}

// 6. `rename_rule` is not a string literal
#[derive(Command)]
#[command(rename_rule = snake_case)]
enum E06 {
    Start,
}

// 7. Duplicate `rename_rule`
#[derive(Command)]
#[command(rename_rule = "lowercase", rename_rule = "snake_case")]
enum E07 {
    Start,
}

// 8. Duplicate enum-level `prefix`
#[derive(Command)]
#[command(prefix = '/', prefix = '!')]
enum E08 {
    Start,
}

// 9. Duplicate enum-level `split`
#[derive(Command)]
#[command(split = ',', split = ';')]
enum E09 {
    Start,
}

// 10. Repeated enum-level `#[command(...)]` attribute
#[derive(Command)]
#[command(prefix = '/')]
#[command(split = ',')]
enum E10 {
    Start,
}

// 11. Unknown enum-level argument
#[derive(Command)]
#[command(unknown = "a")]
enum E11 {
    Start,
}

// 12. Variant-level argument `description` at the enum level
#[derive(Command)]
#[command(description = "a")]
enum E12 {
    Start,
}

// 13. Variant-level argument `hidden` at the enum level
#[derive(Command)]
#[command(hidden)]
enum E13 {
    Start,
}

// 14. `#[command]` without arguments at the enum level
#[derive(Command)]
#[command]
enum E14 {
    Start,
}

// 15. Unknown variant-level argument
#[derive(Command)]
enum E15 {
    #[command(unknown = "a")]
    Start,
}

// 16. Enum-level argument `rename_rule` at the variant level
#[derive(Command)]
enum E16 {
    #[command(rename_rule = "snake_case")]
    Start,
}

// 17. `#[command]` without arguments at the variant level
#[derive(Command)]
enum E17 {
    #[command]
    Start,
}

// 18. Repeated variant-level `#[command(...)]` attribute
#[derive(Command)]
enum E18 {
    #[command(description = "a")]
    #[command(hidden)]
    Start,
}

// 19. Duplicate `description`
#[derive(Command)]
enum E19 {
    #[command(description = "a", description = "b")]
    Start,
}

// 20. Duplicate `hidden`
#[derive(Command)]
enum E20 {
    #[command(hidden, hidden)]
    Start,
}

// 21. Duplicate `aliases`
#[derive(Command)]
enum E21 {
    #[command(aliases = ["a"], aliases = ["b"])]
    Start,
}

// 22. Duplicate `rename`
#[derive(Command)]
enum E22 {
    #[command(rename = "a", rename = "b")]
    Start,
}

// 23. Duplicate variant-level `prefix`
#[derive(Command)]
enum E23 {
    #[command(prefix = '!', prefix = '?')]
    Start,
}

// 24. Duplicate variant-level `split`
#[derive(Command)]
enum E24 {
    #[command(split = ',', split = ';')]
    Start,
}

// 25. `hidden` with a value
#[derive(Command)]
enum E25 {
    #[command(hidden = true)]
    Start,
}

// 26. `description` is not a string literal
#[derive(Command)]
enum E26 {
    #[command(description = 1)]
    Start,
}

// 27. `rename` without a value
#[derive(Command)]
enum E27 {
    #[command(rename)]
    Start,
}

// 28. Enum-level `prefix` is a string literal instead of a char literal
#[derive(Command)]
#[command(prefix = "/")]
enum E28 {
    Start,
}

// 29. Variant-level `prefix` is a string literal instead of a char literal
#[derive(Command)]
enum E29 {
    #[command(prefix = "!")]
    Start,
}

// 30. `split` is a string literal instead of a char literal
#[derive(Command)]
#[command(split = ",")]
enum E30 {
    Start,
}

// 31. `aliases` is not a list
#[derive(Command)]
enum E31 {
    #[command(aliases = "a")]
    Start,
}

// 32. `aliases` with an item that is not a string literal
#[derive(Command)]
enum E32 {
    #[command(aliases = ["a", 1])]
    Start,
}

// 33. Duplicate command name of an alias and a variant
#[derive(Command)]
enum E33 {
    #[command(aliases = ["help"])]
    Start,
    Help,
}

// 34. Duplicate command name of a `rename` and a variant
#[derive(Command)]
enum E34 {
    #[command(rename = "start")]
    Begin,
    Start,
}

// 35. Duplicate command name of variants differing only by case
#[derive(Command)]
enum E35 {
    Start,
    StArt,
}

// 36. Duplicate command name of the aliases of two variants
#[derive(Command)]
enum E36 {
    #[command(aliases = ["go"])]
    Start,
    #[command(aliases = ["go"])]
    Begin,
}

// 37. Duplicate command name of a variant and its own alias
#[derive(Command)]
enum E37 {
    #[command(aliases = ["start"])]
    Start,
}
