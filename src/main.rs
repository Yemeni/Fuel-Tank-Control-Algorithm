use std::thread::sleep;
use std::time::Duration;
use fuel_control::simulator::TankSimulation;
use fuel_control::{ControlAlgorithm, Tank, TankSystem};
use log::debug;

#[derive(Debug)]
pub struct SimpleControlAlgorithm {
    last_switch: std::time::Instant,
}

impl SimpleControlAlgorithm {
    pub fn new() -> Self {
        Self {
            last_switch: std::time::Instant::now() - std::time::Duration::from_secs(1),
        }
    }
}

impl ControlAlgorithm for SimpleControlAlgorithm {
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank {
        let left_level = system.level_left();
        let right_level = system.level_right();
        let mut total_level = right_level + left_level;
        let current_time = std::time::Instant::now();

        // Ensure we do not switch too frequently
        if current_time.duration_since(self.last_switch) < std::time::Duration::from_nanos(800) {
            return system.get_valve();
        }

        if left_level > right_level + 10.0 {
            // println!("Switching to drain the Left tank.");
            self.last_switch = current_time;
            Tank::Left
        } else if right_level > left_level + 10.0{

            // println!("Switching to drain the Right tank.");
            self.last_switch = current_time;
            Tank::Right
        }else{
            // println!("Not switching, less than 10 difference");
            system.get_valve()
        }
    }
}

fn main() {
    let mut simulation = TankSimulation::new(100.0, 100.0);
    let mut algorithm = SimpleControlAlgorithm::new();

    simulation.set_control_algorithm(algorithm);
    simulation.advance(Duration::from_secs(60));

    if(simulation.overflowed_right()){
        println!("Overflowed right.");
    }
    if(simulation.overflowed_left()){
        println!("Overflowed left.");
    }
    println!("Final Left Tank Level: {}", simulation.level_left());
    println!("Final Right Tank Level: {}", simulation.level_right());
    println!("Final Valve Position: {:?}", simulation.get_valve());
}
