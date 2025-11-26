# AI Instructions for Gazelle

This document provides comprehensive instructions for AI agents working with the Gazelle structural engineering analysis engine.

## Project Overview

Gazelle is a **safety-critical structural engineering analysis engine** built with F# and .NET 9, emphasizing:
- **Type safety** with compile-time units of measure validation
- **Transparent algorithms** to prevent dangerous engineering errors  
- **Modern CLI** with colorful interface and emoji branding (🦌💨)
- **AI-friendly** structured JSON I/O and batch processing

## Core Architecture

### Key Components
- **`src/`** - Core F# library with units system (`src/units/`) and domain modeling (`src/concrete/`)
- **`cli/`** - CLI tool using Spectre.Console, packaged as global .NET tool `gz`
- **`tests/`** - XUnit test suite
- **`docs/`** - Professional website with binary downloads
- **`ai-agents/`** - AI integration resources and schemas

### Critical Safety Features
- **Units of measure**: F# compile-time validation (`float<kN>`, `float<m>`, `Stress<'TForce, 'TLength>`)
- **Domain types**: Engineering concepts (LimitState, DesignSituation, Density, Stress)
- **Structured validation**: JSON schemas for AI workflows

## AI Agent Workflows

### Model Creation & Analysis
```bash
# Create structural models from templates
gz create truss.json --template truss --span 10.0 --height 4.0 --loads 25.0
gz create beam.json --template cantilever --span 6.0 --loads 50.0
gz create frame.json --template portal --span 12.0 --height 4.0 --loads 15.0,100.0

# Validate models (essential for AI-generated content)
gz validate model.json --format json --detailed

# Analyze with structured output
gz analyze model.json --type static --output results.json --format json

# Get model information
gz info model.json --format json
```

### Batch Processing for AI Workloads
```bash
# Process multiple models with progress tracking
gz batch-analyze models/*.json --output-dir results/ --format json --progress

# Parallel processing for optimization workflows  
gz analyze-parallel design-variants/ --workers 8 --format json
```

### Template System
Available templates with parameters:
- **Truss**: `--span <meters> --height <meters> --loads <kN>`
- **Cantilever**: `--span <meters> --loads <kN>`
- **Portal**: `--span <meters> --height <meters> --loads <wind_kN>,<dead_kN>`

```bash
# List available templates
gz templates list --format json
```

## JSON Data Structures

### Model Format
```json
{
  "info": {
    "name": "Model Name",
    "description": "AI-generated structural model",
    "units": "SI", 
    "version": "1.0"
  },
  "nodes": {
    "n1": {"id": "n1", "x": 0.0, "y": 0.0, "z": 0.0}
  },
  "elements": {
    "e1": {
      "id": "e1",
      "type": "Truss2D",
      "nodes": ["n1", "n2"], 
      "material": "steel",
      "properties": {"area": 0.01}
    }
  },
  "materials": {
    "steel": {
      "elastic_modulus": 200e9,
      "density": 7850,
      "yield_strength": 355e6
    }
  }
}
```

### Analysis Results
```json
{
  "type": "static",
  "converged": true,
  "iterations": 1,
  "max_displacement": 0.000025,
  "max_reaction": 12500,
  "strain_energy": 0.3125,
  "displacements": {
    "n1": [0, -0.000025, 0]
  },
  "reactions": {
    "n1": [0, 12500, 0]
  }
}
```

## F# Type System Integration

### Units of Measure (Critical for Safety)
```fsharp
// Type-safe calculations prevent unit mixing disasters
let span = 10.0<m>          // Meters - compile-time validated
let load = 25000.0<N>       // Newtons - compile-time validated  
let stress = load / (0.01<m^2>)  // Automatic unit checking

// Custom engineering types
type Stress<'TForce, 'TLength> = Stress of float<'TForce / 'TLength^2>
type Density<'TMass, 'TLength> = Density of float<'TMass / 'TLength^3>
```

### Domain Modeling
```fsharp
// Structural engineering concepts
type LimitState = 
  | ULS of DesignSituation  // Ultimate Limit State
  | SLS                     // Serviceability Limit State

type DesignSituation = 
  | Persistent    // Normal conditions
  | Transient     // Temporary conditions  
  | Accidental    // Emergency conditions
```

## Development Guidelines for AI

### Safety-Critical Code Patterns
- **Always use units**: Prefer `float<kN>` over plain `float` for engineering values
- **Validate inputs**: Use `gz validate` before analysis on AI-generated models
- **Check convergence**: Verify `converged: true` in analysis results
- **Error context**: Include engineering meaning in error messages

### CLI Development Patterns
- **Modern UX**: Use Spectre.Console for colorful output
- **JSON first**: All outputs support `--format json` for AI consumption
- **Progress reporting**: Use `--progress` for long-running operations
- **Emoji branding**: 🦌 Gazelle 💨 in headers only (not body text)

### Version Management
```bash
# Update all project versions consistently
./scripts/update-version.sh 0.0.5

# Build and test
dotnet build
dotnet test
dotnet pack cli/
```

## Common AI Use Cases

### 1. Design Space Exploration
```bash
# Generate multiple design variants
for span in 8.0 10.0 12.0; do
  gz create truss_${span}m.json --template truss --span $span --height 4.0 --loads 25.0
done

# Batch analyze all variants
gz batch-analyze truss_*.json --output-dir exploration/ --format json
```

### 2. Parameter Optimization
```bash
# Create parametric models with varying loads
for load in 20.0 25.0 30.0 35.0; do
  gz create cantilever_${load}kN.json --template cantilever --span 6.0 --loads $load
  gz analyze cantilever_${load}kN.json --output results_${load}kN.json --format json
done
```

### 3. Model Validation Pipeline  
```bash
# AI-generated model validation workflow
gz validate ai-model.json --format json --detailed > validation.json
if [ $? -eq 0 ]; then
  gz analyze ai-model.json --output analysis.json --format json
else
  echo "Model validation failed - see validation.json for details"
fi
```

## Error Handling & Debugging

### Common Issues for AI-Generated Content
- **Unit mixing**: F# prevents at compile-time, but check input units
- **Invalid geometry**: Use validation before analysis
- **Convergence failures**: Check model constraints and loading
- **File format errors**: Validate JSON structure against schemas

### Debugging Commands
```bash
# Verbose analysis with detailed logging
gz analyze model.json --verbose --output results.json

# Detailed model information
gz info model.json --format json --detailed

# Validation with specific error reporting
gz validate model.json --format json --detailed
```

## Performance Expectations

- **Analysis time**: Microsecond-level execution for typical models
- **File I/O**: JSON serialization optimized for batch processing
- **Memory usage**: Minimal footprint for embedded AI workflows
- **Batch processing**: Scales linearly with model count

## Integration Examples

### Python AI Integration
```python
import subprocess
import json

# Create model via CLI
subprocess.run([
    'gz', 'create', 'ai-model.json', 
    '--template', 'truss',
    '--span', '10.0', '--height', '4.0', '--loads', '25.0'
])

# Analyze and parse results
result = subprocess.run([
    'gz', 'analyze', 'ai-model.json', '--format', 'json'
], capture_output=True, text=True)

analysis = json.loads(result.stdout)
max_displacement = analysis['max_displacement']
```

## Resources & Documentation

- **Main docs**: `/DOCS.md` - Complete user documentation
- **AI integration**: `/ai-agents/AI_INTEGRATION.md` - Detailed AI patterns
- **JSON schemas**: `/ai-agents/schemas/` - Model validation schemas  
- **CLI reference**: `gz help` - Interactive command reference
- **GitHub**: `https://github.com/jsbayley/gazelle` - Source code and issues

## Key Success Patterns

1. **Validate first**: Always validate AI-generated models before analysis
2. **Use templates**: Leverage built-in templates for consistent model generation
3. **Check convergence**: Verify analysis results indicate successful convergence
4. **Batch processing**: Use parallel processing for optimization workflows
5. **Structured output**: Always use `--format json` for machine consumption
6. **Safety focus**: Leverage F# type system to prevent engineering errors

## 🚨 CRITICAL: Documentation & Version Maintenance

**MANDATORY REQUIREMENT**: After completing ANY action, you MUST review and update ALL documentation and versioning to ensure consistency across the entire project.

### Required Post-Action Review Checklist

**Every time you finish an action, verify and update:**

1. **Version Synchronization**
   - `Directory.Build.props` - Central version source
   - `DOCS.md` - NuGet installation instructions
   - `docs/index.html` - Website download versions
   - `src/README.md` - NuGet package documentation
   - `cli/README.md` - CLI installation instructions

2. **Documentation Consistency**
   - `DOCS.md` ↔ `docs/docs.html` - Must match exactly
   - `README.md` - Project overview and badges
   - `CHANGELOG.md` - Version history and features
   - `ai-agents/AI_INTEGRATION.md` - AI workflow documentation
   - `.github/copilot-instructions.md` - AI agent guidance

3. **Website & Package Materials**
   - `docs/index.html` - Download links and version references
   - `docs/docs.html` - Complete user documentation
   - `src/README.md` - NuGet package description
   - All `.md` files in `/docs/` directory

4. **Technical Specifications**
   - Project file versions (.fsproj references)
   - Badge versions in README files
   - API documentation and examples
   - Schema files and integration guides

### Version Update Workflow
```bash
# Use the centralized version script
./scripts/update-version.sh <new-version>

# Verify all files updated correctly
grep -r "version\|Version" --include="*.md" --include="*.html" --include="*.json" .
```

### Documentation Sync Verification
```bash
# Check for inconsistencies between DOCS.md and docs.html
# Manual review required to ensure content matches

# Verify NuGet package README is current
cat src/README.md

# Check website version references
grep -n "v[0-9]\+\.[0-9]\+\.[0-9]\+" docs/index.html
```

**NEVER** complete an action without performing this comprehensive review. Inconsistent documentation and versioning undermines the professional quality and reliability that Gazelle represents to the engineering community.

Remember: Gazelle prioritizes safety-critical engineering accuracy over speed. Always validate models and check results for engineering reasonableness.