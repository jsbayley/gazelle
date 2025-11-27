# Contributing

For those interested in helping to build <a href="https://github.com/jsbayley/gazelle" target="_blank">Gazelle</a>, please ⭐️ and 'watch' this repository so that you can track its progress in real-time.

We are always on the lookout for new contributors to help: 

- Propose design improvements,
- Develop and maintain the engine, 
- Enhance our testing and performance suite,
- Verify algorithmic correctness.

## Development Setup

### Prerequisites
- .NET 9 SDK or later
- F# support (included with .NET SDK)
- Git for version control

### Building from Source
```bash
git clone https://github.com/jsbayley/gazelle.git
cd gazelle
dotnet build
dotnet test
```

### CLI Development
```bash
# Install CLI tool locally
dotnet tool install --global --add-source ./cli/bin/Release Gazelle.CLI

# Test CLI commands
gz --help
```

## Release Process

### Version Updates
Use the automated version update script:
```bash
./scripts/update-version.sh 0.0.8
git add .
git commit -m "Update version to 0.0.8"
git tag v0.0.8
git push origin main --tags
```

This updates all version references across:
- `Directory.Build.props` (central version management)
- Documentation files
- Website files
- Issue templates

## macOS Binary Distribution

### Current Status
❌ **Not Implemented**: macOS binaries are unsigned  
⚠️ **User Workaround Required**: Manual quarantine removal needed

### Implementation Options

#### 1. Full Code Signing (Recommended)
**Requirements:**
- Apple Developer Account ($99/year)
- macOS build environment
- Developer ID certificate

**GitHub Actions Integration:**
```yaml
- name: Import Code Signing Certificate (macOS)
  if: matrix.runtime == 'osx-x64' || matrix.runtime == 'osx-arm64'
  uses: apple-actions/import-codesign-certs@v1
  with:
    p12-file-base64: ${{ secrets.MACOS_CERTIFICATE }}
    p12-password: ${{ secrets.MACOS_CERTIFICATE_PASSWORD }}

- name: Code Sign & Notarize (macOS)  
  if: matrix.runtime == 'osx-x64' || matrix.runtime == 'osx-arm64'
  run: |
    codesign --deep --force --verify --verbose --sign "Developer ID Application" artifacts/gz.${{ matrix.runtime }}
    xcrun notarytool submit artifacts/gz.${{ matrix.runtime }} --wait
    xcrun stapler staple artifacts/gz.${{ matrix.runtime }}
```

**Required Secrets:**
- `MACOS_CERTIFICATE`: Base64-encoded certificate
- `MACOS_CERTIFICATE_PASSWORD`: Certificate password  
- `APPLE_ID`: Apple ID for notarization
- `APPLE_ID_PASSWORD`: App-specific password
- `TEAM_ID`: Developer Team ID

#### 2. Alternative Solutions
- **Homebrew Package**: Free distribution through package manager
- **Self-Signed Certificate**: Free but still requires user trust
- **Enhanced Documentation**: Current approach with clear user guidance

### Implementation Priority
1. ✅ **Current**: CI executable permissions + user documentation
2. 🎯 **Future**: Full code signing pipeline for seamless user experience

## Website Development

The Gazelle landing page (`.github/pages/`) is a single-page static website for hosting CLI downloads.

### Structure
```
.github/pages/
├── index.html              # Main landing page
├── styles.css              # Responsive design and styling
└── script.js               # Download functionality
```

### Local Development
```bash
# Serve locally for testing
cd .github/pages && python -m http.server 8000
# or
cd .github/pages && npx serve
```

### Deployment
The website is automatically deployed via GitHub Actions:
- **Trigger**: On pushes to main branch
- **Process**: CI builds binaries → uploads to GitHub Pages
- **URL**: Hosted at https://gazelle.sh

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>