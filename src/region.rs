#![allow(dead_code)]

use std::sync::atomic::AtomicU32;

use crate::{population::Population, transportation::Port};

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


#[cfg(test)]
mod tests {
    use crate::transportation::Port;

    use super::Region;

    #[test]
    fn region_construction_test() {
        let small_port = Port::new(100);
        let big_port = Port::new(1000);
        let huge_port = Port::new(10_000_000);

        let country = Region::new("Super".to_owned(), 100, vec![small_port, big_port]);
        let big_country = Region::new("Mega".to_owned(), 1_000_000, vec![huge_port]);

        // make sure countries have unique ID
        assert_ne!(country.id, big_country.id);

        // make sure each country's ports have their region's id
        for port in country.get_ports() {
            assert_eq!(port.region.unwrap(), country.id)
        }

        for port in big_country.get_ports() {
            assert_eq!(port.region.unwrap(), big_country.id)
        }
    }
}

