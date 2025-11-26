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

- `gz info <model>` - Show model information  
- `gz analyze <model>` - Analyze structural model
- `gz validate <model>` - Validate model structure  
- `gz create --template <name>` - Create new model from template
- `gz templates list` - List available templates
- `gz help` - Show help information

## Examples

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

## Global Options

- `--format <json|text>` - Output format (default: text)
- `--output <file>` - Output file path  
- `--verbose` - Enable verbose output
- `--help` - Show help information

## Status

The CLI is fully functional with colorful modern interface using Spectre.Console and emoji branding. It includes complete integration with the Gazelle analysis engine for structural modeling and analysis.