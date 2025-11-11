# 🦌 Gazelle Documentation

> *Complete guide to the fast engine for structural engineering*

Welcome to Gazelle - **a fast, cross-platform structural analysis engine**. This documentation will guide you from installation to advanced usage, helping you leverage Gazelle's type-safe, daemon-based architecture for your structural analysis needs.

## 🎯 Mission Statement

Gazelle is an **open-source, community-led project** designed to:

- **🚀 Accelerate AEC innovation** through modern software architecture
- **📚 Underpin academic research** with transparent, auditable algorithms  
- **🎓 Support Structural Engineering education** with accessible tooling
- **🤝 Attract like-minded Engineers** to build the future together
- **🔌 Provide stable platform** for high-level language bindings
- **🔄 Propose modern interpretation** of structural engineering toolchain

---

## 🚀 Quick Navigation

- [**🔧 Installation**](#-installation) - Get up and running
- [**⚡ Quick Start**](#-quick-start) - Your first analysis in minutes  
- [**🏗️ Architecture**](#-architecture) - Understanding the daemon model
- [**🐍 Python Bindings**](#-python-bindings) - Using Gazelle from Python
- [**🔬 Type-Safe Engineering**](#-type-safe-engineering) - Units and safety
- [**🏢 Materials Library**](#-materials-library) - Real engineering materials
- [**📊 Analysis Types**](#-analysis-types) - Static, modal, and dynamic
- [**🔌 Extensibility**](#-extensibility) - Plugins and custom elements

---

## 🔧 Installation

### Prerequisites
- **Rust** 1.70+ (for building from source)
- **Python** 3.8+ (for Python bindings - optional)
- **Git** (for cloning the repository)

### From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/jsbayley/gazelle
cd gazelle

# Build with daemon support
cargo build --release --features daemon

# Install CLI globally
cargo install --path . --features daemon

# Verify installation
gazelle --version
```

### Development Setup
```bash
# Clone with dev container support
git clone https://github.com/jsbayley/gazelle
cd gazelle

# Open in VS Code with dev container (recommended)
code .

# Or build locally with all features
cargo build --features "daemon,python,benchmarks"
```

---

## ⚡ Quick Start

### 1. Start the Daemon
```bash
# Start Gazelle service (runs on localhost:3000 by default)
gazelle daemon start

# In another terminal, verify it's running
curl http://localhost:3000/status
```

### 2. Create Your First Model
```bash
# Create a simple truss model
gazelle create truss \
  --span 10.0 \
  --height 3.0 \
  --load 50.0 \
  --output my_truss.json

# Analyze it
gazelle analyze my_truss.json --format json
```

### 3. Use from Python (Coming Soon)
```python
import gazelle

# Connect to daemon
gz = gazelle.Gazelle("localhost:3000")

# Create and analyze a model
model = gz.create_truss(span=10.0, load=50.0)
results = gz.analyze(model, analysis_type="static")

print(f"Max displacement: {results.max_displacement:.2f} mm")
```

### 4. Library Usage
```rust
use gazelle::prelude::*;

// Type-safe model creation
let mut model = Model::new();
model.add_node(Node::new(0, 0.0, 0.0, 0.0))?;
model.add_node(Node::new(1, 4000.0, 0.0, 0.0))?; // 4m span

// Add steel material with type safety
let steel = Material::steel(0, "S355".to_string());
model.add_material(steel)?;

// Add truss element
let props = ElementProperties::truss(2500.0); // 25cm²
model.add_element(Element::new(
    0, ElementType::Truss2D, vec![0, 1], 0, props
))?;

// Add constraints and loads
model.add_constraint(Constraint::fixed_support(0, 0))?;
model.add_load(Load::nodal_force(0, 1, Dof::Ux, 10000.0, "Live".to_string()))?;

// Analyze with type-safe results
let results = Analysis::new(model).static_analysis()?;
```

---

## 🏗️ Architecture

Gazelle follows a **daemon architecture** inspired by Docker - a persistent service that multiple clients can connect to:

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│   Python Client    │    │    CLI Client       │    │    Web Interface    │
│   gz.analyze(...)   │    │  gz daemon start    │    │  http://localhost   │
└─────────┬───────────┘    └─────────┬───────────┘    └─────────┬───────────┘
          │                          │                          │
          └──────────────────────────┼──────────────────────────┘
                                     │ HTTP/gRPC API
                        ┌─────────────────────┐
                        │   Gazelle Daemon    │
                        │   localhost:3000    │
                        │   Session Manager   │
                        └─────────┬───────────┘
                                  │
                        ┌─────────────────────┐
                        │   Analysis Engine   │
                        │   Type-Safe Core    │
                        │   Rust Performance  │
                        └─────────────────────┘
```

### Key Benefits
- **🔄 Persistent Sessions** - Keep analysis state between operations
- **🌐 Multi-Client** - CLI, Python, web interfaces use same backend
- **🚀 High Performance** - Rust core with optimized linear algebra
- **🛡️ Type Safety** - Prevent unit mixing and engineering errors
- **📈 Scalable** - Easy to deploy on servers or cloud platforms

### REST API Endpoints
```bash
# Session management
POST   /sessions              # Create analysis session
GET    /sessions              # List active sessions  
GET    /sessions/{id}         # Get session details
DELETE /sessions/{id}         # Delete session

# Analysis operations
POST   /sessions/{id}/analyze # Run analysis
GET    /sessions/{id}/results # Get results

# System status
GET    /status                # Daemon status
GET    /health               # Health check
```

---

## 🔬 Type-Safe Engineering

One of Gazelle's key innovations is **type-safe units of measure**, preventing the kind of unit mixing disasters that have plagued engineering software.

### Units System
```rust
use gazelle::units::*;

// Type-safe quantities prevent mixing errors
let length = Length::new(4000.0);     // mm (millimeters)
let force = Force::new(50.0);         // kN (kilonewtons)
let age = Age::new(28.0);             // days

// Operations preserve units
let total_length = length + Length::new(2000.0); // ✅ Valid
// let invalid = length + force; // ❌ Compile error!
```

### Semantic Area Types
```rust
// Prevent mixing cross-sectional area with surface area
let cross_section = Area::cross_sectional(2500.0); // mm²
let surface = Area::surface(5000.0);               // mm²

// These are different types - prevents accidental mixing!
assert_ne!(cross_section, surface);
```

### Stress and Material Properties
```rust
// Type-safe stress calculations
let stress = Stress::new(25.0); // MPa
println!("Stress: {} N/mm²", stress.n_per_mm2());
println!("Stress: {} MPa", stress.mpa());
```

---

## 🏢 Materials Library

Gazelle includes a comprehensive materials library based on real engineering standards.

### Concrete Materials (Eurocode Compliant)
```rust
use gazelle::concrete::*;

// Create type-safe concrete
let concrete = Concrete::try_create(
    CylinderStrength::Uk(UkConcreteGrade::Fck30),
    Aggregate::Limestone,    // Affects elastic modulus
    Cement::ClassN,          // Affects time-dependent properties
    WeightClass::NormalWeight,
    28.0                     // Age in days (validated!)
)?;

// Access engineering properties
println!("fck: {} MPa", concrete.fck().value());        // 30.0
println!("fcm: {} MPa", concrete.fcm().value());        // 38.0  
println!("Ecm: {} MPa", concrete.ecm());                // 29,700 (limestone factor)
println!("Density: {} kg/m³", concrete.density(true));  // 2500 (reinforced)
```

### Available Concrete Grades
- **UK Grades**: Fck12 through Fck90
- **Aggregates**: Basalt, Limestone, Sandstone, Quartzite  
- **Cement Classes**: R, N, S (affect time-dependent properties)
- **Time-Dependent**: Automatic strength evolution calculations

### Steel Materials (Coming Soon)
```rust
// Planned steel materials
let steel = Steel::try_create(
    SteelGrade::S355,
    SteelType::HotRolled,
    Temperature::new(20.0) // °C
)?;
```

---

## 📊 Analysis Types

Gazelle supports multiple analysis types with a consistent API.

### Static Analysis
```rust
let results = Analysis::new(model).static_analysis()?;

// Access results
println!("Max displacement: {:.2} mm", results.max_displacement());
println!("Max stress: {:.2} MPa", results.max_stress());

// Get nodal displacements
for (node_id, displacement) in results.displacements() {
    println!("Node {}: dx={:.2}, dy={:.2}", 
             node_id, displacement.x, displacement.y);
}
```

### Modal Analysis
```rust
let results = Analysis::new(model).modal_analysis(10)?; // 10 modes

// Access modal properties
for (i, mode) in results.modes().iter().enumerate() {
    println!("Mode {}: f = {:.2} Hz, T = {:.2} s", 
             i + 1, mode.frequency, mode.period);
}
```

### Time History Analysis (Coming Soon)
```rust
let results = Analysis::new(model)
    .time_history_analysis(0.01, 10.0)?; // dt=0.01s, duration=10s
```

---

## 🐍 Python Bindings

Gazelle will provide first-class Python bindings for seamless integration with the Python ecosystem.

### Planned Python API
```python
import gazelle

# Connect to daemon
gz = gazelle.Gazelle("localhost:3000")

# Create models programmatically
model = gz.Model()
model.add_node(0, x=0, y=0, z=0)
model.add_node(1, x=4000, y=0, z=0)

# Add materials with validation
steel = gz.materials.Steel.S355()
model.add_material(steel)

# Add elements
truss = gz.elements.Truss2D(nodes=[0, 1], material=steel, area=2500.0)
model.add_element(truss)

# Add loads and constraints  
model.add_fixed_support(0)
model.add_load(1, fx=10000.0, load_case="Live")

# Analyze
results = gz.analyze(model, analysis_type="static")
print(f"Max displacement: {results.max_displacement:.2f} mm")
```

### Jupyter Notebook Integration
```python
# Planned visualization support
import gazelle
import matplotlib.pyplot as plt

gz = gazelle.Gazelle()
model = gz.examples.simple_truss()
results = gz.analyze(model)

# Plot deformed shape
gz.plot.deformed_shape(model, results, scale=100)
plt.show()
```

---

## 🔌 Extensibility  

Gazelle is designed for extensibility through plugins and custom elements.

### Plugin Architecture (Planned)
```rust
// Custom design code plugin
pub struct Eurocode2Plugin;

impl DesignCodePlugin for Eurocode2Plugin {
    fn design_concrete_beam(&self, geometry: &Geometry, loads: &Loads) -> DesignResult {
        // Eurocode 2 implementation
    }
}

// Register plugin
gazelle.register_plugin(Box::new(Eurocode2Plugin));
```

### Custom Element Types
```rust
// Implement custom element behavior
pub struct PlateElement {
    nodes: Vec<usize>,
    thickness: f64,
}

impl ElementBehavior for PlateElement {
    fn stiffness_matrix(&self, nodes: &[Node]) -> Matrix<f64> {
        // Plate stiffness implementation
    }
}
```

---

## 📚 Examples

### Simple Truss Analysis
Complete example of analyzing a Warren truss:

```rust
use gazelle::prelude::*;

fn warren_truss_example() -> Result<()> {
    let mut model = Model::new();
    
    // Add nodes for 5-panel Warren truss
    for i in 0..6 {
        let x = i as f64 * 2000.0; // 2m panels
        model.add_node(Node::new(i, x, 0.0, 0.0))?;
        model.add_node(Node::new(i + 6, x, 0.0, 0.0))?;
    }
    
    // Add apex nodes
    for i in 0..5 {
        let x = (i as f64 + 0.5) * 2000.0;
        let y = 2000.0; // 2m height
        model.add_node(Node::new(i + 12, x, y, 0.0))?;
    }
    
    // Add steel material
    let steel = Material::steel(0, "S355".to_string());
    model.add_material(steel)?;
    
    // Add truss elements
    let props = ElementProperties::truss(1600.0); // 16cm²
    
    // Bottom chord
    for i in 0..5 {
        model.add_element(Element::new(
            i, ElementType::Truss2D, vec![i, i + 1], 0, props.clone()
        ))?;
    }
    
    // Diagonals and verticals
    // ... (element creation continues)
    
    // Add supports
    model.add_constraint(Constraint::fixed_support(0, 0))?;
    model.add_constraint(Constraint::roller_support_y(1, 5))?;
    
    // Add loads (distributed as point loads)
    for i in 0..5 {
        model.add_load(Load::nodal_force(
            i, i + 12, Dof::Uy, -10000.0, "Dead + Live".to_string()
        ))?;
    }
    
    // Analyze
    let results = Analysis::new(model).static_analysis()?;
    
    println!("Warren Truss Analysis Results:");
    println!("Max displacement: {:.2} mm", results.max_displacement());
    println!("Max stress: {:.2} MPa", results.max_stress());
    
    Ok(())
}
```

### Concrete Column Design
```rust
use gazelle::prelude::*;
use gazelle::concrete::*;

fn concrete_column_example() -> Result<()> {
    // Create high-strength concrete
    let concrete = Concrete::try_create(
        CylinderStrength::Uk(UkConcreteGrade::Fck40),
        Aggregate::Basalt,      // High modulus aggregate
        Cement::ClassR,         // Rapid hardening
        WeightClass::NormalWeight,
        28.0
    )?;
    
    // Validate material properties
    assert_eq!(concrete.fck().value(), 40.0);
    assert!(concrete.ecm() > 40000.0); // Basalt gives higher modulus
    
    println!("✅ C40 Basalt Concrete Properties:");
    println!("   fck: {} MPa", concrete.fck().value());
    println!("   fcm: {} MPa", concrete.fcm().value());
    println!("   Ecm: {} MPa", concrete.ecm());
    println!("   Age-adjusted fcm: {} MPa", concrete.fcm_t().value());
    
    Ok(())
}
```

---

## 🛠️ Advanced Usage

### Custom Analysis Workflows
```rust
use gazelle::prelude::*;

fn parametric_study() -> Result<()> {
    // Study effect of span length on truss deflection
    let spans = [8.0, 10.0, 12.0, 14.0, 16.0]; // meters
    
    for span in spans {
        let model = create_truss_model(span * 1000.0)?; // Convert to mm
        let results = Analysis::new(model).static_analysis()?;
        
        println!("Span: {}m, Max deflection: {:.2}mm, Deflection ratio: L/{:.0}", 
                 span, 
                 results.max_displacement(),
                 (span * 1000.0) / results.max_displacement());
    }
    
    Ok(())
}
```

### Performance Optimization
```rust
// Enable parallel processing for large models
use rayon::prelude::*;

fn parallel_analysis(models: Vec<Model>) -> Vec<AnalysisResults> {
    models.into_par_iter()
        .map(|model| Analysis::new(model).static_analysis())
        .collect::<Result<Vec<_>>>()
        .unwrap()
}
```

---

## 🔍 Troubleshooting

### Common Issues

#### Singular Matrix Errors
```
Error: Singular matrix encountered
```
**Solution**: Check for:
- Unconstrained degrees of freedom
- Duplicate nodes at same location  
- Elements with zero stiffness
- Missing material properties

#### Unit Validation Errors  
```
Error: Invalid concrete age: Concrete age <= 0 days
```
**Solution**: Gazelle validates engineering inputs - ensure realistic values

#### Connection Issues
```
Error: Connection refused (localhost:3000)
```
**Solution**: Start the daemon first:
```bash
gazelle daemon start
```

---

## 📖 API Reference

For complete API documentation, run:
```bash
cargo doc --open --features daemon
```

Or visit the [online documentation](https://docs.rs/gazelle) (coming soon).

---

## 🤝 Contributing

We welcome contributions! See our [Contributing Guide](./CONTRIBUTING.md) for:
- Code style guidelines
- Testing requirements  
- Documentation standards
- Pull request process

---

## 📜 License

Gazelle is licensed under [AGPL-3.0](./LICENSE) to ensure it remains free and open source while preventing proprietary forks.

---

<div align="center">
  <p><strong>🦌 Built with ❤️ for the global engineering community</strong></p>
  <p><em>Fast • Stable • Reliable • Transparent • Cross-platform • Extensible</em></p>
</div>
