#!/bin/sh
set -ex

# run cargo
cargo +nightly build --target wasm32-unknown-unknown

# run wasm bindgen
mkdir -p wasm_bindgen
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_bindgen_hello.wasm \
  --out-dir wasm_bindgen

npm install
