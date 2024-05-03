// Responsible for calculating ways to allocate people to transportation

use crate::{point::{Point2D}, math_utils::{get_random, pick_random}, population_types::{population::Population, PopulationType}, region::{Port, PortID, Region, RegionID}};



/** Determines how to create a transport job when given a starting port and its region and where it can travel to */
/** Implementations must satisfy the following properties: */
/** - The total population must be able to be extracted from the start region */
/**     - For example, you cannot transport 2 infected individuals from a population of 50 healthy ones */
/** - Use None to communicate that no jobs could be created, e.g. region is uninhabited */
pub trait TransportAllocator<P = Population> where P: PopulationType {
    fn calculate_transport<'a>(&self, start_port: &Port, start_region: &Region<P>, destination_port_choices: Vec<&Port>) -> Option<Vec<TransportJob>>;
}

/// Randomly choose a port to travel to, and transport a random number of people up to the starting port's capacity
/// transport_probability refers to chance that a transport will occur, if possible
/** Population transported reflects composition of starting region
 * For example, this allocator will have a transport consisting of roughly 50% infected if starting region is also 50% infected */
pub struct RandomTransportAllocator {
    pub transport_probability: f32
}

impl RandomTransportAllocator {
    pub fn new(transport_probability: f32) -> Self {
        Self {transport_probability}
    }
}

impl<P: PopulationType> TransportAllocator <P> for RandomTransportAllocator {
    fn calculate_transport<'a>(&self, start_port: &Port, start_region: &Region<P>, destination_port_choices: Vec<&Port>) -> Option<Vec<TransportJob>> {
        // only prepare a transport if random chance favors it
        if (get_random() as f32) < self.transport_probability {
            let random_dest = pick_random(destination_port_choices);
            match random_dest {
                Some(dest) => {
                    let random_pop = ((start_port.capacity + 1) as f64*get_random()) as u32;
                    // do not transport if empty
                    if random_pop == 0 {
                        return None;
                    }
                    let transported_population;
                    // transport entire population
                    if random_pop >= start_region.population.population().get_total() {
                        transported_population = start_region.population.population();
                    } 
                    // transport only portion
                    else {
                        let scale_factor = (random_pop as f64)/(start_region.population.population().get_total() as f64);
                        transported_population = start_region.population.population().scale(scale_factor);
                    }
                    debug_assert!(transported_population.healthy <= start_region.population.population().healthy, "{}", 
                    format!("Unable to remove {} healthy from {} healthy", transported_population.healthy, start_region.population.population().healthy));
                    debug_assert!(transported_population.dead <= start_region.population.population().dead, "{}", 
                    format!("Unable to remove {} dead from {} dead", transported_population.dead, start_region.population.population().dead));
                    debug_assert!(transported_population.infected <= start_region.population.population().infected, "{}", 
                    format!("Unable to remove {} infected from {} infected", transported_population.infected, start_region.population.population().infected));
                    debug_assert!(transported_population.recovered <= start_region.population.population().recovered, "{}", 
                    format!("Unable to remove {} recovered from {} recovered", transported_population.recovered, start_region.population.population().recovered));
                    // TODO! Change time calculation later to allow changes in speed
                    let distance = start_port.pos.distance(&dest.pos) as u32;
                    Some(vec![TransportJob {start_region: start_region.id(), start_port: start_port.id, end_region: dest.region, end_port: dest.id, population: transported_population, time: distance}])
                },
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct TransportJob {
    pub start_port: PortID,
    pub start_region: RegionID,
    pub end_port: PortID,
    pub end_region: RegionID,
    pub population: Population,
    pub time: u32
}

#[cfg(test)]
mod test {
    use crate::{point::Point2D, population_types::population::Population, region::{PortID, Region}};

    use super::{RandomTransportAllocator, TransportAllocator};

    /** This test may pass or fail by random chance */
    #[test]
    fn random_transport_allocator() {
        let mut brazil: Region = Region::new("Brazil".to_owned(), Population::new_healthy(50000));
        brazil.population = Population::new_random(50000);
        let braz_port = brazil.add_port(PortID(0), 500, Point2D::new(0.0, 0.0));

        let mut benin: Region = Region::new("Benin".to_owned(), Population::new_healthy(30000));
        let benin_port = benin.add_port(PortID(1), 500, Point2D::new(10.0, 2.0));
        benin.population = Population::new_random(30000);

        let random_alloc = RandomTransportAllocator::new(1.0);
        // Repeat process 30 times to prevent chance of test passing by fluke
        for i in 0..=30 {
            let brazil_curr_pop = brazil.population;
            let brasil_to_benin_jobs = random_alloc.calculate_transport(&braz_port, &brazil, vec![&benin_port]);

            // try to transport
            for job in brasil_to_benin_jobs.unwrap() {
                let transport_pop = job.population;
                let result = brazil_curr_pop.emigrate(transport_pop);
                assert!(&result.is_ok(), "{}", format!("Error on update {}: {}", i + 1, result.err().unwrap()));
            }
        }


    }
}