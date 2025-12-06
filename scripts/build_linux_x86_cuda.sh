#!/bin/bash
set -e

# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev clang

# Set Rust flags
export RUSTFLAGS="-L/usr/local/cuda/lib64 -L/usr/local/cuda/lib64/stubs"

# Build the project
cargo build --release --features cuda
