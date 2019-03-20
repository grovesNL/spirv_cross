#!/bin/bash
export CXX=clang++

cargo build --verbose --all --all-features
cargo test --verbose --all --all-features
