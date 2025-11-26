# Gazelle AI Copilot Instructions

## Project Overview
Gazelle is a **safety-critical structural engineering analysis engine** built with F# and .NET 9. The project prioritizes **type safety, units of measure validation, and transparent algorithms** to prevent dangerous engineering errors.

## Architecture & Components

### Core Structure
- **`src/`** - Main library with units of measure system (`src/units/`) and concrete engineering domain (`src/concrete/`)  
- **`cli/`** - Modern CLI tool using Spectre.Console with emoji branding (🦌💨), packaged as global .NET tool `gz`
- **`tests/`** - XUnit test suite
- **`docs/`** - Professional landing page website for binary downloads

### Critical Safety Features
- **Units of measure**: F# compile-time validation prevents unit mixing disasters (`src/units/Units.fs`)
- **Strong typing**: Domain types for engineering concepts (stress, density, limit states)  
- **JSON schemas**: Machine-readable validation for AI workflows (`ai-agents/schemas/`)

## Development Patterns

### F# Conventions
- Files ordered by dependency in `.fsproj` (Units.fs → Annotation.fs → Conversion.fs → Math.fs)
- Domain modeling with discriminated unions: `LimitState = ULS | SLS`, `DesignSituation = Persistent | Transient | Accidental`
- Units of measure: `float<kN>`, `float<m>`, custom types like `Stress<'TForce, 'TLength>`

### CLI Development (`cli/Program.fs`)
- Custom F#-idiomatic argument parser using pattern matching (avoid System.CommandLine)
- Colorful output with Spectre.Console: `AnsiConsole.MarkupLine("[green]Success![/]")`
- JSON serialization with camelCase: `JsonNamingPolicy.CamelCase`
- Template system: beam, truss, portal structures with parameterization

### Version Management
- **Centralized**: `Directory.Build.props` contains `<Version>0.0.4</Version>` inherited by all projects
- **Script**: `./scripts/update-version.sh` updates all files consistently
- **Global tool**: CLI packaged as `gz` command via `PackAsTool=true`

## Key Workflows

### Building & Testing
```bash
dotnet build                    # Build solution
dotnet test                     # Run XUnit tests  
dotnet run --project cli -- help  # Test CLI directly
dotnet pack cli/                # Package CLI tool
```

### CLI Development
```bash
# Test CLI features
dotnet run --project cli -- info model.json --format json
dotnet run --project cli -- create --template truss --output test.json
dotnet run --project cli -- analyze model.json --verbose
```

### Version Updates  
```bash
./scripts/update-version.sh 0.0.5   # Updates Directory.Build.props + all docs
git add . && git commit -m "Update version to 0.0.5"
```

## Project-Specific Guidelines

### Safety-Critical Engineering Code
- Always validate units: prefer `float<kN>` over `float` for engineering values
- Use domain types: `Stress<kN, m>` not generic numbers
- Include engineering context in error messages: "Load exceeds yield strength"

### CLI UX Standards  
- Emoji branding: 🦌 Gazelle 💨 in headers only (not in body text)
- Colorful output: green for success, red for errors, cyan for info
- JSON output option: `--format json` for machine consumption
- Template system: consistent parameterization (span, height, loads)

### AI/Automation Integration
- Structured JSON I/O for all models and results
- Batch processing support: `gz batch-analyze pattern`  
- Template system for consistent model generation
- Error handling with machine-readable diagnostics

## Dependencies & Integration
- **Spectre.Console 0.49.1**: Modern terminal UI (required for CLI)
- **System.Text.Json 9.0.0**: High-performance JSON (avoid Newtonsoft.Json)
- **XUnit 2.4.2**: Testing framework  
- **AGPL-3.0**: Open source license - respect copyleft requirements

## AI-Specific Features
- JSON schemas in `ai-agents/schemas/` for model validation
- Structured error reporting with specific diagnostic codes
- Template-based model generation for consistent AI workflows
- Type-safe domain modeling prevents common engineering mistakes