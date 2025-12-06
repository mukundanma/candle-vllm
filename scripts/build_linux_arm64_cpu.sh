#!/bin/bash
set -e

# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev gcc-aarch64-linux-gnu

# Build the project
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
