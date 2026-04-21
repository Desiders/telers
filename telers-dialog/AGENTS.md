# telers-dialog

- Use `aiogram-dialog` only as behavior reference; keep Rust APIs idiomatic over `telers`.
- Keep state `serde`-serializable for `telers::fsm::Context`.
- Store shared dialog registration in `DialogRegistry` inside dispatcher extensions.
- Prefer `DialogObserverExt::setup_dialogs::<S>()` for observer wiring.
- Register command/custom handlers before `.setup_dialogs::<S>()` when they must win over dialog input.
- `DialogContextMiddleware` must run before `DialogManagerMiddleware`.
- Keep builder APIs owned (`self -> Self`) and hide internal wrapping like `Arc`.
- Keep callback data stable and scoped: `td:{intent_id}:{button_id}[:payload]`.
- Ignore stale callbacks from other intent ids.
- Keep `DialogManager::done()` and `ShowMode::Auto` aligned with `aiogram-dialog` unless Rust design needs differ.
- Do not run formatting unless asked.
- Keep `PROGRESS.md` up to date after meaningful changes.
- Keep output short: changes, blockers, validation only.
- Validate with `cargo check -p telers-dialog --all-features`; run tests/clippy only when relevant or requested.
