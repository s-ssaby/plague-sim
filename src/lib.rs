#![warn(clippy::arithmetic_side_effects, clippy::default_numeric_fallback)]

pub mod region;
pub mod transportation_graph;
pub mod pathogen_types;
pub mod population_types;
pub mod config;
pub mod transportation_allocator;
pub mod math_utils;
pub mod point;
pub mod simulation_geography;


#[cfg(test)]
mod tests {

    use crate::{point::Point2D, population_types::population::Population, region::{Port, PortID}, transportation_graph::PortGraph};

    use self::region::Region;
    use super::*;

    #[test]
    fn it_works() {
        // create countries
        let mut us = Region::new("United States".to_string(), Population::new_healthy(1000));
        let mut us_ports = vec![];
        let us_port1 = us.add_port(PortID::new(0), 100, Point2D::default());
        let us_port2 = us.add_port(PortID::new(1), 200, Point2D::default());
        us_ports.push(us_port1);
        us_ports.push(us_port2);

        let mut china = Region::new("China".to_string(), Population::new_healthy(10000));
        let mut china_ports = vec![];
        let china_port1 = china.add_port(PortID::new(2), 100, Point2D::default());
        let china_port2 = china.add_port(PortID::new(3), 200, Point2D::default());
        let china_port3 = china.add_port(PortID::new(4), 200, Point2D::default());
        china_ports.push(china_port1);
        china_ports.push(china_port2);
        china_ports.push(china_port3);

        let mut port_graph = PortGraph::new();

        // add ports
        for port in china.get_ports() {
            port_graph.add_port(port.clone());
        }

        for port in us.get_ports() {
            port_graph.add_port(port.clone());
        }
        
        // connect countries together
        for china_port in china.get_ports() {
            for amer_port in us.get_ports() {
                port_graph.add_directed_connection(china_port.id, amer_port.id);
                port_graph.add_directed_connection(amer_port.id, china_port.id);
            }
        }

        let first_us_airport = us.get_ports().get(0).unwrap();

        // TODO! Gross vec conversions, any way to fix?
        let us_airport_dests = port_graph.get_dest_ports(first_us_airport.id).unwrap();
        let all_china_ports_ref: Vec<&Port> = china.get_ports().iter().map(|f| f).collect();
        assert_eq!(us_airport_dests, all_china_ports_ref);

        
    }
}
