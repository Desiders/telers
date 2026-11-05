host := `uname -a`

# Show help message
help:
    just -l

# Run the tests with cargo
test:
    cargo test --lib --tests --all --all-features -- --nocapture

# Run the Criterion benchmarks (opt-in; not part of the normal test/build flow)
bench:
    cargo bench -p telers

# Run the clippy linter
clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

# Run the cargo format tool
fmt:
    cargo +nightly fmt --all

# Run the cargo build tool
build:
    cargo build --all --all-features

# Generate Telegram API bindings and their framework integration
codegen:
    cargo run -p telers-codegen -- --schema telers-codegen/schemas/api.json --gen-dir telers --types-gen-dir telers-types --common-gen-dir telers-types-common --input-gen-dir telers-types-input --messages-gen-dir telers-types-messages --methods-gen-dir telers-methods --tests --types-path telers_types
    cargo +nightly fmt -p telers -p telers-types -p telers-types-common -p telers-types-input -p telers-types-messages -p telers-methods
