# Gazelle AI Copilot Instructions

## Project Overview
Gazelle is a **safety-critical structural engineering analysis engine** built with F# and .NET 9. The project prioritizes **type safety, units of measure validation, and transparent algorithms** to prevent dangerous engineering errors.

## Architecture & Components

### Cross-Platform Design
- **Conditional compilation**: `#if WINDOWS` for ETABS COM interop
- **Universal CLI**: `gz` command works on Windows, macOS, and Linux
- **Graceful degradation**: Platform-specific features show informative error messages

### Critical Safety Features
- **Units of measure**: F# compile-time validation prevents unit mixing disasters
- **Strong typing**: Domain types for engineering concepts (stress, density, limit states)  
- **Platform awareness**: Clear error messages for Windows-only features
- **Consolidated codebase**: Single library eliminates inter-project dependency issues

## Development Patterns

### F# Conventions
- Files ordered by dependency in `.fsproj` (Units.fs → Annotation.fs → Conversion.fs → Math.fs)
- Domain modelling with discriminated unions: `LimitState = ULS | SLS`, `DesignSituation = Persistent | Transient | Accidental`
- Units of measure: `float<kN>`, `float<m>`, custom types like `Stress<'TForce, 'TLength>`

### CLI Development (`cli/Program.fs`)
- Custom F#-idiomatic argument parser using pattern matching (avoid System.CommandLine)
- Coloured output with Spectre.Console: `AnsiConsole.MarkupLine("[green]Success![/]")`
- JSON serialization with camelCase: `JsonNamingPolicy.CamelCase`

### Version Management
- **Centralized**: `Directory.Build.props` contains `<Version>0.0.8</Version>` inherited by all projects
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
dotnet run --project cli -- analyse model.json --verbose
```

### Version Updates  
```bash
./scripts/update-version.sh 0.0.8   # Updates Directory.Build.props + all docs
git add . && git commit -m "Update version to 0.0.8"
```

## Project-Specific Guidelines

### Safety-Critical Engineering Code
- Always validate units: prefer `float<kN>` over `float` for engineering values
- Use domain types: `Stress<kN, m>` not generic numbers
- Include engineering context in error messages: "Load exceeds yield strength"
- Leverage consolidated architecture: all functionality in single `Gazelle.dll`

### Cross-Platform Compatibility
- Use conditional compilation `#if WINDOWS` for ETABS-specific code
- Provide graceful error messages for Windows-only features on other platforms
- Test CLI functionality on Linux dev container environment
- Ensure core analysis works universally across all platforms

### CLI UX Standards  
- Emoji branding: 🦌 Gazelle 💨 in headers only (not in body text)
- Colorful output: green for success, red for errors, cyan for info, yellow for warnings
- JSON output option: `--format json` for machine consumption
- Cross-platform help: show platform-specific feature availability

### Documentation Standards
- **No duplicate DOCS.md files**: There should only be a single `DOCS.md` in project
- Use British English spellings (e.g., "modelling", "optimisation").

### AI/Automation Integration
- Structured JSON I/O for all models and results
- Error handling with machine-readable diagnostics
- Platform-aware workflows: handle Windows-only features gracefully
