#!/usr/bin/env bash

set -x

cargo test --all --verbose

cd fuzz
cargo build --all --verbose
cd ..
