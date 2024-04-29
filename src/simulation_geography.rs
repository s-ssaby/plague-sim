use std::{fmt::format, slice::Iter};

use crate::{location::{Location, Point2D}, population_types::{population::Population, PopulationType}, region::{Port, PortID, Region, RegionID}, transportation_graph::PortGraph};

/// Responsible for storing simulation geography data and communicating changes across its components
/// 
/// Assumes that every port in the graph has a unique ID
/// 
/// Assumes that every port in all the regions has a unique ID
/// 
/// Assumes that all ports contained in the regions are the same as all the ports in the graph
pub struct SimulationGeography<P: PopulationType, T = Point2D> where P: PopulationType, T: Location {
    graph: PortGraph<T>,
    regions: Vec<Region<P, T>>
}

// Invariants:
// If a port with a certain ID exists in both graph and regions, their states must be equal
impl<P, T> SimulationGeography <P, T> where T: Location, P: PopulationType {
    pub fn new(graph: PortGraph<T>, regions: Vec<Region<P, T>>) -> Self {
        Self { graph, regions }
    }

    fn find_port_in_regions(&self, port_id: PortID) -> Option<&Port<T>> {
        for region in &self.regions {
            let result = region.get_ports().iter().find(|port| port.id == port_id);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    /* Find region with given ID, if it exists */
    pub fn get_region(&self, region_id: RegionID) -> Option<&Region<P, T>> {
        self.regions.iter().find(|region| region.id() == region_id)
    }

    fn get_region_mut(&mut self, region_id: RegionID) -> Option<&mut Region<P, T>> {
        self.regions.iter_mut().find(|region| region.id() == region_id)
    }

    /* Find port with given ID, if it exists */
    pub fn get_port(&self, port_id: PortID) -> Option<&Port<T>> {
        self.graph.get_port(port_id)
    }


    /* Returns population of specified region, if it exists */
    pub fn get_population(&self, region_id: RegionID) -> Option<&P> {
        let region = self.get_region(region_id);
        region.map(|some_region| &some_region.population)
    }

    /* Set population of specified region, if it exists */
    pub fn set_population(&mut self, region_id: RegionID, population: Population) -> Result<(), String> {
        self.get_region_mut(region_id).map(|region| region.population.set_population(population)).ok_or(format!("Cannot find region ID {}", region_id))
    }

    /* Add given population to population of specified region, if it exists */
    pub fn add_population(&mut self, region_id: RegionID, population: Population) -> Result<Population, String> {
        let region = self.get_region_mut(region_id);
        match region {
            Some(unwrapped_region) => {
                let resulting_pop = unwrapped_region.population.population() + population;
                Ok(resulting_pop)
            },
            None => Err(format!("Cannot find region ID {}", region_id)),
        }
    
    }

    /// Removes given population from region, if found
    /// Returns new population of region
    /// # Errors
    /// * Fails if region ID not found
    /// * Fails if the given population cannot be subtracted from the region's population
    pub fn subtract_population(&mut self, region_id: RegionID, population: Population) -> Result<Population, String> {
        let region = self.get_region_mut(region_id);
        match region {
            Some(unwrapped_region) => {
                // for debugging purposes
                let start_pop = unwrapped_region.population.population().get_total();

                let resulting_pop = unwrapped_region.population.population().emigrate(population);
                match resulting_pop {
                    Ok(new_pop) => {
                        debug_assert_eq!(start_pop, new_pop.get_total() + population.get_total());
                        unwrapped_region.population.set_population(new_pop);
                        Ok(new_pop)
                    },
                    Err(e) => Err(e),
                }
            },
            None => Err(format!("Cannot find region ID {}", region_id)),
        }
    }

    /* Returns contained regions */
    pub fn get_regions(&self) -> Iter<'_, Region<P, T>> {
        self.regions.iter()
    }

    /* Returns IDs of contained regions */
    pub fn get_region_ids(&self) -> Vec<RegionID> {
        self.regions.iter().map(|reg| reg.id()).collect()
    }

    /* Returns contained ports */
    pub fn get_ports(&self) -> Vec<&Port<T>> {
        self.graph.get_ports()
    }

    /* Gets possible destination ports of a port, if it exists */
    pub fn get_all_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
       self.graph.get_dest_ports(id)
    }

    /* Gets open destination ports of a port, if it exists */
    pub fn get_open_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
        self.graph.get_open_dest_ports(id)
    }

    /* Closes port with given ID, if it exists  */
    pub fn close_port(&mut self, port_id: PortID) -> Result<(), String>{
        let region_port = self.find_port_in_regions(port_id);
        let graph_port = self.graph.get_port(port_id);
        if region_port.is_none() {
            Err(format!("Cannot close port with ID {} because it wasn't found in any region", port_id.0))
        } else if graph_port.is_none() {
            Err(format!("Cannot close port with ID {} because it wasn't found in graph", port_id.0))
        } else {
            region_port.unwrap().close_port();
            graph_port.unwrap().close_port();
            Ok(())
        }
    }
}