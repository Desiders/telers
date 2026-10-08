//! Every error branch of `#[derive(State)]`
#![allow(dead_code)]

use telers_macros::State;

fn main() {}

// 1. Struct instead of an enum
#[derive(State)]
struct E01;

// 2. Union instead of an enum
#[derive(State)]
union E02 {
    field: i32,
}

// 3. Generic enum
#[derive(State)]
enum E03<T> {
    Start(T),
}

// 4. Enum with a lifetime
#[derive(State)]
enum E04<'a> {
    Start(&'a str),
}

// 5. Tuple variant
#[derive(State)]
enum E05 {
    Start(i32),
}

// 6. Named variant
#[derive(State)]
enum E06 {
    Start { at: i32 },
}

// 7. Duplicate state name
#[derive(State)]
enum E07 {
    Start,
    START,
}