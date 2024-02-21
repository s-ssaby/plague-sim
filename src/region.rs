use crate::transportation::Port;

/** Represents a region of the world with a human population */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Region {
    name: String,
    population: Population,
    ports: Vec<Port>
}


#[derive(Debug, Clone, Default, PartialEq)]
/** Represents any group of people */
struct Population {
    alive: u32,
    dead: u32,
    recovered: u32
}