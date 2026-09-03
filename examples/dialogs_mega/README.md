# dialogs_mega

A "mega" demo bot for the `telers-dialog` crate. It combines the smaller dialog examples into one bot: a root *main menu* dialog launches each feature dialog on top of the stack, and every feature screen carries a `☰ Main menu` button to return.

## What it does

Send `/start` and the bot shows a main menu of inline buttons, one per feature area. Tapping a button opens that feature's dialog on top of the menu; navigating with `Next`/`Back`/`Switch to` buttons moves between its windows, and `☰ Main menu` (a `Button::done`) closes the feature dialog and pops back to the menu. `/start` always resets the whole dialog stack and returns to a clean menu.

Each feature dialog is self-contained and exercises one part of the widget toolkit — text rendering, keyboards, selection state, scrolling, calendars, inputs, media, and so on (full list below).

## How it works

`main.rs` wires the dialog runtime into a normal telers `Router`/`Dispatcher`:

- An FSM is provided by `FSMContextMiddleware` over a `MemoryStorage`, registered as an outer middleware on the update observer with `Strategy::UserInChat`.
- `DialogObserverExt::setup_dialogs::<MemoryStorage>()` is attached to both the message observer and the callback-query observer so the dialog engine processes those updates.
- A `Handler` filtered by `CommandStart` calls `DialogManager::start(..., MAIN_MENU_STATE, Value::Null, StartMode::ResetStack)` to (re)open the menu.
- `registry()` builds a `DialogRegistry`, registering every `dialogs::<feature>::dialog()` in turn (each `register` returns a `Result`, chained with `and_then`, validated at startup so duplicate state ids fail fast).
- The registry is passed to the `Dispatcher` as an `extension`; `allowed_updates` is `[Message, CallbackQuery]`.

Each module under `src/dialogs/` exposes a `dialog()` returning `impl Dialog`, built from `window(state, [widgets...])` screens. Widgets come from `telers_dialog::widgets` — `text`/`format_text`/`fn_text`, `keyboard`/`InlineKeyboard`, `Button` and `ButtonAction` (`next`, `back`, `switch_to`, `done`, `chain`, `set_dialog_value`, `extend_dialog_data`, `on_click`), selection widgets, scrolls, pagers, and input widgets. The root menu uses `LaunchMode::Root` so it always resets the stack; feature dialogs are started from menu buttons with `StartMode::Normal`. Shared helpers (the `main_menu_button` and the `FRUITS` catalog) live in `src/common.rs`.

Feature dialogs covered:

- **Text widgets** — static `text`, `format_text`/`FormatText`, computed `fn_text`/`FnText`, and `ListText`, combined in one window.
- **Template text** — `TemplateText` (minijinja) with the default environment and a custom `TemplateEnvBuilder` adding a `currency` filter and a `brand` global (requires the `template` feature).
- **Scrolling widgets** — `ScrollingGroup`, `ScrollingText`, a paged `ListText`, `StubScroll` driving custom text, and two scrolls synced with `sync_scroll`, all paged by `NumberedPager`.
- **Keyboard layouts** — the same `Select` arranged via `Group` `items_per_row` at width 4, 1, and 2.
- **Selection widgets** — `Select`, `Radio`, `Multiselect` (with min/max selected), and `Toggle`.
- **Combined widgets** — `Checkbox`, `Radio`, `Multiselect`, and `Counter` living in a single window, each keeping its own `widget_data`.
- **Counter & progress** — a `Counter` paired with a custom progress-bar `Text` that reads the counter's `widget_data` from the render context.
- **Calendar & time** — a default `Calendar`, a customized one via `CalendarAppearance::text_renderer`, and a `TimeSelect` grid.
- **Multi-step input** — a `Next`/`Back` flow across windows with a `Case` summary of the chosen plan.
- **Reply keyboard** — `RequestContact`, `RequestLocation`, and `RequestPoll` request widgets, each paired with a `MessageInput` that stores the Telegram-native payload and advances.
- **Text & force reply** — `TextInput` with typed `i64` parsing and an `on_error` branch, plus `ForceReply` paired with a `MessageInput`.
- **Button styles** — `.danger()`/`.success()`/`.primary()` coloured buttons and dynamic payloads (`Button::url_dynamic`, `copy_text_dynamic`, `switch_inline_query_dynamic`, `web_app_dynamic` rendered from `FormatText`).
- **Button actions** — declarative `ButtonAction::chain` transitions versus a `Button::on_click` async handler that validates dialog data before acting.
- **Link preview** — `LinkPreview` options: disabled, prefer small/large media, and show-above-text.
- **Media widgets** — `StaticMedia` from a URL, `DynamicMedia` from a data field, and a `MediaScroll` gallery paged by `NumberedPager`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package dialogs_mega
```

## Docker

### Run the published image (no build)

```bash
docker run --rm -e BOT_TOKEN=<your_bot_token> ghcr.io/desiders/telers/dialogs-mega:latest
```

### Build it yourself

```bash
docker build -f examples/dialogs_mega/Dockerfile -t dialogs-mega .
docker run --rm -e BOT_TOKEN=<your_bot_token> dialogs-mega
```
