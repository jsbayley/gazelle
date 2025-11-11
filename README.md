<div align="center">
  <img src="./.github/assets/gazelle.png" width="100px" height="100px" />
  <h1>Gazelle™</h1>
  <p>🦌 A Fast Engine for Structural Engineering 💨</p>
  <p><em>Fast • Stable • Reliable • Transparent • Cross-platform • Extensible</em></p>
  
  [![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/jsbayley/gazelle)
  [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](https://github.com/jsbayley/gazelle/blob/main/.github/CODE_OF_CONDUCT.md)

  [![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-00add8)](https://choosealicense.com/licenses/agpl-3.0/)
  [![CI](https://github.com/jsbayley/gazelle/actions/workflows/ci.yml/badge.svg)](https://github.com/jsbayley/gazelle/actions/workflows/ci.yml)

  [![Rust](https://img.shields.io/badge/Rust-1.87.0-ce412b?logo=rust)](https://www.rust-lang.org)
  [![Python](https://img.shields.io/badge/Python-3.9+-4584b6?logo=python&logoColor=f5f5f5)](https://www.python.org)
  [![Daemon](https://img.shields.io/badge/Daemon-HTTP%2FgRPC-green)](https://localhost:3000)
</div>

---

## ⭐ The Vision

Gazelle is a **fast engine for structural engineering** that empowers innovation, research, and education in the AEC industry. Built with Rust's performance and safety, featuring type-safe units, daemon architecture, and comprehensive Python bindings.

### 🎯 **Core Principles**
- **🚀 Fast** - Rust-native performance with optimized linear algebra
- **🛡️ Stable** - Type-safe units prevent engineering disasters 
- **🔒 Reliable** - Comprehensive test suite with analytical validation
- **🌍 Transparent** - Open-source algorithms, auditable calculations
- **📱 Cross-platform** - Runs everywhere: Linux, macOS, Windows, Web
- **🎯 Great DX** - Developer-friendly APIs and "batteries included"
- **🐍 Python Ready** - First-class Python bindings out of the box
- **🔧 Extensible** - Plugin architecture for design codes and elements

---

## 🚀 Quick Start

### Daemon Mode (Recommended)
```bash
# Start the Gazelle service
gazelle daemon start --port 3000

# Use from Python
pip install gazelle
python -c "
import gazelle
gz = gazelle.Gazelle('localhost:3000')
results = gz.analyze_truss(span=10.0, load=50.0)
print(f'Max displacement: {results.max_displacement} mm')
"

# Use from CLI  
gazelle create truss --span 10m --load 50kN
gazelle analyze model.json --format vtk
```

### Library Mode
```rust
use gazelle::prelude::*;

// Type-safe engineering with units
let concrete = Concrete::try_create(
    CylinderStrength::Uk(UkConcreteGrade::Fck30),
    Aggregate::Limestone,
    Cement::ClassN,
    WeightClass::NormalWeight,
    28.0 // days - validated!
)?;

// Build and analyze
let mut model = Model::new();
model.add_rc_column(Length::new(3000.0), concrete)?;
let results = Analysis::new(model).static_analysis()?;
```

---

## 🏗️ Architecture

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│   Python Bindings  │    │    CLI Client       │    │    Web Interface    │
│   gz.analyze(...)   │    │  gz daemon start    │    │  Browser Dashboard  │
└─────────┬───────────┘    └─────────┬───────────┘    └─────────┬───────────┘
          │                          │                          │
          └──────────────────────────┼──────────────────────────┘
                                     │
                        ┌─────────────────────┐
                        │   HTTP/gRPC API     │
                        │  localhost:3000     │
                        └─────────┬───────────┘
                                  │
                        ┌─────────────────────┐
                        │  Gazelle Daemon     │
                        │  Session Manager    │
                        └─────────┬───────────┘
                                  │
                        ┌─────────────────────┐
                        │   Analysis Engine   │
                        │  Type-Safe Core     │
                        └─────────────────────┘
```

## 🎯 Key Features

### 🔬 **Type-Safe Engineering**
```rust
// Prevents unit mixing disasters (Mars Climate Orbiter style!)
let stress = Stress::new(25.0); // MPa, cannot mix with kPa
let area = Area::cross_sectional(2500.0); // mm², cannot mix with surface area
let force = Force::new(100.0); // kN, type-checked operations
```

### 🏢 **Real Engineering Materials**  
```rust
// Eurocode-compliant concrete with time-dependent properties
let c30_limestone = Concrete::try_create(
    CylinderStrength::Uk(UkConcreteGrade::Fck30),
    Aggregate::Limestone,  // Affects elastic modulus (0.9x factor)
    Cement::ClassN,        // Affects time-dependent strength  
    WeightClass::NormalWeight,
    28.0 // Age validation prevents unrealistic inputs
)?;

println!("fck: {} MPa", c30_limestone.fck().value()); // 30.0
println!("Ecm: {} MPa", c30_limestone.ecm());         // 29,700 (adjusted)
```

### 🌐 **Daemon Architecture**
```bash
# Start service once, use from anywhere
gazelle daemon start

# REST API for ecosystem integration
curl -X POST http://localhost:3000/sessions \
  -H "Content-Type: application/json" \
  -d '{"model": {...}}'

# Python, CLI, web apps all use the same robust backend
```

### 🧪 **Rigorous Testing**
- **13/13 tests passing** with analytical validation
- **Property-based testing** for edge case coverage
- **Integration tests** for complete engineering workflows  
- **Benchmark suite** for performance regression detection

---

## 📚 Table of Contents

- [Installation](#installation)
- [Documentation](#documentation) 
- [Examples](#examples)
- [Get Involved](#get-involved)
- [Open Source Philosophy](#open-source-philosophy)
- [Safety & Errata](#safety--errata)

## 🔧 Installation

### Prerequisites
- **Rust** 1.70+ (for building from source)
- **Python** 3.8+ (for Python bindings)

### From Source
```bash
git clone https://github.com/jsbayley/gazelle
cd gazelle
cargo build --release --features daemon
```

### Python Package (Coming Soon)
```bash
pip install gazelle
```

## 📖 Documentation

- **[Getting Started Guide](./DOCS.md)** - Complete setup and first analysis
- **[Architecture Guide](./docs/ARCHITECTURE.md)** - System design with Mermaid diagrams
- **[Workflow Guide](./docs/WORKFLOWS.md)** - Visual user journeys and usage patterns  
- **[Technical Guide](./docs/TECHNICAL.md)** - Stack overview and performance characteristics
- **[API Reference](./docs/api/)** - Comprehensive API documentation  
- **[Examples](./examples/)** - Real engineering problems solved
- **[Contributing](./CONTRIBUTING.md)** - How to contribute to the project

## 💡 Examples

### Simple Truss Analysis
```rust
use gazelle::prelude::*;

let mut model = Model::new();

// Add nodes
model.add_node(Node::new(0, 0.0, 0.0, 0.0))?;
model.add_node(Node::new(1, 4000.0, 0.0, 0.0))?; // 4m span
model.add_node(Node::new(2, 2000.0, 3000.0, 0.0))?; // 3m height

// Add steel material
let steel = Material::steel(0, "S355".to_string());
model.add_material(steel)?;

// Add truss elements  
let props = ElementProperties::truss(2500.0); // 25cm² area
model.add_element(Element::new(0, ElementType::Truss2D, vec![0, 1], 0, props))?;
model.add_element(Element::new(1, ElementType::Truss2D, vec![1, 2], 0, props))?;
model.add_element(Element::new(2, ElementType::Truss2D, vec![2, 0], 0, props))?;

// Add constraints and loads
model.add_constraint(Constraint::fixed_support(0, 0))?;
model.add_constraint(Constraint::fixed_support(1, 1))?;
model.add_load(Load::nodal_force(0, 2, Dof::Uy, -50000.0, "Dead Load".to_string()))?;

// Analyze
let results = Analysis::new(model).static_analysis()?;
println!("Max displacement: {:.2} mm", results.max_displacement());
```

### Concrete Column Design
```rust  
use gazelle::prelude::*;

// Create type-safe concrete
let concrete = Concrete::try_create(
    CylinderStrength::Uk(UkConcreteGrade::Fck35),
    Aggregate::Limestone,
    Cement::ClassN, 
    WeightClass::NormalWeight,
    28.0
)?;

// Validate engineering properties
assert_eq!(concrete.fck().value(), 35.0); // MPa
assert!(concrete.ecm() > 30000.0); // MPa
assert_eq!(concrete.density(true), 2500.0); // kg/m³ reinforced

println!("✅ C35 Limestone Concrete validated for 28-day strength");
```

## Get Involved

See our guidance on [how to get involved](./CONTRIBUTING.md).

## 🤝 Get Involved

We believe in **community-driven development**. Here's how you can contribute:

- **🐛 [Report Issues](https://github.com/jsbayley/gazelle/issues)** - Help us identify bugs and improvements
- **💡 [Feature Requests](https://github.com/jsbayley/gazelle/discussions)** - Suggest new capabilities  
- **📖 [Documentation](./CONTRIBUTING.md#documentation)** - Help improve guides and examples
- **🧪 [Testing](./CONTRIBUTING.md#testing)** - Add test cases for edge conditions
- **🔌 [Plugins](./docs/plugins.md)** - Develop design code extensions

See our [Contributing Guide](./CONTRIBUTING.md) for detailed information.

---

## 🌟 Open Source Philosophy

<p align="justify">
Engineers accept phenomenal responsibility when dedicating their lives to improving our built environment. However, the vast majority of professional engineering software is, regrettably, <strong>closed source and proprietary</strong>. This is unfair and must change.
</p>

<p align="justify">
Engineers should be offered the <strong>respect and freedom</strong> to inspect, validate and influence the algorithms used to design our buildings and bridges. Structural engineering software should be <strong>transparent, auditable, and community-driven</strong>.
</p>

<p align="justify">
Gazelle is proudly <a href="./LICENSE" target="_blank"><strong>AGPL-3.0 licensed</strong></a> to ensure it remains free and open for the engineering community while preventing proprietary forks that would fragment the ecosystem.
</p>

### 🎯 **Our Mission**
> To accelerate AEC innovation, underpin academic research, and support Civil & Structural Engineering education through world-class open-source tools.

---

## ⚠️ Safety & Errata

<div align="center">
<strong>🏗️ SAFETY CRITICAL SOFTWARE 🏗️</strong>
</div>

<p align="justify">
We consider structural engineering software to be <strong>safety critical</strong>. Human lives depend on the accuracy of structural calculations. We strive to ensure stability, robustness and correctness throughout the source code, test suite and companion documentation.
</p>

<p align="justify">
Nevertheless, we are human and mistakes are possible. <strong>Always validate results</strong> against hand calculations, code requirements, and engineering judgment. Never rely solely on any software for critical design decisions.
</p>

### 🛡️ **Quality Assurance**
- ✅ **Comprehensive test suite** with analytical validation  
- ✅ **Property-based testing** for edge case coverage
- ✅ **Continuous integration** with automated checks
- ✅ **Type safety** prevents entire classes of errors
- ✅ **Open peer review** through GitHub

### 📢 **Report Issues**
Please submit error reports, suggestions for improvement, and safety concerns via [GitHub Issues](https://github.com/jsbayley/gazelle/issues). **Engineering safety is everyone's responsibility.**

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><em>Fast • Stable • Reliable • Transparent • Cross-platform • Extensible</em></p>
</div>
