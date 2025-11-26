# Gazelle IO Documentation

**Cross-Platform IO with Windows ETABS Integration**

## Overview

The Gazelle IO module provides robust file operations and optional Windows-specific ETABS interop as part of the main Gazelle library. It features cross-platform compatibility with conditional compilation for Windows-only features, consolidated architecture, and comprehensive type safety.

## Quick Start

### Building the Library

```bash
# Build the main Gazelle library (includes IO functionality)
dotnet build src/Gazelle.fsproj

# Or build entire solution
dotnet build
```

### Basic Usage

```fsharp
// Reference the main Gazelle library
dotnet add reference path/to/src/Gazelle.fsproj

// Use in your code - everything in one library
open Gazelle.Units      // Units of measure system
open Gazelle.IO         // File operations and validation
open Gazelle.IO.ETABS   // ETABS interop (Windows only)
```

## Library Architecture

### Consolidated Project Structure

```
src/
├── Gazelle.fsproj             # Main unified library
├── units/                     # Core units system
│   ├── Units.fs              # F# units of measure
│   ├── Annotation.fs         # Unit annotations
│   ├── Conversion.fs         # Unit conversions
│   └── Math.fs               # Mathematical operations
├── Geometry.fs               # Core geometry
├── io/                        # IO functionality (consolidated)
│   ├── Types.fs              # All domain types and validation
│   ├── IO.fs                 # File operations and user interaction
│   └── ETABS.fs              # ETABS interop (Windows conditional)
├── .d/DLLs/                   # ETABS COM libraries (Windows)
└── concrete/                  # Concrete engineering domain
```

### Core Features

- **Cross-Platform** - Runs on Windows, macOS, and Linux (.NET 9)
- **Conditional Compilation** - Windows ETABS features, cross-platform core
- **Consolidated Architecture** - Single library with all functionality
- **Type Safety** - F# compile-time validation for engineering calculations
- **Robust Error Handling** - `Result<'T, IOError>` pattern throughout
- **Multi-Version ETABS** - V17/V19 compatibility (Windows only)

## Cross-Platform Compatibility

### Platform Support

| Platform | Core Features | ETABS Integration | CLI Tool |
|----------|---------------|-------------------|-----------|
| **Windows** | ✅ Full | ✅ V17/V19 | ✅ gz command |
| **macOS** | ✅ Full | ❌ Graceful error | ✅ gz command |
| **Linux** | ✅ Full | ❌ Graceful error | ✅ gz command |

### Conditional Compilation

```fsharp
#if WINDOWS
// Full ETABS COM interop available
type ETABSObject = V17 of ETABSv17.cOAPI | V19 of ETABSv1.cOAPI
#else
// Graceful degradation on non-Windows platforms
module ETABS = 
  let start() = Error(UnsupportedVersion "ETABS integration is only available on Windows platforms.")
#endif
```

## ETABS Integration (Windows Only)

### Basic ETABS Operations

```fsharp
open Gazelle.IO.ETABS

// Launch ETABS (Windows only)
let result = ETABS.start()

match result with
| Ok (etabsApp, sapModel) ->
    printfn "✅ ETABS connected successfully (Windows)"
    
    // Initialize a clean model environment
    ETABS.initialise sapModel
    
    // Refresh the ETABS view
    ETABS.refreshView sapModel
    
    // ... perform your structural analysis operations ...
    
    // Close ETABS (with save)
    ETABS.close result SaveFile
    
| Error e ->
    // Will show platform-specific error on macOS/Linux
    printfn "❌ ETABS operation failed: %A" e
```

### ETABS Version Support

The library supports multiple ETABS versions:

```fsharp
type Version =
  | V17  // ETABS 17
  | V19  // ETABS 19

type InstanceType =
  | NewAppInstance           // Launch new ETABS instance
  | ExistingLiveInstance     // Attach to running ETABS (TODO: implementation pending)
```

### Save Options

```fsharp
type SaveOption =
  | SaveFile        // Save the model before closing
  | DoNotSaveFile   // Close without saving
```

## Error Handling

### Result Pattern

All operations use the `Result<'T, IOError>` pattern for robust error handling:

```fsharp
type IOError =
  | BadUserInput
  | UnsupportedVersion of string
  | FileNotFound of string
  // ... other error cases
```

### Interactive Input with Validation

```fsharp
open Gazelle.IO

// All operations return Result<'T, IOError> for robust error handling
let validateInput (input: string) : Result<ValidatedInput, IOError> =
    match input.Trim().ToUpper() with
    | "VALID" -> Ok (ValidatedInput input)
    | _ -> Error (BadUserInput "Invalid input provided")

// Interactive user input with validation
let userChoice = IO.askForUserInput "Enter your choice:" validateInput

match userChoice with
| Ok validChoice -> 
    // Process valid input
    printfn "Processing: %A" validChoice
| Error err ->
    // Handle error gracefully
    IOError.print err
```

## Type Safety

### Validated Types Pattern

The library emphasizes type safety with validated domain types:

```fsharp
// Unvalidated input types
type UnvalidatedInput = string

// Validated domain types  
type ValidatedInput = ValidatedInput of string

// Conversion with validation
let validate : UnvalidatedInput -> Result<ValidatedInput, IOError>
```

### Type-Safe Engineering

```fsharp
// Future: Full units of measure integration
type MaterialProperties = {
    ElasticModulus: float    // Pa
    Density: float           // kg/m³ 
    YieldStrength: float     // Pa
}

// Error handling pattern
type ETABSResult<'T> = 
    | Success of 'T
    | Error of string
```

## Build Status

- ✅ **Clean Build**: Zero compilation errors
- ✅ **Library Output**: Proper .NET library (not executable)
- ✅ **Gazelle Integration**: Full access to structural analysis engine
- ✅ **Modern F#**: Latest .NET 9 language features
- ✅ **Type Safety**: Compile-time validation throughout

## Development Roadmap

### Current Status: Foundation Complete ✅

The library has a solid foundation with:
- Proven architectural pattern
- Robust error handling
- ETABS integration framework
- Type-safe domain modeling

### Phase 2: Enhanced ETABS Integration

- **COM Interop**: Complete Marshal.GetActiveObject implementation
- **Model Generation**: Expand structural element creation
- **Units Integration**: Seamless Gazelle.Units compatibility

### Phase 3: Advanced Features

- **Batch Processing**: Multiple model operations
- **Template System**: Standardized model generation
- **Export/Import**: Model serialization capabilities

## Integration Examples

### Unified Library Usage

```fsharp
// Everything available in single library
open Gazelle.Units        // Cross-platform units system
open Gazelle.IO           // Cross-platform file operations 
open Gazelle.IO.ETABS     // Windows-only ETABS features

// Type-safe engineering calculations (all platforms)
let columnHeight = 3.5<m>
let beamSpan = 6.0<m> 
let loadValue = 2.5<kN/m^2>

// Platform-aware ETABS usage
#if WINDOWS
let result = ETABS.start()  // Full functionality
#else
let result = ETABS.start()  // Returns appropriate error
#endif
```

### Project Integration

```xml
<!-- Single library reference -->
<ItemGroup>
  <ProjectReference Include="../src/Gazelle.fsproj" />
</ItemGroup>
```

```fsharp
// All functionality available from single import
open Gazelle.Units
open Gazelle.IO
open Gazelle.IO.ETABS

// Cross-platform by default, Windows features conditional
```

## CLI Usage

### Cross-Platform Commands

```bash
# Install CLI tool (works on all platforms)
dotnet tool install --global Gazelle.CLI

# Core functionality (all platforms)
gz help                           # Show help
gz create --template beam         # Create structural models
gz analyze model.json             # Perform analysis
gz templates list                 # List available templates

# Windows-specific ETABS integration
gz etabs demo                     # Windows: Full demo, Others: Error message
gz etabs connect                  # Windows: Connect to ETABS, Others: Error
```

### Platform Behavior

```bash
# On Windows
$ gz etabs demo
✅ ETABS integration demo running...

# On macOS/Linux  
$ gz etabs demo
❌ ETABS integration is only available on Windows platforms.
```

## Best Practices

1. **Cross-Platform Awareness**: Design code to work on all platforms where possible
2. **Conditional Features**: Use conditional compilation for Windows-only functionality  
3. **Result Pattern**: Handle all operations with proper error checking
4. **Graceful Degradation**: Provide clear error messages for unsupported features
5. **Type Safety**: Leverage F#'s compile-time validation for engineering calculations
6. **Resource Management**: Always close ETABS connections properly (Windows)

## Support

For issues, questions, or contributions:
- **Repository**: https://github.com/jsbayley/gazelle
- **License**: AGPL-3.0-or-later
- **Version**: Built with .NET 9 and F# latest