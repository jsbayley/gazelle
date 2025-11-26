# Documentation

Gazelle is a **community-led** project designed to:

- 🚀 Accelerate AEC innovation,
- 📚 Underpin academic research,
- 🔄 Offer a modern toolchain interpretation,
- 🎓 Support Structural Engineering education, and
- 🫱🏻‍🫲🏾 Connect like-minded Engineers.

## Installation

### Prerequisites
- .NET 9 SDK or later
- F# support (included with .NET SDK)

### Build from source
```bash
git clone https://github.com/jsbayley/gazelle.git
cd gazelle
dotnet build
dotnet run --project src/main.fs
```

### Install from NuGet
```bash
dotnet add package Gazelle --version 0.0.2
```

## CLI Usage

The `gz` command provides the primary interface for creating, validating, and analyzing structural models.

### Available Commands

```bash
gz create [file]     # Create new structural models
gz info [file]       # Display model information
gz validate [file]   # Check model integrity
gz analyze [file]    # Perform structural analysis
```

### Global Flags

- `--verbose, -v`: Enable verbose logging
- `--config`: Specify configuration file
- `--help, -h`: Show help information

## Model Creation

### Basic Syntax
```bash
gz create [output-file] --example [type] [options]
```

### Available Examples

#### Truss Structure
```bash
gz create truss.json --example truss --span 10.0 --height 4.0 --loads 25.0
```

Creates a 3-node triangular truss with:
- Span: 10.0 meters
- Height: 4.0 meters  
- Applied load: 25.0 kN (downward at apex)
- Pin supports at base nodes

#### Cantilever Beam
```bash
gz create beam.json --example cantilever --span 6.0 --loads 50.0
```

Creates a 2-node cantilever beam with:
- Length: 6.0 meters
- Fixed support at base
- Applied load: 50.0 kN (downward at tip)

#### Portal Frame
```bash
gz create frame.json --example portal --span 12.0 --height 4.0 --loads 15.0,100.0
```

Creates a 4-node portal frame with:
- Span: 12.0 meters
- Height: 4.0 meters
- Wind load: 15.0 kN (horizontal)
- Dead load: 100.0 kN (vertical)
- Fixed supports at base

### Creation Options

- `--span`: Horizontal dimension in meters
- `--height`: Vertical dimension in meters  
- `--loads`: Load values in kN (comma-separated for multiple loads)

## Model Analysis

### Static Analysis
```bash
gz analyze model.json --type static --output results.json
```

Performs linear static analysis and calculates:
- Node displacements
- Support reactions
- Strain energy
- Maximum displacement and reaction values

### Modal Analysis
```bash
gz analyze model.json --type modal
```

Performs eigenvalue analysis and determines:
- Natural frequencies (Hz)
- Mode shapes
- Dynamic characteristics

### Analysis Options

- `--type`: Analysis type (static, modal, dynamic)
- `--output, -o`: Output file for results
- `--solver`: Solver type (auto, cholesky, lu)
- `--tolerance`: Convergence tolerance (default: 1e-9)
- `--max-iterations`: Maximum solver iterations (default: 1000)

## Model Validation

```bash
gz validate model.json
```

Performs comprehensive model checking:
- Structural integrity verification
- Node connectivity analysis
- Constraint sufficiency checking
- Load case validation
- Material property verification

## Model Information

```bash
gz info model.json
```

Displays detailed model statistics:
- Node count and coordinates
- Element types and properties
- Material definitions
- Applied loads and constraints
- Geometric bounds and dimensions

## File Formats

### Model Files
Gazelle uses JSON format for structural models with the following structure:

```json
{
  "info": {
    "name": "Model Name",
    "description": "Model Description", 
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
      "id": "steel",
      "name": "Structural Steel",
      "type": "Steel",
      "elastic_modulus": 200e9,
      "density": 7850,
      "yield_strength": 355e6
    }
  },
  "loads": {
    "l1": {
      "id": "l1",
      "type": "Force",
      "node": "n1",
      "direction": "Fy", 
      "magnitude": -10000.0
    }
  },
  "constraints": {
    "c1": {
      "id": "c1",
      "type": "Fixed",
      "node": "n1",
      "dof": ["Ux", "Uy", "Rz"]
    }
  }
}
```

### Results Files
Analysis results are saved in JSON format:

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

## Worked Examples

### Example 1: Simple Truss Analysis

Create and analyze a 8m span truss with 30 kN load:

```bash
# Create model
gz create example1.json --example truss --span 8.0 --height 3.0 --loads 30.0

# Validate model  
gz validate example1.json

# Run static analysis
gz analyze example1.json --type static --output example1_results.json

# View results
gz info example1.json
```

Expected results:
- Maximum displacement: ~2.5e-5 m
- Support reactions: ~15 kN each
- Analysis time: <5 microseconds

### Example 2: Cantilever Beam Design

Design a 5m cantilever beam for 40 kN tip load:

```bash
# Create beam model
gz create cantilever.json --example cantilever --span 5.0 --loads 40.0

# Analyze deflection
gz analyze cantilever.json --output cantilever_results.json

# Check natural frequencies
gz analyze cantilever.json --type modal
```

Expected results:
- Tip deflection: ~6.0e-5 m
- Fixed support reaction: 40 kN
- First natural frequency: ~10 Hz

### Example 3: Portal Frame Analysis

Analyze a 10m x 4m portal frame with combined loading:

```bash
# Create frame with wind and dead loads
gz create portal.json --example portal --span 10.0 --height 4.0 --loads 20.0,80.0

# Perform comprehensive analysis
gz validate portal.json
gz analyze portal.json --type static --output portal_static.json
gz analyze portal.json --type modal

# Review model properties
gz info portal.json
```

Expected results:
- Maximum displacement: ~9.0e-5 m
- Base reactions: Variable based on load distribution
- Multiple natural frequencies: 10-25 Hz range



## Units and Conventions

- Length: meters (m)
- Force: Newtons (N) 
- Stress: Pascals (Pa)
- Mass: kilograms (kg)
- Time: seconds (s)
- Frequency: Hertz (Hz)
- Load input: kilonewtons (kN) - automatically converted to N

## Error Handling

Gazelle provides comprehensive error checking:
- Units of measure validation eliminating dangerous unit mixing disasters
- Strong type safety with F# preventing common engineering mistakes
- Model validation errors with specific diagnostic messages
- File I/O error reporting with recovery suggestions

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
