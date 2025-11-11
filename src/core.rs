//! Core data structures for structural analysis

use crate::error::{GazelleError, Result, Validate};
use crate::materials::Material;
use crate::loads::Load;
use crate::constraints::Constraint;
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a node in the structural model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub constraints: Vec<DofConstraint>,
}

impl Node {
    pub fn new(id: usize, x: f64, y: f64, z: f64) -> Self {
        Self {
            id,
            x,
            y,
            z,
            constraints: Vec::new(),
        }
    }

    pub fn position(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn add_constraint(&mut self, constraint: DofConstraint) {
        self.constraints.push(constraint);
    }
}

impl Validate for Node {
    fn validate(&self) -> Result<()> {
        if self.x.is_nan() || self.y.is_nan() || self.z.is_nan() {
            return Err(GazelleError::ValidationError(
                format!("Node {} has invalid coordinates", self.id)
            ));
        }
        Ok(())
    }
}

/// Degrees of freedom enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dof {
    Ux, // Translation in X
    Uy, // Translation in Y
    Uz, // Translation in Z
    Rx, // Rotation about X
    Ry, // Rotation about Y
    Rz, // Rotation about Z
}

impl Dof {
    pub fn all_translational() -> Vec<Dof> {
        vec![Dof::Ux, Dof::Uy, Dof::Uz]
    }

    pub fn all_rotational() -> Vec<Dof> {
        vec![Dof::Rx, Dof::Ry, Dof::Rz]
    }

    pub fn all() -> Vec<Dof> {
        vec![Dof::Ux, Dof::Uy, Dof::Uz, Dof::Rx, Dof::Ry, Dof::Rz]
    }
}

/// Constraint applied to a degree of freedom
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DofConstraint {
    pub dof: Dof,
    pub value: f64,
    pub is_prescribed: bool,
}

impl DofConstraint {
    pub fn fixed(dof: Dof) -> Self {
        Self {
            dof,
            value: 0.0,
            is_prescribed: true,
        }
    }

    pub fn prescribed(dof: Dof, value: f64) -> Self {
        Self {
            dof,
            value,
            is_prescribed: true,
        }
    }
}

/// Element connectivity and properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Element {
    pub id: usize,
    pub element_type: ElementType,
    pub nodes: Vec<usize>,
    pub material_id: usize,
    pub properties: ElementProperties,
}

impl Element {
    pub fn new(
        id: usize,
        element_type: ElementType,
        nodes: Vec<usize>,
        material_id: usize,
        properties: ElementProperties,
    ) -> Self {
        Self {
            id,
            element_type,
            nodes,
            material_id,
            properties,
        }
    }

    pub fn dofs_per_node(&self) -> usize {
        match self.element_type {
            ElementType::Truss2D => 2,
            ElementType::Truss3D => 3,
            ElementType::Beam2D => 3,
            ElementType::Beam3D => 6,
            ElementType::Frame2D => 3,
            ElementType::Frame3D => 6,
            ElementType::Plate => 3,
            ElementType::Shell => 6,
            ElementType::Solid => 3,
        }
    }

    pub fn total_dofs(&self) -> usize {
        self.nodes.len() * self.dofs_per_node()
    }
}

impl Validate for Element {
    fn validate(&self) -> Result<()> {
        if self.nodes.is_empty() {
            return Err(GazelleError::ValidationError(
                format!("Element {} has no nodes", self.id)
            ));
        }

        let expected_nodes = match self.element_type {
            ElementType::Truss2D | ElementType::Truss3D => 2,
            ElementType::Beam2D | ElementType::Beam3D => 2,
            ElementType::Frame2D | ElementType::Frame3D => 2,
            ElementType::Plate => 3, // Triangle, could be 4 for quad
            ElementType::Shell => 3, // Triangle, could be 4 for quad
            ElementType::Solid => 4, // Tetrahedron, could be 8 for hex
        };

        if self.nodes.len() != expected_nodes {
            return Err(GazelleError::ValidationError(
                format!(
                    "Element {} of type {:?} expects {} nodes, got {}",
                    self.id, self.element_type, expected_nodes, self.nodes.len()
                )
            ));
        }

        Ok(())
    }
}

/// Types of structural elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementType {
    Truss2D,
    Truss3D,
    Beam2D,
    Beam3D,
    Frame2D,
    Frame3D,
    Plate,
    Shell,
    Solid,
}

/// Element-specific properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementProperties {
    pub area: Option<f64>,              // Cross-sectional area
    pub inertia_y: Option<f64>,         // Moment of inertia about y-axis
    pub inertia_z: Option<f64>,         // Moment of inertia about z-axis
    pub torsional_constant: Option<f64>, // Torsional constant
    pub thickness: Option<f64>,         // Plate/shell thickness
    pub width: Option<f64>,             // Beam width
    pub height: Option<f64>,            // Beam height
}

impl ElementProperties {
    pub fn truss(area: f64) -> Self {
        Self {
            area: Some(area),
            inertia_y: None,
            inertia_z: None,
            torsional_constant: None,
            thickness: None,
            width: None,
            height: None,
        }
    }

    pub fn beam(area: f64, inertia_y: f64, inertia_z: f64, torsional_constant: f64) -> Self {
        Self {
            area: Some(area),
            inertia_y: Some(inertia_y),
            inertia_z: Some(inertia_z),
            torsional_constant: Some(torsional_constant),
            thickness: None,
            width: None,
            height: None,
        }
    }

    pub fn plate(thickness: f64) -> Self {
        Self {
            area: None,
            inertia_y: None,
            inertia_z: None,
            torsional_constant: None,
            thickness: Some(thickness),
            width: None,
            height: None,
        }
    }
}

/// Analysis results container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub displacements: DVector<f64>,
    pub reactions: DVector<f64>,
    pub element_forces: HashMap<usize, ElementForces>,
    pub strain_energy: f64,
    pub analysis_type: AnalysisType,
    pub convergence_info: ConvergenceInfo,
}

impl AnalysisResults {
    pub fn new(
        displacements: DVector<f64>,
        reactions: DVector<f64>,
        analysis_type: AnalysisType,
    ) -> Self {
        Self {
            displacements,
            reactions,
            element_forces: HashMap::new(),
            strain_energy: 0.0,
            analysis_type,
            convergence_info: ConvergenceInfo::default(),
        }
    }

    pub fn add_element_forces(&mut self, element_id: usize, forces: ElementForces) {
        self.element_forces.insert(element_id, forces);
    }
}

/// Element internal forces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementForces {
    pub axial: Option<f64>,
    pub shear_y: Option<f64>,
    pub shear_z: Option<f64>,
    pub moment_x: Option<f64>,
    pub moment_y: Option<f64>,
    pub moment_z: Option<f64>,
}

impl ElementForces {
    pub fn truss(axial: f64) -> Self {
        Self {
            axial: Some(axial),
            shear_y: None,
            shear_z: None,
            moment_x: None,
            moment_y: None,
            moment_z: None,
        }
    }

    pub fn beam_2d(axial: f64, shear: f64, moment: f64) -> Self {
        Self {
            axial: Some(axial),
            shear_y: Some(shear),
            shear_z: None,
            moment_x: None,
            moment_y: None,
            moment_z: Some(moment),
        }
    }
}

/// Types of analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisType {
    Static,
    Modal,
    TimeHistory,
    Buckling,
    NonlinearStatic,
}

/// Convergence information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvergenceInfo {
    pub iterations: usize,
    pub residual_norm: f64,
    pub converged: bool,
    pub tolerance: f64,
}

/// Main model container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub nodes: HashMap<usize, Node>,
    pub elements: HashMap<usize, Element>,
    pub materials: HashMap<usize, Material>,
    pub loads: Vec<Load>,
    pub constraints: Vec<Constraint>,
    pub analysis_settings: AnalysisSettings,
}

impl Model {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            elements: HashMap::new(),
            materials: HashMap::new(),
            loads: Vec::new(),
            constraints: Vec::new(),
            analysis_settings: AnalysisSettings::default(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<()> {
        node.validate()?;
        if self.nodes.contains_key(&node.id) {
            return Err(GazelleError::ValidationError(
                format!("Node with ID {} already exists", node.id)
            ));
        }
        self.nodes.insert(node.id, node);
        Ok(())
    }

    pub fn add_element(&mut self, element: Element) -> Result<()> {
        element.validate()?;
        
        // Check if all nodes exist
        for &node_id in &element.nodes {
            if !self.nodes.contains_key(&node_id) {
                return Err(GazelleError::InvalidNodeId(node_id));
            }
        }

        // Check if material exists
        if !self.materials.contains_key(&element.material_id) {
            return Err(GazelleError::ValidationError(
                format!("Material with ID {} does not exist", element.material_id)
            ));
        }

        if self.elements.contains_key(&element.id) {
            return Err(GazelleError::ValidationError(
                format!("Element with ID {} already exists", element.id)
            ));
        }

        self.elements.insert(element.id, element);
        Ok(())
    }

    pub fn add_material(&mut self, material: Material) -> Result<()> {
        material.validate()?;
        if self.materials.contains_key(&material.id) {
            return Err(GazelleError::ValidationError(
                format!("Material with ID {} already exists", material.id)
            ));
        }
        self.materials.insert(material.id, material);
        Ok(())
    }

    pub fn add_load(&mut self, load: Load) {
        self.loads.push(load);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn total_dofs(&self) -> usize {
        self.nodes.len() * 6 // Assuming 6 DOF per node (3D frame)
    }

    pub fn get_node(&self, id: usize) -> Result<&Node> {
        self.nodes.get(&id).ok_or(GazelleError::InvalidNodeId(id))
    }

    pub fn get_element(&self, id: usize) -> Result<&Element> {
        self.elements.get(&id).ok_or(GazelleError::InvalidElementId(id))
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Validate for Model {
    fn validate(&self) -> Result<()> {
        // Validate all nodes
        for node in self.nodes.values() {
            node.validate()?;
        }

        // Validate all elements
        for element in self.elements.values() {
            element.validate()?;
        }

        // Validate all materials
        for material in self.materials.values() {
            material.validate()?;
        }

        // Check if model has at least one node and element
        if self.nodes.is_empty() {
            return Err(GazelleError::ValidationError(
                "Model must have at least one node".to_string()
            ));
        }

        if self.elements.is_empty() {
            return Err(GazelleError::ValidationError(
                "Model must have at least one element".to_string()
            ));
        }

        Ok(())
    }
}

/// Analysis settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSettings {
    pub tolerance: f64,
    pub max_iterations: usize,
    pub solver_type: SolverType,
    pub eigen_modes: Option<usize>,
    pub time_step: Option<f64>,
    pub duration: Option<f64>,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            max_iterations: 100,
            solver_type: SolverType::Direct,
            eigen_modes: None,
            time_step: None,
            duration: None,
        }
    }
}

/// Solver types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SolverType {
    Direct,
    Iterative,
    Sparse,
}