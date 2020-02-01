#!/bin/bash
export CXX=clang++

rustup target add wasm32-unknown-unknown
(cd examples && cargo build --target=wasm32-unknown-unknown --verbose --bin glsl)
