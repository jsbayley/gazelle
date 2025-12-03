# CLI Commands

Status: pre-1.0. Commands and flags may evolve.

## Installation
- Download platform binary from the website (see `README.md`).
- macOS/Linux: `chmod +x ./gz.*` if needed; remove quarantine on macOS.
- Optionally place the binary on your `PATH`.

## Supported Platforms
- Windows x64: `gz.win-x64.exe`
- macOS Intel: `gz.osx-x64`
- macOS Apple Silicon: `gz.osx-arm64`
- Linux x86_64: `gz.linux-x64`
- Linux ARM64: `gz.linux-arm64`

## Usage
```bash
gz --help
gz <command> [options]
```

## Global Flags (planned)
- `--format json|text` output format
- `--verbose` extra diagnostics
- `--no-color` disable ANSI colors

## Commands
- `geometry`: geometry computations and transforms
  - `geometry area --input <file>`: compute polygon/section area
  - `geometry centroid --input <file>`: compute centroid for shapes
  - `geometry transform --input <file> --matrix <file>`: apply a transform
- `etabs` (Windows-only): ETABS integration
  - `etabs import --file <model>`: import ETABS model
  - `etabs export --file <model> --out <file>`: export to JSON
- `convert`: format conversions (roadmap)
  - `convert --from <fmt> --to <fmt> --input <file> --out <file>`

## Examples
```bash
# Show help
gz --help

# Compute polygon area (JSON input)
gz geometry area --input polygon.json --format json

# ETABS interop (Windows-only)
gz etabs import --file model.etabs
gz etabs export --file model.etabs --out model.json

# Apply transform using a matrix
gz geometry transform --input shape.json --matrix matrix.json --format json
```

## Exit Codes
- `0`: success
- `>0`: error (command-specific codes)

## Troubleshooting
- macOS: use `osx-arm64` on Apple Silicon to avoid Rosetta/ELF errors.
- Linux: ensure execute permission via `chmod +x`.
- Windows: SmartScreen may require "Run anyway".
- Verify artifact integrity with `.sha256` files.

---

<div align="center">
   <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
