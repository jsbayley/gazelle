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
//! 
//! // Create a simple truss model
//! let mut model = Model::new();
//! model.add_node(Node::new(0, 0.0, 0.0, 0.0));
//! model.add_node(Node::new(1, 1.0, 0.0, 0.0));
//! // ... add elements, constraints, loads
//! 
//! // Perform analysis
//! let results = model.static_analysis()?;
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