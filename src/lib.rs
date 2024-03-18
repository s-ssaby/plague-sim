#![warn(clippy::arithmetic_side_effects, clippy::default_numeric_fallback)]

pub mod region;
pub mod transportation;
pub mod transportation_graph;
pub mod pathogen;
pub mod population;
pub mod region_transportation_mediator;


#[cfg(test)]
mod tests {
    use crate::transportation::PortID;

    use self::{region::Region, transportation::Port, transportation_graph::PortGraph};

    use super::*;

    #[test]
    fn it_works() {
        // create countries
        let mut us_ports = vec![];
        let us_port1 = Port::new(PortID::new(0), 100);
        let us_port2 = Port::new(PortID::new(1), 200);
        us_ports.push(us_port1);
        us_ports.push(us_port2);
        let us = Region::new("United States".to_string(), 1000, us_ports);

        let mut china_ports = vec![];
        let china_port1 = Port::new(PortID::new(2), 100);
        let china_port2 = Port::new(PortID::new(3), 200);
        let china_port3 = Port::new(PortID::new(4), 200);
        china_ports.push(china_port1);
        china_ports.push(china_port2);
        china_ports.push(china_port3);
        let china = Region::new("China".to_string(), 10000, china_ports);

        let mut port_graph = PortGraph::new();

        // add ports
        for port in &china.ports {
            port_graph.add_port(port.clone());
        }

        for port in &us.ports {
            port_graph.add_port(port.clone());
        }
        
        // connect countries together
        for china_port in &china.ports {
            for amer_port in &us.ports {
                port_graph.add_connection(china_port.id, amer_port.id);
                port_graph.add_connection(amer_port.id, china_port.id);
            }
        }

        let first_us_airport = &us.ports[0];

        // TODO! Gross vec conversions, any way to fix?
        let us_airport_dests = port_graph.get_dest_ports(first_us_airport.id).unwrap();
        let all_china_ports_ref: Vec<&Port> = china.ports.iter().map(|f| f).collect();
        assert_eq!(us_airport_dests, all_china_ports_ref);

        
    }
}
