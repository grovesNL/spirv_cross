#!/bin/bash
export CXX=g++-5

cargo build --verbose --all
cargo test --verbose --all
