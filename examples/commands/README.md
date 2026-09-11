# commands

Demonstrates typed commands with the `Command` derive macro: a command and its arguments are parsed straight into an enum that handlers take as an argument, the command list is published with `setMyCommands`, and one command parses its arguments with a custom type.

## What it does

- On startup, registers `help`, `username`, `username_and_age` and `settings` with `SetMyCommands`, so they show up in the Telegram command menu.
- `/help` replies with the commands and their descriptions.
- `/username <name>` and `/username_and_age <name> <age>` reply with the parsed values, e.g. `/username_and_age alice 25`; `<age>` must fit in a `u8`.
- `/settings key=value, key=value` replies with the settings one per line, e.g. `/settings lang=en, notifications=off`.
- A command with wrong arguments gets the reason: `/username` and `/settings` answer "Missing argument for the field at position 0", `/username_and_age alice abc` answers "Invalid value `abc` for the field at position 1" and `/settings foo` answers "Expected `key=value`, got `foo`".

## How it works

- `Commands` derives `Command` with `#[command(rename_rule = "snake_case")]`, so the variant `UsernameAndAge` becomes the command `username_and_age`. Every variant has a `description`, which feeds the generated `Commands::descriptions()` (the `/help` text) and `Commands::bot_commands()` (the list sent with `SetMyCommands`).
- The macro implements `Extractor` for `Commands`: it reads the `CommandObject` that the `Command` filter puts into the context, matches the command name case-insensitively and parses the arguments into the fields of the variant through the `CommandArg` trait, one argument per field in declaration order. A missing or extra argument or a parse failure is an extraction error, so the handler is not called and the error goes to the error observer.
- `Settings` is a custom field type: it implements `CommandArg` itself and takes all the arguments from the `ArgsCursor`, splitting each into `key=value`; without arguments it reports `CommandArgsError::Missing`. Its variant sets `split = ','`, so the arguments are separated by commas instead of whitespace, and whitespace around them is ignored. An argument without `=` is reported with `CommandArgsError::from_display`.
- The handlers are registered on the message observer behind the `Command` filter with the kinds of the commands instead of texts: the macro generates `CommandsType`, the kind of a command without its arguments, and the filter matches a kind by the name and the prefix of its declaration (aliases included). `CommandFilter::one(CommandsType::Help)` passes only `/help` to `help_handler`, `CommandFilter::many([...])` passes two commands to `username_handler`, and `CommandFilter::all::<Commands>()` would pass all of them. The filter puts the `CommandObject` into the context for the extractor. Texts and regexes still work in the same filter, for example `CommandFilter::one("help")`.
- The extraction error keeps the `CommandArgsError` as its source, so `on_command_args_error` is registered on the error observer with `ErrorType::<CommandArgsError>::new()` and takes it as `EventError<CommandArgsError>`. It replies with the text of the error, so only the argument errors reach the chat. Other errors are not handled by the example and only get logged.
- The `Dispatcher` is built with `allowed_update(UpdateType::Message)` and run via `run_polling`.

## Running

```bash
BOT_TOKEN=<your_bot_token> cargo run --package commands
```

Set the optional `RUST_LOG` variable to control log output (e.g. `RUST_LOG=info`).
