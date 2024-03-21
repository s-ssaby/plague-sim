// Responsible for calculating ways to allocate people to transportation

use crate::{location::Location, math_utils::{get_random, pick_random}, population::Population, region::{Port, Region, RegionID}};



/** Determines how to create a transport job when given a starting port and its region and where it can travel to */
/** Implementations must return a TransportJob satisfy the following properties: */
/** - The population must be able to be extracted from the start region */
/**     - For example, you cannot transport 2 infected individuals from a population of 50 healthy ones */
pub trait TransportAllocator<T> where T: Location {
    fn calculate_transport<'a>(&self, start_port: &Port<T>, start_region: &Region<T>, destination_port_choices: Vec<&Port<T>>) -> Option<TransportJob>;
}

/** Randomly choose a port to travel to, and transport a random number of people up to the starting port's capacity */
///
/** Population transported reflects composition of starting region
 * For example, this allocator will have a transport consisting of roughly 50% infected if starting region is also 50% infected */
pub struct RandomTransportAllocator;

impl<T: Location> TransportAllocator <T> for RandomTransportAllocator {
    fn calculate_transport<'a>(&self, start_port: &Port<T>, start_region: &Region<T>, destination_port_choices: Vec<&Port<T>>) -> Option<TransportJob> {
        let random_dest = pick_random(destination_port_choices);
        random_dest.map(|dest| {
            let random_pop = ((start_port.capacity + 1) as f64*get_random()) as u32;
            let scale_factor = (random_pop as f64)/(start_region.population.get_total() as f64);
            let transported_population = start_region.population.scale(scale_factor); 
            // TODO! Change time calculation later
            TransportJob {start_region: start_region.id, end_region: dest.region, population: transported_population, time: 5}
        })
    }
}

pub struct TransportJob {
    pub start_region: RegionID,
    pub end_region: RegionID,
    pub population: Population,
    pub time: u32
}