use std::collections::HashMap;

use functionality::{location::Location, population_types::population::Population, region::{Region, RegionID}, simulation_geography::SimulationGeography, transportation_allocator::{TransportAllocator, TransportJob}};



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
pub struct Simulation<A, T> where A: Location, T: TransportAllocator<A>{
    pub geography: SimulationGeography<A>,
    allocator: T,
    pub ongoing_transport: Vec<InProgressJob>,
    pub statistics: MediatorStatistics
}

impl<'a, A: Location + 'a, T: TransportAllocator<A>> Simulation<A, T> {
    pub fn new(geography: SimulationGeography<A>, allocator: T) -> Self {
        let total_pop = Self::calculate_regions_population(geography.get_regions());
        Self {geography, ongoing_transport: vec![], statistics: MediatorStatistics::new(total_pop), allocator}
    }

    /** Calculates population contained in simulation's regions */
    fn calculate_regions_population (regions: impl Iterator<Item = &'a Region<A>>) -> Population {
        regions.map(|reg| reg.population).fold(Population::new_healthy(0), |acc, pop| acc + pop)
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
                        self.geography.add_population(unwrapped_end_reg.id, job.job.population);
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
            all_new_jobs.extend(new_jobs);
        }


        self.ongoing_transport.extend(all_new_jobs);

        // update stats
        self.update_statistics()
    }

    // calculate transport jobs for a region
    fn calculate_transport_jobs(geography: &mut SimulationGeography<A>, region_id: RegionID, allocator: &T) -> Vec<InProgressJob> {
        let mut new_jobs: Vec<InProgressJob> = vec![];
        
        {
        let region = geography.get_region(region_id).unwrap();
        // look at each port
        for port in &region.ports {
            // where can each port go to?
            let port_dests = geography.get_open_dest_ports(port.id).unwrap();

            // calculate transport jobs
            let calculated_jobs = allocator.calculate_transport(port, region, port_dests);
            for job in calculated_jobs.unwrap_or(vec![]) {
                match region.population.emigrate(job.population) {
                    Ok(new_pop) => {
                        // assume transportation takes 2 days
                        new_jobs.push(InProgressJob::new(job))
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
        }

        // now modify region
        for job in &new_jobs {
            geography.subtract_population(region_id, job.job.population);
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