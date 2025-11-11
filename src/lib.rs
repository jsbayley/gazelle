//! # 🦌 Gazelle Structural Analysis Engine
//! 
//! > *A Fast Engine for Structural Engineering - Safe, Reliable, Extensible*
//!
//! Gazelle is a fast, cross-platform structural analysis engine written in Rust,
//! designed to be the foundation for modern engineering workflows.
//! 
//! ## 🎯 Key Features
//! 
//! - **🚀 High Performance** - Rust-native speed with optimized linear algebra
//! - **🛡️ Type Safety** - Units of measure prevent engineering disasters
//! - **🌐 Daemon Architecture** - HTTP/gRPC service for multi-language access
//! - **🔬 Real Materials** - Eurocode-compliant concrete with time-dependent properties
//! - **🧪 Rigorous Testing** - 13/13 tests passing with analytical validation
//! - **🐍 Python Ready** - First-class bindings (coming soon)
//! - **🔌 Extensible** - Plugin architecture for design codes
//! 
//! ## ⚡ Quick Start
//! 
//! ### Type-Safe Engineering
//! ```rust
//! use gazelle::prelude::*;
//! use gazelle::{Length, Force, Concrete, CylinderStrength, UkConcreteGrade, Aggregate, Cement, WeightClass};
//! 
//! // Type-safe units prevent disasters
//! let length = Length::new(4000.0);  // mm
//! let force = Force::new(50.0);      // kN
//! 
//! // Real engineering materials
//! let concrete = Concrete::try_create(
//!     CylinderStrength::Uk(UkConcreteGrade::Fck30),
//!     Aggregate::Limestone,
//!     Cement::ClassN,
//!     WeightClass::NormalWeight,
//!     28.0  // days - validated!
//! )?;
//! # Ok::<(), gazelle::GazelleError>(())
//! ```
//! 
//! ### Complete Analysis Example
//! ```rust
//! use gazelle::prelude::*;
//! use gazelle::analysis::Analysis;
//! 
//! // Create a simple truss model with type safety
//! let mut model = Model::new();
//! model.add_node(Node::new(0, 0.0, 0.0, 0.0))?;
//! model.add_node(Node::new(1, 4000.0, 0.0, 0.0))?; // 4m span
//! 
//! // Add steel material
//! let steel = Material::steel(0, "S355".to_string());
//! model.add_material(steel)?;
//! 
//! // Add truss element with cross-sectional area
//! let props = ElementProperties::truss(2500.0); // 25cm²
//! model.add_element(Element::new(0, ElementType::Truss2D, vec![0, 1], 0, props))?;
//! 
//! // Add boundary conditions and loads
//! model.add_constraint(Constraint::fixed_support(0, 0));
//! model.add_load(Load::nodal_force(0, 1, Dof::Ux, 50000.0, "Live Load".to_string()));
//! 
//! // Perform analysis with validated results
//! let results = Analysis::new(model).static_analysis()?;
//! println!("Max displacement: {:.2} mm", results.max_displacement());
//! # Ok::<(), gazelle::GazelleError>(())
//! ```
//! 
//! ## 🏗️ Daemon Architecture
//! 
//! ```rust,ignore
//! #[cfg(feature = "daemon")]
//! use gazelle::daemon::{GazelleDaemon, DaemonConfig};
//! 
//! #[cfg(feature = "daemon")]
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Start Gazelle as a service
//!     let config = DaemonConfig::default(); // localhost:3000
//!     let daemon = GazelleDaemon::new(config);
//!     
//!     // Now accessible via HTTP API from any language!
//!     // POST http://localhost:3000/sessions
//!     // POST http://localhost:3000/sessions/{id}/analyze
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## 🔬 Engineering Materials
//! 
//! ```rust
//! use gazelle::concrete::*;
//! 
//! // Eurocode 2 compliant concrete
//! let c35_limestone = Concrete::try_create(
//!     CylinderStrength::Uk(UkConcreteGrade::Fck35),
//!     Aggregate::Limestone,   // Affects elastic modulus
//!     Cement::ClassN,         // Affects time-dependent properties
//!     WeightClass::NormalWeight,
//!     28.0                    // Age validation
//! )?;
//! 
//! // Access engineering properties
//! assert_eq!(c35_limestone.fck().value(), 35.0); // MPa
//! assert_eq!(c35_limestone.fcm().value(), 43.0); // MPa  
//! assert!(c35_limestone.ecm() > 30_000.0);       // MPa (limestone adjusted)
//! # Ok::<(), gazelle::concrete::ConcreteError>(())
//! ```
//! 
//! ## 🌐 Multi-Language Ecosystem
//! 
//! Gazelle's daemon architecture enables usage from any programming language:
//! 
//! **Python (Coming Soon):**
//! ```python
//! import gazelle
//! gz = gazelle.Gazelle("localhost:3000")  
//! results = gz.analyze_truss(span=10.0, load=50.0)
//! ```
//! 
//! **CLI:**
//! ```bash
//! gazelle daemon start
//! gazelle analyze model.json --format vtk
//! ```
//! 
//! **REST API:**
//! ```bash
//! curl -X POST http://localhost:3000/sessions \
//!   -H "Content-Type: application/json" \
//!   -d '{"model": {...}}'
//! ```

pub mod prelude;
pub mod core;
pub mod elements;
pub mod materials;
pub mod loads;
pub mod constraints;
pub mod solvers;
pub mod analysis;
pub mod matrix;
pub mod error;
pub mod io;
pub mod units;
pub mod concrete;

#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(feature = "python")]
pub mod python;

pub use error::{GazelleError, Result};
pub use core::*;

// Re-export common types for convenience
pub use nalgebra as na;
pub use units::*;
pub use concrete::*;