host := `uname -a`

# Show help message
help:
    just -l

# Run the tests with cargo
test:
    cargo test --lib --tests --all --all-features -- --nocapture

# Run the clippy linter
clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

# Run the cargo format tool
fmt:
    cargo +nightly fmt --all

# Run the cargo build tool
build:
    cargo build --all --all-features

# Generate Telegram API bindings into the telers crate
codegen:
    cargo run -p telers-codegen -- --schema telers-codegen/schemas/api.json --gen-dir telers --types-path telers
    just fmt
