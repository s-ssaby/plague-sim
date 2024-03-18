use std::collections::HashMap;

use crate::{population::Population, region::{Region, RegionID}, transportation_graph::PortGraph};


/** Stores data not necessary for mediator's functioning, but may be useful for clients */
pub struct MediatorStatistics {
    /** Number of people currently in transit */
    pub in_transit: u32
}

impl MediatorStatistics {
    fn new () -> Self {
        Self { in_transit: 0 }
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
        Self { regions: region_map, port_graph, ongoing_transport: vec![], statistics: MediatorStatistics::new()}
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
        let mut total = 0;
        for job in &self.ongoing_transport {
            total += job.population.infected + job.population.dead + job.population.recovered + job.population.healthy;
        }
        self.statistics.in_transit = total;
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

    use crate::{population::Population, region::{self, Region, RegionBuilder}, transportation::{Port, PortID}, transportation_graph::PortGraph};

    use super::RegionTransportationMediator;

    #[test]
    fn test_mediator_intra_country_transport() {

    }

    #[test]
    fn test_mediator_inter_country_transport() {

    }
    #[test]
    fn test_mediator_all_transport() {
        let mut ports: Vec<Port> = vec![];
        let file = fs::read_to_string("src/countries.txt").expect("Cannot find file");
        let mut file = file.lines();
        let mut current_line = file.next();
        let mut next_line = file.next();
        let mut regions: Vec<Region> = vec![];
        'outer: while let Some(current_line_unwrap) = current_line {
            //set current region
            if current_line_unwrap.starts_with("Region:") {
                let mut current_region = RegionBuilder::new();
                let mut parts = current_line_unwrap.split(":");
                let country_name = parts.nth(1).unwrap().to_owned();
                let population: u32 = parts.nth(0).unwrap().parse().expect("Region line doesn't have population");
                &current_region.set_name(country_name);
                &current_region.set_population(Population::new(population));
                'inner: while let Some(next_line_unwrap) = next_line {
                    let new_next_line = file.next();
                    if new_next_line.is_none() {
                        // add final port, then build region
                        let mut connections = next_line_unwrap.split(":");
                        let port_id: u32 = connections.next().unwrap().parse().expect("Port ID not found");
                        let capacity: u32 = connections.next().unwrap().parse().expect("Capacity not found");
                        let port = Port::new(PortID(port_id), capacity);
                        &current_region.ports.push(port.clone());
                        ports.push(port);
                        current_line = next_line;
                        next_line = new_next_line;
                        regions.push(current_region.build().expect("Failed to build region"));
                        break 'outer;
                    } else if next_line_unwrap.starts_with("Region:") {
                        regions.push(current_region.build().expect("Failed to build region"));
                        assert_ne!(current_line.unwrap(), next_line.unwrap());
                        current_line = next_line;
                        assert_ne!(next_line.unwrap(), new_next_line.unwrap());
                        next_line = new_next_line;
                        break 'inner;
                    } else {
                        // add port
                        let mut connections = next_line_unwrap.split(":");
                        let port_id: u32 = connections.next().unwrap().parse().expect("Port ID not found");
                        let capacity: u32 = connections.next().unwrap().parse().expect("Capacity not found");
                        let port = Port::new(PortID(port_id), capacity);
                        &current_region.ports.push(port.clone());
                        ports.push(port);
                        assert_ne!(current_line.unwrap(), next_line.unwrap());
                        current_line = next_line;
                        assert_ne!(next_line.unwrap(), new_next_line.unwrap());
                        next_line = new_next_line;
                    }
                }
            }
        }

        let expected_names = ["United States", "Europe", "China"];
        // Assert that regions correctly read in
        assert_eq!(regions[0].name, expected_names[0]);
        assert_eq!(regions[1].name, expected_names[1]);
        assert_eq!(regions[2].name, expected_names[2]);


        // create graph
        let mut graph = PortGraph::new();
        // add ports
        for port in ports {
            graph.add_port(port);
        }

        assert!(graph.in_graph(PortID(0)));
        assert!(graph.in_graph(PortID(1)));
        assert!(graph.in_graph(PortID(2)));
        assert!(graph.in_graph(PortID(3)));
        assert!(graph.in_graph(PortID(4)));
        assert!(graph.in_graph(PortID(5)));
        assert!(!graph.in_graph(PortID(6)));

        // read connections
        let mut connections = fs::read_to_string("src/connections.txt").expect("Cannot find file");
        let mut connections = connections.lines();
        while let Some(current_line) = connections.next() {
            let mut parts = current_line.split(":");
            let start_id = PortID(parts.next().unwrap().parse().expect("msg"));
            let end_id = PortID(parts.next().unwrap().parse().expect("msg"));
            graph.add_connection(start_id, end_id);
        }

        // all proper connections here?
        assert_eq!(graph.get_dest_ports(PortID(0)).unwrap(), vec![graph.get_port(PortID(1)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(1)).unwrap(), vec![graph.get_port(PortID(2)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(2)).unwrap(), vec![graph.get_port(PortID(3)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(3)).unwrap(), vec![graph.get_port(PortID(4)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(4)).unwrap(), vec![graph.get_port(PortID(5)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(5)).unwrap(), vec![graph.get_port(PortID(0)).unwrap()]);
     
        // create mediator, add regions
        let med = RegionTransportationMediator::new(graph, regions);
        
        
    }
}

