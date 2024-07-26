//! Implementation of the control algorithm. IMPLEMENT YOUR CONTROL ALGORITHM HERE

use crate::{ControlAlgorithm, Tank, TankSystem};

#[derive(Debug)]
struct SimpleControlAlgorithm {
    // Here you can add variables to your control algorithm if you need any
    // This may be helpful if your algorithm needs to remember anything in between calls to `control`
}

impl SimpleControlAlgorithm {
    pub fn new() -> Self {
        Self {}
    }
}

impl ControlAlgorithm for SimpleControlAlgorithm {
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank {
        let left_level = system.level_left();
        let right_level = system.level_right();

        if left_level > right_level {
            println!("Switching to drain the Left tank.");
            Tank::Left
        } else {
            println!("Switching to drain the Right tank.");
            Tank::Right
        }
    }
}
