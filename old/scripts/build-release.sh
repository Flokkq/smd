#!/bin/bash

# Script to compile Rust project for multiple platforms

# Stop on any error
set -e

# Compile for macOS Intel
echo "Compiling for macOS Intel (x86_64-apple-darwin)..."
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Compile for macOS ARM
echo "Compiling for macOS ARM (aarch64-apple-darwin)..."
rustup target add aarch64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Compile for Linux
echo "Compiling for Linux (x86_64-unknown-linux-gnu)..."
# Note: Assuming the default target is for Linux. Adjust if needed.
cargo build --release --target x86_64-unknown-linux-gnu

# Compile for Windows
echo "Compiling for Windows (x86_64-pc-windows-msvc)..."
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc

echo "Compilation completed!"