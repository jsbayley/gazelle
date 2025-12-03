# Documentation

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Download Binaries](#download-binaries)
  - [Build from source](#build-from-source)
  - [Install from NuGet](#install-from-nuget)

## Quick Start

1. Visit <a href="https://gazelle.sh">Gazelle.sh</a> and <a href="https://gazelle.sh#download">download</a> one of the following platform-specific binaries:
  - gz.win-x64.exe
  - gz.osx-x64
  - gz.osx-arm64
  - gz.linux-x64
  - gz.linux-arm64

2. For MacOS and Linux Users, enable binary execution permissions:

```bash
# Linux (x64)
chmod +x gz.linux-x64
```

```bash
# MacOS (ARM64)
chmod +x gz.osx-arm64
```

3. For security reasons to ensure file integrity and no tampering has occurred, verify the checksum:

```bash
# Linux
sha256sum -c gz.linux-x64.sha256
```

```bash
# MacOS
shasum -a 256 -c gz.osx-arm64.sha256
```

```powershell
# Windows (PowerShell)
Get-FileHash .\gz.win-x64.exe -Algorithm SHA256
```

4. Fire up the help docs:

```bash
# Linux (x64)
./gz.linux-x64 --help
```

```bash
# MacOS (ARM64)
./gz.osx-arm64 --help
```

```powershell
# Windows (PowerShell)
.\gz.win-x64.exe --help
```

## Installation

### Prerequisites
- .NET 9 SDK or later
- F# support (included with .NET SDK)

### Download Binaries

Pre-built binaries are available for Windows, macOS, and Linux at [Gazelle.sh](https://gazelle.sh#download).

#### macOS Security Notes

macOS requires additional steps for downloaded binaries due to Gatekeeper security:

1. **Make executable and remove quarantine**:
   ```bash
   chmod +x gz.osx-x64      # or gz.osx-arm64
   sudo xattr -rd com.apple.quarantine gz.osx-x64
   ```

2. **Alternative method** (if the above doesn't work):
   ```bash
   # Remove all extended attributes
   sudo xattr -c gz.osx-x64
   chmod +x gz.osx-x64
   ```

3. **If you still get security warnings**:
   - Go to **System Preferences** → **Security & Privacy** → **General**
   - Click **"Allow Anyway"** next to the blocked application message
   - Or use: `sudo spctl --add gz.osx-x64` to whitelist the binary

> **Note**: These steps are required because the macOS binaries are not currently code-signed or notarized. This is a common requirement for distributing macOS applications.

### Build from source
```bash
git clone https://github.com/jsbayley/gazelle.git
cd gazelle
dotnet build

# Install CLI tool globally
dotnet tool install --global --add-source ./cli/bin/Release Gazelle.CLI
```

### Install from NuGet
```bash
dotnet add package Gazelle --version 0.0.8
```

> **ℹ️ Cross-Platform Note**: The CLI tool (`gz`) works on all platforms. ETABS integration features are Windows-only due to COM interop requirements.

## CLI Usage

The `gz` command provides a cross-platform interface for creating, validating, and analyzing structural models.

### Cross-Platform Support

| Platform | Core Features | ETABS Integration | Status |
|----------|---------------|-------------------|---------|
| **Windows** | ✅ Full | ✅ V17/V19 | ✅ Complete |
| **macOS** | ✅ Full | ❌ Graceful error | ✅ Complete |
| **Linux** | ✅ Full | ❌ Graceful error | ✅ Complete |

### Available Commands

```bash
# Core functionality (all platforms)
gz create [file]     # Create new structural models
gz info [file]       # Display model information
gz validate [file]   # Check model integrity
gz analyze [file]    # Perform structural analysis
gz templates list    # List available templates

# Windows-specific ETABS integration
gz etabs demo        # ETABS interop demonstration
gz etabs connect     # Connect to existing ETABS instance
gz etabs units       # Units of measure examples
```

---

<div align="center">
   <p><strong>Built with ❤️ for the global engineering community</strong></p>
   <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>

---
