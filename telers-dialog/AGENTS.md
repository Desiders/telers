# telers-dialog Guidelines

- Use the sibling `aiogram_dialog` checkout as a behavioral reference, but translate concepts into idiomatic Rust and `telers` APIs instead of copying Python structure.
- Keep dialog state `serde`-serializable so it can live inside `telers::fsm::Context`.
- Integrate through `telers` primitives only: routers, observers, middlewares, extractors, filters, and dispatcher `extensions`.
- Shared dialog registration belongs in `Dialogs` stored in dispatcher `extensions`.
- Prefer `DialogObserverExt::setup_dialogs::<S>()` for observer wiring. Keep setup explicit; avoid hidden router patching or large global setup helpers.
- Public builder-style APIs should prefer owned `self -> Self` chaining. Public dialog/window registration should accept concrete values and wrap them internally instead of requiring manual `Arc::new(...)`.
- Follow `aiogram-dialog` semantics where they matter, especially for `DialogManager::done()` and `ShowMode::Auto`, unless there is a clear Rust-side reason to diverge.
- Callback payloads are scoped as `td:{intent_id}:{button_id}` and must reject stale buttons from other dialog intents.
- Use `tracing`, but keep logs high-signal. Favor lifecycle, callback, setup-failure, and message-update decisions; avoid noisy constructor/render-helper logs.
- Validate with `cargo check -p telers-dialog` and `cargo test -p telers-dialog`. If integration or examples changed, also run the narrowest affected example checks, usually `cargo check -p dialogs`.
- Keep `PROGRESS.md` updated with the current architecture, validated behavior, known gaps, and the next recommended slice.
- Keep `../examples/dialogs` in sync with the public integration API.
