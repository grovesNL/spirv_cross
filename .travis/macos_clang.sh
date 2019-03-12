#!/bin/bash
export CXX=clang++
export MACOSX_DEPLOYMENT_TARGET=10.7

cargo build --verbose --all --all-features
cargo test --verbose --all --all-features
