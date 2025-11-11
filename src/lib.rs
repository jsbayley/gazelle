//! # Gazelle Structural Analysis Engine
//! 
//! Gazelle is a high-performance structural analysis engine written in Rust.
//! It provides finite element analysis capabilities for various structural 
//! engineering applications.
//! 
//! ## Features
//! 
//! - High-performance linear algebra operations
//! - Various structural element types (truss, beam, frame, plate, shell)
//! - Static, modal, and dynamic analysis capabilities
//! - Python bindings for easy integration
//! - Comprehensive testing suite
//! 
//! ## Example
//! 
//! ```rust
//! use gazelle::prelude::*;
//! use gazelle::analysis::Analysis;
//! 
//! // Create a simple truss model
//! let mut model = Model::new();
//! model.add_node(Node::new(0, 0.0, 0.0, 0.0)).unwrap();
//! model.add_node(Node::new(1, 1.0, 0.0, 0.0)).unwrap();
//! 
//! // Add material
//! let steel = Material::steel(0, "Steel".to_string());
//! model.add_material(steel).unwrap();
//! 
//! // Add element
//! let props = ElementProperties::truss(0.01);
//! model.add_element(Element::new(0, ElementType::Truss2D, vec![0, 1], 0, props)).unwrap();
//! 
//! // Add constraints and loads
//! model.add_constraint(Constraint::fixed_support(0, 0));
//! model.add_load(Load::nodal_force(0, 1, Dof::Ux, 1000.0, "Load".to_string()));
//! 
//! // Perform analysis
//! let analysis = Analysis::new(model);
//! let results = analysis.static_analysis().unwrap();
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

#[cfg(feature = "python")]
pub mod python;

pub use error::{GazelleError, Result};
pub use core::*;

// Re-export common types for convenience
pub use nalgebra as na;