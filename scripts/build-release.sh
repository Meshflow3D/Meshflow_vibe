#!/bin/bash
set -e

APP_NAME="Meshflow Vibe"
VERSION="0.3.1"
BUILD_DIR="build"
APP_BUNDLE="$BUILD_DIR/$APP_NAME.app"
DMG_NAME="meshflow-vibe-$VERSION-macos.dmg"
DMG_PATH="$BUILD_DIR/$DMG_NAME"

echo "=== Building Meshflow Vibe Release ==="

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
cat > "$APP_BUNDLE/Contents/Info.plist" << 'EOF'
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
    <string>0.3.1</string>
    <key>CFBundleVersion</key>
    <string>0.3.1</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Copy binary to .app bundle
cp "$BINARY_PATH" "$APP_BUNDLE/Contents/MacOS/cube_demo"

# Make binary executable
chmod +x "$APP_BUNDLE/Contents/MacOS/cube_demo"

# Create DMG using hdiutil (built-in macOS tool)
echo "Creating DMG: $DMG_PATH"
hdiutil create -volname "$APP_NAME" \
    -srcfolder "$APP_BUNDLE" \
    -format UDZO \
    -o "$DMG_PATH" \
    -size 500m

echo "=== Build Complete ==="
echo "DMG location: $DMG_PATH"
echo "Size: $(ls -lh "$DMG_PATH" | awk '{print $5}')"

# Display DMG info
hdiutil info "$DMG_PATH" | grep -E "(Volume|Size|Format)"
