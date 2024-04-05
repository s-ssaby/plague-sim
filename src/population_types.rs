use self::{populated_area::PopulatedArea, population::Population};

pub mod populated_area;
pub mod population;

/// Represents a type's ability to represent a population of people
/// 
/// This trait may not be implemented by clients on their types
pub trait PopulationType : private::Sealed {
    fn population(&self) -> Population;
}

impl PopulationType for Population {
    fn population(&self) -> Population {
        *self
    }
}

impl PopulationType for PopulatedArea {
    fn population(&self) -> Population {
        self.get_population()
    }
}

/// Represents a type's ability to have an associated population density
/// 
/// This trait may not be implemented by clients on their types
pub trait Density {
    /// Gets population density
    fn density(&self) -> f32;
}

impl Density for PopulatedArea {
    fn density(&self) -> f32 {
        self.total_density()
    }
}

mod private {
    use super::{populated_area::PopulatedArea, population::Population};

    pub trait Sealed {}

    // Should cover all Population types specified in population_types module
    impl Sealed for PopulatedArea {}
    impl Sealed for Population {}
}