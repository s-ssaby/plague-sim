#![allow(dead_code)]

use std::sync::atomic::AtomicU32;

use serde::{Deserialize, Serialize};

use crate::{location::Location, population::Population};



use std::cell::RefCell;



#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct PortID(pub u32);

impl PortID {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/** Represents a specific site of travel, such as an airport/seaport */
/** Should only be constructed using an associated region */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Port<T> where T: Location {
    // maximum amount of transportation 
    pub capacity: u32,
    // whether port is operating or not
    closed: RefCell<bool>,
    // ID of region this port is in
    pub region: RegionID,
    // ID of this port
    pub id: PortID,
    // Position of this port
    pub pos: T
}

impl<T> Port<T> where T: Location {
    /** Creates a new open port capable of transporting specified capacity */
    /** Users of Port must ensure that all Ports they create have unique IDs to avoid unwanted behavior */
    fn new(id: PortID, region: RegionID, capacity: u32, pos: T) -> Self {
        Self {capacity, closed: RefCell::new(false), region, id, pos}
    }

    pub fn close_port(&self) {
        self.closed.replace(false);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.borrow().to_owned()
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }
}


// Responsible for assigning a unique ID to every region
static CURRENT_REGION_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RegionID(u32);

impl RegionID {
    fn new() -> Self{
        let val = CURRENT_REGION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        RegionID(val)
    }
}

/** Represents a region of the world with a human population */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region<T> where T: Location {
    pub id: RegionID,
    pub name: String,
    pub population: Population,
    pub ports: Vec<Port<T>>
}

// impl TryFrom<RegionBuilder> for Region {
//     type Error = String;

//     fn try_from(value: RegionBuilder) -> Result<Self, Self::Error> {
//         let name = value.name;
//         let initial_pop = value.population;
//         let ports = value.ports;
//         match (name, initial_pop) {
//             (None, _) => Err("Region must have a name".to_owned()),
//             (Some(name), None) => Err(format!("Region {} must have a population", name)),
//             (Some(name), Some(initial_pop)) => Ok(Region::new_custom(name, initial_pop, ports)),
//         }
//     }
// }

impl<T> Region <T> where T: Location {
    /** Creates region of healthy people */
    pub fn new(name: String, initial_pop: u32) -> Self {
        let id = RegionID::new();
        Region {name, population: Population::new(initial_pop), ports: vec![], id }
    }

    /** Creates region of people with specified population distribution */
    pub fn new_custom(name: String, initial_pop: Population, mut ports: Vec<Port<T>>) -> Self {
        let id = RegionID::new();
        Region {name, population: initial_pop, ports, id }
    }

    /** Adds port to Region and returns a copy */
    pub fn add_port(&mut self, port_id: PortID, capacity: u32, pos: T) -> Port<T> {
        let port = Port::new(port_id, self.id, capacity, pos);
        let clone = port.clone();
        self.ports.push(port);
        clone
    }  

    /** Retrieves reference to port if it exists in Region */
    pub fn get_port(&self, id: PortID) -> Option<&Port<T>> {
        self.ports.iter().find(|port| port.id == id)
    }

    pub fn close_ports(&mut self) {
        for port in &mut self.ports {
            port.close_port();
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{location::Point2D, population::Population, region::{Port, PortID}};

    use super::{Region};
    

    #[test]
    fn region_find_port_test() {
        let mut country = Region::new("Super".to_owned(), 100);
        let small_port = country.add_port(PortID(0), 100, Point2D::default());
        let big_port = country.add_port(PortID(1), 1000, Point2D::default());

        assert!(country.get_port(PortID::new(0)).is_some());
        assert!(country.get_port(PortID::new(1)).is_some());
        assert!(country.get_port(PortID::new(2)).is_none());
        assert!(country.get_port(PortID::new(3)).is_none());
    }

    #[test]
    fn region_construction_test() {
        let mut country = Region::new("Super".to_owned(), 100);
        let mut big_country = Region::new("Mega".to_owned(), 1_000_000);

        let small_port = country.add_port(PortID::new(0), 100, Point2D::default());
        let big_port = country.add_port(PortID::new(1), 1000, Point2D::default());
        let huge_port = big_country.add_port(PortID::new(2), 10_000_000, Point2D::default());


        // make sure countries have unique ID
        assert_ne!(country.id, big_country.id);

        // make sure each country's ports have their region's id
        for port in country.ports {
            assert_eq!(port.region, country.id)
        }

        for port in big_country.ports {
            assert_eq!(port.region, big_country.id)
        }
    }
}

