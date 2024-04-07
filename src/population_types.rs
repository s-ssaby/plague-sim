use self::{populated_area::PopulatedArea, population::Population};

pub mod populated_area;
pub mod population;

/// Represents a type's ability to represent a population of people
/// 
/// This trait may not be implemented by clients on their types
pub trait PopulationType : private::Sealed {
    fn population(&self) -> Population;

    fn set_population(&mut self, population: Population);
}

impl PopulationType for Population {
    fn population(&self) -> Population {
        *self
    }
    
    fn set_population(&mut self, population: Population) {
        self.dead = population.dead;
        self.healthy = population.healthy;
        self.infected = population.infected;
        self.recovered = population.recovered;
    }
}

impl PopulationType for PopulatedArea {
    fn population(&self) -> Population {
        self.get_population()
    }
    
    fn set_population(&mut self, population: Population) {
        self.set_population(population);
    }

    
}

/// Represents a type's ability to have an associated population density
/// 
/// This trait may not be implemented by clients on their types
pub trait Density {
    /// Gets total population density
    fn total_density(&self) -> f32;

    /// Gets alive population density
    fn alive_density(&self) -> f32;
}

impl Density for PopulatedArea {
    fn total_density(&self) -> f32 {
        self.total_density()
    }

    fn alive_density(&self) -> f32 {
        self.alive_density()
    }
}

mod private {
    use super::{populated_area::PopulatedArea, population::Population};

    pub trait Sealed {}

    // Should cover all Population types specified in population_types module
    impl Sealed for PopulatedArea {}
    impl Sealed for Population {}
}