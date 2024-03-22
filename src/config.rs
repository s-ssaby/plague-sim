use std::{error::Error, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{location::{Location, Point2D}, region::{Port, PortID, Region}, transportation_graph::PortGraph};

/** Responsible for holding configuration data of plague simulation */
#[derive(Deserialize, Serialize)]
pub struct ConfigData <T = Point2D> where T: Location{
    pub regions: Vec<Region<T>>,
    pub graph: PortGraph<T>
}

impl <T> ConfigData <T> where T: Location {
    pub fn new(regions: Vec<Region<T>>, graph: PortGraph<T>) -> Self{
        Self { regions, graph}
    }
}


pub fn load_config_data<P>(config_data_path: P) -> Result<ConfigData, Box<dyn Error>> where P: AsRef<Path> {
    let regions_data = fs::read_to_string(config_data_path)?;
    let json: ConfigData<Point2D> = serde_json::from_str(&regions_data)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use crate::{config::{load_config_data, ConfigData}, location::Point2D, region::PortID};


    #[test]
    fn test_config() {
        let config_data = load_config_data("test_data/data.json");
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

