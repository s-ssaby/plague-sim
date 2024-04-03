use super::population::Population;



/// Represents a human population with an associated area and population density
/// 
/// Not to be confused with Region
pub struct PopulatedArea {
    population: Population,
    area: f32,
    alive_density: f32,
    total_density: f32
}

/// Invariants:
/// * total density * area = population total
/// * alive density * area = non dead total
impl PopulatedArea {
    /// Creates based on a given area and population
    pub fn new_from_area(area: f32, population: Population) {
        todo!()
    }

    /// Creates based on a given population density and population
    pub fn new_from_density(density: f32, population: Population) {
        todo!()
    }

    /// Get population
    pub fn get_population(&self) -> Population {
        todo!()
    }

    /// Get area
    pub fn get_area(&self) -> f32 {
        todo!()
    }

    /// Set population
    /// Returns new total density and new alive density
    pub fn set_population(&mut self, population: Population) {
        todo!()
    }

    /// Set area
    /// Returns new total density and new alive density
    pub fn set_area(&mut self, area: f32) {
        todo!()
    }
}