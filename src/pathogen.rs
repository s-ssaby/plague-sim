use crate::population_types::{population::Population, Density, PopulationType};

// Marker trait used to represent a pathogen, which are entities that act on populations without removing, or adding to them
pub trait Pathogen {}

// Represents a pathogen that acts on a population using its healthy, infected, etc counts with no other info about it
pub trait BasicPathogen : Pathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType;
}

/// Represents a pathogen that acts on a population using its healthy, infected, etc counts in combination with population density information
pub trait AdvancedPathogen : Pathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType + Density;
}


/// Represents a pathogen that can spontaneously spawn into populations without any infected individuals
pub struct SpontaneousPathogen<T> where T: Pathogen {
    pub spawn_chance: f32,
    pub pathogen: T
}

impl<T> Pathogen for SpontaneousPathogen<T> where T: Pathogen {}

impl<T> SpontaneousPathogen<T> where T: Pathogen {
    pub fn new(spawn_chance: f32, pathogen: T) -> Self {
        Self {spawn_chance, pathogen}
    }
}

impl<P> BasicPathogen for SpontaneousPathogen<P> where P: BasicPathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType {
        todo!()
    }
}

impl<P> AdvancedPathogen for SpontaneousPathogen<P> where P: BasicPathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType {
        todo!()
    }
}


// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PathogenStruct {
    pub name: String,
    // probability of transmission when interacting with another person
    pub infectivity: f64,
    // probability of dying each day
    pub lethality: f64,
    // probability of recovering each day
    pub recovery_rate: f64   
}

impl PathogenStruct {
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