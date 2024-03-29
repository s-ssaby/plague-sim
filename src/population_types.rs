use self::{populated_area::PopulatedArea, population::Population};

pub mod populated_area;
pub mod population;

pub enum PopulationType {
    Population(Population),
    PopulatedArea(PopulatedArea)
}