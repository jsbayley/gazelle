//! Load and boundary condition definitions

use crate::core::Dof;
use crate::error::{GazelleError, Result, Validate};
use serde::{Deserialize, Serialize};
use nalgebra::Vector3;

/// Load applied to the structural model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Load {
    pub id: usize,
    pub load_type: LoadType,
    pub load_case: String,
    pub factor: f64,
}

impl Load {
    pub fn new(id: usize, load_type: LoadType, load_case: String) -> Self {
        Self {
            id,
            load_type,
            load_case,
            factor: 1.0,
        }
    }

    pub fn with_factor(mut self, factor: f64) -> Self {
        self.factor = factor;
        self
    }

    /// Create a nodal force load
    pub fn nodal_force(id: usize, node_id: usize, dof: Dof, magnitude: f64, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::NodalForce {
                node_id,
                dof,
                magnitude,
            },
            load_case,
        )
    }

    /// Create a distributed load on an element
    pub fn distributed(id: usize, element_id: usize, direction: Vector3<f64>, magnitude: f64, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::Distributed {
                element_id,
                direction,
                magnitude,
            },
            load_case,
        )
    }

    /// Create a pressure load
    pub fn pressure(id: usize, element_id: usize, magnitude: f64, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::Pressure {
                element_id,
                magnitude,
            },
            load_case,
        )
    }

    /// Create a thermal load
    pub fn thermal(id: usize, element_id: usize, temperature_change: f64, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::Thermal {
                element_id,
                temperature_change,
            },
            load_case,
        )
    }

    /// Create a gravity load
    pub fn gravity(id: usize, acceleration: Vector3<f64>, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::Gravity { acceleration },
            load_case,
        )
    }

    /// Create a seismic load
    pub fn seismic(id: usize, acceleration_history: Vec<Vector3<f64>>, time_step: f64, load_case: String) -> Self {
        Self::new(
            id,
            LoadType::Seismic {
                acceleration_history,
                time_step,
            },
            load_case,
        )
    }

    /// Apply the load factor to the load magnitude
    pub fn apply_factor(&self) -> LoadType {
        match &self.load_type {
            LoadType::NodalForce { node_id, dof, magnitude } => {
                LoadType::NodalForce {
                    node_id: *node_id,
                    dof: *dof,
                    magnitude: magnitude * self.factor,
                }
            }
            LoadType::Distributed { element_id, direction, magnitude } => {
                LoadType::Distributed {
                    element_id: *element_id,
                    direction: *direction,
                    magnitude: magnitude * self.factor,
                }
            }
            LoadType::Pressure { element_id, magnitude } => {
                LoadType::Pressure {
                    element_id: *element_id,
                    magnitude: magnitude * self.factor,
                }
            }
            LoadType::Thermal { element_id, temperature_change } => {
                LoadType::Thermal {
                    element_id: *element_id,
                    temperature_change: temperature_change * self.factor,
                }
            }
            LoadType::Gravity { acceleration } => {
                LoadType::Gravity {
                    acceleration: acceleration * self.factor,
                }
            }
            LoadType::Seismic { acceleration_history, time_step } => {
                LoadType::Seismic {
                    acceleration_history: acceleration_history.iter().map(|a| a * self.factor).collect(),
                    time_step: *time_step,
                }
            }
        }
    }
}

impl Validate for Load {
    fn validate(&self) -> Result<()> {
        match &self.load_type {
            LoadType::NodalForce { magnitude, .. } => {
                if magnitude.is_nan() || magnitude.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid magnitude", self.id)
                    ));
                }
            }
            LoadType::Distributed { magnitude, direction, .. } => {
                if magnitude.is_nan() || magnitude.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid magnitude", self.id)
                    ));
                }
                if direction.norm() == 0.0 {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has zero direction vector", self.id)
                    ));
                }
            }
            LoadType::Pressure { magnitude, .. } => {
                if magnitude.is_nan() || magnitude.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid pressure magnitude", self.id)
                    ));
                }
            }
            LoadType::Thermal { temperature_change, .. } => {
                if temperature_change.is_nan() || temperature_change.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid temperature change", self.id)
                    ));
                }
            }
            LoadType::Gravity { acceleration } => {
                if acceleration.iter().any(|&a| a.is_nan() || a.is_infinite()) {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid gravity acceleration", self.id)
                    ));
                }
            }
            LoadType::Seismic { acceleration_history, time_step } => {
                if *time_step <= 0.0 || time_step.is_nan() || time_step.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has invalid time step", self.id)
                    ));
                }
                if acceleration_history.is_empty() {
                    return Err(GazelleError::ValidationError(
                        format!("Load {} has empty acceleration history", self.id)
                    ));
                }
                for (i, acc) in acceleration_history.iter().enumerate() {
                    if acc.iter().any(|&a| a.is_nan() || a.is_infinite()) {
                        return Err(GazelleError::ValidationError(
                            format!("Load {} has invalid acceleration at time step {}", self.id, i)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Types of loads
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadType {
    /// Point force/moment at a node
    NodalForce {
        node_id: usize,
        dof: Dof,
        magnitude: f64,
    },
    /// Distributed load on an element
    Distributed {
        element_id: usize,
        direction: Vector3<f64>,
        magnitude: f64, // Force per unit length/area
    },
    /// Pressure load on an element surface
    Pressure {
        element_id: usize,
        magnitude: f64, // Pressure (force per unit area)
    },
    /// Thermal load due to temperature change
    Thermal {
        element_id: usize,
        temperature_change: f64,
    },
    /// Gravity load (body force)
    Gravity {
        acceleration: Vector3<f64>,
    },
    /// Seismic load (time-varying acceleration)
    Seismic {
        acceleration_history: Vec<Vector3<f64>>,
        time_step: f64,
    },
}

/// Load case definition for organizing loads
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadCase {
    pub name: String,
    pub description: String,
    pub loads: Vec<usize>, // Load IDs
    pub is_active: bool,
}

impl LoadCase {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            loads: Vec::new(),
            is_active: true,
        }
    }

    pub fn add_load(&mut self, load_id: usize) {
        if !self.loads.contains(&load_id) {
            self.loads.push(load_id);
        }
    }

    pub fn remove_load(&mut self, load_id: usize) {
        self.loads.retain(|&id| id != load_id);
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

/// Load combination for factored load analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadCombination {
    pub name: String,
    pub description: String,
    pub case_factors: Vec<(String, f64)>, // (load_case_name, factor)
}

impl LoadCombination {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            case_factors: Vec::new(),
        }
    }

    pub fn add_case(&mut self, case_name: String, factor: f64) {
        // Update existing or add new
        if let Some(existing) = self.case_factors.iter_mut().find(|(name, _)| name == &case_name) {
            existing.1 = factor;
        } else {
            self.case_factors.push((case_name, factor));
        }
    }

    pub fn remove_case(&mut self, case_name: &str) {
        self.case_factors.retain(|(name, _)| name != case_name);
    }

    /// Create standard LRFD combinations for building design
    pub fn lrfd_combinations() -> Vec<LoadCombination> {
        vec![
            // Basic combinations from ASCE 7
            LoadCombination {
                name: "1.4D".to_string(),
                description: "Dead load only".to_string(),
                case_factors: vec![("Dead".to_string(), 1.4)],
            },
            LoadCombination {
                name: "1.2D+1.6L".to_string(),
                description: "Dead and live loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.2),
                    ("Live".to_string(), 1.6),
                ],
            },
            LoadCombination {
                name: "1.2D+1.0W+1.0L".to_string(),
                description: "Dead, live, and wind loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.2),
                    ("Live".to_string(), 1.0),
                    ("Wind".to_string(), 1.0),
                ],
            },
            LoadCombination {
                name: "1.2D+1.0E+1.0L".to_string(),
                description: "Dead, live, and seismic loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.2),
                    ("Live".to_string(), 1.0),
                    ("Seismic".to_string(), 1.0),
                ],
            },
            LoadCombination {
                name: "0.9D+1.0W".to_string(),
                description: "Dead and wind loads (uplift check)".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 0.9),
                    ("Wind".to_string(), 1.0),
                ],
            },
            LoadCombination {
                name: "0.9D+1.0E".to_string(),
                description: "Dead and seismic loads (uplift check)".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 0.9),
                    ("Seismic".to_string(), 1.0),
                ],
            },
        ]
    }

    /// Create standard ASD combinations for building design
    pub fn asd_combinations() -> Vec<LoadCombination> {
        vec![
            LoadCombination {
                name: "D".to_string(),
                description: "Dead load only".to_string(),
                case_factors: vec![("Dead".to_string(), 1.0)],
            },
            LoadCombination {
                name: "D+L".to_string(),
                description: "Dead and live loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.0),
                    ("Live".to_string(), 1.0),
                ],
            },
            LoadCombination {
                name: "D+0.75L+0.75W".to_string(),
                description: "Dead, live, and wind loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.0),
                    ("Live".to_string(), 0.75),
                    ("Wind".to_string(), 0.75),
                ],
            },
            LoadCombination {
                name: "D+0.75L+0.75E".to_string(),
                description: "Dead, live, and seismic loads".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 1.0),
                    ("Live".to_string(), 0.75),
                    ("Seismic".to_string(), 0.75),
                ],
            },
            LoadCombination {
                name: "0.6D+W".to_string(),
                description: "Dead and wind loads (uplift check)".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 0.6),
                    ("Wind".to_string(), 1.0),
                ],
            },
            LoadCombination {
                name: "0.6D+E".to_string(),
                description: "Dead and seismic loads (uplift check)".to_string(),
                case_factors: vec![
                    ("Dead".to_string(), 0.6),
                    ("Seismic".to_string(), 1.0),
                ],
            },
        ]
    }
}