#!/usr/bin/env bash

set -x

cargo build --no-default-features
cargo build --no-default-features --feature "color"
cargo test --all --verbose

cd fuzz
cargo build --all --verbose
cd ..
