#![allow(dead_code)]

use std::{cell::RefCell, ptr, vec};

use crate::transportation::{Port, PortID};


struct PortNode<'a> {
    port: Port,
    dests: RefCell<Vec<&'a PortNode<'a>>>
}

impl<'a> PortNode<'a> {
    pub fn new (port: Port) -> Self {
        Self {port, dests: RefCell::new(vec![]) }
    }
}

/** Represents a graph of port connections */
pub struct PortGraph<'a> {
    ports: Vec<PortNode<'a>>,
    connections: RefCell<Vec<PortConnection<'a>>>
}

impl <'a> PortGraph<'a> {
    pub fn new() -> Self{
        PortGraph {ports: vec![], connections: RefCell::new(vec![])}
    }

    pub fn add_port(&mut self, port: Port) {
        let node = PortNode::new(port);
        self.ports.push(node);
    }

    pub fn in_graph(&'a self, id: PortID) -> bool {
        self.get_port(id).is_some()
    }

    fn get_node(&self, id: PortID) -> Option<&PortNode> {
        self.ports.iter().find(|node| node.port.id == id)
    }

    pub fn get_port(&'a self, id: PortID) -> Option<&Port> {
        let node = self.ports.iter().find(|node| node.port.id == id);
        match node {
            Some(n) => {
                Some(&n.port)
            },
            None => None,
        }
    }

    // gets possible destination ports of a port in graph
    pub fn get_dest_ports(&'a self, id: PortID) -> Vec<&Port> {
        let mut dests: Vec<&Port> = vec![];
        let node = self.get_node(id);
        if let Some(node) = node {
            for p in node.dests.borrow().iter() {
                dests.push(&p.port);
            }
        }
        dests
    }

    pub fn get_open_dest_ports(&'a self, id: PortID) -> Vec<&Port> {
        let dests = self.get_dest_ports(id);
        let mut open_dests: Vec<&Port> = vec![];
        for dest in &dests {
            if !dest.is_closed() {
                open_dests.push(dest);
            }
        }
        dests
    }

    // Directed
    pub fn add_connection(&'a self, start: PortID, end: PortID) {
        let start_node = self.get_node(start).unwrap();
        let end_node = self.get_node(end).unwrap();
        start_node.dests.borrow_mut().push(end_node);
        let connection = PortConnection {start: start_node, end: end_node};
        self.connections.borrow_mut().push(connection);
    }

}

struct PortConnection<'a> {
    start: &'a PortNode<'a>,
    end: &'a PortNode<'a>
}

#[cfg(test)]
mod tests {

    use crate::transportation::PortID;

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
        for amer_port in american_ports {
            graph.add_port(amer_port);
        }

        
        for eu_port in europe_ports  {
            graph.add_port(eu_port);
        }

        // close all ports in europe
        for eu_port in europe_ports {
            eu_port.close_port();
        }

        let mut iter = graph.ports.iter();
        assert_eq!(iter.next().unwrap().port.get_capacity(), 150);
        assert_eq!(iter.next().unwrap().port.get_capacity(), 170);
        assert_eq!(iter.next().unwrap().port.get_capacity(), 190);
        assert_eq!(iter.next().unwrap().port.get_capacity(), 300);
        assert_eq!(iter.next().unwrap().port.get_capacity(), 500);
        assert_eq!(iter.next().unwrap().port.get_capacity(), 800);
        assert!(iter.next().is_none());

        // add connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_connection(eu_port.id, am_port.id);
            }
        }

        // 8 directed relationships
        assert_eq!(graph.connections.borrow().len(), 8);

        // add reverse connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_connection(am_port.id, eu_port.id);
            }
        }

        assert_eq!(graph.connections.borrow().len(), 16);

        // Get a list of first American port destination
        let am_ref = graph.ports[0].port;
        assert_eq!(am_ref.get_capacity(), 150);
        let am_dests = graph.get_dest_ports(am_ref.id);
        assert_eq!(am_dests.len(), 4);
        let mut iter = am_dests.iter();
        assert_eq!(iter.next().unwrap().get_capacity(), 190);
        assert_eq!(iter.next().unwrap().get_capacity(), 300);
        assert_eq!(iter.next().unwrap().get_capacity(), 500);
        assert_eq!(iter.next().unwrap().get_capacity(), 800);
    }
}
