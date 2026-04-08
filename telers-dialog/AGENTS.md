# telers-dialog Guidelines

## Scope and direction
- Use `aiogram-dialog` as a behavioral reference, but implement idiomatic Rust APIs over `telers` primitives.
- Keep dialog state `serde`-serializable so it can be persisted in `telers::fsm::Context`.
- Keep integration explicit: dispatcher `extensions`, observer middleware chain, and extractors.
- Keep the crate intentionally smaller than `aiogram-dialog`; add parity only where it improves core dialog UX.

## Architecture rules
- Store shared dialog registration in `DialogRegistry` inside dispatcher `extensions`.
- Prefer `DialogObserverExt::setup_dialogs::<S>()` for observer wiring.
- If command/custom handlers must win over dialog message input, register them before `.setup_dialogs::<S>()` on that observer.
- `DialogManagerMiddleware<S>` expects `DialogContextMiddleware` to run first on the same observer chain.

## API design rules
- Public builder APIs should prefer owned chaining (`self -> Self`) over mutable chaining (`&mut self`).
- Public registration APIs should accept concrete values and wrap internally (avoid forcing `Arc::new(...)` at call sites).
- Keep callback payload namespace stable and scoped: `td:{intent_id}:{button_id}[:payload]`.
- Stale callbacks from other intent ids must be ignored.
- Keep `DialogManager::done()` and `ShowMode::Auto` behavior aligned with `aiogram-dialog` semantics unless there is a clear Rust-side reason to diverge.

## Code style
- Follow workspace formatting from [`rustfmt.toml`](/workspace/rustfmt.toml) and keep code compatible with the configured rules (edition 2021, crate-level import granularity, normalized docs/comments).
- Use `clippy::pedantic` clean style for touched code paths when practical.
- Prefer small, explicit methods with strong types (`Cow<'static, str>`, enums, newtypes) over stringly APIs.
- Keep logs high-signal with `tracing`; focus on lifecycle/actions/errors, avoid render-time noise.
- Add tests with behavior changes, especially manager transitions, callback routing, and input/widget state interactions.
- In tests, avoid `.expect(...)`; prefer `.unwrap()` for assertion flow consistency.

## Workspace scripts (`justfile`)
- Prefer workspace recipes from [`justfile`](/workspace/justfile):
- `just fmt` -> `cargo +nightly fmt --all`
- `just clippy` -> `cargo clippy --all --all-features -- -W clippy::pedantic`
- `just test` -> `cargo test --lib --tests --all --all-features -- --nocapture`
- `just build` -> `cargo build --all --all-features`
- If `just` is unavailable in the environment, run equivalent `cargo` commands directly.

## Validation checklist for telers-dialog changes
- `cargo check -p telers-dialog`
- `cargo test -p telers-dialog`
- If API/integration behavior changed: `cargo check -p dialogs_message_input`, `cargo check -p dialogs_select_widget`, and any directly affected `examples/dialogs_*` package.

## Documentation hygiene
- Keep [`PROGRESS.md`](/workspace/telers-dialog/PROGRESS.md) updated with architecture, verified behavior, parity/gaps vs `aiogram-dialog`, and next slice.
- Keep `examples/dialogs_*` packages aligned with the public API and middleware integration pattern.
