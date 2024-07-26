use std::time::Duration;
use fuel_control::simulator::TankSimulation;
use fuel_control::{ControlAlgorithm, Tank, TankSystem};

#[derive(Debug)]
struct SimpleControlAlgorithm;

impl SimpleControlAlgorithm {
    pub fn new() -> Self {
        Self
    }
}

impl ControlAlgorithm for SimpleControlAlgorithm {
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank {
        let left_level = system.level_left();
        let right_level = system.level_right();
        let mut total_level = right_level + left_level;
        if left_level > right_level {
            println!("Switching to drain the Left tank.");
            Tank::Left
        } else {
            println!("Switching to drain the Right tank.");
            Tank::Right
        }
    }
}

fn main() {
    let mut simulation = TankSimulation::new(50.0, 100.0);
    let mut algorithm = SimpleControlAlgorithm::new();

    simulation.run_simulation(&mut algorithm, Duration::from_secs(60));

    println!("Final Left Tank Level: {}", simulation.level_left());
    println!("Final Right Tank Level: {}", simulation.level_right());
    println!("Final Valve Position: {:?}", simulation.get_valve());
}
