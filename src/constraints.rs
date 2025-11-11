//! Constraint and boundary condition definitions

use crate::core::Dof;
use crate::error::{GazelleError, Result, Validate};
use serde::{Deserialize, Serialize};

/// Constraint applied to the structural model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub id: usize,
    pub constraint_type: ConstraintType,
}

impl Constraint {
    pub fn new(id: usize, constraint_type: ConstraintType) -> Self {
        Self {
            id,
            constraint_type,
        }
    }

    /// Create a fixed support (all DOF constrained)
    pub fn fixed_support(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![
                    (Dof::Ux, 0.0),
                    (Dof::Uy, 0.0),
                    (Dof::Uz, 0.0),
                    (Dof::Rx, 0.0),
                    (Dof::Ry, 0.0),
                    (Dof::Rz, 0.0),
                ],
            },
        )
    }

    /// Create a pinned support (translations constrained, rotations free)
    pub fn pinned_support(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![
                    (Dof::Ux, 0.0),
                    (Dof::Uy, 0.0),
                    (Dof::Uz, 0.0),
                ],
            },
        )
    }

    /// Create a roller support (vertical translation and rotations constrained)
    pub fn roller_support_y(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![(Dof::Uy, 0.0)],
            },
        )
    }

    /// Create a roller support (horizontal translation constrained)
    pub fn roller_support_x(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![(Dof::Ux, 0.0)],
            },
        )
    }

    /// Create a roller support (axial translation constrained)
    pub fn roller_support_z(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![(Dof::Uz, 0.0)],
            },
        )
    }

    /// Create a prescribed displacement
    pub fn prescribed_displacement(id: usize, node_id: usize, dof: Dof, value: f64) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![(dof, value)],
            },
        )
    }

    /// Create symmetry constraints
    pub fn symmetry_x(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![
                    (Dof::Ux, 0.0),
                    (Dof::Ry, 0.0),
                    (Dof::Rz, 0.0),
                ],
            },
        )
    }

    pub fn symmetry_y(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![
                    (Dof::Uy, 0.0),
                    (Dof::Rx, 0.0),
                    (Dof::Rz, 0.0),
                ],
            },
        )
    }

    pub fn symmetry_z(id: usize, node_id: usize) -> Self {
        Self::new(
            id,
            ConstraintType::NodalConstraint {
                node_id,
                constraints: vec![
                    (Dof::Uz, 0.0),
                    (Dof::Rx, 0.0),
                    (Dof::Ry, 0.0),
                ],
            },
        )
    }

    /// Create a rigid link between nodes
    pub fn rigid_link(id: usize, master_node: usize, slave_nodes: Vec<usize>, linked_dofs: Vec<Dof>) -> Self {
        Self::new(
            id,
            ConstraintType::RigidLink {
                master_node,
                slave_nodes,
                linked_dofs,
            },
        )
    }

    /// Create equal displacement constraint between nodes
    pub fn equal_displacement(id: usize, nodes: Vec<usize>, dof: Dof) -> Self {
        Self::new(
            id,
            ConstraintType::EqualDisplacement { nodes, dof },
        )
    }

    /// Create a linear constraint (general multi-point constraint)
    pub fn linear_constraint(id: usize, terms: Vec<ConstraintTerm>, rhs: f64) -> Self {
        Self::new(
            id,
            ConstraintType::Linear { terms, rhs },
        )
    }
}

impl Validate for Constraint {
    fn validate(&self) -> Result<()> {
        match &self.constraint_type {
            ConstraintType::NodalConstraint { constraints, .. } => {
                if constraints.is_empty() {
                    return Err(GazelleError::ValidationError(
                        format!("Constraint {} has no DOF constraints", self.id)
                    ));
                }
                for &(_, value) in constraints {
                    if value.is_nan() || value.is_infinite() {
                        return Err(GazelleError::ValidationError(
                            format!("Constraint {} has invalid prescribed value", self.id)
                        ));
                    }
                }
            }
            ConstraintType::RigidLink { slave_nodes, .. } => {
                if slave_nodes.is_empty() {
                    return Err(GazelleError::ValidationError(
                        format!("Rigid link constraint {} has no slave nodes", self.id)
                    ));
                }
            }
            ConstraintType::EqualDisplacement { nodes, .. } => {
                if nodes.len() < 2 {
                    return Err(GazelleError::ValidationError(
                        format!("Equal displacement constraint {} needs at least 2 nodes", self.id)
                    ));
                }
            }
            ConstraintType::Linear { terms, rhs } => {
                if terms.is_empty() {
                    return Err(GazelleError::ValidationError(
                        format!("Linear constraint {} has no terms", self.id)
                    ));
                }
                if rhs.is_nan() || rhs.is_infinite() {
                    return Err(GazelleError::ValidationError(
                        format!("Linear constraint {} has invalid RHS value", self.id)
                    ));
                }
                for term in terms {
                    if term.coefficient.is_nan() || term.coefficient.is_infinite() {
                        return Err(GazelleError::ValidationError(
                            format!("Linear constraint {} has invalid coefficient", self.id)
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Types of constraints
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintType {
    /// Direct constraints on nodal DOF
    NodalConstraint {
        node_id: usize,
        constraints: Vec<(Dof, f64)>, // (DOF, prescribed_value)
    },
    /// Rigid link between master and slave nodes
    RigidLink {
        master_node: usize,
        slave_nodes: Vec<usize>,
        linked_dofs: Vec<Dof>,
    },
    /// Equal displacement between multiple nodes
    EqualDisplacement {
        nodes: Vec<usize>,
        dof: Dof,
    },
    /// General linear constraint: sum(coefficient * DOF) = rhs
    Linear {
        terms: Vec<ConstraintTerm>,
        rhs: f64,
    },
}

/// Term in a linear constraint equation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintTerm {
    pub node_id: usize,
    pub dof: Dof,
    pub coefficient: f64,
}

impl ConstraintTerm {
    pub fn new(node_id: usize, dof: Dof, coefficient: f64) -> Self {
        Self {
            node_id,
            dof,
            coefficient,
        }
    }
}

/// Boundary condition sets for common structural problems
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundaryConditionSet {
    pub name: String,
    pub description: String,
    pub constraints: Vec<Constraint>,
}

impl BoundaryConditionSet {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Create standard boundary conditions for a cantilever beam
    pub fn cantilever_beam(node_id: usize) -> Self {
        let mut bc_set = Self::new(
            "Cantilever Beam".to_string(),
            "Fixed support at one end".to_string(),
        );
        bc_set.add_constraint(Constraint::fixed_support(0, node_id));
        bc_set
    }

    /// Create standard boundary conditions for a simply supported beam
    pub fn simply_supported_beam(left_node: usize, right_node: usize) -> Self {
        let mut bc_set = Self::new(
            "Simply Supported Beam".to_string(),
            "Pinned supports at both ends".to_string(),
        );
        bc_set.add_constraint(Constraint::pinned_support(0, left_node));
        bc_set.add_constraint(Constraint::roller_support_y(1, right_node));
        bc_set
    }

    /// Create standard boundary conditions for a fixed-fixed beam
    pub fn fixed_fixed_beam(left_node: usize, right_node: usize) -> Self {
        let mut bc_set = Self::new(
            "Fixed-Fixed Beam".to_string(),
            "Fixed supports at both ends".to_string(),
        );
        bc_set.add_constraint(Constraint::fixed_support(0, left_node));
        bc_set.add_constraint(Constraint::fixed_support(1, right_node));
        bc_set
    }

    /// Create standard boundary conditions for a building base
    pub fn building_base(node_ids: Vec<usize>) -> Self {
        let mut bc_set = Self::new(
            "Building Base".to_string(),
            "Fixed supports at foundation level".to_string(),
        );
        for (i, &node_id) in node_ids.iter().enumerate() {
            bc_set.add_constraint(Constraint::fixed_support(i, node_id));
        }
        bc_set
    }

    /// Create symmetry boundary conditions
    pub fn symmetry_plane_x(node_ids: Vec<usize>) -> Self {
        let mut bc_set = Self::new(
            "Symmetry Plane X".to_string(),
            "Symmetry boundary conditions for YZ plane".to_string(),
        );
        for (i, &node_id) in node_ids.iter().enumerate() {
            bc_set.add_constraint(Constraint::symmetry_x(i, node_id));
        }
        bc_set
    }

    pub fn symmetry_plane_y(node_ids: Vec<usize>) -> Self {
        let mut bc_set = Self::new(
            "Symmetry Plane Y".to_string(),
            "Symmetry boundary conditions for XZ plane".to_string(),
        );
        for (i, &node_id) in node_ids.iter().enumerate() {
            bc_set.add_constraint(Constraint::symmetry_y(i, node_id));
        }
        bc_set
    }

    pub fn symmetry_plane_z(node_ids: Vec<usize>) -> Self {
        let mut bc_set = Self::new(
            "Symmetry Plane Z".to_string(),
            "Symmetry boundary conditions for XY plane".to_string(),
        );
        for (i, &node_id) in node_ids.iter().enumerate() {
            bc_set.add_constraint(Constraint::symmetry_z(i, node_id));
        }
        bc_set
    }
}

/// Support conditions for common structural elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportType {
    Fixed,
    Pinned,
    RollerX,
    RollerY,
    RollerZ,
    Free,
    Guided, // Can translate in one direction only
}

impl SupportType {
    /// Get the constrained DOF for this support type
    pub fn constrained_dofs(&self) -> Vec<Dof> {
        match self {
            SupportType::Fixed => Dof::all(),
            SupportType::Pinned => Dof::all_translational(),
            SupportType::RollerX => vec![Dof::Uy, Dof::Uz],
            SupportType::RollerY => vec![Dof::Ux, Dof::Uz],
            SupportType::RollerZ => vec![Dof::Ux, Dof::Uy],
            SupportType::Free => vec![],
            SupportType::Guided => vec![], // Depends on specific direction
        }
    }

    /// Create constraint for this support type at a node
    pub fn create_constraint(&self, constraint_id: usize, node_id: usize) -> Constraint {
        match self {
            SupportType::Fixed => Constraint::fixed_support(constraint_id, node_id),
            SupportType::Pinned => Constraint::pinned_support(constraint_id, node_id),
            SupportType::RollerX => Constraint::roller_support_x(constraint_id, node_id),
            SupportType::RollerY => Constraint::roller_support_y(constraint_id, node_id),
            SupportType::RollerZ => Constraint::roller_support_z(constraint_id, node_id),
            SupportType::Free => {
                // No constraints - create empty constraint
                Constraint::new(
                    constraint_id,
                    ConstraintType::NodalConstraint {
                        node_id,
                        constraints: vec![],
                    },
                )
            }
            SupportType::Guided => {
                // Default to roller in Y direction
                Constraint::roller_support_y(constraint_id, node_id)
            }
        }
    }
}