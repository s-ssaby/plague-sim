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