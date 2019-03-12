#!/bin/bash
export CXX=g++-5

cargo build --verbose --all --all-features
cargo test --verbose --all --all-features
