# Gazelle.IO Documentation

**IO Library for Gazelle Structural Analysis Engine with ETABS Interop**

## Overview

`Gazelle.IO` is a library that provides robust IO operations and ETABS interop for the Gazelle structural analysis engine. It follows a proven architectural pattern with strong type safety and comprehensive error handling.

## Quick Start

### Building the Library

```bash
# Build the library
dotnet build io/Gazelle.IO.fsproj

# Use in your projects
dotnet add reference io/Gazelle.IO.fsproj
```

### Basic Usage

```fsharp
// Reference the library in your project
dotnet add reference path/to/Gazelle.IO.fsproj

// Use in your code
open Gazelle.IO
open Gazelle.IO.ETABS
open Gazelle.Units  // From Gazelle core
```

## Library Architecture

### Project Structure

```
io/
├── Gazelle.IO.fsproj          # Main library project file
├── Shared/                    # Core IO utilities
│   ├── UnvalidatedTypes.fs   # Input validation types
│   ├── ValidatedTypes.fs     # Validated domain types
│   ├── ErrorHandling.fs      # Robust error management
│   ├── Messages.fs           # User interface messaging
│   ├── IO.fs                 # File I/O and user interaction
│   └── Model.fs              # Core model definitions
├── ETABS/                     # ETABS-specific integration
│   ├── Types.fs              # ETABS type definitions
│   ├── ETABS.fs              # Main ETABS API wrapper
│   └── Renderer.fs           # Model rendering utilities
├── DLLs/                      # ETABS COM interop libraries
└── DOCS.md                    # This documentation
```

### Core Features

- **F# Project** - Clean, modern architecture with .NET 9
- **Gazelle Integration** - Full access to `Gazelle.Units` system
- **Type Safety** - Compile-time validation for engineering calculations
- **Error Handling** - Robust `Result<'T, IOError>` pattern
- **Multi-Version Support** - ETABS V17/V19 compatibility

## ETABS Integration

### Basic ETABS Operations

```fsharp
open Gazelle.IO.ETABS

// Launch ETABS and create a new model
let result = ETABS.start()

match result with
| Ok (etabsApp, sapModel) ->
    printfn "✅ ETABS connected successfully"
    
    // Initialize a clean model environment
    ETABS.initialise sapModel
    
    // Refresh the ETABS view
    ETABS.refreshView sapModel
    
    // ... perform your structural analysis operations ...
    
    // Close ETABS (with save)
    ETABS.close result SaveFile
    
| Error e ->
    printfn "❌ Failed to connect to ETABS: %A" e
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

### Using with Gazelle Core

```fsharp
open Gazelle.Units
open Gazelle.IO
open Gazelle.IO.ETABS

// Combine Gazelle units with ETABS operations
let columnHeight = 3.5<m>
let beamSpan = 6.0<m>
let loadValue = 2.5<kN/m^2>

// Use validated types for safe operations
let result = ETABS.start()
// ... structural analysis with type-safe units
```

### Project Integration

```xml
<!-- In your project file -->
<ItemGroup>
  <ProjectReference Include="../io/Gazelle.IO.fsproj" />
</ItemGroup>
```

```fsharp
// In your F# code
open Gazelle.IO
open Gazelle.IO.ETABS

// Ready to use all Gazelle.IO functionality
```

## Best Practices

1. **Always use Result pattern**: Handle all operations with proper error checking
2. **Validate inputs**: Use the UnvalidatedTypes → ValidatedTypes pattern
3. **Resource management**: Always close ETABS connections properly
4. **Type safety**: Leverage F#'s type system for engineering calculations
5. **Error messaging**: Provide clear, actionable error messages

## Support

For issues, questions, or contributions:
- **Repository**: https://github.com/jsbayley/gazelle
- **License**: AGPL-3.0-or-later
- **Version**: Built with .NET 9 and F# latest