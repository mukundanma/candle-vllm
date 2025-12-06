#!/bin/bash
set -e

# Install dependencies
brew install pkg-config openssl

# Build the project
cargo build --release --features metal
