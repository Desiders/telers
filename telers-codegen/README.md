# Telegram Bot API code generation

Generate the API models into `telers-types`, methods into `telers-methods`, and
framework integration into `telers`:

```sh
cargo run -p telers-codegen -- \
  --schema telers-codegen/schemas/api.json \
  --gen-dir telers --types-gen-dir telers-types --methods-gen-dir telers-methods \
  --tests --types-path telers_types
```

The types destination receives the API types, discriminator enums and serialization
tests. The methods destination receives API method structs, request builders and
their module exports. These builders prepare requests without a `Bot` or HTTP
client. The framework destination receives type extractors, shortcuts, the observer
enum and smart filters.

Add `--check` to check generated files without changing them. Omitting
`--types-gen-dir` and `--methods-gen-dir` retains the single-crate generation layout
for other consumers. Each destination can also be specified independently.
