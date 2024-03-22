use std::collections::HashMap;

use crate::{location::Location, population::Population, region::{Region, RegionID}, transportation_allocator::{RandomTransportAllocator, TransportAllocator, TransportJob}, transportation_graph::PortGraph};


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
pub struct RegionTransportationMediator<A, T> where A: Location, T: TransportAllocator<A>{
    regions: HashMap<RegionID, Region<A>>,
    port_graph: PortGraph<A>,
    allocator: T,
    ongoing_transport: Vec<TransportJob>,
    pub statistics: MediatorStatistics
}

impl<'a, A: Location + 'a, T: TransportAllocator<A>> RegionTransportationMediator<A, T> {
    pub fn new(port_graph: PortGraph<A>, regions: Vec<Region<A>>, allocator: T) -> Self {
        let mut region_map = HashMap::new();
        for region in regions {
            region_map.insert(region.id, region);
        }
        let total_pop = Self::calculate_regions_population(region_map.values());
        Self { regions: region_map, port_graph, ongoing_transport: vec![], statistics: MediatorStatistics::new(total_pop), allocator}
    }

    /** Calculates population contained in simulation's regions */
    fn calculate_regions_population (regions: impl Iterator<Item = &'a Region<A>>) -> Population {
        regions.map(|reg| reg.population).fold(Population::new(0), |acc, pop| acc + pop)
    }

    /** Calculates population currently in transit */
    fn calculate_transit_population (jobs: impl Iterator<Item = &'a TransportJob>) -> Population {
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
            let new_jobs = Self::calculate_transport_jobs(&self.port_graph, region, &self.allocator);
            all_new_jobs.extend(new_jobs);
        }

        self.ongoing_transport.extend(all_new_jobs);

        // update stats
        self.update_statistics()
    }

    // calculate transport jobs for a region
    fn calculate_transport_jobs(port_graph: &PortGraph<A>, region: &mut Region<A>, allocator: &T) -> Vec<TransportJob> {
        let mut new_jobs: Vec<TransportJob> = vec![];
        // look at each port
        for port in &region.ports {
            // where can each port go to?
            let port_dests = port_graph.get_dest_ports(port.id).unwrap();

            // calculate transport jobs
            let calculated_jobs = allocator.calculate_transport(port, region, port_dests);
            for job in calculated_jobs {
                match region.population.emigrate(job.population) {
                    Ok(new_pop) => {
                        region.population = new_pop;
                        // assume transportation takes 2 days
                        new_jobs.push(job)
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
        new_jobs
    }
}

#[cfg(test)]
mod tests {


    use crate::{config::{load_config_data, ConfigData}, location::Point2D, region::{PortID, Region}, transportation_allocator::RandomTransportAllocator, transportation_graph::PortGraph};

    use super::RegionTransportationMediator;

    #[test]
    /** Tests simulations where all transport connections occur within same region */
    fn test_mediator_intra_country_transport() {
        let china_pop = 5000;
        let mut china = Region::new("China".to_owned(), china_pop);
        let port1 = china.add_port(PortID(1), 100, Point2D::default());
        let port2 = china.add_port(PortID(2), 200, Point2D::default());
        let port3 = china.add_port(PortID(3), 500, Point2D::default());
        let port4 = china.add_port(PortID(4), 50, Point2D::default());

        let mut graph = PortGraph::new();
        graph.add_port(port1);
        graph.add_port(port2);
        graph.add_port(port3);
        graph.add_port(port4);

        graph.add_directed_connection(PortID(1), PortID(2));
        graph.add_directed_connection(PortID(2), PortID(3));
        graph.add_directed_connection(PortID(3), PortID(4));
        graph.add_directed_connection(PortID(4), PortID(1));
        graph.add_directed_connection(PortID(3), PortID(1));

        // make mediator
        let mut med: RegionTransportationMediator<Point2D, RandomTransportAllocator> = RegionTransportationMediator::new(graph, vec![china], RandomTransportAllocator);

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
        let config = load_config_data("test_data/data.json").unwrap();
     
        // create mediator, add regions
        let mut med: RegionTransportationMediator<Point2D, RandomTransportAllocator> = RegionTransportationMediator::new(config.graph, config.regions, RandomTransportAllocator);

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
        let config: ConfigData<Point2D> = load_config_data("test_data/data.json").unwrap();

        let mut graph = config.graph;
        // add all possible connections, ignoring errors
        for start_id in 0..=10 {
            for end_id in 0..=10 {
                let _ = graph.add_directed_connection(PortID(start_id), PortID(end_id));
            }
        }

        // create mediator, add regions
        let mut med: RegionTransportationMediator<Point2D, RandomTransportAllocator> = RegionTransportationMediator::new(graph, config.regions, RandomTransportAllocator);

        // make sure that number of people living in regions plus number in transit always stays same
        let total = med.statistics.in_transit + med.statistics.region_population;
        for _ in 0..=20 {
            med.update();
            assert_eq!(med.statistics.in_transit + med.statistics.region_population, total);
        }
    }
}

