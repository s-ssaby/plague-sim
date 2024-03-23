// Responsible for calculating ways to allocate people to transportation

use crate::{location::{Location, Point2D}, math_utils::{get_random, pick_random}, population::Population, region::{Port, Region, RegionID}};



/** Determines how to create a transport job when given a starting port and its region and where it can travel to */
/** Implementations must return a TransportJob satisfy the following properties: */
/** - The population must be able to be extracted from the start region */
/**     - For example, you cannot transport 2 infected individuals from a population of 50 healthy ones */
pub trait TransportAllocator<T = Point2D> where T: Location {
    fn calculate_transport<'a>(&self, start_port: &Port<T>, start_region: &Region<T>, destination_port_choices: Vec<&Port<T>>) -> Vec<TransportJob>;
}

/** Randomly choose a port to travel to, and transport a random number of people up to the starting port's capacity */
///
/** Population transported reflects composition of starting region
 * For example, this allocator will have a transport consisting of roughly 50% infected if starting region is also 50% infected */
pub struct RandomTransportAllocator;

impl<T: Location> TransportAllocator <T> for RandomTransportAllocator {
    fn calculate_transport<'a>(&self, start_port: &Port<T>, start_region: &Region<T>, destination_port_choices: Vec<&Port<T>>) -> Vec<TransportJob> {
        let random_dest = pick_random(destination_port_choices);
        match random_dest {
            Some(dest) => {
                let random_pop = ((start_port.capacity + 1) as f64*get_random()) as u32;
                let transported_population;
                // transport entire population
                if random_pop >= start_region.population.get_total() {
                    transported_population = start_region.population;
                } 
                // transport only portion
                else {
                    let scale_factor = (random_pop as f64)/(start_region.population.get_total() as f64);
                    transported_population = start_region.population.scale(scale_factor);
                }
                debug_assert!(transported_population.healthy <= start_region.population.healthy, "{}", 
                format!("Unable to remove {} healthy from {} healthy", transported_population.healthy, start_region.population.healthy));
                debug_assert!(transported_population.dead <= start_region.population.dead, "{}", 
                format!("Unable to remove {} dead from {} dead", transported_population.dead, start_region.population.dead));
                debug_assert!(transported_population.infected <= start_region.population.infected, "{}", 
                format!("Unable to remove {} infected from {} infected", transported_population.infected, start_region.population.infected));
                debug_assert!(transported_population.recovered <= start_region.population.recovered, "{}", 
                format!("Unable to remove {} recovered from {} recovered", transported_population.recovered, start_region.population.recovered));
                // TODO! Change time calculation later
                vec![TransportJob {start_region: start_region.id, end_region: dest.region, population: transported_population, time: 5}]
            },
            None => vec![],
        }
    }
}

pub struct TransportJob {
    pub start_region: RegionID,
    pub end_region: RegionID,
    pub population: Population,
    pub time: u32
}

#[cfg(test)]
mod test {
    use crate::{location::Point2D, population::Population, region::{PortID, Region}};

    use super::{RandomTransportAllocator, TransportAllocator};

    /** This test may pass or fail by random chance */
    #[test]
    fn random_transport_allocator() {
        let mut brazil: Region<Point2D> = Region::new("Brazil".to_owned(), 50000);
        brazil.population = Population::new_random(50000);
        let braz_port = brazil.add_port(PortID(0), 500, Point2D::new(0.0, 0.0));

        let mut benin: Region<Point2D> = Region::new("Benin".to_owned(), 30000);
        let benin_port = benin.add_port(PortID(1), 500, Point2D::new(10.0, 2.0));
        benin.population = Population::new_random(30000);

        let random_alloc = RandomTransportAllocator;
        // Repeat process 30 times to prevent chance of test passing by fluke
        for i in 0..=30 {
            let brazil_curr_pop = brazil.population;
            let brasil_to_benin_jobs = random_alloc.calculate_transport(&braz_port, &brazil, vec![&benin_port]);

            // try to transport
            for job in brasil_to_benin_jobs {
                let transport_pop = job.population;
                let result = brazil_curr_pop.emigrate(transport_pop);
                assert!(&result.is_ok(), "{}", format!("Error on update {}: {}", i + 1, result.err().unwrap()));
            }
        }


    }
}