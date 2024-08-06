#![allow(dead_code)]

use std::{cell::Cell, fmt::{write, Display}, sync::atomic::AtomicU32};

use serde::{Deserialize, Serialize};

use crate::{point::{Point2D}, population_types::{population::Population, PopulationType}};



use std::cell::RefCell;



#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct PortID(pub u32);

impl PortID {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl Display for PortID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum PortStatus {
    #[default] Open,
    Closed
}

/** Represents a specific site of travel, such as an airport/seaport */
/** Should only be constructed using an associated region */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Port {
    // maximum amount of transportation 
    pub capacity: u32,
    // whether port is operating or not
    status: Cell<PortStatus>,
    // ID of region this port is in
    region: RegionID,
    // ID of this port
    pub id: PortID,
    // Position of this port
    pub pos: Point2D
}

impl Port {
    /** Creates a new open port capable of transporting specified capacity */
    /** Users of Port must ensure that all Ports they create have unique IDs to avoid unwanted behavior */
    fn new(id: PortID, region: RegionID, capacity: u32, pos: Point2D) -> Self {
        Self {capacity, status: Cell::new(PortStatus::Open), region, id, pos}
    }

    pub fn close_port(&self) {
        self.status.replace(PortStatus::Closed);
    }

    pub fn port_status(&self) -> PortStatus {
        self.status.get()
    }

    pub fn set_status(&self, status: PortStatus) {
        self.status.replace(status);
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }

    pub fn region(&self) -> RegionID {
        self.region
    }
}


// Responsible for assigning a unique ID to every region
static CURRENT_REGION_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RegionID(pub u32);

impl RegionID {
    fn new() -> Self{
        let val = CURRENT_REGION_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        RegionID(val)
    }
}

impl Display for RegionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/** Represents a region of the world with a human population */

// Invariants to be preserved
// RegionID always matched RegionID of ports it contains
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Region<P = Population> where P: PopulationType {
    id: RegionID,
    pub name: String,
    pub population: P,
    ports: Vec<Port>
}

impl<P> Region <P> where P: PopulationType {
    /** Creates region of people with specified population*/
    pub fn new(name: String, initial_pop: P) -> Self {
        let id = RegionID::new();
        Region {name, population: initial_pop, ports: vec![], id }
    }

    pub fn id(&self) -> RegionID {
        self.id
    }

    pub fn get_ports(&self) -> &[Port] {
        &self.ports
    }

    /** Adds port to Region and returns a copy */
    pub fn add_port(&mut self, port_id: PortID, capacity: u32, pos: Point2D) -> Port {
        let port = Port::new(port_id, self.id, capacity, pos);
        let clone = port.clone();
        self.ports.push(port);
        clone
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
    use crate::{point::Point2D, population_types::population::Population, region::PortID};

    use super::{Region};
    

    #[test]
    fn region_find_port_test() {
        let mut country = Region::new("Super".to_owned(), Population::new_healthy(100));
        let small_port = country.add_port(PortID(0), 100, Point2D::default());
        let big_port = country.add_port(PortID(1), 1000, Point2D::default());

        assert!(country.get_port(PortID::new(0)).is_some());
        assert!(country.get_port(PortID::new(1)).is_some());
        assert!(country.get_port(PortID::new(2)).is_none());
        assert!(country.get_port(PortID::new(3)).is_none());
    }

    #[test]
    fn region_construction_test() {
        let mut country = Region::new("Super".to_owned(), Population::new_healthy(100));
        let mut big_country = Region::new("Mega".to_owned(), Population::new_healthy(1_000_000));

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

