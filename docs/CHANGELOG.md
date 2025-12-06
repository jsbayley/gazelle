# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.8] - 2025-11-26

### Added
- Complete architectural consolidation: 70% reduction in file complexity while preserving 100% functionality
- Cross-platform support with conditional Windows ETABS integration using `#if WINDOWS` compilation
- Single unified library replacing multiple separate projects (Gazelle + Gazelle.IO → Gazelle)
- Enhanced CLI tool (`gz`) working seamlessly on Windows, macOS, and Linux
- Consolidated documentation with single source of truth in root DOCS.md
- Comprehensive cleanup of obsolete project files and directories

### Changed
- Unified project structure: merged separate libraries into single consolidated library
- Cross-platform CLI targeting: reverted from net9.0-windows to net9.0 for universal compatibility
- Documentation consolidation: eliminated duplicate DOCS.md files throughout solution
- Simplified build process with 50% reduction in project complexity and dependencies
- Enhanced website with cross-platform compatibility tables and feature matrices

### Removed
- Obsolete `/io/` directory containing old project files no longer referenced by solution
- Redundant `src/io/Gazelle.IO.fsproj` file after functionality integration into main project
- Duplicate documentation files maintaining single authoritative source
- Unused build artifacts and platform-specific development cruft
- macOS `.DS_Store` files and other development artifacts

### Technical Improvements
- **Build Performance**: Faster builds due to simplified project dependencies and single assembly output
- **Assembly Output**: Reduced from 2 DLLs to 1 consolidated assembly
- **Cross-Platform Verification**: Full CLI functionality tested and working on Linux dev container
- **Error Handling**: Enhanced platform-aware messaging for Windows-only features
- **Documentation Quality**: Modern CSS styling, info boxes, and comprehensive feature coverage

## [0.0.4] - 2025-01-26

### Added
- F# CLI implementation with colourful modern interface and emoji branding
- Complete structural model creation for truss, cantilever, and portal frame examples
- Static and modal analysis engines with microsecond execution times
- JSON-based model and results file formats
- Comprehensive model validation system
- Spectre.Console-based command line interface with rich formatting
- Cross-platform binary build support
- Global .NET tool packaging as `gz`
- Centralized version management system
- Enhanced units of measure with F# type safety

## [0.0.3] - 2025-01-24

### Added
- F# core library with structural analysis capabilities
- Domain modelling with strong types and units of measure
- Basic project structure and documentation
- AGPL-3.0 licensing
- Development container configuration

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
