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

Or manually using .NET publish:

```bash
# Windows
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r win-x64 --self-contained -o web/releases/win-x64
cp web/releases/win-x64/gz.exe web/releases/gazelle-windows-x64.exe

# macOS x64
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r osx-x64 --self-contained -o web/releases/osx-x64
cp web/releases/osx-x64/gz web/releases/gazelle-macos-x64

# macOS ARM64
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r osx-arm64 --self-contained -o web/releases/osx-arm64
cp web/releases/osx-arm64/gz web/releases/gazelle-macos-arm64

# Linux
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r linux-x64 --self-contained -o web/releases/linux-x64
cp web/releases/linux-x64/gz web/releases/gazelle-linux-x64
```

## Checksums

Generate checksums for verification:

```bash
cd docs/releases
sha256sum gazelle-* > checksums.txt
```

## File Sizes (Approximate)

- Windows: ~8.2 MB
- macOS: ~7.8 MB  
- Linux: ~7.5 MB

Sizes may vary depending on .NET version and publish optimizations.