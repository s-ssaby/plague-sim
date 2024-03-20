// Responsible for calculating ways to allocate people to transportation

use crate::{population::Population, region::{Port, Region, RegionID}};


/** Determines how to create a transport job when given a starting port and its region and where it can travel to */
/** Implementations must return a TransportJob satisfy the following properties: */
/** - The population must be able to be extracted from the start region */
/**     - For example, you cannot transport 2 infected individuals from a population of 50 healthy ones */
pub trait TransportAllocator {
    fn calculate_transport<'a>(start_port: &Port, start_region: &Region, destination_port_choices: impl IntoIterator<Item = &'a Port>) -> TransportJob;
}

/** */
pub struct RandomTransportAllocator;

impl TransportAllocator for RandomTransportAllocator {
    fn calculate_transport<'a>(start_port: &Port, start_region: &Region, destination_port_choices: impl IntoIterator<Item = &'a Port>) -> TransportJob {
        todo!()
    }
}

pub struct TransportJob {
    pub start_region: RegionID,
    pub end_region: RegionID,
    pub population: Population,
    pub time: u32
}