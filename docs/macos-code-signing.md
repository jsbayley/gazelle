# macOS Code Signing Guide

This document outlines how to properly code sign and notarize Gazelle binaries for macOS distribution.

## Current Status

❌ **Not Implemented**: Binaries are currently unsigned  
⚠️ **User Workaround Required**: Manual quarantine removal and executable permission setting

## Required Setup (Future Implementation)

### Prerequisites
- Apple Developer Account ($99/year)
- macOS build environment  
- Xcode Command Line Tools

### 1. Code Signing Certificate

```bash
# Create certificate signing request
security create-keypair -b 2048 -t RSA -f OPENSSH2 -s DeveloperIDApplication

# Download certificate from Apple Developer portal
# Install certificate in macOS Keychain
```

### 2. GitHub Actions Integration

Add secrets to GitHub repository:
- `MACOS_CERTIFICATE`: Base64-encoded Developer ID certificate
- `MACOS_CERTIFICATE_PASSWORD`: Certificate password  
- `APPLE_ID`: Apple ID for notarization
- `APPLE_ID_PASSWORD`: App-specific password
- `TEAM_ID`: Developer Team ID

### 3. Updated Workflow

```yaml
- name: Import Code Signing Certificate (macOS)
  if: matrix.runtime == 'osx-x64' || matrix.runtime == 'osx-arm64'
  uses: apple-actions/import-codesign-certs@v1
  with:
    p12-file-base64: ${{ secrets.MACOS_CERTIFICATE }}
    p12-password: ${{ secrets.MACOS_CERTIFICATE_PASSWORD }}

- name: Code Sign (macOS)  
  if: matrix.runtime == 'osx-x64' || matrix.runtime == 'osx-arm64'
  run: |
    codesign --deep --force --verify --verbose --sign "Developer ID Application" artifacts/gz.${{ matrix.runtime }}
    
- name: Notarize (macOS)
  if: matrix.runtime == 'osx-x64' || matrix.runtime == 'osx-arm64' 
  run: |
    # Create zip for notarization
    zip -r gz.${{ matrix.runtime }}.zip artifacts/gz.${{ matrix.runtime }}
    
    # Submit for notarization
    xcrun notarytool submit gz.${{ matrix.runtime }}.zip \
      --apple-id "${{ secrets.APPLE_ID }}" \
      --password "${{ secrets.APPLE_ID_PASSWORD }}" \
      --team-id "${{ secrets.TEAM_ID }}" \
      --wait
      
    # Staple notarization  
    xcrun stapler staple artifacts/gz.${{ matrix.runtime }}
```

## Alternative Solutions

### 1. Self-Signing (Free)
- Create self-signed certificate
- Requires users to manually trust certificate
- Still requires quarantine removal

### 2. Homebrew Distribution  
- Package for Homebrew installation
- Homebrew handles security for popular packages
- Wider distribution through package manager

### 3. User Documentation (Current)
- Provide clear installation instructions
- Document security steps required
- Include troubleshooting guide

## Implementation Priority

1. ✅ **Current**: User documentation and workflow improvements
2. 🔄 **Next**: Enhanced executable permissions in CI  
3. 🎯 **Future**: Full code signing and notarization pipeline

## Cost Analysis

| Solution | Cost | Setup Time | User Friction |
|----------|------|------------|---------------|
| Documentation | Free | 1 hour | Medium |
| Self-signing | Free | 4 hours | Medium |  
| Official signing | $99/year | 8 hours | None |
| Homebrew | Free | 16 hours | None |

## References

- [Apple Code Signing Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [GitHub Actions macOS Signing](https://docs.github.com/en/actions/deployment/deploying-xcode-applications/installing-an-apple-certificate-on-macos-runners-for-xcode-development)
- [Notarization Process](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow)