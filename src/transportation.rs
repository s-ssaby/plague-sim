#![allow(dead_code)]

use std::cell::RefCell;

use crate::region::RegionID;



#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub struct PortID(pub u32);

impl PortID {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/** Represents a specific site of travel, such as an airport/seaport */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Port {
    // maximum amount of transportation 
    pub capacity: u32,
    // whether port is operating or not
    closed: RefCell<bool>,
    // ID of region this port is in
    pub region: Option<RegionID>,
    // ID of this port
    pub id: PortID
}

impl Port {
    /** Creates a new open port capable of transporting specified capacity */
    /** Users of Port must ensure that all Ports they create have unique IDs to avoid unwanted behavior */
    pub fn new(id: PortID, capacity: u32) -> Self {
        Self {capacity, closed: RefCell::new(false), region: None, id }
    }

    pub fn close_port(&self) {
        self.closed.replace(false);
    }

    pub fn is_closed(&self) -> bool {
        self.closed.borrow().to_owned()
    }

    pub fn get_capacity(&self) -> u32 {
        self.capacity
    }
}
