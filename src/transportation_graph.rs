#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, vec};

use crate::transportation::{Port, PortID};


struct PortNode {
    port: Port,
    dests: RefCell<Vec<PortID>>
}

impl PortNode {
    pub fn new (port: Port) -> Self {
        Self {port, dests: RefCell::new(vec![])}
    }
}

/** Represents a graph of port connections */
pub struct PortGraph {
    ports: HashMap<PortID, PortNode>,
}

impl PortGraph {
    pub fn new() -> Self{
        PortGraph {ports: HashMap::new()}
    }

    pub fn add_port(&mut self, port: Port) {
        let id = port.id;
        let node = PortNode::new(port);
        self.ports.insert(id, node);
    }

    pub fn in_graph(&self, id: PortID) -> bool {
        self.get_port(id).is_some()
    }

    fn get_node(&self, id: PortID) -> Option<&PortNode> {
        self.ports.values().find(|node| node.port.id == id)
    }

    pub fn get_port(&self, id: PortID) -> Option<&Port> {
        let node = self.ports.values().find(|node| node.port.id == id);
        match node {
            Some(node) => {
                Some(&node.port)
            },
            None => None,
        }
    }

    // gets possible destination ports of a port in graph, if it exists
    pub fn get_dest_ports(&self, id: PortID) -> Option<Vec<&Port>> {
        // check if port in graph
        if !self.in_graph(id) {
            None
        } else {
            let mut dests: Vec<&Port> = vec![];
            let node = self.get_node(id);
            if let Some(node) = node {
                for p_id in node.dests.borrow().iter() {
                    // find port
                    dests.push(self.get_port(*p_id).unwrap());
                }
            }
            Some(dests)
        }
    }

    pub fn get_open_dest_ports(&self, id: PortID) -> Option<Vec<&Port>> {
        if !self.in_graph(id) {
            None
        } else {
            let dests = self.get_dest_ports(id).unwrap();
            let mut open_dests: Vec<&Port> = vec![];
            for dest in &dests {
                if !dest.is_closed() {
                    open_dests.push(dest);
                }
            }
            Some(dests)
        }
    }

    // Directed
    pub fn add_connection(&self, start: PortID, end: PortID) -> Result<(), String> {
        // check if both IDs exist in graph
        if !self.in_graph(start) || !self.in_graph(end) {
            Err(format!("At least one Port ID of {} or {} doesn't exist in graph", start.0, end.0).to_owned())
        } else {
            let start_node = self.get_node(start).unwrap();
            start_node.dests.borrow_mut().push(end);
            Ok(())
        }
    }

}

struct PortConnection<'a> {
    start: &'a PortNode,
    end: &'a PortNode
}

#[cfg(test)]
mod tests {

    use crate::transportation::{self, PortID};

    use super::*;

    #[test]
    fn graph_add_ports() {
        let mut american_ports: Vec<Port> = vec![];
        let mut europe_ports: Vec<Port> = vec![];
        
        let amer1 = Port::new(PortID::new(0), 150);
        let amer2 = Port::new(PortID::new(1), 170);

        let eu1 = Port::new(PortID::new(2), 190);
        let eu2 = Port::new(PortID::new(3), 300);
        let eu3 = Port::new(PortID::new(4), 500);
        let eu4 = Port::new(PortID::new(5), 800);

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
                graph.add_connection(eu_port.id, am_port.id);
            }
        }

        // Europeans can travel now, but not Americans
        assert_eq!(graph.get_dest_ports(PortID(0)), Some(vec![]));
        assert_eq!(graph.get_dest_ports(PortID(3)), Some(vec![graph.get_port(PortID(0)).unwrap(), graph.get_port(PortID(1)).unwrap()]));

        // add reverse connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_connection(am_port.id, eu_port.id);
            }
        }

        // Everyone can travel now
        assert_eq!(graph.get_dest_ports(PortID(0)), Some(vec![graph.get_port(PortID(2)).unwrap(), graph.get_port(PortID(3)).unwrap(), graph.get_port(PortID(4)).unwrap(), graph.get_port(PortID(5)).unwrap()]));
        assert_eq!(graph.get_dest_ports(PortID(3)), Some(vec![graph.get_port(PortID(0)).unwrap(), graph.get_port(PortID(1)).unwrap()]));

    }
}
