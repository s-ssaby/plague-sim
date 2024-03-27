#![allow(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{location::{Location, Point2D}, region::{Port, PortID}};



#[derive(Deserialize, Serialize, Debug)]
struct PortNode <T> where T: Location {
    port: Port<T>,
    dests: Vec<PortID>
}

impl<T> PortNode<T> where T: Location {
    pub fn new (port: Port<T>) -> Self {
        Self {port, dests: vec![]}
    }
}

/** Represents a graph of port connections */
#[derive(Deserialize, Serialize, Debug)]
pub struct PortGraph<T = Point2D> where T: Location {
    port_nodes: HashMap<PortID, PortNode<T>>
}

/* Ensure following invariants: */
// Every port in ports has a reference to its corresponding port node
// Every connection exists between nodes that exist in graph
impl <T> PortGraph <T> where T: Location {
    pub fn new() -> Self{
        PortGraph {port_nodes: HashMap::new()}
    }

    /** Returns references to all ports in graph */
    pub fn get_ports(&self) -> Vec<&Port<T>> {
        self.port_nodes.values().map(|node| &node.port).collect()
    }

    pub fn add_port(&mut self, port: Port<T>) -> Result<(), String> {
        let id = port.id;
        if self.in_graph(id) {
            Err(format!("Port with ID: {} already in graph", id.0))
        } else {
            let node = PortNode::new(port);
            self.port_nodes.insert(id, node);
            Ok(())
        }
    }

    pub fn in_graph(&self, id: PortID) -> bool {
        self.port_nodes.contains_key(&id)
    }

    fn get_node(&self, id: PortID) -> Option<&PortNode<T>> {
        self.port_nodes.values().find(|node| node.port.id == id)
    }

    fn get_mut_node(&mut self, id: PortID) -> Option<&mut PortNode<T>> {
        self.port_nodes.values_mut().find(|node| node.port.id == id)
    }

    pub fn get_port(&self, id: PortID) -> Option<&Port<T>> {
        let node = self.port_nodes.values().find(|node| node.port.id == id);
        match node {
            Some(node) => {
                Some(&node.port)
            },
            None => None,
        }
    }

    // gets possible destination ports of a port in graph, if it exists
    pub fn get_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
        // check if port in graph
        if !self.in_graph(id) {
            None
        } else {
            let mut dests: Vec<&Port<T>> = vec![];
            let node = self.get_node(id);
            if let Some(node) = node {
                for p_id in node.dests.iter() {
                    // find port
                    dests.push(self.get_port(*p_id).unwrap());
                }
            }
            Some(dests)
        }
    }

    pub fn get_open_dest_ports(&self, id: PortID) -> Option<Vec<&Port<T>>> {
        if !self.in_graph(id) {
            None
        } else {
            let dests = self.get_dest_ports(id).unwrap();
            let mut open_dests: Vec<&Port<T>> = vec![];
            for dest in &dests {
                if !dest.is_closed() {
                    open_dests.push(dest);
                }
            }
            Some(dests)
        }
    }

    pub fn add_directed_connection(&mut self, start: PortID, end: PortID) -> Result<(), String> {
        // make sure both IDs are different
        if start == end {
            Err(format!("Cannot connect PortIDs {} and {}, must be different", start.0, end.0))
        }
        // check if both IDs exist in graph
        else if !self.in_graph(start) || !self.in_graph(end) {
            Err(format!("At least one Port ID of {} or {} doesn't exist in graph", start.0, end.0).to_owned())
        } else {
            let start_node: &mut PortNode<T> = self.get_mut_node(start).unwrap();
            // make sure connection doesn't already exist
            if start_node.dests.iter().any(|id| *id == end) {
                Err(format!("Connection betweem start ID {} and end ID {} already exists in graph", start.0, end.0))
            } else {
                start_node.dests.push(end);
                Ok(())
            }
        }
    }

    pub fn add_undirected_connection(&mut self, port1: PortID, port2: PortID) -> Result<(), String> {
        // make sure both IDs are different
        if port1 == port2 {
            Err(format!("Cannot connect PortIDs {} and {}, must be different", port1.0, port2.0))
        }
        // check if both IDs exist in graph
        else if !self.in_graph(port1) || !self.in_graph(port2) {
            Err(format!("At least one Port ID of {} or {} doesn't exist in graph", port1.0, port2.0).to_owned())
        } else {
            // use scoping to avoid having two mutable references at same time
            {
                let port1_node: &mut PortNode<T> = self.get_mut_node(port1).unwrap();
                // make sure either connection doesn't exist already
                if port1_node.dests.iter().any(|id| *id == port2) {
                    return Err(format!("Connection betweem start ID {} and end ID {} already exists in graph", port1.0, port2.0));
                }
            }
            {
                let port2_node: &mut PortNode<T> = self.get_mut_node(port2).unwrap();
                if port2_node.dests.iter().any(|id| *id == port1) {
                    return Err(format!("Connection betweem start ID {} and end ID {} already exists in graph", port2.0, port1.0));
                }
                port2_node.dests.push(port1);
            }
            let port1_node = self.get_mut_node(port1).unwrap();
            port1_node.dests.push(port2);
            Ok(())
        }
    }

}

#[cfg(test)]
mod tests {


    use crate::{location::Point2D, region::Region};

    use super::*;

    #[test]
    fn graph_add_ports() {
        let mut america = Region::new("America".to_owned(), 3000);
        let mut europe = Region::new("Europe".to_owned(), 5000);
        let mut american_ports: Vec<Port<Point2D>> = vec![];
        let mut europe_ports: Vec<Port<Point2D>> = vec![];
        
        let amer1 = america.add_port(PortID::new(0), 150, Point2D::default());
        let amer2 = america.add_port(PortID::new(1), 170, Point2D::default());

        let eu1 = europe.add_port(PortID::new(2), 190, Point2D::default());
        let eu2 = europe.add_port(PortID::new(3), 300, Point2D::default());
        let eu3 = europe.add_port(PortID::new(4), 500, Point2D::default());
        let eu4 = europe.add_port(PortID::new(5), 800, Point2D::default());

        american_ports.push(amer1);
        american_ports.push(amer2);

        europe_ports.push(eu1);
        europe_ports.push(eu2);
        europe_ports.push(eu3);
        europe_ports.push(eu4);

        let mut graph = PortGraph::new();

        // check that no ports added
        assert!(!graph.in_graph(PortID(0)));
        assert!(!graph.in_graph(PortID(1)));
        assert!(!graph.in_graph(PortID(2)));
        assert!(!graph.in_graph(PortID(3)));
        assert!(!graph.in_graph(PortID(4)));
        assert!(!graph.in_graph(PortID(5)));

        for amer_port in &american_ports {
            graph.add_port(amer_port.clone());
        }

        // check that all american ports added
        assert!(graph.in_graph(PortID(0)));
        assert!(graph.in_graph(PortID(1)));
        assert!(!graph.in_graph(PortID(2)));
        assert!(!graph.in_graph(PortID(3)));
        assert!(!graph.in_graph(PortID(4)));
        assert!(!graph.in_graph(PortID(5)));

        
        for eu_port in &europe_ports  {
            graph.add_port(eu_port.clone());
        }

        // check that all ports added
        assert!(graph.in_graph(PortID(0)));
        assert!(graph.in_graph(PortID(1)));
        assert!(graph.in_graph(PortID(2)));
        assert!(graph.in_graph(PortID(3)));
        assert!(graph.in_graph(PortID(4)));
        assert!(graph.in_graph(PortID(5)));

        // check where people can travel to (nowhere)
        assert_eq!(graph.get_dest_ports(PortID(0)), Some(vec![]));
        assert_eq!(graph.get_dest_ports(PortID(3)), Some(vec![]));

        // add connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_directed_connection(eu_port.id, am_port.id);
            }
        }

        // try adding same connection again
        assert!(graph.add_directed_connection(PortID(2), PortID(0)).is_err());
        assert!(graph.add_directed_connection(PortID(2), PortID(0)).is_err());
        assert!(graph.add_directed_connection(PortID(3), PortID(0)).is_err());
        assert!(graph.add_directed_connection(PortID(4), PortID(0)).is_err());
        assert!(graph.add_directed_connection(PortID(5), PortID(0)).is_err());

        // try adding undirected connection when a directed connection already exists
        assert!(graph.add_undirected_connection(PortID(0), PortID(5)).is_err());


        // try adding nonsense connections
        assert!(graph.add_directed_connection(PortID(55), PortID(0)).is_err());
        assert!(graph.add_directed_connection(PortID(0), PortID(59)).is_err());
        assert!(graph.add_directed_connection(PortID(509), PortID(99)).is_err());

        // Europeans can travel now, but not Americans
        assert_eq!(graph.get_dest_ports(PortID(0)), Some(vec![]));
        assert_eq!(graph.get_dest_ports(PortID(3)), Some(vec![graph.get_port(PortID(0)).unwrap(), graph.get_port(PortID(1)).unwrap()]));

        // add reverse connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_directed_connection(am_port.id, eu_port.id);
            }
        }

        // Everyone can travel now
        assert_eq!(graph.get_dest_ports(PortID(0)), Some(vec![graph.get_port(PortID(2)).unwrap(), graph.get_port(PortID(3)).unwrap(), graph.get_port(PortID(4)).unwrap(), graph.get_port(PortID(5)).unwrap()]));
        assert_eq!(graph.get_dest_ports(PortID(3)), Some(vec![graph.get_port(PortID(0)).unwrap(), graph.get_port(PortID(1)).unwrap()]));

    }
}
