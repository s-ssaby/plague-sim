/** Represents a specific site of travel, such as an airport/seaport */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Port {
    // maximum amount of transportation
    capacity: u32,
    // whether port is operating or not
    closed: bool,
    // what ports it can send people to
    connections: Vec<Port>
}

/** Represents a graph of port connections */
pub struct PortGraph {
    ports: Vec<Port>,
    connections: Vec<PortConnection>
}

impl PortGraph {
    fn new() -> Self{
        PortGraph {ports: vec![], connections: vec![]}
    }

    fn add_port(&mut self, port: Port) {
        self.ports.push(port);
    }

    fn add_connection(&mut self, start_port: Port, end_port: Port) {
        self.connections.push(PortConnection{start: start_port, end: end_port});
    }

    // gets possible destination ports of a port in graph
    fn get_dest_ports(&self) {

    }

}

struct PortConnection {
    start: Port,
    end: Port
}