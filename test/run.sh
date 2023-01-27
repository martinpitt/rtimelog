#!/bin/sh
set -eux

# Run tests in debug mode
cargo test

# Build release mode
cargo rustc --release --lib -- -Dwarnings
cargo rustc --release --bin rtimelog -- -Dwarnings

# Run tests in release mode
cargo test --release

# static checks
cargo fmt --all --check
cargo clippy --all-features -- -Dwarnings
