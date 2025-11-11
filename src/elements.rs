//! Structural element implementations with stiffness matrices

use crate::core::{Element, ElementType, Node};
use crate::materials::Material;
use crate::error::{GazelleError, Result};
use nalgebra::{DMatrix, Vector3, Matrix3};

/// Trait for element stiffness matrix computation
pub trait ElementStiffness {
    /// Compute element stiffness matrix in local coordinates
    fn local_stiffness_matrix(&self, element: &Element, material: &Material) -> Result<DMatrix<f64>>;
    
    /// Compute transformation matrix from local to global coordinates
    fn transformation_matrix(&self, nodes: &[Node]) -> Result<DMatrix<f64>>;
    
    /// Compute element stiffness matrix in global coordinates
    fn global_stiffness_matrix(
        &self,
        element: &Element,
        material: &Material,
        nodes: &[Node],
    ) -> Result<DMatrix<f64>> {
        let k_local = self.local_stiffness_matrix(element, material)?;
        let t = self.transformation_matrix(nodes)?;
        Ok(&t.transpose() * k_local * &t)
    }

    /// Compute element mass matrix
    fn mass_matrix(&self, element: &Element, material: &Material, nodes: &[Node]) -> Result<DMatrix<f64>>;
}

/// 2D Truss element implementation
pub struct Truss2D;

impl ElementStiffness for Truss2D {
    fn local_stiffness_matrix(&self, element: &Element, material: &Material) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required for truss element".to_string(),
        })?;
        
        let e = material.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus required".to_string(),
        })?;

        // 2D truss element has 4 DOF (2 nodes × 2 DOF per node)
        let mut k = DMatrix::zeros(4, 4);
        let ea = e * area;

        // Local stiffness matrix for 2D truss
        k[(0, 0)] = ea;  k[(0, 2)] = -ea;
        k[(2, 0)] = -ea; k[(2, 2)] = ea;

        Ok(k)
    }

    fn transformation_matrix(&self, nodes: &[Node]) -> Result<DMatrix<f64>> {
        if nodes.len() != 2 {
            return Err(GazelleError::ValidationError(
                "Truss2D element requires exactly 2 nodes".to_string()
            ));
        }

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let length = (dx * dx + dy * dy).sqrt();

        if length < 1e-12 {
            return Err(GazelleError::ValidationError(
                "Truss element has zero length".to_string()
            ));
        }

        let cos_theta = dx / length;
        let sin_theta = dy / length;

        let mut t = DMatrix::zeros(4, 4);
        t[(0, 0)] = cos_theta;  t[(0, 1)] = sin_theta;
        t[(1, 0)] = -sin_theta; t[(1, 1)] = cos_theta;
        t[(2, 2)] = cos_theta;  t[(2, 3)] = sin_theta;
        t[(3, 2)] = -sin_theta; t[(3, 3)] = cos_theta;

        Ok(t)
    }

    fn mass_matrix(&self, element: &Element, material: &Material, nodes: &[Node]) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let density = material.properties.density.ok_or(GazelleError::InvalidMaterial {
            property: "Material density required".to_string(),
        })?;

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let length = (dx * dx + dy * dy).sqrt();

        let mass = density * area * length;
        let mut m = DMatrix::zeros(4, 4);
        
        // Consistent mass matrix
        m[(0, 0)] = mass / 3.0;  m[(0, 2)] = mass / 6.0;
        m[(1, 1)] = mass / 3.0;  m[(1, 3)] = mass / 6.0;
        m[(2, 0)] = mass / 6.0;  m[(2, 2)] = mass / 3.0;
        m[(3, 1)] = mass / 6.0;  m[(3, 3)] = mass / 3.0;

        Ok(m)
    }
}

/// 3D Truss element implementation
pub struct Truss3D;

impl ElementStiffness for Truss3D {
    fn local_stiffness_matrix(&self, element: &Element, material: &Material) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required for truss element".to_string(),
        })?;
        
        let e = material.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus required".to_string(),
        })?;

        // 3D truss element has 6 DOF (2 nodes × 3 DOF per node)
        let mut k = DMatrix::zeros(6, 6);
        let ea = e * area;

        // Local stiffness matrix for 3D truss (axial direction only)
        k[(0, 0)] = ea;  k[(0, 3)] = -ea;
        k[(3, 0)] = -ea; k[(3, 3)] = ea;

        Ok(k)
    }

    fn transformation_matrix(&self, nodes: &[Node]) -> Result<DMatrix<f64>> {
        if nodes.len() != 2 {
            return Err(GazelleError::ValidationError(
                "Truss3D element requires exactly 2 nodes".to_string()
            ));
        }

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let dz = node2.z - node1.z;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();

        if length < 1e-12 {
            return Err(GazelleError::ValidationError(
                "Truss element has zero length".to_string()
            ));
        }

        let l = dx / length;
        let m = dy / length;
        let n = dz / length;

        let mut t = DMatrix::zeros(6, 6);
        
        // Direction cosines
        t[(0, 0)] = l; t[(0, 1)] = m; t[(0, 2)] = n;
        t[(3, 3)] = l; t[(3, 4)] = m; t[(3, 5)] = n;

        Ok(t)
    }

    fn mass_matrix(&self, element: &Element, material: &Material, nodes: &[Node]) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let density = material.properties.density.ok_or(GazelleError::InvalidMaterial {
            property: "Material density required".to_string(),
        })?;

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let dz = node2.z - node1.z;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();

        let mass = density * area * length;
        let mut m = DMatrix::zeros(6, 6);
        
        // Consistent mass matrix (diagonal terms)
        for i in 0..3 {
            m[(i, i)] = mass / 3.0;
            m[(i + 3, i + 3)] = mass / 3.0;
            m[(i, i + 3)] = mass / 6.0;
            m[(i + 3, i)] = mass / 6.0;
        }

        Ok(m)
    }
}

/// 2D Beam element (Euler-Bernoulli beam theory)
pub struct Beam2D;

impl ElementStiffness for Beam2D {
    fn local_stiffness_matrix(&self, element: &Element, material: &Material) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let inertia = element.properties.inertia_z.ok_or(GazelleError::InvalidMaterial {
            property: "Moment of inertia required".to_string(),
        })?;
        
        let e = material.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus required".to_string(),
        })?;

        // Get length from nodes (this should be passed differently in a real implementation)
        // For now, assume unit length - this needs to be fixed
        let length = 1.0; // This is a placeholder
        
        let ea = e * area;
        let ei = e * inertia;
        let l = length;
        let l2 = l * l;
        let l3 = l2 * l;

        // 2D beam element has 6 DOF (2 nodes × 3 DOF per node: u, v, θ)
        let mut k = DMatrix::zeros(6, 6);

        // Axial terms
        k[(0, 0)] = ea / l;     k[(0, 3)] = -ea / l;
        k[(3, 0)] = -ea / l;    k[(3, 3)] = ea / l;

        // Bending terms
        k[(1, 1)] = 12.0 * ei / l3;   k[(1, 2)] = 6.0 * ei / l2;   k[(1, 4)] = -12.0 * ei / l3;  k[(1, 5)] = 6.0 * ei / l2;
        k[(2, 1)] = 6.0 * ei / l2;    k[(2, 2)] = 4.0 * ei / l;    k[(2, 4)] = -6.0 * ei / l2;   k[(2, 5)] = 2.0 * ei / l;
        k[(4, 1)] = -12.0 * ei / l3;  k[(4, 2)] = -6.0 * ei / l2;  k[(4, 4)] = 12.0 * ei / l3;   k[(4, 5)] = -6.0 * ei / l2;
        k[(5, 1)] = 6.0 * ei / l2;    k[(5, 2)] = 2.0 * ei / l;    k[(5, 4)] = -6.0 * ei / l2;   k[(5, 5)] = 4.0 * ei / l;

        Ok(k)
    }

    fn transformation_matrix(&self, nodes: &[Node]) -> Result<DMatrix<f64>> {
        if nodes.len() != 2 {
            return Err(GazelleError::ValidationError(
                "Beam2D element requires exactly 2 nodes".to_string()
            ));
        }

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let length = (dx * dx + dy * dy).sqrt();

        if length < 1e-12 {
            return Err(GazelleError::ValidationError(
                "Beam element has zero length".to_string()
            ));
        }

        let cos_theta = dx / length;
        let sin_theta = dy / length;

        let mut t = DMatrix::zeros(6, 6);
        
        // Node 1
        t[(0, 0)] = cos_theta;  t[(0, 1)] = sin_theta;
        t[(1, 0)] = -sin_theta; t[(1, 1)] = cos_theta;
        t[(2, 2)] = 1.0;
        
        // Node 2
        t[(3, 3)] = cos_theta;  t[(3, 4)] = sin_theta;
        t[(4, 3)] = -sin_theta; t[(4, 4)] = cos_theta;
        t[(5, 5)] = 1.0;

        Ok(t)
    }

    fn mass_matrix(&self, element: &Element, material: &Material, nodes: &[Node]) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let density = material.properties.density.ok_or(GazelleError::InvalidMaterial {
            property: "Material density required".to_string(),
        })?;

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let length = (dx * dx + dy * dy).sqrt();

        let mass_per_length = density * area;
        let total_mass = mass_per_length * length;
        let l = length;

        let mut m = DMatrix::zeros(6, 6);

        // Consistent mass matrix for 2D beam
        // Axial terms
        m[(0, 0)] = total_mass / 3.0;
        m[(0, 3)] = total_mass / 6.0;
        m[(3, 0)] = total_mass / 6.0;
        m[(3, 3)] = total_mass / 3.0;

        // Transverse terms
        m[(1, 1)] = 13.0 * total_mass / 35.0;
        m[(1, 2)] = 11.0 * total_mass * l / 210.0;
        m[(1, 4)] = 9.0 * total_mass / 70.0;
        m[(1, 5)] = -13.0 * total_mass * l / 420.0;

        m[(2, 1)] = 11.0 * total_mass * l / 210.0;
        m[(2, 2)] = total_mass * l * l / 105.0;
        m[(2, 4)] = 13.0 * total_mass * l / 420.0;
        m[(2, 5)] = -total_mass * l * l / 140.0;

        m[(4, 1)] = 9.0 * total_mass / 70.0;
        m[(4, 2)] = 13.0 * total_mass * l / 420.0;
        m[(4, 4)] = 13.0 * total_mass / 35.0;
        m[(4, 5)] = -11.0 * total_mass * l / 210.0;

        m[(5, 1)] = -13.0 * total_mass * l / 420.0;
        m[(5, 2)] = -total_mass * l * l / 140.0;
        m[(5, 4)] = -11.0 * total_mass * l / 210.0;
        m[(5, 5)] = total_mass * l * l / 105.0;

        Ok(m)
    }
}

/// 3D Frame element (space frame with 6 DOF per node)
pub struct Frame3D;

impl ElementStiffness for Frame3D {
    fn local_stiffness_matrix(&self, element: &Element, material: &Material) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let iy = element.properties.inertia_y.ok_or(GazelleError::InvalidMaterial {
            property: "Moment of inertia about y-axis required".to_string(),
        })?;
        
        let iz = element.properties.inertia_z.ok_or(GazelleError::InvalidMaterial {
            property: "Moment of inertia about z-axis required".to_string(),
        })?;
        
        let j = element.properties.torsional_constant.ok_or(GazelleError::InvalidMaterial {
            property: "Torsional constant required".to_string(),
        })?;
        
        let e = material.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus required".to_string(),
        })?;
        
        let g = material.shear_modulus()?;

        // Placeholder length - this should be computed from nodes
        let length = 1.0;
        let l = length;
        let l2 = l * l;
        let l3 = l2 * l;

        // 3D frame element has 12 DOF (2 nodes × 6 DOF per node)
        let mut k = DMatrix::zeros(12, 12);

        // Axial stiffness
        let ea_l = e * area / l;
        k[(0, 0)] = ea_l;     k[(0, 6)] = -ea_l;
        k[(6, 0)] = -ea_l;    k[(6, 6)] = ea_l;

        // Torsional stiffness
        let gj_l = g * j / l;
        k[(3, 3)] = gj_l;     k[(3, 9)] = -gj_l;
        k[(9, 3)] = -gj_l;    k[(9, 9)] = gj_l;

        // Bending about y-axis (in x-z plane)
        let ei_y = e * iy;
        k[(2, 2)] = 12.0 * ei_y / l3;   k[(2, 4)] = 6.0 * ei_y / l2;   k[(2, 8)] = -12.0 * ei_y / l3;  k[(2, 10)] = 6.0 * ei_y / l2;
        k[(4, 2)] = 6.0 * ei_y / l2;    k[(4, 4)] = 4.0 * ei_y / l;    k[(4, 8)] = -6.0 * ei_y / l2;   k[(4, 10)] = 2.0 * ei_y / l;
        k[(8, 2)] = -12.0 * ei_y / l3;  k[(8, 4)] = -6.0 * ei_y / l2;  k[(8, 8)] = 12.0 * ei_y / l3;   k[(8, 10)] = -6.0 * ei_y / l2;
        k[(10, 2)] = 6.0 * ei_y / l2;   k[(10, 4)] = 2.0 * ei_y / l;   k[(10, 8)] = -6.0 * ei_y / l2;  k[(10, 10)] = 4.0 * ei_y / l;

        // Bending about z-axis (in x-y plane)
        let ei_z = e * iz;
        k[(1, 1)] = 12.0 * ei_z / l3;   k[(1, 5)] = -6.0 * ei_z / l2;  k[(1, 7)] = -12.0 * ei_z / l3;  k[(1, 11)] = -6.0 * ei_z / l2;
        k[(5, 1)] = -6.0 * ei_z / l2;   k[(5, 5)] = 4.0 * ei_z / l;    k[(5, 7)] = 6.0 * ei_z / l2;    k[(5, 11)] = 2.0 * ei_z / l;
        k[(7, 1)] = -12.0 * ei_z / l3;  k[(7, 5)] = 6.0 * ei_z / l2;   k[(7, 7)] = 12.0 * ei_z / l3;   k[(7, 11)] = 6.0 * ei_z / l2;
        k[(11, 1)] = -6.0 * ei_z / l2;  k[(11, 5)] = 2.0 * ei_z / l;   k[(11, 7)] = 6.0 * ei_z / l2;   k[(11, 11)] = 4.0 * ei_z / l;

        Ok(k)
    }

    fn transformation_matrix(&self, nodes: &[Node]) -> Result<DMatrix<f64>> {
        if nodes.len() != 2 {
            return Err(GazelleError::ValidationError(
                "Frame3D element requires exactly 2 nodes".to_string()
            ));
        }

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let dz = node2.z - node1.z;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();

        if length < 1e-12 {
            return Err(GazelleError::ValidationError(
                "Frame element has zero length".to_string()
            ));
        }

        // Local x-axis (along element)
        let x_local = Vector3::new(dx / length, dy / length, dz / length);
        
        // Local y-axis (perpendicular to x and global z, or global y if x is parallel to z)
        let global_z = Vector3::new(0.0, 0.0, 1.0);
        let mut y_local = x_local.cross(&global_z);
        
        if y_local.norm() < 1e-6 {
            // Element is parallel to global z-axis, use global y
            let global_y = Vector3::new(0.0, 1.0, 0.0);
            y_local = x_local.cross(&global_y);
        }
        y_local.normalize_mut();
        
        // Local z-axis
        let z_local = x_local.cross(&y_local);

        // Build rotation matrix
        let rotation = Matrix3::from_columns(&[x_local, y_local, z_local]);

        // Build full transformation matrix (12x12)
        let mut t = DMatrix::zeros(12, 12);
        
        // Apply rotation to both nodes
        for i in 0..4 {
            let start_row = i * 3;
            let start_col = i * 3;
            t.fixed_view_mut::<3, 3>(start_row, start_col).copy_from(&rotation);
        }

        Ok(t)
    }

    fn mass_matrix(&self, element: &Element, material: &Material, nodes: &[Node]) -> Result<DMatrix<f64>> {
        let area = element.properties.area.ok_or(GazelleError::InvalidMaterial {
            property: "Cross-sectional area required".to_string(),
        })?;
        
        let density = material.properties.density.ok_or(GazelleError::InvalidMaterial {
            property: "Material density required".to_string(),
        })?;

        let node1 = &nodes[0];
        let node2 = &nodes[1];
        let dx = node2.x - node1.x;
        let dy = node2.y - node1.y;
        let dz = node2.z - node1.z;
        let length = (dx * dx + dy * dy + dz * dz).sqrt();

        let mass_per_length = density * area;
        let total_mass = mass_per_length * length;

        // Simplified lumped mass matrix for 3D frame
        let mut m = DMatrix::zeros(12, 12);
        let node_mass = total_mass / 2.0;

        // Translational mass at each node
        for i in 0..3 {
            m[(i, i)] = node_mass;           // Node 1
            m[(i + 6, i + 6)] = node_mass;   // Node 2
        }

        // Rotational inertia (simplified)
        let rotational_inertia = total_mass * length * length / 12.0;
        for i in 3..6 {
            m[(i, i)] = rotational_inertia;           // Node 1
            m[(i + 6, i + 6)] = rotational_inertia;   // Node 2
        }

        Ok(m)
    }
}

/// Element factory for creating appropriate element implementations
pub struct ElementFactory;

impl ElementFactory {
    /// Create element stiffness calculator based on element type
    pub fn create_element(element_type: ElementType) -> Box<dyn ElementStiffness> {
        match element_type {
            ElementType::Truss2D => Box::new(Truss2D),
            ElementType::Truss3D => Box::new(Truss3D),
            ElementType::Beam2D => Box::new(Beam2D),
            ElementType::Frame2D => Box::new(Beam2D), // Same as Beam2D for now
            ElementType::Beam3D | ElementType::Frame3D => Box::new(Frame3D),
            ElementType::Plate => Box::new(PlateElement), // To be implemented
            ElementType::Shell => Box::new(ShellElement), // To be implemented
            ElementType::Solid => Box::new(SolidElement), // To be implemented
        }
    }

    /// Compute element stiffness matrix
    pub fn compute_stiffness_matrix(
        element: &Element,
        material: &Material,
        nodes: &[Node],
    ) -> Result<DMatrix<f64>> {
        let element_impl = Self::create_element(element.element_type);
        element_impl.global_stiffness_matrix(element, material, nodes)
    }

    /// Compute element mass matrix
    pub fn compute_mass_matrix(
        element: &Element,
        material: &Material,
        nodes: &[Node],
    ) -> Result<DMatrix<f64>> {
        let element_impl = Self::create_element(element.element_type);
        element_impl.mass_matrix(element, material, nodes)
    }
}

/// Placeholder implementations for advanced element types
/// These would need full implementation for production use

pub struct PlateElement;

impl ElementStiffness for PlateElement {
    fn local_stiffness_matrix(&self, _element: &Element, _material: &Material) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Plate elements not yet implemented".to_string(),
        })
    }

    fn transformation_matrix(&self, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Plate elements not yet implemented".to_string(),
        })
    }

    fn mass_matrix(&self, _element: &Element, _material: &Material, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Plate elements not yet implemented".to_string(),
        })
    }
}

pub struct ShellElement;

impl ElementStiffness for ShellElement {
    fn local_stiffness_matrix(&self, _element: &Element, _material: &Material) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Shell elements not yet implemented".to_string(),
        })
    }

    fn transformation_matrix(&self, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Shell elements not yet implemented".to_string(),
        })
    }

    fn mass_matrix(&self, _element: &Element, _material: &Material, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Shell elements not yet implemented".to_string(),
        })
    }
}

pub struct SolidElement;

impl ElementStiffness for SolidElement {
    fn local_stiffness_matrix(&self, _element: &Element, _material: &Material) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Solid elements not yet implemented".to_string(),
        })
    }

    fn transformation_matrix(&self, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Solid elements not yet implemented".to_string(),
        })
    }

    fn mass_matrix(&self, _element: &Element, _material: &Material, _nodes: &[Node]) -> Result<DMatrix<f64>> {
        Err(GazelleError::UnsupportedElement {
            element_type: "Solid elements not yet implemented".to_string(),
        })
    }
}