use std::{cell::RefCell, collections::HashMap};

use crate::{population::Population, region::{Region, RegionID}, transportation_graph::PortGraph};


// Controls transportation interactions between the regions it possesses
/** Assumes that every port in provided port graph belongs to a region */
struct RegionTransportationMediator <'a> {
    regions: HashMap<RegionID, Region>,
    port_graph: &'a PortGraph<'a>,
    ongoing_transport: Vec<TransportJob>
}

impl<'med> RegionTransportationMediator<'med> {
    fn new(port_graph: &'med PortGraph<'med>) -> Self {
        Self { regions: HashMap::new(), port_graph, ongoing_transport: vec![]}
    }

    fn add_region(&mut self, region: Region) {
        let id = region.id;
        self.regions.insert(id, region);
    }

    // create interactions between regions for each region
    // also updates populations of regions when people leave
    fn update(&'med mut self) {
        // process jobs
        self.ongoing_transport.retain_mut(|job| {
            if (job.time == 0) {
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
        for (_region_id, region) in &mut self.regions {
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
}

struct TransportJob {
    start_region: RegionID,
    end_region: RegionID,
    population: Population,
    time: u32
}

impl TransportJob {
    // new job but day less
    // Errors if job already finished
    pub fn tick(&self) -> Result<Self, String> {
        if self.time == 0 {
            Err("Unable to tick finished job, time already 0".to_owned())
        } else {
            Ok(TransportJob {start_region: self.start_region, end_region: self.end_region, population: self.population, time: self.time - 1 })
        }
    }
}