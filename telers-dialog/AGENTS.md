# telers-dialog Guidelines

- Use the sibling `aiogram_dialog` checkout as a behavioral reference, but translate concepts into idiomatic Rust and `telers` APIs instead of copying Python structure.
- Keep dialog state `serde`-serializable so it can live inside `telers::fsm::Context`.
- Integrate through `telers` primitives only: routers, observers, middlewares, extractors, filters, and dispatcher `extensions`.
- Shared dialog registration belongs in `DialogRegistry` stored in dispatcher `extensions`.
- Prefer `DialogObserverExt::setup_dialogs::<S>()` for observer wiring. Keep setup explicit at the observer level; when command handlers must win before dialog message processing, register them before calling `setup_dialogs::<S>()` on that observer.
- Public builder-style APIs should prefer owned `self -> Self` chaining. Public dialog/window registration should accept concrete values and wrap them internally instead of requiring manual `Arc::new(...)`.
- Follow `aiogram-dialog` semantics where they matter, especially for `DialogManager::done()` and `ShowMode::Auto`, unless there is a clear Rust-side reason to diverge.
- Callback payloads are scoped as `td:{intent_id}:{button_id}` and must reject stale buttons from other dialog intents.
- Use `tracing`, but keep logs high-signal. Favor lifecycle, callback, setup-failure, and message-update decisions; avoid noisy constructor/render-helper logs.
- Prefer the repo `justfile` helpers for standard maintenance commands when applicable, especially `just fmt` and `just clippy`.
- Validate with `cargo check -p telers-dialog` and `cargo test -p telers-dialog`. If integration or examples changed, also run the narrowest affected example checks for the specific `dialogs_*` example package(s) you touched.
- Keep `PROGRESS.md` updated with the current architecture, validated behavior, known gaps, and the next recommended slice.
- Keep the `../examples/dialogs_*` example packages in sync with the public integration API.
