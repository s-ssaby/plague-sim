#![allow(dead_code)]

use std::sync::atomic::AtomicU32;

use crate::{population::Population, transportation::{Port, PortID}};

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

#[derive(Debug, PartialEq)]
/** Represents a struct for building Regions */
/** Used primarily in configuration when not all information for Region available at once */
pub struct RegionBuilder {
    pub name: Option<String>,
    pub population: Option<Population>,
    pub ports: Vec<Port>
}

impl RegionBuilder {
    pub fn new() -> Self{
        let name = None;
        let population = None;
        let ports = vec![];
        RegionBuilder {name, population, ports}
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_population(&mut self, population: Population) {
        self.population = Some(population);
    }

    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port);
    }

    /** Creates a Region  */
    /** Consumes builder on creation or failure */
    pub fn build(self) -> Result<Region, String> {
        let reg = Region::try_from(self);
        reg        
    }
}

/** Represents a region of the world with a human population */
#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub id: RegionID,
    pub name: String,
    pub population: Population,
    pub ports: Vec<Port>
}

impl TryFrom<RegionBuilder> for Region {
    type Error = String;

    fn try_from(value: RegionBuilder) -> Result<Self, Self::Error> {
        let name = value.name;
        let initial_pop = value.population;
        let ports = value.ports;
        match (name, initial_pop) {
            (None, _) => Err("Region must have a name".to_owned()),
            (Some(name), None) => Err(format!("Region {} must have a population", name)),
            (Some(name), Some(initial_pop)) => Ok(Region::new_custom(name, initial_pop, ports)),
        }
    }
}

impl Region {
    /** Creates region of healthy people */
    pub fn new(name: String, initial_pop: u32, mut ports: Vec<Port>) -> Self {
        let id = RegionID::new();
        for port in &mut ports {
            port.region = Some(id);
        }
        Region {name, population: Population::new(initial_pop), ports, id }
    }

    /** Creates region of people with specified population distribution */
    pub fn new_custom(name: String, initial_pop: Population, mut ports: Vec<Port>) -> Self {
        let id = RegionID::new();
        for port in &mut ports {
            port.region = Some(id);
        }
        Region {name, population: initial_pop, ports, id }
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
    use crate::{population::Population, transportation::{Port, PortID}};

    use super::{Region, RegionBuilder};

    #[test]
    fn region_find_port_test() {
        let small_port = Port::new(PortID::new(0), 100);
        let big_port = Port::new(PortID::new(1), 1000);

        let country = Region::new("Super".to_owned(), 100, vec![small_port, big_port]);
        assert!(country.get_port(PortID::new(0)).is_some());
        assert!(country.get_port(PortID::new(1)).is_some());
        assert!(country.get_port(PortID::new(2)).is_none());
        assert!(country.get_port(PortID::new(3)).is_none());
    }

    #[test]
    fn region_builder_test() {
        let mut builder = RegionBuilder::new();
        builder.set_name("Japan".to_string());

        let fail_res = builder.build();
        assert!(fail_res.is_err());

        let mut new_builder = RegionBuilder::new();
        new_builder.set_name("Japan".to_string());

        let small_port = Port::new(PortID::new(0), 100);
        let big_port = Port::new(PortID::new(1), 1000);
        let huge_port = Port::new(PortID::new(2), 10_000_000);

        new_builder.add_port(small_port);
        new_builder.add_port(big_port);
        new_builder.add_port(huge_port);


        new_builder.set_population(Population::new(5000));
        let success_res = new_builder.build();
        assert!(success_res.is_ok());

        let region = success_res.unwrap();
        assert!(region.get_port(PortID(0)).is_some());
        assert!(region.get_port(PortID(1)).is_some());
        assert!(region.get_port(PortID(2)).is_some());
        assert!(region.get_port(PortID(3)).is_none());
        assert_eq!(region.name, "Japan".to_string());
        assert_eq!(region.population, Population::new(5000));
    }

    #[test]
    fn region_construction_test() {
        let small_port = Port::new(PortID::new(0), 100);
        let big_port = Port::new(PortID::new(1), 1000);
        let huge_port = Port::new(PortID::new(2), 10_000_000);

        let country = Region::new("Super".to_owned(), 100, vec![small_port, big_port]);
        let big_country = Region::new("Mega".to_owned(), 1_000_000, vec![huge_port]);

        // make sure countries have unique ID
        assert_ne!(country.id, big_country.id);

        // make sure each country's ports have their region's id
        for port in country.ports {
            assert_eq!(port.region.unwrap(), country.id)
        }

        for port in big_country.ports {
            assert_eq!(port.region.unwrap(), big_country.id)
        }
    }
}

