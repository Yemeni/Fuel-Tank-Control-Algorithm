//! Implementation of the control algorithm. IMPLEMENT YOUR CONTROL ALGORITHM HERE

use crate::{ControlAlgorithm, Tank, TankSystem};

#[derive(Debug)]
struct SimpleControlAlgorithm{
    // Here you can add variables to your control algorithm if you need any
    // This may be helpful if your algorithm needs to remember anything in between calls to `control`
}

impl SimpleControlAlgorithm {
    // Here you can add additional functions to your control algorithm if you need any
}


impl ControlAlgorithm for SimpleControlAlgorithm {
    // THIS SHOULD BE IMPLEMENTED BY YOU
    //
    // Your algorithm should prevent the tank system from overflowing
    //
    // This function provides you with the `system` parameter
    // which represents the tank system.
    // This `system` parameter gives you access to convenient functions for
    // getting information about the tank system
    //  - system.level_left() fuel level of the left tank in liters
    //  - system.level_right() fuel level of the right tank in liters
    //  - system.fuel_capacity_left() maximum fuel level of the left tank in liters
    //  - system.fuel_capacity_right() maximum fuel level of the right tank in liters
    //  - system.get_valve() current position of the valve (either `Left` or `Right`)
    //
    // This functions needs to return either `Tank::Left` or `Tank::Right` specifying
    // the desired position of the valve
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank {
        todo!("Implement your control algorithm here")
    }
}
