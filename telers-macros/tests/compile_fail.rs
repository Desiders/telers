//! Compile-fail (UI) tests of the diagnostics of the derive macros, via `trybuild`.
//! Each fixture in `tests/ui/` misuses a macro in every way it reports; its `.stderr` snapshot
//! pins the exact `compile_error!` message of every error branch.
//!
//! Snapshots are toolchain-sensitive (they capture rustc's rendered diagnostic). Run these on a
//! stable toolchain and regenerate after a compiler or macro change with
//! `TRYBUILD=overwrite cargo test -p telers-macros --test compile_fail`.

#[test]
fn from_context_errors() {
    trybuild::TestCases::new().compile_fail("tests/ui/from_context_errors.rs");
}

#[test]
fn from_event_errors() {
    trybuild::TestCases::new().compile_fail("tests/ui/from_event_errors.rs");
}

#[test]
fn callback_data_errors() {
    trybuild::TestCases::new().compile_fail("tests/ui/callback_data_errors.rs");
}

#[test]
fn command_errors() {
    trybuild::TestCases::new().compile_fail("tests/ui/command_errors.rs");
}
