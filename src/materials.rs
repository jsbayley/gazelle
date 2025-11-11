//! Material properties and constitutive models

use crate::error::{GazelleError, Result, Validate};
use serde::{Deserialize, Serialize};

/// Material definition with properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material {
    pub id: usize,
    pub name: String,
    pub material_type: MaterialType,
    pub properties: MaterialProperties,
}

impl Material {
    pub fn new(id: usize, name: String, material_type: MaterialType, properties: MaterialProperties) -> Self {
        Self {
            id,
            name,
            material_type,
            properties,
        }
    }

    /// Create a linear elastic isotropic material
    pub fn linear_elastic(id: usize, name: String, young_modulus: f64, poisson_ratio: f64, density: f64) -> Self {
        Self {
            id,
            name,
            material_type: MaterialType::LinearElastic,
            properties: MaterialProperties {
                young_modulus: Some(young_modulus),
                poisson_ratio: Some(poisson_ratio),
                density: Some(density),
                yield_strength: None,
                ultimate_strength: None,
                thermal_expansion: None,
                damping_ratio: None,
            },
        }
    }

    /// Create a steel material with typical properties
    pub fn steel(id: usize, name: String) -> Self {
        Self::linear_elastic(id, name, 200e9, 0.3, 7850.0)
            .with_yield_strength(250e6)
            .with_ultimate_strength(400e6)
            .with_thermal_expansion(12e-6)
    }

    /// Create a concrete material with typical properties
    pub fn concrete(id: usize, name: String, compressive_strength: f64) -> Self {
        let young_modulus = 4700.0 * (compressive_strength * 1e-6).sqrt() * 1e6; // ACI formula
        Self::linear_elastic(id, name, young_modulus, 0.2, 2400.0)
            .with_ultimate_strength(compressive_strength)
            .with_thermal_expansion(10e-6)
    }

    /// Create an aluminum material with typical properties
    pub fn aluminum(id: usize, name: String) -> Self {
        Self::linear_elastic(id, name, 70e9, 0.33, 2700.0)
            .with_yield_strength(270e6)
            .with_ultimate_strength(310e6)
            .with_thermal_expansion(23e-6)
    }

    pub fn with_yield_strength(mut self, yield_strength: f64) -> Self {
        self.properties.yield_strength = Some(yield_strength);
        self
    }

    pub fn with_ultimate_strength(mut self, ultimate_strength: f64) -> Self {
        self.properties.ultimate_strength = Some(ultimate_strength);
        self
    }

    pub fn with_thermal_expansion(mut self, thermal_expansion: f64) -> Self {
        self.properties.thermal_expansion = Some(thermal_expansion);
        self
    }

    pub fn with_damping_ratio(mut self, damping_ratio: f64) -> Self {
        self.properties.damping_ratio = Some(damping_ratio);
        self
    }

    /// Calculate shear modulus from Young's modulus and Poisson's ratio
    pub fn shear_modulus(&self) -> Result<f64> {
        let e = self.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus".to_string(),
        })?;
        let nu = self.properties.poisson_ratio.ok_or(GazelleError::InvalidMaterial {
            property: "Poisson's ratio".to_string(),
        })?;
        Ok(e / (2.0 * (1.0 + nu)))
    }

    /// Calculate bulk modulus from Young's modulus and Poisson's ratio
    pub fn bulk_modulus(&self) -> Result<f64> {
        let e = self.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus".to_string(),
        })?;
        let nu = self.properties.poisson_ratio.ok_or(GazelleError::InvalidMaterial {
            property: "Poisson's ratio".to_string(),
        })?;
        Ok(e / (3.0 * (1.0 - 2.0 * nu)))
    }

    /// Calculate Lamé's first parameter
    pub fn lame_lambda(&self) -> Result<f64> {
        let e = self.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
            property: "Young's modulus".to_string(),
        })?;
        let nu = self.properties.poisson_ratio.ok_or(GazelleError::InvalidMaterial {
            property: "Poisson's ratio".to_string(),
        })?;
        Ok(e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu)))
    }

    /// Calculate Lamé's second parameter (shear modulus)
    pub fn lame_mu(&self) -> Result<f64> {
        self.shear_modulus()
    }
}

impl Validate for Material {
    fn validate(&self) -> Result<()> {
        match self.material_type {
            MaterialType::LinearElastic => {
                if self.properties.young_modulus.is_none() {
                    return Err(GazelleError::InvalidMaterial {
                        property: "Young's modulus is required for linear elastic materials".to_string(),
                    });
                }

                if self.properties.poisson_ratio.is_none() {
                    return Err(GazelleError::InvalidMaterial {
                        property: "Poisson's ratio is required for linear elastic materials".to_string(),
                    });
                }

                if let Some(e) = self.properties.young_modulus {
                    if e <= 0.0 {
                        return Err(GazelleError::InvalidMaterial {
                            property: "Young's modulus must be positive".to_string(),
                        });
                    }
                }

                if let Some(nu) = self.properties.poisson_ratio {
                    if nu < -1.0 || nu >= 0.5 {
                        return Err(GazelleError::InvalidMaterial {
                            property: "Poisson's ratio must be between -1 and 0.5".to_string(),
                        });
                    }
                }

                if let Some(density) = self.properties.density {
                    if density <= 0.0 {
                        return Err(GazelleError::InvalidMaterial {
                            property: "Density must be positive".to_string(),
                        });
                    }
                }
            }
            _ => {
                // Add validation for other material types as they are implemented
            }
        }

        Ok(())
    }
}

/// Types of material models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaterialType {
    LinearElastic,
    Plastic,
    Viscoelastic,
    Composite,
    NonlinearElastic,
}

/// Material properties container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialProperties {
    // Elastic properties
    pub young_modulus: Option<f64>,      // Pa
    pub poisson_ratio: Option<f64>,      // dimensionless
    pub density: Option<f64>,            // kg/m³
    
    // Strength properties
    pub yield_strength: Option<f64>,     // Pa
    pub ultimate_strength: Option<f64>,  // Pa
    
    // Other properties
    pub thermal_expansion: Option<f64>,  // 1/K
    pub damping_ratio: Option<f64>,      // dimensionless
}

impl MaterialProperties {
    pub fn new() -> Self {
        Self {
            young_modulus: None,
            poisson_ratio: None,
            density: None,
            yield_strength: None,
            ultimate_strength: None,
            thermal_expansion: None,
            damping_ratio: None,
        }
    }
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Constitutive matrix calculation
pub trait ConstitutiveMatrix {
    /// Calculate the constitutive matrix for the material
    fn constitutive_matrix(&self, analysis_type: ConstitutiveType) -> Result<nalgebra::DMatrix<f64>>;
}

impl ConstitutiveMatrix for Material {
    fn constitutive_matrix(&self, analysis_type: ConstitutiveType) -> Result<nalgebra::DMatrix<f64>> {
        match self.material_type {
            MaterialType::LinearElastic => {
                let e = self.properties.young_modulus.ok_or(GazelleError::InvalidMaterial {
                    property: "Young's modulus".to_string(),
                })?;
                let nu = self.properties.poisson_ratio.ok_or(GazelleError::InvalidMaterial {
                    property: "Poisson's ratio".to_string(),
                })?;

                match analysis_type {
                    ConstitutiveType::PlaneStress => {
                        let factor = e / (1.0 - nu * nu);
                        let mut d = nalgebra::DMatrix::zeros(3, 3);
                        d[(0, 0)] = factor;
                        d[(1, 1)] = factor;
                        d[(0, 1)] = factor * nu;
                        d[(1, 0)] = factor * nu;
                        d[(2, 2)] = factor * (1.0 - nu) / 2.0;
                        Ok(d)
                    }
                    ConstitutiveType::PlaneStrain => {
                        let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
                        let mut d = nalgebra::DMatrix::zeros(3, 3);
                        d[(0, 0)] = factor * (1.0 - nu);
                        d[(1, 1)] = factor * (1.0 - nu);
                        d[(0, 1)] = factor * nu;
                        d[(1, 0)] = factor * nu;
                        d[(2, 2)] = factor * (1.0 - 2.0 * nu) / 2.0;
                        Ok(d)
                    }
                    ConstitutiveType::ThreeDimensional => {
                        let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
                        let mut d = nalgebra::DMatrix::zeros(6, 6);
                        
                        // Normal stresses
                        d[(0, 0)] = factor * (1.0 - nu);
                        d[(1, 1)] = factor * (1.0 - nu);
                        d[(2, 2)] = factor * (1.0 - nu);
                        
                        // Cross terms
                        d[(0, 1)] = factor * nu;
                        d[(0, 2)] = factor * nu;
                        d[(1, 0)] = factor * nu;
                        d[(1, 2)] = factor * nu;
                        d[(2, 0)] = factor * nu;
                        d[(2, 1)] = factor * nu;
                        
                        // Shear stresses
                        let g = e / (2.0 * (1.0 + nu));
                        d[(3, 3)] = g;
                        d[(4, 4)] = g;
                        d[(5, 5)] = g;
                        
                        Ok(d)
                    }
                    ConstitutiveType::Axisymmetric => {
                        // Similar to plane strain but with additional radial component
                        let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
                        let mut d = nalgebra::DMatrix::zeros(4, 4);
                        d[(0, 0)] = factor * (1.0 - nu);
                        d[(1, 1)] = factor * (1.0 - nu);
                        d[(2, 2)] = factor * (1.0 - nu);
                        d[(0, 1)] = factor * nu;
                        d[(0, 2)] = factor * nu;
                        d[(1, 0)] = factor * nu;
                        d[(1, 2)] = factor * nu;
                        d[(2, 0)] = factor * nu;
                        d[(2, 1)] = factor * nu;
                        d[(3, 3)] = factor * (1.0 - 2.0 * nu) / 2.0;
                        Ok(d)
                    }
                }
            }
            _ => Err(GazelleError::UnsupportedElement {
                element_type: format!("{:?}", self.material_type),
            }),
        }
    }
}

/// Types of constitutive relations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutiveType {
    PlaneStress,
    PlaneStrain,
    ThreeDimensional,
    Axisymmetric,
}