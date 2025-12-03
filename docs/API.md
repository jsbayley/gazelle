# API Reference

Status: pre-1.0, evolving. Surface may change; stability improves toward 1.0.

## Versioning
- Tracks Gazelle release versions (see `README.md` and `.github/SECURITY.md`).
- Latest release is supported for security fixes; older versions may be deprecated.

## Design Principles
- Small, composable operations with deterministic outputs.
- Clear error semantics; machine‑parsable text/JSON responses.
- Cross‑platform behavior parity where practical.

## Namespaces & Modules
- `Geometry`: core computational primitives and utilities.
- `IO`: interaction helpers (e.g., user prompts); platform‑specific behavior gated.
- `ETABS` (Windows‑only): interop and data exchange with ETABS.

## Data Types (Examples)
- Vectors, matrices, frames, geometry entities (lines, polygons, sections).
- Domain units and typed quantities where applicable.

## Operations (Examples)
- Geometry computations: area, centroid, transformations.
- Validation and normalization of inputs.
- Import/export routines for common formats (planned).

## Errors
- CLI exit codes are consistent per command.
- API returns structured error details in JSON/text forms.

## Extensibility
- External tool integrations (ETABS available on Windows).
- Configuration via CLI flags and environment variables.

## Stability & Compatibility
- Breaking changes are documented in `CHANGELOG.md`.
- Prefer additive changes; deprecate before removal where feasible.

## Examples
See `CLI.md` for end‑to‑end usage patterns and command examples.

---

<div align="center">
   <p><strong>Built with ❤️ for the global engineering community</strong></p>
   <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
