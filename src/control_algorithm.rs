use crate::{ControlAlgorithm, Tank, TankSystem};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct SimpleControlAlgorithm {
    last_switch: Instant,
    first_valve_switch: Option<Tank>,
}

impl SimpleControlAlgorithm {
    pub fn new() -> Self {
        Self {
            last_switch: Instant::now() - Duration::from_millis(800),
            first_valve_switch: None,
        }
    }

    pub fn get_first_valve_switch(&self) -> Option<Tank> {
        self.first_valve_switch
    }
}

impl ControlAlgorithm for SimpleControlAlgorithm {
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank {
        let left_level = system.level_left();
        let right_level = system.level_right();
        let current_time = Instant::now();

        // Ensure we do not switch too frequently
        if current_time.duration_since(self.last_switch) < Duration::from_millis(800) {
            return system.get_valve();
        }

        self.first_valve_switch = self.get_first_valve_switch();
        if left_level > right_level + 10.0 {
            self.last_switch = current_time;
            return Tank::Left;
        } else if right_level > left_level + 10.0 {
            self.last_switch = current_time;
            return Tank::Right;
        }

        system.get_valve()
    }
}
