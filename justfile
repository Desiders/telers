host := `uname -a`

# Show help message
help:
    just -l

# Run the tests with cargo
test:
    cargo test --lib --tests --all --all-features --verbose -- --nocapture

# Run the clippy linter
clippy:
    cargo clippy --all --all-features -- -W clippy::pedantic

# Run the cargo format tool
fmt:
    cargo fmt --all -- --check

# Run the cargo build tool
build:
    cargo build --all --all-features
