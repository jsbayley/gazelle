# Gazelle Releases

This directory contains the compiled binaries for different platforms.

## File Structure

```
releases/
├── gazelle-windows-x64.exe    # Windows 64-bit executable
├── gazelle-macos-universal     # macOS Universal Binary (Intel + Apple Silicon)
├── gazelle-linux-x64           # Linux 64-bit executable
├── checksums.txt               # SHA256 checksums for verification
└── README.md                   # This file
```

## Building Releases

To build releases for all platforms, run:

```bash
# From project root
./scripts/build-releases.sh
```

Or manually:

```bash
# Windows
GOOS=windows GOARCH=amd64 go build -ldflags "-s -w" -o web/releases/gazelle-windows-x64.exe ./pkg/main.go

# macOS Universal (requires macOS with Go 1.17+)
GOOS=darwin GOARCH=amd64 go build -ldflags "-s -w" -o web/releases/gazelle-macos-amd64 ./pkg/main.go
GOOS=darwin GOARCH=arm64 go build -ldflags "-s -w" -o web/releases/gazelle-macos-arm64 ./pkg/main.go
lipo -create -output web/releases/gazelle-macos-universal web/releases/gazelle-macos-amd64 web/releases/gazelle-macos-arm64

# Linux
GOOS=linux GOARCH=amd64 go build -ldflags "-s -w" -o web/releases/gazelle-linux-x64 ./pkg/main.go
```

## Checksums

Generate checksums for verification:

```bash
cd web/releases
sha256sum gazelle-* > checksums.txt
```

## File Sizes (Approximate)

- Windows: ~8.2 MB
- macOS: ~7.8 MB  
- Linux: ~7.5 MB

Sizes may vary depending on Go version and build optimizations.