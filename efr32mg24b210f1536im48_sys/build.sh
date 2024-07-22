#!/usr/bin/env sh

svd2rust -i EFR32MG24B210F1536IM48.svd
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt