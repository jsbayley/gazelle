//! # Concrete Materials Module
//! 
//! Domain-driven design for concrete materials, inspired by the F# implementation.
//! Provides type-safe concrete properties with design code compliance.

use crate::units::{Stress, Age};
use serde::{Serialize, Deserialize};

/// UK concrete strength grades (matching F# implementation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UkConcreteGrade {
    Fck12, Fck16, Fck20, Fck25, Fck30, Fck35, Fck40, 
    Fck45, Fck50, Fck55, Fck60, Fck70, Fck80, Fck90,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CylinderStrength {
    Uk(UkConcreteGrade),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aggregate {
    Basalt,
    Limestone, 
    Sandstone,
    Quartzite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cement {
    ClassR,
    ClassN,
    ClassS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeightClass {
    NormalWeight,
}

/// Concrete specification matching F# design
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concrete {
    pub age: Age,
    pub aggregate: Aggregate,
    pub cement: Cement,
    pub grade: CylinderStrength,
    pub weight_class: WeightClass,
}

impl Concrete {
    /// Create new concrete with validation (matching F# tryCreate)
    pub fn try_create(
        grade: CylinderStrength,
        aggregate: Aggregate,
        cement: Cement,
        weight_class: WeightClass,
        age_days: f64,
    ) -> Result<Self, ConcreteError> {
        let age = Self::validate_age(age_days)?;
        
        Ok(Self {
            age,
            aggregate,
            cement,
            grade,
            weight_class,
        })
    }
    
    fn validate_age(age_days: f64) -> Result<Age, ConcreteError> {
        if age_days <= 0.0 {
            return Err(ConcreteError::InvalidAge("Concrete age <= 0 days".to_string()));
        }
        if age_days <= 3.0 {
            return Err(ConcreteError::InvalidAge(
                "In-situ strength tests required for age <= 3 days".to_string()
            ));
        }
        Ok(Age::new(age_days))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConcreteError {
    #[error("Invalid concrete age: {0}")]
    InvalidAge(String),
}

/// Concrete properties implementation (matching F# module structure)
impl Concrete {
    /// Characteristic compressive strength (fck)
    pub fn fck(&self) -> Stress {
        match self.grade {
            CylinderStrength::Uk(grade) => match grade {
                UkConcreteGrade::Fck12 => Stress::new(12.0),
                UkConcreteGrade::Fck16 => Stress::new(16.0),
                UkConcreteGrade::Fck20 => Stress::new(20.0),
                UkConcreteGrade::Fck25 => Stress::new(25.0),
                UkConcreteGrade::Fck30 => Stress::new(30.0),
                UkConcreteGrade::Fck35 => Stress::new(35.0),
                UkConcreteGrade::Fck40 => Stress::new(40.0),
                UkConcreteGrade::Fck45 => Stress::new(45.0),
                UkConcreteGrade::Fck50 => Stress::new(50.0),
                UkConcreteGrade::Fck55 => Stress::new(55.0),
                UkConcreteGrade::Fck60 => Stress::new(60.0),
                UkConcreteGrade::Fck70 => Stress::new(70.0),
                UkConcreteGrade::Fck80 => Stress::new(80.0),
                UkConcreteGrade::Fck90 => Stress::new(90.0),
            }
        }
    }
    
    /// Mean compressive strength (fcm)
    pub fn fcm(&self) -> Stress {
        match self.grade {
            CylinderStrength::Uk(grade) => match grade {
                UkConcreteGrade::Fck12 => Stress::new(20.0),
                UkConcreteGrade::Fck16 => Stress::new(24.0),
                UkConcreteGrade::Fck20 => Stress::new(28.0),
                UkConcreteGrade::Fck25 => Stress::new(33.0),
                UkConcreteGrade::Fck30 => Stress::new(38.0),
                UkConcreteGrade::Fck35 => Stress::new(43.0),
                UkConcreteGrade::Fck40 => Stress::new(48.0),
                UkConcreteGrade::Fck45 => Stress::new(53.0),
                UkConcreteGrade::Fck50 => Stress::new(58.0),
                UkConcreteGrade::Fck55 => Stress::new(63.0),
                UkConcreteGrade::Fck60 => Stress::new(68.0),
                UkConcreteGrade::Fck70 => Stress::new(78.0),
                UkConcreteGrade::Fck80 => Stress::new(88.0),
                UkConcreteGrade::Fck90 => Stress::new(98.0),
            }
        }
    }
    
    /// Mean tensile strength (fctm)
    pub fn fctm(&self) -> Stress {
        match self.grade {
            CylinderStrength::Uk(grade) => match grade {
                UkConcreteGrade::Fck12 => Stress::new(1.6),
                UkConcreteGrade::Fck16 => Stress::new(1.9),
                UkConcreteGrade::Fck20 => Stress::new(2.2),
                UkConcreteGrade::Fck25 => Stress::new(2.6),
                UkConcreteGrade::Fck30 => Stress::new(2.9),
                UkConcreteGrade::Fck35 => Stress::new(3.2),
                UkConcreteGrade::Fck40 => Stress::new(3.5),
                UkConcreteGrade::Fck45 => Stress::new(3.8),
                UkConcreteGrade::Fck50 => Stress::new(4.1),
                UkConcreteGrade::Fck55 => Stress::new(4.2),
                UkConcreteGrade::Fck60 => Stress::new(4.4),
                UkConcreteGrade::Fck70 => Stress::new(4.6),
                UkConcreteGrade::Fck80 => Stress::new(4.8),
                UkConcreteGrade::Fck90 => Stress::new(5.0),
            }
        }
    }
    
    /// Elastic modulus (Ecm) with aggregate adjustment
    pub fn ecm(&self) -> f64 {
        let base_ecm = match self.grade {
            CylinderStrength::Uk(grade) => match grade {
                UkConcreteGrade::Fck12 => 27_000.0,
                UkConcreteGrade::Fck16 => 29_000.0,
                UkConcreteGrade::Fck20 => 30_000.0,
                UkConcreteGrade::Fck25 => 31_000.0,
                UkConcreteGrade::Fck30 => 33_000.0,
                UkConcreteGrade::Fck35 => 34_000.0,
                UkConcreteGrade::Fck40 => 35_000.0,
                UkConcreteGrade::Fck45 => 36_000.0,
                UkConcreteGrade::Fck50 => 37_000.0,
                UkConcreteGrade::Fck55 => 38_000.0,
                UkConcreteGrade::Fck60 => 39_000.0,
                UkConcreteGrade::Fck70 => 41_000.0,
                UkConcreteGrade::Fck80 => 42_000.0,
                UkConcreteGrade::Fck90 => 44_000.0,
            }
        };
        
        let aggregate_factor = match self.aggregate {
            Aggregate::Quartzite => 1.0,
            Aggregate::Limestone => 0.9,
            Aggregate::Sandstone => 0.7,
            Aggregate::Basalt => 1.2,
        };
        
        base_ecm * aggregate_factor
    }
    
    /// Density based on weight class
    pub fn density(&self, reinforced: bool) -> f64 {
        match self.weight_class {
            WeightClass::NormalWeight => {
                if reinforced { 2500.0 } else { 2400.0 }
            }
        }
    }
    
    /// Time-dependent strength properties
    pub fn fcm_t(&self) -> Stress {
        let s: f64 = match self.cement {
            Cement::ClassN => 0.25,
            Cement::ClassR => 0.20,
            Cement::ClassS => 0.38,
        };
        
        let age_days = self.age.value();
        let beta_cc_t = if age_days < 28.0 {
            s.exp() * (1.0 - (28.0 / age_days).sqrt())
        } else {
            1.0
        };
        
        let fcm_basic = self.fcm();
        Stress::new(beta_cc_t * fcm_basic.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concrete_creation() {
        let concrete = Concrete::try_create(
            CylinderStrength::Uk(UkConcreteGrade::Fck30),
            Aggregate::Limestone,
            Cement::ClassN,
            WeightClass::NormalWeight,
            28.0,
        ).unwrap();
        
        assert_eq!(concrete.fck().value(), 30.0);
        assert_eq!(concrete.fcm().value(), 38.0);
    }
    
    #[test]
    fn test_age_validation() {
        let result = Concrete::try_create(
            CylinderStrength::Uk(UkConcreteGrade::Fck30),
            Aggregate::Limestone,
            Cement::ClassN,
            WeightClass::NormalWeight,
            2.0,
        );
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_aggregate_adjustment() {
        let limestone_concrete = Concrete::try_create(
            CylinderStrength::Uk(UkConcreteGrade::Fck30),
            Aggregate::Limestone,
            Cement::ClassN,
            WeightClass::NormalWeight,
            28.0,
        ).unwrap();
        
        let basalt_concrete = Concrete::try_create(
            CylinderStrength::Uk(UkConcreteGrade::Fck30),
            Aggregate::Basalt,
            Cement::ClassN,
            WeightClass::NormalWeight,
            28.0,
        ).unwrap();
        
        assert!(basalt_concrete.ecm() > limestone_concrete.ecm());
    }
}