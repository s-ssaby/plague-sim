use std::collections::HashMap;

use crate::{population::Population, region::{Region, RegionID}, transportation_graph::PortGraph};


/** Stores data not necessary for mediator's functioning, but may be useful for clients */
pub struct MediatorStatistics {
    /** Total population currently in transit */
    pub in_transit: Population,
    /** Total population living in regions */
    pub total_population: Population
}

impl MediatorStatistics {
    fn new (total_population: Population) -> Self {
        Self { in_transit: Population::new(0), total_population }
    }
}

// Controls transportation interactions between the regions it possesses
/** Assumes that every port in provided port graph belongs to a region */
/** Once regions added, cannot add more or take away */
pub struct RegionTransportationMediator {
    regions: HashMap<RegionID, Region>,
    port_graph: PortGraph,
    ongoing_transport: Vec<TransportJob>,
    pub statistics: MediatorStatistics
}

impl RegionTransportationMediator {
    pub fn new(port_graph: PortGraph, regions: Vec<Region>) -> Self {
        let mut region_map = HashMap::new();
        for region in regions {
            region_map.insert(region.id, region);
        }
        let total_pop = Self::calculate_regions_population(region_map.values());
        Self { regions: region_map, port_graph, ongoing_transport: vec![], statistics: MediatorStatistics::new(total_pop)}
    }

    /** Calculates population contained in simulation's regions */
    fn calculate_regions_population <'a> (regions: impl Iterator<Item = &'a Region>) -> Population {
        unimplemented!()
    }

    /** Calculates population currently in transit */
    fn calculate_transit_population(jobs: Vec<TransportJob>) -> Population {
        unimplemented!()
    }

    /** Updates statistics of simulation to reflect current state */
    fn update_statistics(&mut self) {
        unimplemented!()
    }

    // create interactions between regions for each region
    // also updates populations of regions when people leave
    pub fn update(&mut self) {
        // process jobs
        self.ongoing_transport.retain_mut(|job| {
            if job.time == 0 {
                // update end region
                let end_region = self.regions.get_mut(&job.end_region);
                match end_region {
                    Some(unwrapped_end_reg) => {
                        unwrapped_end_reg.population = unwrapped_end_reg.population + job.population;
                        return  false;
                    },
                    None => panic!("Region that job is referring to doesn't exist in mediator"),
                }
            } else {
                job.time -= 1;
                return true;
            }
        });

        let mut all_new_jobs: Vec<TransportJob> = vec![];
        // generate new jobs
        for region in self.regions.values_mut() {
            let new_jobs = Self::calculate_transport_jobs(&self.port_graph, region);
            all_new_jobs.extend(new_jobs);
        }

        self.ongoing_transport.extend(all_new_jobs);

        // update stats
        self.update_statistics()
    }

    // calculate transport jobs for a region
    fn calculate_transport_jobs(port_graph: &PortGraph, region: &mut Region) -> Vec<TransportJob> {
        let mut jobs: Vec<TransportJob> = vec![];
        // look at each port
        for port in &region.ports {
            // where can each port go to?
            let port_dests = port_graph.get_dest_ports(port.id);

            // pick random port to travel to
            let port_index = ((port_dests.as_ref().unwrap().len() as f64)*fastrand::f64()) as u32;
            let dest_port = port_dests.unwrap()[port_index as usize];

            // move out people and create transport job            
            let end_region = dest_port.region;
            let population = Population::new(100);
            match region.population.emigrate(population) {
                Ok(_) => {
                    let start_region = region.id;
                    // assume transportation takes 2 days
                    let job = TransportJob {start_region, end_region, population, time: 2};
                    jobs.push(job)
                },
                Err(e) => panic!("{}", e),
            }
        }
        jobs
    }
}

struct TransportJob {
    start_region: RegionID,
    end_region: RegionID,
    population: Population,
    time: u32
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, vec};


    use crate::{config::load_config_data, population::Population, region::{Port, PortID, Region}, transportation_graph::PortGraph};

    use super::RegionTransportationMediator;

    #[test]
    fn test_mediator_intra_country_transport() {

    }

    #[test]
    fn test_mediator_inter_country_transport() {

    }
    #[test]
    fn test_mediator_all_transport() {
        let config = load_config_data("src/countries.txt", "src/connections.txt").unwrap();
     
        // create mediator, add regions
        let med = RegionTransportationMediator::new(config.graph, config.regions);
    }
}

