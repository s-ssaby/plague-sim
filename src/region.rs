#![allow(dead_code)]

use std::{ops::Add, sync::atomic::AtomicU32};

use crate::transportation::Port;

// Responsible for assigning a unique ID to every region
static CURRENT_REGION_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct RegionID(u32);

impl RegionID {
    fn new() -> Self{
        let val = CURRENT_REGION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        RegionID(val)
    }
}

/** Represents a region of the world with a human population */
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    id: RegionID,
    name: String,
    population: Population,
    ports: Vec<Port>
}

impl Region {
    pub fn new(name: String, initial_pop: u32, mut ports: Vec<Port>) -> Self {
        let id = RegionID::new();
        for port in &mut ports {
            port.region = Some(id);
        }
        Region {name, population: Population::new(initial_pop), ports, id }
    }

    pub fn close_ports(&mut self) {
        for port in &mut self.ports {
            port.close_port();
        }
    }

    pub fn get_ports(&self) -> &[Port] {
        &self.ports
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
/** Represents any group of people */
struct Population {
    healthy: u32,
    infected: u32,
    dead: u32,
    recovered: u32
}

impl Add for Population {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let tot_healthy = self.healthy + rhs.healthy;
        let tot_infected = self.infected + rhs.infected;
        let tot_dead = self.dead + rhs.dead;
        let tot_recovered = self.recovered + rhs.recovered;
        Population { healthy: tot_healthy, infected: tot_infected, dead: tot_dead, recovered: tot_recovered }
    }

}

impl Population {
    /* Creates a population of healthy people */
    pub fn new (initial_pop: u32) -> Self{
        Self {healthy: initial_pop, dead: 0, recovered: 0, infected: 0}
    }

    // Transports a subpopulation of people out of this population
    // Modifies this population to match resulting population after transportation
    // Errors if group cannot be extracted from this population
    fn emigrate(&mut self, group: Self) -> Result<(), String> {
        if group.healthy > self.healthy {
            Err(format!("Cannot remove {} healthy people from {} healthy people", group.healthy, self.healthy))
        } else if group.dead > self.dead {
            Err(format!("Cannot remove {} dead people from {} dead people", group.dead, self.dead))
        } else if group.recovered > self.recovered {
            Err(format!("Cannot remove {} recovered people from {} recovered people", group.recovered, self.recovered))
        } else if group.infected > self.infected {
            Err(format!("Cannot remove {} infected people from {} infected people", group.infected, self.infected))
        } else {
            self.healthy -= group.healthy;
            self.dead -= group.dead;
            self.recovered -= group.recovered;
            self.infected -= group.infected;
            Ok(())
        }
    }
}

