#!/bin/bash
export CXX=clang++

cargo build --verbose --all
cargo test --verbose --all
