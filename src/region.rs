#![allow(dead_code)]

use std::sync::atomic::AtomicU32;

use crate::{population::Population};



use std::cell::RefCell;



#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub struct PortID(pub u32);

impl PortID {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/** Represents a specific site of travel, such as an airport/seaport */
/** Should only be constructed using an associated region */
#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    // maximum amount of transportation 
    pub capacity: u32,
    // whether port is operating or not
    closed: RefCell<bool>,
    // ID of region this port is in
    pub region: RegionID,
    // ID of this port
    pub id: PortID
}

impl Port {
    /** Creates a new open port capable of transporting specified capacity */
    /** Users of Port must ensure that all Ports they create have unique IDs to avoid unwanted behavior */
    fn new(id: PortID, region: RegionID, capacity: u32) -> Self {
        Self {capacity, closed: RefCell::new(false), region, id }
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

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash, PartialOrd, Ord)]
pub struct RegionID(u32);

impl RegionID {
    fn new() -> Self{
        let val = CURRENT_REGION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        RegionID(val)
    }
}

// #[derive(Debug, PartialEq)]
// /** Represents a struct for building Regions */
// /** Used primarily in configuration when not all information for Region available at once */
// pub struct RegionBuilder {
//     pub name: Option<String>,
//     pub population: Option<Population>,
//     pub ports: Vec<Port>
// }

// impl RegionBuilder {
//     pub fn new() -> Self{
//         let name = None;
//         let population = None;
//         let ports = vec![];
//         RegionBuilder {name, population, ports}
//     } 

//     pub fn set_name(&mut self, name: String) {
//         self.name = Some(name);
//     }

//     pub fn set_population(&mut self, population: Population) {
//         self.population = Some(population);
//     }

//     /** Creates a Region  */
//     /** Consumes builder on creation or failure */
//     pub fn build(self) -> Result<Region, String> {
//         let reg = Region::try_from(self);
//         reg        
//     }
// }

/** Represents a region of the world with a human population */
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub id: RegionID,
    pub name: String,
    pub population: Population,
    pub ports: Vec<Port>
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

impl Region {
    /** Creates region of healthy people */
    pub fn new(name: String, initial_pop: u32) -> Self {
        let id = RegionID::new();
        Region {name, population: Population::new(initial_pop), ports: vec![], id }
    }

    /** Creates region of people with specified population distribution */
    pub fn new_custom(name: String, initial_pop: Population, mut ports: Vec<Port>) -> Self {
        let id = RegionID::new();
        Region {name, population: initial_pop, ports, id }
    }

    /** Adds port to Region and returns a copy */
    pub fn add_port(&mut self, port_id: PortID, capacity: u32) -> Port {
        let port = Port::new(port_id, self.id, capacity);
        self.ports.push(port);
        port.clone()
    }  

    /** Retrieves reference to port if it exists in Region */
    pub fn get_port(&self, id: PortID) -> Option<&Port> {
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
    use crate::{population::Population, region::{Port, PortID}};

    use super::{Region};
    

    #[test]
    fn region_find_port_test() {
        let mut country = Region::new("Super".to_owned(), 100);
        let small_port = country.add_port(PortID(0), 100);
        let big_port = country.add_port(PortID(1), 1000);

        assert!(country.get_port(PortID::new(0)).is_some());
        assert!(country.get_port(PortID::new(1)).is_some());
        assert!(country.get_port(PortID::new(2)).is_none());
        assert!(country.get_port(PortID::new(3)).is_none());
    }

    #[test]
    fn region_construction_test() {
        let mut country = Region::new("Super".to_owned(), 100);
        let mut big_country = Region::new("Mega".to_owned(), 1_000_000);

        let small_port = country.add_port(PortID::new(0), 100);
        let big_port = country.add_port(PortID::new(1), 1000);
        let huge_port = big_country.add_port(PortID::new(2), 10_000_000);


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

