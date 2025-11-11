//! Error types for Gazelle

use thiserror::Error;

/// Main error type for Gazelle operations
#[derive(Error, Debug)]
pub enum GazelleError {
    #[error("Matrix operation failed: {0}")]
    MatrixError(String),
    
    #[error("Invalid node ID: {0}")]
    InvalidNodeId(usize),
    
    #[error("Invalid element ID: {0}")]
    InvalidElementId(usize),
    
    #[error("Singular matrix encountered")]
    SingularMatrix,
    
    #[error("Analysis convergence failed after {iterations} iterations")]
    ConvergenceFailure { iterations: usize },
    
    #[error("Invalid material property: {property}")]
    InvalidMaterial { property: String },
    
    #[error("Unsupported element type: {element_type}")]
    UnsupportedElement { element_type: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("Python error: {0}")]
    #[cfg(feature = "python")]
    PythonError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
}

// Implement conversion from serde_json::Error to GazelleError
impl From<serde_json::Error> for GazelleError {
    fn from(err: serde_json::Error) -> Self {
        GazelleError::SerializationError(err.to_string())
    }
}

// Implement conversion from ConcreteError to GazelleError  
impl From<crate::concrete::ConcreteError> for GazelleError {
    fn from(err: crate::concrete::ConcreteError) -> Self {
        GazelleError::ValidationError(err.to_string())
    }
}

/// Result type for Gazelle operations
pub type Result<T> = std::result::Result<T, GazelleError>;

/// Validation trait for structural components
pub trait Validate {
    fn validate(&self) -> Result<()>;
}