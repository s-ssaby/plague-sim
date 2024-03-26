use crate::{location::{Location, Point2D}, region::{Port, PortID, Region, RegionID}, transportation_graph::PortGraph};

/// Responsible for storing simulation geography data and communicating changes across its components
/// 
/// Assumes that every port in the graph has a unique ID
/// 
/// Assumes that every port in all the regions has a unique ID
/// 
/// Assumes that all ports contained in the regions are the same as all the ports in the graph
pub struct SimulationGeography<T = Point2D> where T: Location {
    graph: PortGraph<T>,
    regions: Vec<Region<T>>
}

// Invariants:
// If a port with a certain ID exists in both graph and regions, their states must be equal
impl<T> SimulationGeography <T> where T: Location {
    pub fn new(graph: PortGraph<T>, regions: Vec<Region<T>>) -> Self {
        Self { graph, regions }
    }

    fn find_port_in_regions(&self, port_id: PortID) -> Option<&Port<T>> {
        for region in &self.regions {
            let result = region.ports.iter().find(|port| port.id == port_id);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    /* Returns an iterator over contained regions */
    pub fn get_regions(&self) -> impl IntoIterator + '_ {
        &self.regions
    }

    /* Returns an iterator over contained ports */
    pub fn get_ports(&self) -> impl IntoIterator + '_ {
        self.graph.get_ports()
    }

    /* Gets possible destination ports of a port, if it exists */
    pub fn get_all_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
       self.graph.get_dest_ports(id)
    }

    /* Gets open destination ports of a port, if it exists */
    pub fn get_open_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
        self.graph.get_open_dest_ports(id)
    }

    /* Find region with given ID, if it exists */
    pub fn get_region(&self, region_id: RegionID) -> Option<&Region<T>> {
        self.regions.iter().find(|region| region.id == region_id)
    }

    /* Find port with given ID, if it exists */
    pub fn get_port(&self, port_id: PortID) -> Option<&Port<T>> {
        self.graph.get_port(port_id)
    }

    /* Closes port with given ID, if it exists  */
    pub fn close_port(&mut self, port_id: PortID) -> Result<(), String>{
        let region_port = self.find_port_in_regions(port_id);
        let graph_port = self.graph.get_port(port_id);
        if region_port.is_none() {
            Err(format!("Cannot close port with ID {} because it wasn't found in any region", port_id.0))
        } else if graph_port.is_none() {
            Err(format!("Cannot close port with ID {} because it wasn't found in graph", port_id.0))
        } else {
            region_port.unwrap().close_port();
            graph_port.unwrap().close_port();
            Ok(())
        }
    }
}