use crate::{ControlAlgorithm, Tank, TankSystem};

#[derive(Debug)]
pub struct SimpleControlAlgorithm {
    // Here you can add variables to your control algorithm if you need any
    // This may be helpful if your algorithm needs to remember anything in between calls to `control`
}

impl SimpleControlAlgorithm {
    // Here you can add additional functions to your control algorithm if you need any
    pub fn new() -> Self {
        Self {
            // Initialize any variables if needed
        }
    }
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
        let left_level = system.level_left();
        let right_level = system.level_right();
        let left_capacity = system.fuel_capacity_left();
        let right_capacity = system.fuel_capacity_right();

        if left_level > left_capacity * 0.9 {
            return Tank::Right;
        } else if right_level > right_capacity * 0.9 {
            return Tank::Left;
        } else if left_level < right_level {
            return Tank::Left;
        } else {
            return Tank::Right;
        }
    }
}
