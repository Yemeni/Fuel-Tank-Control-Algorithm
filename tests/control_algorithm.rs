use cucumber::{World, given, when, then};
use fuel_control::simulator::TankSimulation;
use fuel_control::{Tank, TankSystem};
use std::time::Duration;
use log::{info, warn};  // Import logging macros

// Define a wrapper for the Tank enum
#[derive(Debug)]
struct TankWrapper(Tank);

impl PartialEq for TankWrapper {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (Tank::Left, Tank::Left) => true,
            (Tank::Right, Tank::Right) => true,
            _ => false,
        }
    }
}

// Define your "given", "when", "then" functions here

#[given(regex = r"the fuel level in the left tank is (\d+\.\d+)L and in the right tank is (\d+\.\d+)L")]
fn set_initial_fuel_levels(world: &mut TankWorld, left: f32, right: f32) {
    world.set_fuel_levels(left, right);
}

#[when(regex = r"the (\w+) tank reach (\d+\.\d+)")]
fn simulate_tank_overflow(world: &mut TankWorld, overflow: String, over_flow_level: f32) {
    // To simulate overflow, we need to set initial levels and advance the simulation
    if overflow == "left" {
        world.set_fuel_levels(over_flow_level, 0.0);
    } else {
        world.set_fuel_levels(0.0, over_flow_level);
    }
}

#[when(regex = r"the simulation time (\d+) seconds")]
fn run_simulation(world: &mut TankWorld, simulation_time: u64) {
    world.advance_simulation(Duration::from_secs(simulation_time));
}

#[then(regex = r"switching the valve to (\w+)")]
fn check_valve_position(world: &mut TankWorld, value_position: String) {
    let expected_position = match value_position.as_str() {
        "left" => TankWrapper(Tank::Left),
        "right" => TankWrapper(Tank::Right),
        _ => panic!("Unknown valve position: {}", value_position),
    };
    assert_eq!(TankWrapper(world.get_valve()), expected_position);
}

impl TankWorld {
    fn set_fuel_levels(&mut self, left: f32, right: f32) {
        self.tank = TankSimulation::new(left, right);
    }

    fn advance_simulation(&mut self, duration: Duration) {
        self.tank.advance(duration);
    }

    fn get_valve(&self) -> Tank {
        self.tank.get_valve()
    }
}

// NO CHANGES NEED TO BE DONE TO THIS WORLD OBJECT (but you can if you want)
#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
}

// DO NOT TOUCH THIS main FUNCTION
fn main() {
    env_logger::init();
    futures::executor::block_on(TankWorld::run("tests/features/control_algorithm.feature"));
}
