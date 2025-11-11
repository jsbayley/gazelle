//! # Units of Measure System
//! 
//! Type-safe units of measure system inspired by F#'s approach.
//! Prevents unit mixing errors that are common in structural engineering.

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};

/// Unit system marker traits
pub trait Unit: Copy + Clone + 'static {
    const SYMBOL: &'static str;
}

/// Base units
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Millimeter;
impl Unit for Millimeter { const SYMBOL: &'static str = "mm"; }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Meter;
impl Unit for Meter { const SYMBOL: &'static str = "m"; }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Kilonewton;
impl Unit for Kilonewton { const SYMBOL: &'static str = "kN"; }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Newton;
impl Unit for Newton { const SYMBOL: &'static str = "N"; }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Megapascal;
impl Unit for Megapascal { const SYMBOL: &'static str = "MPa"; }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Days;
impl Unit for Days { const SYMBOL: &'static str = "days"; }

/// Dimensional quantities with units
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantity<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: Unit> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
    
    pub fn unit_symbol(&self) -> &'static str {
        U::SYMBOL
    }
}

// Arithmetic operations preserving units
impl<U: Unit> Add for Quantity<U> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<U: Unit> Sub for Quantity<U> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<U: Unit> Mul<f64> for Quantity<U> {
    type Output = Self;
    
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.value * scalar)
    }
}

impl<U: Unit> Div<f64> for Quantity<U> {
    type Output = Self;
    
    fn div(self, scalar: f64) -> Self {
        Self::new(self.value / scalar)
    }
}

/// Type aliases matching your F# design
pub type Length = Quantity<Millimeter>;
pub type Force = Quantity<Kilonewton>;
pub type Moment = Quantity<Kilonewton>; // kN⋅m approximation
pub type Age = Quantity<Days>;

/// Area types with semantic meaning (from your F# design)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Area {
    CrossSectional(f64), // mm²
    Surface(f64),        // mm²
}

impl Area {
    pub fn cross_sectional(value: f64) -> Self {
        Self::CrossSectional(value)
    }
    
    pub fn surface(value: f64) -> Self {
        Self::Surface(value)
    }
    
    pub fn value(&self) -> f64 {
        match self {
            Self::CrossSectional(v) | Self::Surface(v) => *v,
        }
    }
}

/// Stress type with units (from your F# design)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stress {
    value: f64, // MPa
}

impl Stress {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
    
    pub fn mpa(&self) -> f64 {
        self.value
    }
    
    pub fn n_per_mm2(&self) -> f64 {
        self.value // MPa = N/mm²
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_safety() {
        let length1 = Length::new(100.0);
        let length2 = Length::new(50.0);
        let total = length1 + length2;
        assert_eq!(total.value(), 150.0);
    }

    #[test]
    fn test_area_semantics() {
        let cross_section = Area::cross_sectional(2500.0);
        let surface = Area::surface(5000.0);
        
        assert_ne!(cross_section, surface);
        assert_eq!(cross_section.value(), 2500.0);
    }

    #[test]
    fn test_stress_units() {
        let stress = Stress::new(25.0);
        assert_eq!(stress.mpa(), 25.0);
        assert_eq!(stress.n_per_mm2(), 25.0);
    }
}