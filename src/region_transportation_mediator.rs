use std::collections::HashMap;

use crate::{population::Population, region::{Region, RegionID}, transportation_graph::PortGraph};


/** Stores data not necessary for mediator's functioning, but may be useful for clients */
pub struct MediatorStatistics {
    /** Total population currently in transit */
    pub in_transit: Population,
    /** Total population living in regions */
    pub region_population: Population
}

impl MediatorStatistics {
    fn new (region_population: Population) -> Self {
        Self { in_transit: Population::new(0), region_population }
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
        regions.map(|reg| reg.population).fold(Population::new(0), |acc, pop| acc + pop)
    }

    /** Calculates population currently in transit */
    fn calculate_transit_population <'a> (jobs: impl Iterator<Item = &'a TransportJob>) -> Population {
        jobs.map(|job| job.population).fold(Population::new(0), |acc, pop| acc + pop)
    }

    /** Updates statistics of simulation to reflect current state */
    fn update_statistics(&mut self) {
        self.statistics.in_transit = Self::calculate_transit_population(self.ongoing_transport.iter());
        self.statistics.region_population = Self::calculate_regions_population(self.regions.values());
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


    use crate::{config::load_config_data, region::{PortID, Region}, transportation_graph::PortGraph};

    use super::RegionTransportationMediator;

    #[test]
    /** Tests simulations where all transport connections occur within same region */
    fn test_mediator_intra_country_transport() {
        let china_pop = 5000;
        let mut china = Region::new("China".to_owned(), china_pop);
        let port1 = china.add_port(PortID(1), 100);
        let port2 = china.add_port(PortID(2), 200);
        let port3 = china.add_port(PortID(3), 500);
        let port4 = china.add_port(PortID(4), 50);

        let mut graph = PortGraph::new();
        graph.add_port(port1);
        graph.add_port(port2);
        graph.add_port(port3);
        graph.add_port(port4);

        graph.add_connection(PortID(1), PortID(2));
        graph.add_connection(PortID(2), PortID(3));
        graph.add_connection(PortID(3), PortID(4));
        graph.add_connection(PortID(4), PortID(1));
        graph.add_connection(PortID(3), PortID(1));

        // make mediator
        let mut med = RegionTransportationMediator::new(graph, vec![china]);

        // make sure that number of people living in regions plus number in transit always stays same
        let total = med.statistics.in_transit + med.statistics.region_population;
        for _ in 0..=20 {
            med.update();
            assert_eq!(med.statistics.in_transit + med.statistics.region_population, total);
        }
    }

    #[test]
    /** Tests simulations where all transport connections occur only between different regions */
    fn test_mediator_inter_country_transport() {
        let config = load_config_data("src/countries.txt", "src/connections.txt").unwrap();
     
        // create mediator, add regions
        let mut med = RegionTransportationMediator::new(config.graph, config.regions);

        // make sure that number of people living in regions plus number in transit always stays same
        let total = med.statistics.in_transit + med.statistics.region_population;
        for _ in 0..=20 {
            med.update();
            assert_eq!(med.statistics.in_transit + med.statistics.region_population, total);
        }
    }
    #[test]
    /** Tests simulations where both inter and intra country transport occurs */
    fn test_mediator_all_transport() {
        let config = load_config_data("src/countries.txt", "src/connections.txt").unwrap();

        let mut graph = config.graph;
        // add all possible connections, ignoring errors
        for start_id in 0..=10 {
            for end_id in 0..=10 {
                let _ = graph.add_connection(PortID(start_id), PortID(end_id));
            }
        }

        // create mediator, add regions
        let mut med = RegionTransportationMediator::new(graph, config.regions);

        // make sure that number of people living in regions plus number in transit always stays same
        let total = med.statistics.in_transit + med.statistics.region_population;
        for _ in 0..=20 {
            med.update();
            assert_eq!(med.statistics.in_transit + med.statistics.region_population, total);
        }
    }
}

