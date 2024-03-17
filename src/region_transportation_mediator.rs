use std::collections::HashMap;

use crate::{population::Population, region::{Region, RegionID}, transportation_graph::PortGraph};


// Controls transportation interactions between the regions it possesses
/** Assumes that every port in provided port graph belongs to a region */
/** Once regions added, cannot add more or take away */
struct RegionTransportationMediator <'a> {
    regions: HashMap<RegionID, Region>,
    port_graph: &'a PortGraph<'a>,
    ongoing_transport: Vec<TransportJob>
}

impl<'med> RegionTransportationMediator<'med> {
    fn new(port_graph: &'med PortGraph<'med>, regions: Vec<Region>) -> Self {
        let mut region_map = HashMap::new();
        for region in regions {
            region_map.insert(region.id, region);
        }
        Self { regions: region_map, port_graph, ongoing_transport: vec![]}
    }

    // create interactions between regions for each region
    // also updates populations of regions when people leave
    fn update(&'med mut self) {
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
            let new_jobs = Self::calculate_transport_jobs(self.port_graph, region);
            all_new_jobs.extend(new_jobs);
        }

        self.ongoing_transport.extend(all_new_jobs);

    }

    // calculate transport jobs for a region
    fn calculate_transport_jobs(port_graph: &'med PortGraph<'med>, region: &'med mut Region) -> Vec<TransportJob> {
        let mut jobs: Vec<TransportJob> = vec![];
        // look at each port
        for port in &region.ports {
            // where can each port go to?
            let port_dests = port_graph.get_dest_ports(port);

            // pick random port to travel to
            let port_index = ((port_dests.len() as f64)*fastrand::f64()) as u32;
            let dest_port = port_dests[port_index as usize];

            // move out people and create transport job            
            match dest_port.region {
                Some(end_region) => {
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
                },
                None => panic!("Destination port not part of a region!"),
            }
        }
        jobs
    }

    /** Retrieves number of people currently in transit (aka not residing in any region currently) */
    pub fn get_in_transit(&self) -> u32 {
        let mut total: u32 = 0;
        for job in self.ongoing_transport {
            let pop_total = job.population.dead + job.population.healthy + job.population.infected + job.population.recovered;
            total += pop_total;
        }
        total
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
    use crate::{region::Region, transportation::Port, transportation_graph::PortGraph};

    use super::RegionTransportationMediator;

    #[test]
    fn test_mediator_intra_country_transport() {

    }

    #[test]
    fn test_mediator_inter_country_transport() {

    }
    #[test]
    fn test_mediator_all_transport() {
        let us_cal_port = Port::new(1000);
        let us_ny_port = Port::new(1000);
        
        let france_port = Port::new(500);
        let poland_port = Port::new(500);

        let beijing_port = Port::new(2000);
        let hong_kong_port = Port::new(2000);

        let mut port_graph = PortGraph::new();
        port_graph.add_port(&us_cal_port);
        port_graph.add_port(&us_ny_port);
        port_graph.add_port(&france_port);
        port_graph.add_port(&poland_port);
        port_graph.add_port(&beijing_port);
        port_graph.add_port(&hong_kong_port);

        // add connections
        // start in US cal
        port_graph.add_connection(&us_cal_port, &us_ny_port);
        port_graph.add_connection(&us_ny_port, &france_port);
        port_graph.add_connection(&france_port, &poland_port);
        port_graph.add_connection(&poland_port, &beijing_port);
        port_graph.add_connection(&beijing_port, &hong_kong_port);
        port_graph.add_connection(&hong_kong_port, &us_cal_port);

        let us_ports = vec![us_cal_port, us_ny_port];
        let europe_ports = vec![france_port, poland_port];
        let china_ports = vec![beijing_port, hong_kong_port];

        // Create regions
        let US = Region::new("United States".to_owned(), 3000, us_ports);
        let china = Region::new("China".to_owned(), 10000, china_ports);
        let europe = Region::new("Europe".to_owned(), 5000, europe_ports);

        // create mediator, add regions
        let med = RegionTransportationMediator::new(&port_graph, vec![US, china, europe]);
        
    }
}

