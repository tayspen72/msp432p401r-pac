#!/usr/bin/env bash
set -x
set -e

# NOTE: Last executed using Rust 1.40.0

cargo install svd2rust
cargo install form
rustup component add rustfmt

rm -rf src
mkdir src
~/.cargo/bin/svd2rust -i ./MSP432P401R.svd
~/.cargo/bin/form -i lib.rs -o src
rm lib.rs
cargo fmt
rustfmt build.rs
