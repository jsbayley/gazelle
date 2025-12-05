# Gazelle CLI

The command-line interface for Gazelle: A Fast Engine for Structural Engineering.

## Installation

Install as a global .NET tool:

```bash
dotnet pack
dotnet tool install --global --add-source ./bin/Release Gazelle.CLI
```

Or run directly:

```bash
dotnet run -- <command> [options]
```

## Commands

### Core Analysis
- `gz info <model>` - Show model information  
- `gz analyze <model>` - Analyze structural model
- `gz validate <model>` - Validate model structure  
- `gz create --template <name>` - Create new model from template
- `gz templates list` - List available templates

### ETABS Integration 🦌💨
- `gz etabs demo` - ETABS interop demonstration
- `gz etabs units` - Units of measure examples  
- `gz etabs connect` - Connect to existing ETABS instance
- `gz etabs` - ETABS command help

### General
- `gz help` - Show help information

## Examples

### Core Analysis
```bash
# Get model information in JSON format
gz info model.json --format json

# Analyze a model with verbose output
gz analyze beam.json --verbose --output results.json

# Create a new truss model
gz create --template truss --output my-truss.json

# List available templates  
gz templates list --format json

# Validate a model with detailed output
gz validate model.json --format json --detailed
```

### ETABS Integration
```bash
# Run ETABS interop demonstration
gz etabs demo --verbose

# Show units of measure examples
gz etabs units

# Test ETABS connection with JSON output
gz etabs connect --format json

# Save ETABS results to file
gz etabs units --output structural-calculations.json --format json
```

## Global Options

- `--format <json|text>` - Output format (default: text)
- `--output <file>` - Output file path  
- `--verbose` - Enable verbose output
- `--help` - Show help information

## Status

The CLI is fully functional with colourful modern interface using Spectre.Console and emoji branding. It includes complete integration with the Gazelle analysis engine for structural modelling and analysis.