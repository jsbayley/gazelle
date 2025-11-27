# Gazelle

🦌 **A Fast Engine for Structural Engineering** 💨

Gazelle is a safety-critical structural analysis library built with F# and .NET 9, featuring compile-time units of measure validation and transparent algorithms designed to prevent dangerous engineering errors.

## Key Features

- **🛡️ Type Safety**: F# units of measure prevent unit mixing disasters (`float<kN>`, `float<m>`)
- **🔍 Transparent**: Open source algorithms you can inspect and verify
- **🌍 Cross-Platform**: Runs on Windows, macOS, and Linux via .NET 9
- **📐 Engineering-First**: Built specifically for structural analysis workflows

## Installation

```bash
dotnet add package Gazelle
```

## Quick Start

```fsharp
open Gazelle.Units
open FSharp.Data.UnitSystems.SI.UnitSymbols

// Type-safe structural calculations
let span = 10.0<m>
let load = 25000.0<N>
let momentOfInertia = 0.0001<m^4>

// Units are validated at compile time
let stress = Stress(load / (0.01<m^2>))  // Automatic unit checking
```

## Domain Types

```fsharp
// Engineering-specific types with units of measure
type Stress<'TForce, 'TLength> = Stress of float<'TForce / 'TLength^2>
type Density<'TMass, 'TLength> = Density of float<'TMass / 'TLength^3>

// Structural engineering concepts
type LimitState = ULS of DesignSituation | SLS
type DesignSituation = Persistent | Transient | Accidental
```

## CLI Tool

Install the companion CLI tool for interactive analysis:

```bash
dotnet tool install --global Gazelle.CLI
gz create --template truss --span 10.0 --height 4.0 --loads 25.0
gz analyze model.json --output results.json
```

## Safety & Reliability

This library is designed for safety-critical engineering applications:
- Compile-time unit validation eliminates dangerous calculation errors
- Strong typing prevents common engineering mistakes  
- Open source algorithms enable verification and validation
- Comprehensive error handling with engineering context

## Documentation

- **GitHub**: [github.com/jsbayley/gazelle](https://github.com/jsbayley/gazelle)
- **CLI Guide**: [Full documentation and examples](https://github.com/jsbayley/gazelle/blob/main/DOCS.md)
- **API Reference**: Generated XML documentation included

## License

AGPL-3.0-or-later - Open source for the global engineering community