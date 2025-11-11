//! High-level analysis interface and utilities

use crate::core::{Model, AnalysisResults, AnalysisType};
use crate::solvers::{AnalysisRunner, StaticSolver, ModalSolver, TimeHistorySolver, Solver};
use crate::error::{GazelleError, Result, Validate};
use log::info;
use std::time::Instant;

/// Main analysis interface for Gazelle
pub struct Analysis {
    pub model: Model,
}

impl Analysis {
    /// Create new analysis from model
    pub fn new(model: Model) -> Self {
        Self { model }
    }

    /// Perform static analysis
    pub fn static_analysis(&self) -> Result<AnalysisResults> {
        info!("Performing static linear analysis");
        let start_time = Instant::now();
        
        let solver = StaticSolver::new();
        let results = solver.solve(&self.model)?;
        
        let elapsed = start_time.elapsed();
        info!("Static analysis completed in {:?}", elapsed);
        
        Ok(results)
    }

    /// Perform modal analysis
    pub fn modal_analysis(&self, num_modes: usize) -> Result<AnalysisResults> {
        info!("Performing modal analysis for {} modes", num_modes);
        let start_time = Instant::now();
        
        let solver = ModalSolver::new(num_modes);
        let results = solver.solve(&self.model)?;
        
        let elapsed = start_time.elapsed();
        info!("Modal analysis completed in {:?}", elapsed);
        
        Ok(results)
    }

    /// Perform time history analysis
    pub fn time_history_analysis(&self, time_step: f64, duration: f64) -> Result<AnalysisResults> {
        info!("Performing time history analysis");
        let start_time = Instant::now();
        
        let solver = TimeHistorySolver::new(time_step, duration);
        let results = solver.solve(&self.model)?;
        
        let elapsed = start_time.elapsed();
        info!("Time history analysis completed in {:?}", elapsed);
        
        Ok(results)
    }

    /// Run analysis based on analysis type
    pub fn run(&self, analysis_type: AnalysisType) -> Result<AnalysisResults> {
        AnalysisRunner::run_analysis(&self.model, analysis_type)
    }

    /// Run multiple analysis types
    pub fn run_multiple(&self, analyses: Vec<AnalysisType>) -> Result<Vec<AnalysisResults>> {
        AnalysisRunner::run_multiple_analyses(&self.model, analyses)
    }

    /// Get model summary
    pub fn model_summary(&self) -> ModelSummary {
        ModelSummary::from_model(&self.model)
    }

    /// Validate model before analysis
    pub fn validate(&self) -> Result<()> {
        self.model.validate()
    }
}

/// Model summary information
#[derive(Debug, Clone)]
pub struct ModelSummary {
    pub num_nodes: usize,
    pub num_elements: usize,
    pub num_materials: usize,
    pub num_loads: usize,
    pub num_constraints: usize,
    pub total_dofs: usize,
    pub element_types: Vec<String>,
    pub material_types: Vec<String>,
}

impl ModelSummary {
    pub fn from_model(model: &Model) -> Self {
        let element_types: Vec<String> = model
            .elements
            .values()
            .map(|e| format!("{:?}", e.element_type))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let material_types: Vec<String> = model
            .materials
            .values()
            .map(|m| format!("{:?}", m.material_type))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        Self {
            num_nodes: model.nodes.len(),
            num_elements: model.elements.len(),
            num_materials: model.materials.len(),
            num_loads: model.loads.len(),
            num_constraints: model.constraints.len(),
            total_dofs: model.total_dofs(),
            element_types,
            material_types,
        }
    }

    /// Print model summary
    pub fn print(&self) {
        println!("Model Summary:");
        println!("  Nodes: {}", self.num_nodes);
        println!("  Elements: {}", self.num_elements);
        println!("  Materials: {}", self.num_materials);
        println!("  Loads: {}", self.num_loads);
        println!("  Constraints: {}", self.num_constraints);
        println!("  Total DOFs: {}", self.total_dofs);
        println!("  Element Types: {}", self.element_types.join(", "));
        println!("  Material Types: {}", self.material_types.join(", "));
    }
}

/// Analysis results post-processing
impl AnalysisResults {
    /// Get maximum displacement
    pub fn max_displacement(&self) -> f64 {
        self.displacements
            .iter()
            .map(|&d| d.abs())
            .fold(0.0, f64::max)
    }

    /// Get maximum reaction
    pub fn max_reaction(&self) -> f64 {
        self.reactions
            .iter()
            .map(|&r| r.abs())
            .fold(0.0, f64::max)
    }

    /// Get displacement at specific DOF
    pub fn displacement_at_dof(&self, dof: usize) -> Option<f64> {
        self.displacements.get(dof).copied()
    }

    /// Get reaction at specific DOF
    pub fn reaction_at_dof(&self, dof: usize) -> Option<f64> {
        self.reactions.get(dof).copied()
    }

    /// Print results summary
    pub fn print_summary(&self) {
        println!("Analysis Results Summary:");
        println!("  Analysis Type: {:?}", self.analysis_type);
        println!("  Converged: {}", self.convergence_info.converged);
        println!("  Iterations: {}", self.convergence_info.iterations);
        println!("  Residual Norm: {:.2e}", self.convergence_info.residual_norm);
        println!("  Max Displacement: {:.6e}", self.max_displacement());
        println!("  Max Reaction: {:.6e}", self.max_reaction());
        println!("  Strain Energy: {:.6e}", self.strain_energy);

        match self.analysis_type {
            AnalysisType::Modal => {
                println!("  Natural Frequencies (Hz):");
                for (i, freq) in self.displacements.iter().enumerate().take(10) {
                    println!("    Mode {}: {:.3}", i + 1, freq);
                }
            }
            _ => {}
        }
    }

    /// Export results to different formats
    pub fn export_to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| GazelleError::SerializationError(e))
    }

    pub fn export_to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| GazelleError::YamlError(e))
    }

    /// Save results to file
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let extension = std::path::Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("json");

        let content = match extension {
            "yaml" | "yml" => self.export_to_yaml()?,
            _ => self.export_to_json()?,
        };

        let mut file = File::create(filename)
            .map_err(|e| GazelleError::IoError(e))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| GazelleError::IoError(e))?;

        info!("Results saved to: {}", filename);
        Ok(())
    }
}

/// Parametric analysis utilities
pub struct ParametricAnalysis {
    base_model: Model,
}

impl ParametricAnalysis {
    pub fn new(base_model: Model) -> Self {
        Self { base_model }
    }

    /// Run parametric study varying material properties
    pub fn vary_material_property<F>(
        &self,
        material_id: usize,
        property_modifier: F,
        values: Vec<f64>,
    ) -> Result<Vec<AnalysisResults>>
    where
        F: Fn(&mut crate::materials::Material, f64),
    {
        let mut results = Vec::new();

        for value in values {
            let mut model = self.base_model.clone();
            
            if let Some(material) = model.materials.get_mut(&material_id) {
                property_modifier(material, value);
                
                let analysis = Analysis::new(model);
                let result = analysis.static_analysis()?;
                results.push(result);
            } else {
                return Err(GazelleError::ValidationError(
                    format!("Material {} not found", material_id)
                ));
            }
        }

        Ok(results)
    }

    /// Run parametric study varying geometry
    pub fn vary_node_coordinate<F>(
        &self,
        node_id: usize,
        coordinate_modifier: F,
        values: Vec<f64>,
    ) -> Result<Vec<AnalysisResults>>
    where
        F: Fn(&mut crate::core::Node, f64),
    {
        let mut results = Vec::new();

        for value in values {
            let mut model = self.base_model.clone();
            
            if let Some(node) = model.nodes.get_mut(&node_id) {
                coordinate_modifier(node, value);
                
                let analysis = Analysis::new(model);
                let result = analysis.static_analysis()?;
                results.push(result);
            } else {
                return Err(GazelleError::InvalidNodeId(node_id));
            }
        }

        Ok(results)
    }
}

/// Optimization utilities
pub struct Optimization;

impl Optimization {
    /// Simple gradient-free optimization using golden section search
    pub fn golden_section_search<F>(
        objective_function: F,
        a: f64,
        b: f64,
        tolerance: f64,
    ) -> Result<f64>
    where
        F: Fn(f64) -> Result<f64>,
    {
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;
        let resphi = 2.0 - phi;

        let mut x1 = a + resphi * (b - a);
        let mut x2 = a + (1.0 - resphi) * (b - a);
        let mut f1 = objective_function(x1)?;
        let mut f2 = objective_function(x2)?;

        let mut a = a;
        let mut b = b;

        while (b - a).abs() > tolerance {
            if f1 > f2 {
                a = x1;
                x1 = x2;
                f1 = f2;
                x2 = a + (1.0 - resphi) * (b - a);
                f2 = objective_function(x2)?;
            } else {
                b = x2;
                x2 = x1;
                f2 = f1;
                x1 = a + resphi * (b - a);
                f1 = objective_function(x1)?;
            }
        }

        Ok((a + b) / 2.0)
    }

    /// Find optimal parameter value that minimizes maximum displacement
    pub fn minimize_displacement(
        base_model: &Model,
        parameter_range: (f64, f64),
        parameter_modifier: impl Fn(&mut Model, f64) + Clone,
    ) -> Result<f64> {
        let objective = |param_value: f64| -> Result<f64> {
            let mut model = base_model.clone();
            parameter_modifier(&mut model, param_value);
            
            let analysis = Analysis::new(model);
            let results = analysis.static_analysis()?;
            Ok(results.max_displacement())
        };

        Self::golden_section_search(objective, parameter_range.0, parameter_range.1, 1e-6)
    }
}