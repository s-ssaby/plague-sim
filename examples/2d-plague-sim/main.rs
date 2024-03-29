use std::time::Duration;

use functionality::{config::load_config_data, location::Point2D, region::RegionID, simulation_geography::SimulationGeography, transportation_allocator::RandomTransportAllocator};
use macroquad::{miniquad::window::set_window_size, prelude::*};
use simulation::Simulation;
mod simulation;

#[macroquad::main("Simulation")]
async fn main() {
    set_window_size(500, 500);
    let config_data = load_config_data("examples/2d-plague-sim/simulation_data.json").unwrap();
    let graph = config_data.graph;
    let regions = config_data.regions;

    let mut simulation: Simulation<Point2D, RandomTransportAllocator> = Simulation::<Point2D, RandomTransportAllocator>::new(SimulationGeography::new(graph, regions), RandomTransportAllocator::new(0.01));

    // TODO: Create a separate loop for simulation, let rendering be own loop

    loop {
        clear_background(BLUE);

        // update
        simulation.update();

        // north america ID: 0
        draw_rectangle(0.0, 0.0, 100.0, 100.0, GREEN);
        draw_text(format!("{}", simulation.geography.get_region(RegionID(0)).unwrap().population.get_total()).as_str(), 0.0, 100.0, 50.0, BLACK);

        // brazil: ID 1
        draw_rectangle(0.0, 150.0, 200.0, 100.0, GREEN);
        draw_text(format!("{}", simulation.geography.get_region(RegionID(1)).unwrap().population.get_total()).as_str(), 0.0, 150.0, 50.0, BLACK);

        // asia: ID 2
        draw_rectangle(400.0, 0.0, 100.0, 100.0, GREEN);
        draw_text(format!("{}", simulation.geography.get_region(RegionID(2)).unwrap().population.get_total()).as_str(), 400.0, 100.0, 50.0, BLACK);

        // africa: ID 3
        draw_rectangle(250.0, 220.0, 100.0, 100.0, GREEN);
        draw_text(format!("{}", simulation.geography.get_region(RegionID(3)).unwrap().population.get_total()).as_str(), 250.0, 220.0, 50.0, BLACK);


        for port in simulation.geography.get_ports() {
            let x = port.pos.x;
            let y = port.pos.y;
            draw_circle(x as f32, y as f32, 10.0, WHITE);
        }


        println!("Transit Population is: {}", simulation.statistics.in_transit.get_total());
        println!("Region Population is: {}", simulation.statistics.region_population.get_total());
        println!("Total Population is: {} ", simulation.statistics.in_transit.get_total() + simulation.statistics.region_population.get_total());

        // render flying planes
        for job in &simulation.ongoing_transport {
            let color = Color::new(f32::min((job.job.population.get_total() as f32)/(1000 as f32), 1.0), 0.0, 0.0, 1.0);
            let prog_percent = (job.job.time as f64)/(job.expected_time as f64);
            let start_port = simulation.geography.get_port(job.job.start_port).unwrap();
            let end_port = simulation.geography.get_port(job.job.end_port).unwrap();

            let plane_x = start_port.pos.x + prog_percent*(end_port.pos.x - start_port.pos.x);
            let plane_y = start_port.pos.y + prog_percent*(end_port.pos.y - start_port.pos.y);

            let radius = (job.job.population.get_total() as f32)/100.0;
            draw_circle(plane_x as f32, plane_y as f32, radius, color);
        }
        std::thread::sleep(Duration::from_millis(16));
        next_frame().await
    }
}