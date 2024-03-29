


use std::{fs::File, io::Write};

use functionality::{config::{load_config_data, ConfigData}, location::Point2D, region::{PortID, Region}, region_transportation_mediator::RegionTransportationMediator, transportation_allocator::RandomTransportAllocator, transportation_graph::PortGraph};
fn main() {
    // let config_data = load_config_data("test_data/data.json").unwrap();
    // // create mediator, add regions
    // let mut med: RegionTransportationMediator<Point2D, RandomTransportAllocator> = RegionTransportationMediator::new(config_data.graph, config_data.regions, RandomTransportAllocator);
    // let mut idx = 0;
    // while idx < 50 {
    //     idx += 1;
    //     {
    //     med.update();
    //     }
    //     println!("{}", med.statistics.in_transit.get_total() + med.statistics.region_population.get_total());
    // }   

    let mut us: Region<Point2D> = Region::new("North America".to_owned(), 5000);
    let us_port_top_left = us.add_port(PortID(0), 500, Point2D::new(50.0, 50.0));
    let us_port_top_right = us.add_port(PortID(1), 500, Point2D::new(100.0, 50.0));
    let us_port_bottom_left = us.add_port(PortID(2), 500, Point2D::new(100.0, 50.0));
    let us_port_bottom_right = us.add_port(PortID(3), 500, Point2D::new(100.0, 100.0));

    let mut brazil: Region<Point2D> = Region::new("Brazil".to_owned(), 3000);
    let brasil_port_top_left = brazil.add_port(PortID(4), 700, Point2D::new(50.0, 170.0));
    let brasil_port_top_right = brazil.add_port(PortID(5), 1000, Point2D::new(150.0, 210.0));

    let mut asia: Region<Point2D> = Region::new("Asia".to_owned(), 30000);
    let asia_port = asia.add_port(PortID(6), 5000, Point2D::new(400.0, 50.0));

    let mut africa: Region<Point2D> = Region::new("Africa".to_owned(), 20000);
    let africa_port = africa.add_port(PortID(7), 5000, Point2D::new(300.0, 300.0));

    let mut graph: PortGraph<Point2D> = PortGraph::new();
    graph.add_port(us_port_bottom_left);
    graph.add_port(us_port_top_left);
    graph.add_port(us_port_top_right);
    graph.add_port(us_port_bottom_right);
    graph.add_port(brasil_port_top_left);
    graph.add_port(brasil_port_top_right);
    graph.add_port(asia_port);
    graph.add_port(africa_port);

    // connections within north america
    graph.add_undirected_connection(PortID(0), PortID(1));
    graph.add_undirected_connection(PortID(1), PortID(3));
    graph.add_undirected_connection(PortID(3), PortID(2));
    graph.add_undirected_connection(PortID(2), PortID(0));
    graph.add_undirected_connection(PortID(1), PortID(2));

    // connections within Brazil
    graph.add_undirected_connection(PortID(4), PortID(5));

    // North America and Brazil connections
    graph.add_undirected_connection(PortID(2), PortID(4));
    graph.add_undirected_connection(PortID(3), PortID(5));

    // America Asia connection
    graph.add_undirected_connection(PortID(1), PortID(6));

    // Brasil Africa connection
    graph.add_undirected_connection(PortID(5), PortID(7));

    // Africa Asia connection
    graph.add_undirected_connection(PortID(6), PortID(7));

    let config_data = ConfigData::new(vec![us, africa, asia, brazil], graph);
    let json = serde_json::to_string(&config_data).unwrap();

    // write to file
    let mut file = File::create("simulation_data.json").unwrap();
    file.write_all(json.as_bytes());

}