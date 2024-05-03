use std::collections::HashMap;

use functionality::{point::Location, population_types::{population::Population, PopulationType}, region::{Region, RegionID}, simulation_geography::SimulationGeography, transportation_allocator::{TransportAllocator, TransportJob}};



/** Stores data not necessary for mediator's functioning, but may be useful for clients */
pub struct MediatorStatistics {
    /** Total population currently in transit */
    pub in_transit: Population,
    /** Total population living in regions */
    pub region_population: Population
}

impl MediatorStatistics {
    fn new (region_population: Population) -> Self {
        Self { in_transit: Population::new_healthy(0), region_population }
    }
}

// Controls transportation interactions between the regions it possesses
/** Assumes that every port in provided port graph belongs to a region */
/** Once regions added, cannot add more or take away */
pub struct Simulation<A, P,  T> where A: Location, P: PopulationType, T: TransportAllocator<P, A>{
    pub geography: SimulationGeography<P, A>,
    allocator: T,
    pub ongoing_transport: Vec<InProgressJob>,
    pub statistics: MediatorStatistics
}

impl<'a, A ,P,T> Simulation<A, P, T> where A: Location + 'a, P: PopulationType + 'a, T: TransportAllocator<P, A>{
    pub fn new(geography: SimulationGeography<P, A>, allocator: T) -> Self {
        let total_pop = Self::calculate_regions_population(geography.get_regions());
        Self {geography, ongoing_transport: vec![], statistics: MediatorStatistics::new(total_pop), allocator}
    }

    /** Calculates population contained in simulation's regions */
    fn calculate_regions_population (regions: impl Iterator<Item = &'a Region<P, A>>) -> Population {
        regions.map(|reg| reg.population.population()).fold(Population::new_healthy(0), |acc, pop| acc + pop.population())
    }

    /** Calculates population currently in transit */
    fn calculate_transit_population (jobs: impl Iterator<Item = &'a InProgressJob>) -> Population {
        jobs.map(|job| job.job.population).fold(Population::new_healthy(0), |acc, pop| acc + pop)
    }

    /** Updates statistics of simulation to reflect current state */
    fn update_statistics(&mut self) {
        self.statistics.in_transit = Self::calculate_transit_population(self.ongoing_transport.iter());
        self.statistics.region_population = Self::calculate_regions_population(self.geography.get_regions());
    }

    // create interactions between regions for each region
    // also updates populations of regions when people leave
    pub fn update(&mut self) {
        // process jobs
        self.ongoing_transport.retain_mut(|job| {
            if job.job.time == 0 {
                // update end region
                let end_region = self.geography.get_region(job.job.end_region);
                match end_region {
                    Some(unwrapped_end_reg) => {
                        self.geography.add_population(unwrapped_end_reg.id(), job.job.population);
                        return  false;
                    },
                    None => panic!("{}", format!("Region with ID {} that job is referring to doesn't exist in mediator", job.job.end_region)),
                }
            } else {
                job.job.time -= 1;
                return true;
            }
        });

        let mut all_new_jobs: Vec<InProgressJob> = vec![];

        // generate new jobs
        for region in self.geography.get_region_ids() {
            let new_jobs = Self::calculate_transport_jobs(&mut self.geography, region, &self.allocator);
            &all_new_jobs.extend(new_jobs);
        }

        // for debugging purposes
        let start_region_population = self.statistics.region_population.get_total();
        let start_transit_population = self.statistics.in_transit.get_total();

        // make people depart from regions after newly created jobs
        for job in &all_new_jobs {
            match self.geography.subtract_population(job.job.start_region, job.job.population) {
                Ok(_) => (),
                Err(e) => panic!("{}", format!("Failed to subtract {} people from region population of {} people. Error: {}", job.job.population.get_total(), self.geography.get_region(job.job.start_region).unwrap().population.population().get_total(), e))
            }
        }

        self.ongoing_transport.extend(all_new_jobs);

        // update stats
        self.update_statistics();

        // for debugging purposes
        let end_region_population = self.statistics.region_population.get_total();
        let end_transit_population = self.statistics.in_transit.get_total();

        debug_assert_eq!(start_region_population + start_transit_population, 
            end_region_population + end_transit_population,
            "{}", format!("Previous region population: {} Previous transit population: {} New region population: {} New transit population: {}",
            start_region_population, start_transit_population, end_region_population, end_transit_population));
    }

    // calculate transport jobs for a region
    fn calculate_transport_jobs(geography: &SimulationGeography<P, A>, region_id: RegionID, allocator: &T) -> Vec<InProgressJob> {
        let mut new_jobs: Vec<InProgressJob> = vec![];
        
        let region = geography.get_region(region_id).unwrap();
        // look at each port
        for port in region.get_ports() {
            // where can each port go to?
            let port_dests = geography.get_open_dest_ports(port.id).unwrap();

            // calculate transport jobs
            let calculated_jobs = allocator.calculate_transport(port, region, port_dests);
            for job in calculated_jobs.unwrap_or(vec![]) {
                match region.population.population().emigrate(job.population) {
                    Ok(new_pop) => {
                        // assume transportation takes 2 days
                        new_jobs.push(InProgressJob::new(job))
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
        new_jobs
    }
}

pub struct InProgressJob {
    pub job: TransportJob,
    pub expected_time: u32
}

impl InProgressJob {
    pub fn new(job: TransportJob) -> Self {
        Self {expected_time: job.time, job}
    }
}

#[cfg(test)]
mod tests {


    use functionality::{config::{load_config_data, ConfigData}, point::Point2D, population_types::population::Population, region::{PortID, Region}, simulation_geography::SimulationGeography, transportation_allocator::RandomTransportAllocator, transportation_graph::PortGraph};


    use super::Simulation;


    #[test]
    /** Tests simulations where all transport connections occur within same region */
    fn test_intra_country_transport() {
        let china_pop = 5000;
        let mut china = Region::new("China".to_owned(), Population::new_healthy(5000));
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

        // make simulation
        let mut sim: Simulation<Point2D, Population, RandomTransportAllocator> = Simulation::new(SimulationGeography::new(graph, vec![china]), RandomTransportAllocator::new(1.0));

        // make sure that number of people living in regions plus number in transit always stays same
        let total = sim.statistics.in_transit + sim.statistics.region_population;
        for _ in 0..=20 {
            sim.update();
            assert_eq!(sim.statistics.in_transit + sim.statistics.region_population, total);
        }
    }

    #[test]
    /** Tests simulations where all transport connections occur only between different regions */
    fn test_inter_country_transport() {
        let config = load_config_data("test_data/data.json").unwrap();
     
        // make simulation
        let mut sim: Simulation<Point2D, Population, RandomTransportAllocator> = Simulation::new(SimulationGeography::new(config.graph, config.regions), RandomTransportAllocator::new(1.0));

        // make sure that number of people living in regions plus number in transit always stays same
        let total = sim.statistics.in_transit + sim.statistics.region_population;
        for _ in 0..=20 {
            sim.update();
            assert_eq!(sim.statistics.in_transit + sim.statistics.region_population, total);
        }
    }

    #[test]
    /** Tests simulations where both inter and intra country transport occurs */
    fn test_all_transport() {
        let config: ConfigData = load_config_data("test_data/data.json").unwrap();

        let mut graph = config.graph;
        // add all possible connections, ignoring errors
        for start_id in 0..=10 {
            for end_id in 0..=10 {
                let _ = graph.add_directed_connection(PortID(start_id), PortID(end_id));
            }
        }

        // create mediator, add regions
        // make simulation
        let mut sim: Simulation<Point2D, Population, RandomTransportAllocator> = Simulation::new(SimulationGeography::new(graph, config.regions), RandomTransportAllocator::new(1.0));

        // make sure that number of people living in regions plus number in transit always stays same
        let total = sim.statistics.in_transit + sim.statistics.region_population;
        for _ in 0..=20 {
            sim.update();
            assert_eq!(sim.statistics.in_transit + sim.statistics.region_population, total);
        }
    }
}

