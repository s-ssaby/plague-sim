use crate::population_types::{population::Population, Density, PopulationType};

// Represents anything that transforms one population into another, including pathogens
pub trait BasicShiftPopulation {
    fn shift_population<T>(&self, population: T) -> T where T: PopulationType;
}

/// Represents anything that transforms a population with a population density into another, including pathogens
pub trait AdvancedShiftPopulation {
    fn shift_population<T>(&self, population: T) -> T where T: PopulationType + Density;
}

// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pathogen {
    pub name: String,
    // probability of transmission when interacting with another person
    pub infectivity: f64,
    // probability of dying each day
    pub lethality: f64,
    // probability of recovering each day
    pub recovery_rate: f64   
}

impl Pathogen {
    pub fn new(name: String, infectivity: f64, lethality: f64, recovery_rate: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&infectivity) {
            return Err(format!("Infectivity must be between 0 and 1, not {infectivity}"));
        }
        if !(0.0..=1.0).contains(&lethality) {
            return Err(format!("Lethality must be between 0 and 1, not {lethality}"));
        }
        if !(0.0..=1.0).contains(&recovery_rate) {
            return Err(format!("Recovery rate must be between 0 and 1, not {recovery_rate}"));
        }
        let sum = recovery_rate + infectivity;
        if sum > 1.0_f64 {
            return Err(format!("Sum of recovery rate and lethality rate cannot exceed 1, sum is {sum}"));
        }

        Ok(Self {name, infectivity, lethality, recovery_rate})
    }
}