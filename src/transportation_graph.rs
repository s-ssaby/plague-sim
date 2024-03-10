#![allow(dead_code)]

use std::{cell::RefCell, ptr, vec};

use crate::transportation::Port;


struct PortNode<'a> {
    port: &'a Port,
    dests: RefCell<Vec<&'a PortNode<'a>>>
}

impl<'a> PortNode<'a> {
    pub fn new (port: &'a Port) -> Self {
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

    pub fn add_port(&mut self, port: &'a Port) {
        let node = PortNode::new(port);
        self.ports.push(node);
    }

    fn in_graph(&'a self, person: &Port) -> bool {
        self.get_node(person).is_some()
    }

    fn get_node(&'a self, person: &Port) -> Option<&PortNode> {
        self.ports.iter().find(|&p| ptr::eq(person, p.port))
    }

    // gets possible destination ports of a port in graph
    pub fn get_dest_ports(&'a self, port: &Port) -> Vec<&Port> {
        let mut dests: Vec<&Port> = vec![];
        let node = self.get_node(port);
        if let Some(node) = node {
            for p in node.dests.borrow().iter() {
                dests.push(p.port);
            }
        }
        dests
    }

    pub fn get_open_dest_ports(&'a self, port: &Port) -> Vec<&Port> {
        let dests = self.get_dest_ports(port);
        let mut open_dests: Vec<&Port> = vec![];
        for dest in &dests {
            if !dest.is_closed() {
                open_dests.push(dest);
            }
        }
        dests
    }

    // Directed
    pub fn add_connection(&'a self, start: &'a Port, end: &'a Port) {
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

    use super::*;

    #[test]
    fn graph_add_ports() {
        let mut american_ports: Vec<Port> = vec![];
        let mut europe_ports: Vec<Port> = vec![];
        
        let amer1 = Port::new(150);
        let amer2 = Port::new(170);

        let eu1 = Port::new(190);
        let eu2 = Port::new(300);
        let eu3 = Port::new(500);
        let eu4 = Port::new(800);

        american_ports.push(amer1);
        american_ports.push(amer2);

        europe_ports.push(eu1);
        europe_ports.push(eu2);
        europe_ports.push(eu3);
        europe_ports.push(eu4);

        let mut graph = PortGraph::new();
        for amer_port in american_ports.iter() {
            graph.add_port(amer_port);
        }

        
        for eu_port in europe_ports.iter() {
            graph.add_port(eu_port);
        }

        // close all ports in europe
        for eu_port in europe_ports.iter() {
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
                graph.add_connection(eu_port, am_port);
            }
        }

        // 8 directed relationships
        assert_eq!(graph.connections.borrow().len(), 8);

        // add reverse connections
        for eu_port in europe_ports.iter() {
            for am_port in american_ports.iter() {
                graph.add_connection(am_port, eu_port);
            }
        }

        assert_eq!(graph.connections.borrow().len(), 16);

        // Get a list of first American port destination
        let am_ref = graph.ports[0].port;
        assert_eq!(am_ref.get_capacity(), 150);
        let am_dests = graph.get_dest_ports(am_ref);
        assert_eq!(am_dests.len(), 4);
        let mut iter = am_dests.iter();
        assert_eq!(iter.next().unwrap().get_capacity(), 190);
        assert_eq!(iter.next().unwrap().get_capacity(), 300);
        assert_eq!(iter.next().unwrap().get_capacity(), 500);
        assert_eq!(iter.next().unwrap().get_capacity(), 800);
    }
}
