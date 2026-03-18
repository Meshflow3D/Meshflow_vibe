#!/bin/bash
set -e

echo "Building Meshflow Vibe dungeon example in release mode..."

# Detect macOS ARM
if [[ "$OSTYPE" == "darwin"* ]] && [[ "$(uname -m)" == "arm64" ]]; then
    echo "Detected macOS ARM architecture"
    cargo build --release --example dungeon
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS Intel architecture"
    cargo build --release --example dungeon
else
    echo "Detected non-macOS platform, building with default settings"
    cargo build --release --example dungeon
fi

echo "Build complete! Binary location:"
find target/release/examples -name "dungeon" -type f -exec ls -lh {} \;
