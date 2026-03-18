#!/bin/bash
set -e

APP_NAME="Meshflow Vibe"
BUILD_DIR="build"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Validate we're at repository root (check for .git directory and Cargo.toml)
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "ERROR: Not at repository root (no .git directory found)"
    echo "Current directory: $(pwd)"
    echo "Expected root: $PROJECT_ROOT"
    exit 1
fi

if [ ! -f "$PROJECT_ROOT/Cargo.toml" ]; then
    echo "ERROR: Cargo.toml not found at $PROJECT_ROOT"
    exit 1
fi

# Extract and validate version from Cargo.toml with fail-fast semantics
VERSION=$(grep -m1 '^version = ' "$PROJECT_ROOT/Cargo.toml" | sed 's/version = "\([^"]*\)"/\1/')
if [ -z "$VERSION" ]; then
    echo "ERROR: Failed to extract version from Cargo.toml"
    echo "Expected format: version = \"X.Y.Z\""
    exit 1
fi

# Validate version format (basic semantic versioning check)
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+'; then
    echo "ERROR: Invalid version format: $VERSION"
    echo "Expected semantic versioning format: X.Y.Z"
    exit 1
fi
APP_BUNDLE="$BUILD_DIR/$APP_NAME.app"
DMG_NAME="meshflow-vibe-${VERSION}-macos.dmg"
DMG_PATH="$BUILD_DIR/$DMG_NAME"

echo "=== Building Meshflow Vibe Release ==="

# Change to repository root for all relative operations
cd "$PROJECT_ROOT"

# Clean build directory
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Build release binary
echo "Building release binary..."
cargo build --release --example cube_demo

# Find the built binary
BINARY_PATH=$(find target/release/examples -name "cube_demo" -type f | head -n 1)
if [ -z "$BINARY_PATH" ]; then
    echo "ERROR: Could not find cube_demo binary"
    exit 1
fi

echo "Found binary: $BINARY_PATH"

# Create .app bundle structure
echo "Creating .app bundle..."
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Create Info.plist
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>cube_demo</string>
    <key>CFBundleIdentifier</key>
    <string>com.meshflow.vibe</string>
    <key>CFBundleName</key>
    <string>Meshflow Vibe</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Copy binary to .app bundle
cp "$BINARY_PATH" "$APP_BUNDLE/Contents/MacOS/cube_demo"

# Make binary executable
chmod +x "$APP_BUNDLE/Contents/MacOS/cube_demo"

# Copy assets directory to .app bundle (Bevy reads from base_path + "assets", resolves relative to Contents/MacOS/)
echo "Copying assets to .app bundle..."
cp -r "$PROJECT_ROOT/assets" "$APP_BUNDLE/Contents/MacOS/"

# Create DMG using hdiutil (built-in macOS tool)
echo "Creating DMG: $DMG_PATH"
hdiutil create -volname "$APP_NAME" \
    -srcfolder "$APP_BUNDLE" \
    -format UDZO \
    -ov \
    -o "$DMG_PATH" \
    -size 500m

echo "=== Build Complete ==="
echo "DMG location: $DMG_PATH"
echo "Size: $(ls -lh "$DMG_PATH" | awk '{print $5}')"

# Display DMG info using ls (hdiutil info doesn't accept file paths as arguments)
echo "DMG details:"
ls -lh "$DMG_PATH" | awk '{print "  Size: " $5 "\n  Location: " $9}'
