use std::{error::Error, fs, path::Path};

use crate::{location::{Location, Point2D}, region::{Port, PortID, Region}, transportation_graph::PortGraph};

/** Responsible for holding configuration data of plague simulation */
pub struct ConfigData <T> where T: Location{
    pub regions: Vec<Region<T>>,
    pub graph: PortGraph<T>
}

impl <T> ConfigData <T> where T: Location {
    pub fn new(regions: Vec<Region<T>>, graph: PortGraph<T>) -> Self{
        Self { regions, graph}
    }
}


pub fn load_config_data<P>(regions_data_file_path: P, connections_data_file_path: P) -> Result<ConfigData<Point2D>, Box<dyn Error>> where P: AsRef<Path> {
    let mut ports: Vec<Port<Point2D>> = vec![];
    let file = fs::read_to_string(regions_data_file_path)?;
    let mut file = file.lines();
    let mut current_line = file.next();
    let mut next_line = file.next();
    let mut regions: Vec<Region<Point2D>> = vec![];
    'outer: while let Some(current_line_unwrap) = current_line {
        //set current region
        if current_line_unwrap.starts_with("Region:") {
            let mut parts = current_line_unwrap.split(":");
            let country_name = parts.nth(1).unwrap().to_owned();
            let population: u32 = parts.nth(0).unwrap().parse().expect("Region line doesn't have population");
            let mut current_region: Region<Point2D> = Region::new(country_name, population);
            'inner: while let Some(next_line_unwrap) = next_line {
                let new_next_line = file.next();
                if new_next_line.is_none() {
                    // add final port, then build region
                    let mut connections = next_line_unwrap.split(":");
                    let port_id: u32 = connections.next().unwrap().parse().expect("Port ID not found");
                    let capacity: u32 = connections.next().unwrap().parse().expect("Capacity not found");
                    let port = current_region.add_port(PortID(port_id), capacity, Point2D::default());
                    current_region.ports.push(port.clone());
                    ports.push(port);
                    current_line = next_line;
                    next_line = new_next_line;
                    regions.push(current_region);
                    break 'outer;
                } else if next_line_unwrap.starts_with("Region:") {
                    regions.push(current_region);
                    current_line = next_line;
                    next_line = new_next_line;
                    break 'inner;
                } else {
                    // add port
                    let mut connections = next_line_unwrap.split(":");
                    let port_id: u32 = connections.next().unwrap().parse().expect("Port ID not found");
                    let capacity: u32 = connections.next().unwrap().parse().expect("Capacity not found");
                    let port = current_region.add_port(PortID(port_id), capacity, Point2D::default());
                    current_region.ports.push(port.clone());
                    ports.push(port);
                    current_line = next_line;
                    next_line = new_next_line;
                }
            }
        }
    }

    // create graph
    let mut graph = PortGraph::new();
    // add ports
    for port in ports {
        graph.add_port(port);
    }

    // read connections
    let connections = fs::read_to_string(connections_data_file_path)?;
    let mut connections = connections.lines();
    while let Some(current_line) = connections.next() {
        let mut parts = current_line.split(":");
        let start_id = PortID(parts.next().unwrap().parse().expect("msg"));
        let end_id = PortID(parts.next().unwrap().parse().expect("msg"));
        graph.add_connection(start_id, end_id)?;
    }

    let result = ConfigData::new(regions, graph);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{config::{load_config_data, ConfigData}, location::Point2D, region::PortID};


    #[test]
    fn test_config() {
        let config_data = load_config_data("src/countries.txt", "src/connections.txt");
        assert!(config_data.is_ok());
        let config_data: ConfigData<Point2D> = config_data.unwrap();
        let graph = config_data.graph;
        let regions = config_data.regions;

        let expected_names = ["United States", "Europe", "China"];
        // Assert that regions correctly read in
        assert_eq!(regions[0].name, expected_names[0]);
        assert_eq!(regions[1].name, expected_names[1]);
        assert_eq!(regions[2].name, expected_names[2]);


        // Assert that all ports loaded in graph
        assert!(graph.in_graph(PortID(0)));
        assert!(graph.in_graph(PortID(1)));
        assert!(graph.in_graph(PortID(2)));
        assert!(graph.in_graph(PortID(3)));
        assert!(graph.in_graph(PortID(4)));
        assert!(graph.in_graph(PortID(5)));
        // shouldn't be in graph
        assert!(!graph.in_graph(PortID(6)));

        let us_id = regions[0].id;
        let europe_id = regions[1].id;
        let china_id = regions[2].id;

        assert_eq!(graph.get_port(PortID(0)).unwrap().region, us_id);
        assert_eq!(graph.get_port(PortID(1)).unwrap().region, us_id);
        assert_eq!(graph.get_port(PortID(2)).unwrap().region, europe_id);
        assert_eq!(graph.get_port(PortID(3)).unwrap().region, europe_id);
        assert_eq!(graph.get_port(PortID(4)).unwrap().region, china_id);
        assert_eq!(graph.get_port(PortID(5)).unwrap().region, china_id);

        // all proper connections here?
        assert_eq!(graph.get_dest_ports(PortID(0)).unwrap(), vec![graph.get_port(PortID(1)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(1)).unwrap(), vec![graph.get_port(PortID(2)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(2)).unwrap(), vec![graph.get_port(PortID(3)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(3)).unwrap(), vec![graph.get_port(PortID(4)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(4)).unwrap(), vec![graph.get_port(PortID(5)).unwrap()]);
        assert_eq!(graph.get_dest_ports(PortID(5)).unwrap(), vec![graph.get_port(PortID(0)).unwrap()]);
              
    }
}

