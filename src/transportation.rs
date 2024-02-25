#![allow(dead_code)]

use std::cell::RefCell;



/** Represents a specific site of travel, such as an airport/seaport */
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Port {
    // maximum amount of transportation
    capacity: u32,
    // whether port is operating or not
    closed: RefCell<bool>
}

impl Port {
    /** Creates a new open port capable of transporting specified capacity */
    pub fn new(capacity: u32) -> Self {
        Self {capacity, closed: RefCell::new(false)}
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
