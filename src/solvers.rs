//! Structural analysis solvers

use crate::core::{Model, AnalysisResults, AnalysisType, ConvergenceInfo, AnalysisSettings, SolverType};
use crate::error::{GazelleError, Result, Validate};
use crate::matrix::MatrixOps;
use crate::elements::ElementFactory;
use nalgebra::{DMatrix, DVector};
use log::{info, warn};
use std::time::Instant;

/// Trait for structural analysis solvers
pub trait Solver {
    /// Perform the analysis
    fn solve(&self, model: &Model) -> Result<AnalysisResults>;
    
    /// Validate the model before analysis
    fn validate_model(&self, model: &Model) -> Result<()>;
}

/// Static linear analysis solver
pub struct StaticSolver {
    pub settings: AnalysisSettings,
}

impl StaticSolver {
    pub fn new() -> Self {
        Self {
            settings: AnalysisSettings::default(),
        }
    }

    pub fn with_settings(settings: AnalysisSettings) -> Self {
        Self { settings }
    }

    /// Assemble global stiffness matrix
    fn assemble_stiffness_matrix(&self, model: &Model) -> Result<DMatrix<f64>> {
        // Calculate actual total DOFs based on active DOFs per element type
        let total_dofs = self.calculate_active_dofs(model);
        info!("Assembling global stiffness matrix ({} active DOFs)", total_dofs);
        
        let start_time = Instant::now();
        
        // Collect element stiffness matrices and DOF mappings
        let mut element_matrices = Vec::new();
        
        for element in model.elements.values() {
            // Get element nodes
            let element_nodes: Result<Vec<_>> = element.nodes.iter()
                .map(|&id| model.get_node(id).cloned())
                .collect();
            let nodes = element_nodes?;
            
            // Get element material
            let material = model.materials.get(&element.material_id)
                .ok_or(GazelleError::ValidationError(
                    format!("Material {} not found for element {}", element.material_id, element.id)
                ))?;
            
            // Compute element stiffness matrix
            let k_elem = ElementFactory::compute_stiffness_matrix(element, material, &nodes)?;
            
            // Map element DOFs to global DOFs
            let global_dofs = self.map_element_dofs_efficient(model, element, &nodes);
            
            element_matrices.push((global_dofs, k_elem));
        }
        
        // Assemble global stiffness matrix
        let global_k = MatrixOps::assemble_global_stiffness(&element_matrices, total_dofs);
        
        let assembly_time = start_time.elapsed();
        info!("Stiffness matrix assembled in {:?}", assembly_time);
        
        Ok(global_k)
    }

    /// Assemble global load vector
    fn assemble_load_vector(&self, model: &Model) -> Result<DVector<f64>> {
        let total_dofs = self.calculate_active_dofs(model);
        let mut global_f = DVector::zeros(total_dofs);
        
        info!("Assembling load vector for {} loads", model.loads.len());
        
        for load in &model.loads {
            // Apply load based on type
            match &load.load_type {
                crate::loads::LoadType::NodalForce { node_id, dof, magnitude } => {
                    let _node = model.get_node(*node_id)?;
                    let global_dof = self.map_nodal_dof(*node_id, *dof);
                    // println!("Applying load: Node {} {:?} -> DOF {} = {}", node_id, dof, global_dof, magnitude * load.factor);
                    if global_dof < total_dofs {
                        global_f[global_dof] += magnitude * load.factor;
                    }
                }
                crate::loads::LoadType::Distributed { element_id: _, direction: _, magnitude: _ } => {
                    // TODO: Implement distributed load assembly
                    warn!("Distributed loads not yet implemented");
                }
                crate::loads::LoadType::Gravity { acceleration: _ } => {
                    // Apply gravity to all elements
                    for element in model.elements.values() {
                        let material = model.materials.get(&element.material_id).unwrap();
                        if let Some(_density) = material.properties.density {
                            // TODO: Compute element gravity load vector
                            warn!("Gravity loads not yet fully implemented");
                        }
                    }
                }
                _ => {
                    warn!("Load type not yet implemented: {:?}", load.load_type);
                }
            }
        }
        
        Ok(global_f)
    }

    /// Apply constraints to the system
    fn apply_constraints(&self, model: &Model, k: &mut DMatrix<f64>, f: &mut DVector<f64>) -> Result<()> {
        let mut constrained_dofs = Vec::new();
        let mut prescribed_values = Vec::new();
        
        info!("Applying {} constraints", model.constraints.len());
        
        // Determine which DOFs are actually active in the model
        let active_dofs = self.get_active_dofs_for_constraints(model);
        
        // println!("Active DOFs in model: {:?}", active_dofs.iter().collect::<Vec<_>>());
        
        for constraint in &model.constraints {
            match &constraint.constraint_type {
                crate::constraints::ConstraintType::NodalConstraint { node_id, constraints } => {
                    for &(dof, value) in constraints {
                        // Skip DOFs that don't exist for this model type
                        if !self.is_dof_active_for_model(model, dof) {
                            // println!("Skipping constraint: {:?} (DOF not active for model type)", dof);
                            continue;
                        }
                        
                        let global_dof = self.map_nodal_dof(*node_id, dof);
                        
                        // Only apply constraint if this DOF is active in the model
                        if active_dofs.contains(&global_dof) && global_dof < k.nrows() {
                            // println!("Applying constraint: DOF {} ({:?}) = {}", global_dof, dof, value);
                            constrained_dofs.push(global_dof);
                            prescribed_values.push(value);
                        } else {
                            // println!("Skipping constraint: DOF {} ({:?}) (not active or out of range)", global_dof, dof);
                        }
                    }
                }
                _ => {
                    warn!("Complex constraint types not yet implemented");
                }
            }
        }
        
        MatrixOps::apply_constraints(k, f, &constrained_dofs, &prescribed_values)?;
        
        // Apply automatic constraints to uncoupled DOFs to prevent singularity
        self.apply_automatic_constraints(k, f)?;
        
        Ok(())
    }

    /// Get DOFs that are actually active (connected by elements)
    fn get_active_dofs_for_constraints(&self, model: &Model) -> std::collections::HashSet<usize> {
        let mut active_dofs = std::collections::HashSet::new();
        
        // Add DOFs used by elements
        for element in model.elements.values() {
            let element_nodes: Vec<_> = element.nodes.iter()
                .filter_map(|&id| model.nodes.get(&id))
                .cloned()
                .collect();
            
            let global_dofs = self.map_element_dofs_efficient(model, element, &element_nodes);
            // println!("Element DOFs: {:?}", global_dofs);
            for dof in global_dofs {
                active_dofs.insert(dof);
            }
        }
        
        active_dofs
    }

    /// Map element DOFs to global DOF indices
    /// Calculate total active DOFs based on model structure  
    /// This determines which DOFs are actually needed for the analysis
    fn calculate_active_dofs(&self, model: &Model) -> usize {
        // For 2D problems, use compact DOF numbering: only 2 DOFs per node
        // For 3D problems, use 3 or 6 DOFs per node as needed
        
        // Determine dimensionality from elements
        let mut is_2d = true;
        let mut max_dofs_per_node = 0;
        
        for element in model.elements.values() {
            let dofs_per_node = element.dofs_per_node();
            max_dofs_per_node = max_dofs_per_node.max(dofs_per_node);
            
            match element.element_type {
                crate::core::ElementType::Truss3D | crate::core::ElementType::Beam3D | crate::core::ElementType::Frame3D | 
                crate::core::ElementType::Shell | crate::core::ElementType::Solid => {
                    is_2d = false;
                }
                _ => {} // 2D elements
            }
        }
        
        // Use compact numbering for 2D problems
        if is_2d && max_dofs_per_node <= 3 {
            model.nodes.len() * max_dofs_per_node
        } else {
            // Use traditional 6-DOF numbering for 3D/complex problems
            let mut active_dofs = std::collections::HashSet::new();
            
            // Collect all DOFs used by elements
            for element in model.elements.values() {
                let dofs_per_node = element.dofs_per_node();
                for &node_id in &element.nodes {
                    let node_base = node_id * 6;
                    for local_dof in 0..dofs_per_node {
                        active_dofs.insert(node_base + local_dof);
                    }
                }
            }
            
            // Also include DOFs referenced by loads and constraints
            for load in &model.loads {
                if let crate::loads::LoadType::NodalForce { node_id, dof, .. } = &load.load_type {
                    let global_dof = self.map_nodal_dof_traditional(*node_id, *dof);
                    active_dofs.insert(global_dof);
                }
            }
            
            for constraint in &model.constraints {
                if let crate::constraints::ConstraintType::NodalConstraint { node_id, constraints } = &constraint.constraint_type {
                    for (dof, _) in constraints {
                        let global_dof = self.map_nodal_dof_traditional(*node_id, *dof);
                        active_dofs.insert(global_dof);
                    }
                }
            }
            
            let max_dof = active_dofs.iter().max().copied().unwrap_or(0);
            max_dof + 1
        }
    }

    /// More efficient DOF mapping that only uses active DOFs
    fn map_element_dofs_efficient(&self, model: &Model, element: &crate::core::Element, nodes: &[crate::core::Node]) -> Vec<usize> {
        let mut global_dofs = Vec::new();
        let dofs_per_node = element.dofs_per_node();
        
        // Determine if this is a 2D problem with compact numbering
        let use_compact = self.is_2d_problem(model);
        
        for node in nodes {
            if use_compact && dofs_per_node <= 3 {
                // Compact numbering: node_id * max_dofs_per_element + local_dof
                let max_dofs = self.max_dofs_per_node(model);
                let node_base_dof = node.id * max_dofs;
                for local_dof in 0..dofs_per_node {
                    global_dofs.push(node_base_dof + local_dof);
                }
            } else {
                // Traditional 6-DOF numbering
                let node_base_dof = node.id * 6;
                for local_dof in 0..dofs_per_node {
                    global_dofs.push(node_base_dof + local_dof);
                }
            }
        }
        
        global_dofs
    }

    /// Check if this is a 2D problem
    fn is_2d_problem(&self, model: &Model) -> bool {
        for element in model.elements.values() {
            match element.element_type {
                crate::core::ElementType::Truss3D | crate::core::ElementType::Beam3D | crate::core::ElementType::Frame3D | 
                crate::core::ElementType::Shell | crate::core::ElementType::Solid => {
                    return false;
                }
                _ => {} // 2D elements
            }
        }
        true
    }

    /// Get maximum DOFs per node for the model
    fn max_dofs_per_node(&self, model: &Model) -> usize {
        let mut max_dofs = 0;
        for element in model.elements.values() {
            max_dofs = max_dofs.max(element.dofs_per_node());
        }
        max_dofs
    }

    fn map_element_dofs(&self, element: &crate::core::Element, nodes: &[crate::core::Node]) -> Vec<usize> {
        let mut global_dofs = Vec::new();
        let dofs_per_node = element.dofs_per_node();
        
        for node in nodes {
            let node_base_dof = node.id * 6; // Assuming 6 DOF per node maximum
            for local_dof in 0..dofs_per_node {
                global_dofs.push(node_base_dof + local_dof);
            }
        }
        
        global_dofs
    }

    /// Map nodal DOF to global DOF index (traditional method)
    fn map_nodal_dof_traditional(&self, node_id: usize, dof: crate::core::Dof) -> usize {
        let node_base_dof = node_id * 6;
        let dof_offset = match dof {
            crate::core::Dof::Ux => 0,
            crate::core::Dof::Uy => 1,
            crate::core::Dof::Uz => 2,
            crate::core::Dof::Rx => 3,
            crate::core::Dof::Ry => 4,
            crate::core::Dof::Rz => 5,
        };
        node_base_dof + dof_offset
    }

    /// Map nodal DOF to global DOF index
    fn map_nodal_dof(&self, node_id: usize, dof: crate::core::Dof) -> usize {
        // Use compact mapping for 2D problems, traditional for 3D
        // This should match the mapping used in elements
        let dof_offset = match dof {
            crate::core::Dof::Ux => 0,
            crate::core::Dof::Uy => 1,
            crate::core::Dof::Uz => 2,
            crate::core::Dof::Rx => 3,
            crate::core::Dof::Ry => 4,
            crate::core::Dof::Rz => 5,
        };
        
        // For now, assume 2 DOFs per node for 2D problems
        // TODO: Make this more sophisticated based on element types in model
        node_id * 2 + dof_offset
    }

    /// Check if a DOF should be included for this model type
    fn is_dof_active_for_model(&self, model: &Model, dof: crate::core::Dof) -> bool {
        // For 2D problems, only include Ux and Uy
        let is_2d = self.is_2d_problem(model);
        let max_dofs = self.max_dofs_per_node(model);
        
        if is_2d && max_dofs <= 2 {
            matches!(dof, crate::core::Dof::Ux | crate::core::Dof::Uy)
        } else if is_2d && max_dofs <= 3 {
            matches!(dof, crate::core::Dof::Ux | crate::core::Dof::Uy | crate::core::Dof::Rz)
        } else {
            true // Include all DOFs for 3D problems
        }
    }

    /// Map nodal DOF to global DOF index with compact numbering for 2D problems
    fn map_nodal_dof_compact(&self, model: &Model, node_id: usize, dof: crate::core::Dof) -> usize {
        if self.is_2d_problem(model) {
            let max_dofs = self.max_dofs_per_node(model);
            let node_base_dof = node_id * max_dofs;
            let dof_offset = match dof {
                crate::core::Dof::Ux => 0,
                crate::core::Dof::Uy => 1,
                crate::core::Dof::Uz => 2,
                crate::core::Dof::Rx => 3,
                crate::core::Dof::Ry => 4,
                crate::core::Dof::Rz => 5,
            };
            node_base_dof + dof_offset.min(max_dofs - 1)
        } else {
            self.map_nodal_dof_traditional(node_id, dof)
        }
    }

    /// Apply automatic constraints to uncoupled DOFs to prevent matrix singularity
    fn apply_automatic_constraints(&self, k: &mut DMatrix<f64>, f: &mut DVector<f64>) -> Result<()> {
        println!("Checking for uncoupled DOFs...");
        
        let penalty_factor = 1e12;
        let mut auto_constraints = 0;
        
        for i in 0..k.nrows() {
            // Check if this DOF is uncoupled (no stiffness connections)
            let mut has_connection = false;
            
            // Check diagonal term
            if k[(i, i)].abs() > 1e-12 {
                has_connection = true;
            }
            
            // Check off-diagonal terms  
            if !has_connection {
                for j in 0..k.ncols() {
                    if i != j && (k[(i, j)].abs() > 1e-12 || k[(j, i)].abs() > 1e-12) {
                        has_connection = true;
                        break;
                    }
                }
            }
            
            // If DOF is uncoupled, apply constraint
            if !has_connection {
                println!("Applying automatic constraint to uncoupled DOF {}", i);
                k[(i, i)] = penalty_factor;
                f[i] = 0.0; // Constrain to zero displacement
                auto_constraints += 1;
            }
        }
        
        println!("Applied {} automatic constraints", auto_constraints);
        Ok(())
    }
}

impl Default for StaticSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver for StaticSolver {
    fn solve(&self, model: &Model) -> Result<AnalysisResults> {
        self.validate_model(model)?;
        
        info!("Starting static analysis");
        let analysis_start = Instant::now();
        
        // Assemble system matrices
        let mut k = self.assemble_stiffness_matrix(model)?;
        let mut f = self.assemble_load_vector(model)?;
        
        // Apply constraints
        self.apply_constraints(model, &mut k, &mut f)?;
        
        // Check matrix condition
        let condition_number = MatrixOps::condition_number(&k)?;
        if condition_number > 1e12 {
            warn!("Poor matrix conditioning detected: {:.2e}", condition_number);
        }
        
        // Solve system
        info!("Solving linear system");
        let solve_start = Instant::now();
        let displacements = match self.settings.solver_type {
            SolverType::Direct => MatrixOps::solve_linear_system(&k, &f)?,
            SolverType::Iterative => MatrixOps::solve_iterative(&k, &f)?,
            SolverType::Sparse => {
                warn!("Sparse solver not implemented, using direct solver");
                MatrixOps::solve_linear_system(&k, &f)?
            }
        };
        let solve_time = solve_start.elapsed();
        info!("System solved in {:?}", solve_time);
        
        // Compute reactions
        let reactions = &k * &displacements - &f;
        
        // Compute element forces
        // TODO: Implement element force recovery
        
        let total_time = analysis_start.elapsed();
        info!("Static analysis completed in {:?}", total_time);
        
        let mut results = AnalysisResults::new(
            displacements,
            reactions,
            AnalysisType::Static,
        );
        
        results.convergence_info = ConvergenceInfo {
            iterations: 1,
            residual_norm: MatrixOps::residual_norm(&k, &results.displacements, &f),
            converged: true,
            tolerance: self.settings.tolerance,
        };
        
        Ok(results)
    }

    fn validate_model(&self, model: &Model) -> Result<()> {
        info!("Validating structural model");
        
        // Basic model validation
        model.validate()?;
        
        // Check for sufficient constraints
        let _num_nodes = model.nodes.len();
        let num_constraints = model.constraints.iter()
            .map(|c| match &c.constraint_type {
                crate::constraints::ConstraintType::NodalConstraint { constraints, .. } => constraints.len(),
                _ => 0,
            })
            .sum::<usize>();
        
        if num_constraints == 0 {
            return Err(GazelleError::ValidationError(
                "Model has no constraints - structure is unstable".to_string()
            ));
        }
        
        // Check for loads
        if model.loads.is_empty() {
            warn!("Model has no applied loads");
        }
        
        info!("Model validation passed");
        Ok(())
    }
}

/// Modal analysis solver for eigenvalue problems
pub struct ModalSolver {
    pub settings: AnalysisSettings,
    pub num_modes: usize,
}

impl ModalSolver {
    pub fn new(num_modes: usize) -> Self {
        Self {
            settings: AnalysisSettings::default(),
            num_modes,
        }
    }

    pub fn with_settings(num_modes: usize, settings: AnalysisSettings) -> Self {
        Self { settings, num_modes }
    }

    /// Assemble mass matrix
    fn assemble_mass_matrix(&self, model: &Model) -> Result<DMatrix<f64>> {
        let total_dofs = model.total_dofs();
        info!("Assembling global mass matrix ({} DOFs)", total_dofs);
        
        let mut element_matrices = Vec::new();
        
        for element in model.elements.values() {
            let element_nodes: Result<Vec<_>> = element.nodes.iter()
                .map(|&id| model.get_node(id).cloned())
                .collect();
            let nodes = element_nodes?;
            
            let material = model.materials.get(&element.material_id)
                .ok_or(GazelleError::ValidationError(
                    format!("Material {} not found", element.material_id)
                ))?;
            
            let m_elem = ElementFactory::compute_mass_matrix(element, material, &nodes)?;
            let global_dofs = self.map_element_dofs(element, &nodes);
            
            element_matrices.push((global_dofs, m_elem));
        }
        
        let global_m = MatrixOps::assemble_global_stiffness(&element_matrices, total_dofs);
        Ok(global_m)
    }

    fn map_element_dofs(&self, element: &crate::core::Element, nodes: &[crate::core::Node]) -> Vec<usize> {
        let mut global_dofs = Vec::new();
        let dofs_per_node = element.dofs_per_node();
        
        for node in nodes {
            let node_base_dof = node.id * 6;
            for local_dof in 0..dofs_per_node {
                global_dofs.push(node_base_dof + local_dof);
            }
        }
        
        global_dofs
    }
}

impl Solver for ModalSolver {
    fn solve(&self, model: &Model) -> Result<AnalysisResults> {
        self.validate_model(model)?;
        
        info!("Starting modal analysis for {} modes", self.num_modes);
        
        // Assemble stiffness and mass matrices
        let static_solver = StaticSolver::with_settings(self.settings.clone());
        let k = static_solver.assemble_stiffness_matrix(model)?;
        let _m = self.assemble_mass_matrix(model)?;
        
        // Solve generalized eigenvalue problem: K φ = λ M φ
        info!("Solving eigenvalue problem");
        
        // For now, use standard eigensolve on M^-1 K (not ideal for large systems)
        let (eigenvalues, eigenvectors) = if k.nrows() < 1000 {
            MatrixOps::eigensolve_symmetric(&k)?
        } else {
            MatrixOps::eigensolve_subset(&k, self.num_modes, self.settings.tolerance, self.settings.max_iterations)?
        };
        
        // Natural frequencies are sqrt(eigenvalues) / (2π)
        let frequencies = eigenvalues.map(|lambda| {
            if lambda > 0.0 {
                lambda.sqrt() / (2.0 * std::f64::consts::PI)
            } else {
                0.0
            }
        });
        
        info!("Modal analysis completed");
        for (i, freq) in frequencies.iter().enumerate().take(self.num_modes.min(10)) {
            info!("Mode {}: {:.3} Hz", i + 1, freq);
        }
        
        // Store modal results in displacement field (frequencies) and eigenvectors in reactions
        let mut results = AnalysisResults::new(
            frequencies,
            DVector::zeros(eigenvectors.nrows()), // Placeholder
            AnalysisType::Modal,
        );
        
        results.convergence_info = ConvergenceInfo {
            iterations: 1,
            residual_norm: 0.0,
            converged: true,
            tolerance: self.settings.tolerance,
        };
        
        Ok(results)
    }

    fn validate_model(&self, model: &Model) -> Result<()> {
        info!("Validating model for modal analysis");
        
        // All materials must have density for modal analysis
        for material in model.materials.values() {
            if material.properties.density.is_none() {
                return Err(GazelleError::ValidationError(
                    format!("Material {} missing density for modal analysis", material.id)
                ));
            }
        }
        
        Ok(())
    }
}

/// Time history analysis solver
pub struct TimeHistorySolver {
    pub settings: AnalysisSettings,
    pub time_step: f64,
    pub duration: f64,
    pub damping_ratio: f64,
}

impl TimeHistorySolver {
    pub fn new(time_step: f64, duration: f64) -> Self {
        Self {
            settings: AnalysisSettings::default(),
            time_step,
            duration,
            damping_ratio: 0.05, // 5% damping
        }
    }

    pub fn with_damping(mut self, damping_ratio: f64) -> Self {
        self.damping_ratio = damping_ratio;
        self
    }
}

impl Solver for TimeHistorySolver {
    fn solve(&self, _model: &Model) -> Result<AnalysisResults> {
        info!("Starting time history analysis");
        info!("Time step: {} s, Duration: {} s", self.time_step, self.duration);
        
        // TODO: Implement Newmark-β or other time integration method
        Err(GazelleError::UnsupportedElement {
            element_type: "Time history analysis not yet implemented".to_string(),
        })
    }

    fn validate_model(&self, model: &Model) -> Result<()> {
        // Similar to modal analysis validation
        for material in model.materials.values() {
            if material.properties.density.is_none() {
                return Err(GazelleError::ValidationError(
                    format!("Material {} missing density for dynamic analysis", material.id)
                ));
            }
        }
        
        // Check for time-dependent loads
        let has_dynamic_loads = model.loads.iter().any(|load| {
            matches!(load.load_type, crate::loads::LoadType::Seismic { .. })
        });
        
        if !has_dynamic_loads {
            warn!("No time-dependent loads found for time history analysis");
        }
        
        Ok(())
    }
}

/// Analysis runner for coordinating different solver types
pub struct AnalysisRunner;

impl AnalysisRunner {
    /// Run analysis based on the specified type
    pub fn run_analysis(model: &Model, analysis_type: AnalysisType) -> Result<AnalysisResults> {
        match analysis_type {
            AnalysisType::Static => {
                let solver = StaticSolver::new();
                solver.solve(model)
            }
            AnalysisType::Modal => {
                let num_modes = model.analysis_settings.eigen_modes.unwrap_or(10);
                let solver = ModalSolver::new(num_modes);
                solver.solve(model)
            }
            AnalysisType::TimeHistory => {
                let time_step = model.analysis_settings.time_step.unwrap_or(0.01);
                let duration = model.analysis_settings.duration.unwrap_or(1.0);
                let solver = TimeHistorySolver::new(time_step, duration);
                solver.solve(model)
            }
            _ => Err(GazelleError::UnsupportedElement {
                element_type: format!("Analysis type {:?} not yet implemented", analysis_type),
            })
        }
    }

    /// Run multiple analysis types in sequence
    pub fn run_multiple_analyses(
        model: &Model,
        analyses: Vec<AnalysisType>,
    ) -> Result<Vec<AnalysisResults>> {
        let mut results = Vec::new();
        
        for analysis_type in analyses {
            info!("Running {:?} analysis", analysis_type);
            let result = Self::run_analysis(model, analysis_type)?;
            results.push(result);
        }
        
        Ok(results)
    }
}