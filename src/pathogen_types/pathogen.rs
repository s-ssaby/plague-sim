use crate::{math_utils::get_random, population_types::{population::Population, Density, PopulationType}};

// Represents a pathogen, which are entities that transform populations without removing people from, or adding people to them
pub trait Pathogen {
    fn calculate_population<T>(&self, population: T) -> T where T: PopulationType;
}

// Represents a disease that can spread from person to person

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PathogenStruct {
    pub name: String,
    // probability of transmission when interacting with another person
    pub infectivity: f64,
    // probability of dying each day
    pub lethality: f64,
}

impl PathogenStruct {
    pub fn new(name: String, infectivity: f64, lethality: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&infectivity) {
            return Err(format!("Infectivity must be between 0 and 1, not {infectivity}"));
        }
        if !(0.0..=1.0).contains(&lethality) {
            return Err(format!("Lethality must be between 0 and 1, not {lethality}"));
        }

        Ok(Self {name, infectivity, lethality})
    }
}