#!/bin/bash
set -e

echo "Starting iOS build process for touchHLE..."

# Ensure we are in the touchHLE_src directory
cd "$(dirname "$0")/../touchHLE_src"

# Add the iOS target if not already present
rustup target add aarch64-apple-ios

# Install cargo-lipo for building universal iOS binaries (optional but helpful)
# cargo install cargo-lipo

# Set up CMake toolchain for C++ dependencies (dynarmic)
export CMAKE_TOOLCHAIN_FILE="../ios/ios_toolchain.cmake"
export CC=clang
export CXX=clang++

echo "Building Rust library for aarch64-apple-ios..."
cargo build --target aarch64-apple-ios --release

echo "Build complete. The static library is located at:"
echo "touchHLE_src/target/aarch64-apple-ios/release/libtouchHLE.a"
echo "You can now link this in the Xcode project."
