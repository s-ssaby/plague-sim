use std::{error::Error, ops::{Add, Sub}};

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

impl Add for Population {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let tot_alive = self.alive + rhs.alive;
        let tot_dead = self.dead + rhs.dead;
        let tot_recovered = self.recovered + rhs.recovered;
        Population { alive: tot_alive, dead: tot_dead, recovered: tot_recovered }
    }

}

impl Population {
    // Transports a subpopulation of people out of this population
    // Returns resulting population after transportation
    // Errors if group cannot be extracted from this population
    fn emigrate(self, group: Self) -> Result<Population, String> {
        if group.alive > self.alive {
            Err(format!("Cannot remove {} alive people from {} alive people", group.alive, self.alive))
        } else if group.dead > self.dead {
            Err(format!("Cannot remove {} dead people from {} dead people", group.dead, self.dead))
        } else if group.recovered > self.recovered {
            Err(format!("Cannot remove {} recovered people from {} recovered people", group.recovered, self.recovered))
        } else {
            let remaining_alive = self.alive - group.alive;
            let remaining_dead = self.dead - group.dead;
            let remaining_recovered = self.recovered - group.recovered;
            Ok(Population { alive: remaining_alive, dead: remaining_dead, recovered: remaining_recovered })
        }
    }
}

