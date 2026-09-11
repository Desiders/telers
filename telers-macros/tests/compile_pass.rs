//! Compile-and-run (UI) tests of the derive macros with combinations of their arguments, via `trybuild`.
//! Each fixture in `tests/ui/` derives a macro with the supported combinations of its arguments
//! and asserts the generated code at runtime.

#[test]
fn from_context_combinations() {
    trybuild::TestCases::new().pass("tests/ui/from_context_combinations.rs");
}

#[test]
fn from_event_combinations() {
    trybuild::TestCases::new().pass("tests/ui/from_event_combinations.rs");
}

#[test]
fn callback_data_combinations() {
    trybuild::TestCases::new().pass("tests/ui/callback_data_combinations.rs");
}

#[test]
fn command_combinations() {
    trybuild::TestCases::new().pass("tests/ui/command_combinations.rs");
}
