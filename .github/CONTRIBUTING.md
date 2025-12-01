# Contributing

For those interested in helping to build <a href="https://github.com/jsbayley/gazelle" target="_blank">Gazelle</a>, please ⭐️ and 'watch' this repository so that you can track its progress in real-time.

We are always on the lookout for new contributors to help: 

- Propose design improvements,
- Develop and maintain the engine, 
- Enhance our testing and performance suite,
- Verify algorithmic correctness.

## Development Setup
- .NET 9 SDK
- Git
- Optional: Use the preconfigured Dev Container.

```bash
git clone https://github.com/jsbayley/gazelle.git
cd gazelle
dotnet build
dotnet test
```

### Running the CLI from source
```bash
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r linux-arm64 --self-contained -p:PublishSingleFile=true -o ./artifacts/
./artifacts/gz --help
```
Adapt `-r` to: win-x64, osx-x64, osx-arm64, linux-x64, linux-arm64.

## Website
Static site lives in `.github/pages/`. CI copies build artifacts into `.github/pages/releases/` and deploys via official GitHub Pages actions.

Local preview:
```bash
cd .github/pages && python -m http.server 8000
# or
cd .github/pages && npx serve
```

## Release Process
Use the version script; latest-only security support.
```bash
./scripts/update-version.sh 0.0.8
git add -A
git commit -m "chore: bump version to 0.0.8"
git tag v0.0.8
git push origin main --tags
```

## macOS Distribution
Binaries are unsigned today.
- Users may need: `chmod +x` and `xattr -rd com.apple.quarantine <file>`
- Future: code signing and notarization via GitHub Actions (see comments in workflow for sample steps)

## Security
We accept private reports via GitHub Security Advisories. Policy: `.github/SECURITY.md`.

## Code Style
- F#: idiomatic, pure where possible, explicit types in public APIs
- Keep CLI outputs deterministic and machine‑parsable
- Small, composable commands; clear errors and exit codes

## CI notes
- Artifacts uploaded with actions/upload-artifact@v4
- Permissions restored in publish job for macOS/Linux binaries
- Pages deployed via: configure-pages → upload-pages-artifact → deploy-pages

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>