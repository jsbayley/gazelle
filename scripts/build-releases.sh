#!/bin/bash

# Build script for creating Gazelle releases
set -e

echo "Building Gazelle releases..."

# Create releases directory if it doesn't exist
mkdir -p web/releases

# Build flags for smaller binaries
BUILD_FLAGS="-trimpath -ldflags='-s -w'"

# Get version from git or use default
VERSION=${1:-$(git describe --tags --always --dirty 2>/dev/null || echo "v1.0.0")}

echo "Building version: $VERSION"

# Windows x64
echo "Building Windows x64..."
GOOS=windows GOARCH=amd64 go build -trimpath -ldflags="-s -w" -o web/releases/gazelle-windows-x64.exe ./pkg/main.go

# macOS Binaries (Intel and Apple Silicon)
echo "Building macOS Intel x64..."
GOOS=darwin GOARCH=amd64 go build -trimpath -ldflags="-s -w" -o web/releases/gazelle-macos-intel ./pkg/main.go

echo "Building macOS Apple Silicon (ARM64)..."
GOOS=darwin GOARCH=arm64 go build -trimpath -ldflags="-s -w" -o web/releases/gazelle-macos-arm64 ./pkg/main.go

# Create Universal Binary if lipo is available (on macOS)
if command -v lipo >/dev/null 2>&1; then
    echo "Creating macOS Universal Binary..."
    lipo -create -output web/releases/gazelle-macos-universal web/releases/gazelle-macos-intel web/releases/gazelle-macos-arm64
    echo "✓ Universal Binary created (Intel + Apple Silicon)"
else
    echo "ℹ️  lipo not available - keeping separate Intel and ARM64 binaries"
    echo "ℹ️  Users can download the appropriate binary for their Mac"
fi

# Linux x64
echo "Building Linux x64..."
GOOS=linux GOARCH=amd64 go build -trimpath -ldflags="-s -w" -o web/releases/gazelle-linux-x64 ./pkg/main.go

# Generate checksums
echo "Generating checksums..."
cd web/releases
sha256sum gazelle-* > checksums.txt

echo "Build complete! Files created:"
ls -la gazelle-*
echo ""
echo "Checksums:"
cat checksums.txt

cd ../..
echo ""
echo "✅ Release build complete for version $VERSION"
echo "📦 Files available in web/releases/"