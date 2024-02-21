/** Represents a region of the world with a human population */
pub struct Region {
    name: String,
    population: Population
}


/** Represents any group of people */
struct Population {
    alive: u32,
    dead: u32,
    recovered: u32
}