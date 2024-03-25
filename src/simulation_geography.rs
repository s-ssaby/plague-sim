use crate::{region::{Port, PortID, Region}, transportation_graph::PortGraph};

/// Responsible for storing simulation geography data and communicating changes across its components
/// 
/// Assumes that every port in the graph has a unique ID
/// 
/// Assumes that every port in all the regions has a unique ID
pub struct SimulationGeography {
    graph: PortGraph,
    regions: Vec<Region>
}

// Invariants:
// If a port with a certain ID exists in both graph and regions, their states must be equal
impl SimulationGeography {
    pub fn new(graph: PortGraph, regions: Vec<Region>) -> Self {
        Self { graph, regions }
    }

    fn find_port_in_regions(&self, port_id: PortID) -> Option<&Port> {
        for region in &self.regions {
            let result = region.ports.iter().find(|port| port.id == port_id);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    /* Find port with given ID, if it exists */
    pub fn get_port(&self, port_id: PortID) -> Option<&Port> {
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