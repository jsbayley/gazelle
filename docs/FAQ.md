# FAQ

- [Which platforms are supported?](#which-platforms-are-supported)
- [How do I choose the right download?](#how-do-i-choose-the-right-download)
- [macOS: "file is not executable" or Gatekeeper warnings](#macos-file-is-not-executable-or-gatekeeper-warnings)
- [Windows: SmartScreen blocked my download](#windows-smartscreen-blocked-my-download)
- [Linux: "Permission denied"](#linux-permission-denied)
- [How do I verify checksums?](#how-do-i-verify-checksums)
- [Why does the website show a different size than my download?](#why-does-the-website-show-a-different-size-than-my-download)
- [How do I run the CLI?](#how-do-i-run-the-cli)
- [Can I build it locally from source?](#can-i-build-it-locally-from-source)
- [The site shows an older version. Is that expected?](#the-site-shows-an-older-version-is-that-expected)
- [What is supported for security fixes?](#what-is-supported-for-security-fixes)
- [Where are the docs and support?](#where-are-the-docs-and-support)
- [Could Gazelle leverage the Speckle database?](#could-gazelle-leverage-the-speckle-database)
- [Is Gazelle a Web App or a downloadable tool?](#is-gazelle-a-web-app-or-a-downloadable-tool)
- [It’s written in F#. Is that similar to Python? Could I still use AI tools?](#its-written-in-f-is-that-similar-to-python-could-i-still-use-ai-tools)
- [Do calculations run locally, or can this scale to the cloud?](#do-calculations-run-locally-or-can-this-scale-to-the-cloud)
- [What’s the first simple case you plan to test?](#whats-the-first-simple-case-you-plan-to-test)

---

## Which platforms are supported?

We publish single-file, self-contained binaries for:

- Windows: `win-x64` (`gz.win-x64.exe`)
- macOS (Intel): `osx-x64` (`gz.osx-x64`)
- macOS (Apple Silicon): `osx-arm64` (`gz.osx-arm64`)
- Linux (x86_64): `linux-x64` (`gz.linux-x64`)
- Linux (ARM64): `linux-arm64` (`gz.linux-arm64`)


## How do I choose the right download?

- Apple Silicon Macs (M1/M2/M3): use `osx-arm64`.
- Intel Macs: use `osx-x64`.
- Most modern Linux servers/desktops: use `linux-x64`.
- Raspberry Pi 4/5 or ARM servers: use `linux-arm64`.
- Windows 10/11 on x64 PCs: use `win-x64`.

If automatic detection on the website picks the wrong one, select the correct card manually from the downloads grid.


## macOS: "file is not executable" or Gatekeeper warnings

On first run, you may need to set the executable bit and remove quarantine attributes:

```bash
chmod +x ./gz.osx-*
xattr -d com.apple.quarantine ./gz.osx-*
```

If you still see "can’t be opened because it is from an unidentified developer", open System Settings → Privacy & Security and choose "Open Anyway" for the last blocked app.

If you encounter Rosetta/ELF errors on Apple Silicon, you likely downloaded the Intel build. Use `osx-arm64` instead.


## Windows: SmartScreen blocked my download

Right-click the file → Properties → check "Unblock" → OK. Or when prompted by SmartScreen, choose "More info" → "Run anyway".


## Linux: "Permission denied"

Ensure the file is executable, then run it directly:

```bash
chmod +x ./gz.linux-*
./gz.linux-x64   # or ./gz.linux-arm64
```


## How do I verify checksums?

Each binary has a matching `.sha256` file in the releases folder.

- Linux:
  ```bash
  sha256sum -c gz.linux-x64.sha256   # expects "OK"
  ```
- macOS:
  ```bash
  shasum -a 256 gz.osx-arm64 | awk '{print tolower($1)}' && cat gz.osx-arm64.sha256
  # Compare the two hashes visually (shasum -c is not available by default)
  ```
- Windows (PowerShell):
  ```powershell
  Get-FileHash .\gz.win-x64.exe -Algorithm SHA256
  Get-Content .\gz.win-x64.exe.sha256
  ```


## Why does the website show a different size than my download?

File sizes on the site are fetched via HTTP HEAD requests and may differ slightly from what your OS shows due to transfer encoding, caching, or compression estimates. The checksum always reflects the exact artifact bytes.


## How do I run the CLI?

After downloading the right binary for your platform, run:

```bash
./gz.osx-arm64 --help    # macOS (Apple Silicon)
./gz.osx-x64 --help      # macOS (Intel)
./gz.linux-x64 --help    # Linux x86_64
./gz.linux-arm64 --help  # Linux ARM64
gz.win-x64.exe --help    # Windows
```

## Can I build it locally from source?

Yes. With .NET 9 installed, publish a single-file, self-contained binary for your RID:

```bash
dotnet publish cli/Gazelle.CLI.fsproj \
  -c Release \
  -r linux-arm64 \
  --self-contained true \
  -p:PublishSingleFile=true \
  -p:PublishTrimmed=true \
  -p:TrimMode=link \
  -p:AssemblyName=gz.linux-arm64 \
  -o artifacts/
```

Replace `linux-arm64` and `gz.linux-arm64` with your target RID and filename (e.g., `osx-arm64`, `win-x64`).

## The site shows an older version. Is that expected?

We only support the latest release. If you’re developing the site locally and see a stale version, ensure you’ve run our version update script and deployed. End users on the public site should always see the latest after a successful deployment.

## What is supported for security fixes?

Per our policy in [.github/SECURITY.md](../.github/SECURITY.md), we support security fixes for the latest release only. Please report vulnerabilities privately via GitHub Security Advisories as described there.

## Where are the docs and support?

- Quick start and usage: [README.md](./README.md).
- Contribution and release process: [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md).
- Security policy: [.github/SECURITY.md](../.github/SECURITY.md).
- Open a discussion or issue: GitHub [Issues](https://github.com/jsbayley/gazelle/issues)/[Discussions](https://github.com/jsbayley/gazelle/discussions) on the repository.

## Could Gazelle leverage the Speckle database?

Yes, it could. But this capability has not yet been implemented. In theory, Gazelle could integrate bi-directionally with Speckle: 

- Either pulling model data from Speckle itself, or 
- Running computations as a Speckle Automate task.

## Is Gazelle a Web App or a downloadable tool?

Gazelle is currently a downloadable binary that runs as a CLI tool. It acts as middleware: a fast engine that other applications or interfaces can be built on top of.

## It’s written in F#. Is that similar to Python? Could I still use AI tools?

F# is closer to Python than people expect: it’s concise, indentation-oriented, and works well with Copilot. Importantly, you don’t need to write F# to use Gazelle: the CLI is language-agnostic. Commands output plain text or structured data that any language can consume or pipe into.

## Do calculations run locally, or can this scale to the cloud?

Today, all computation runs locally for privacy and speed. A daemon-style entrypoint would allow distributed or cloud-cluster execution and this is an item already planned on the roadmap.

## What’s the first simple case you plan to test?

A focussed structural element design. Likely something like Pile Caps or Reinforced Concrete Columns to prove the concept before expanding the library.
 
---

<div align="center">
   <p><strong>Built with ❤️ for the global engineering community</strong></p>
   <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
