use cucumber::{World, given, when, then};
use fuel_control::simulator::TankSimulation;
use fuel_control::{Tank, TankSystem};
use std::time::Duration;

#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
}

#[derive(Debug, Clone, Copy)]
pub struct TankWrapper(Tank);

impl PartialEq for TankWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 as u8 == other.0 as u8
    }
}

impl Eq for TankWrapper {}

impl From<Tank> for TankWrapper {
    fn from(tank: Tank) -> Self {
        TankWrapper(tank)
    }
}

#[given(regex = r"the fuel level in the left tank is (\d+\.\d+)L and in the right tank is (\d+\.\d+)L")]
async fn set_initial_fuel_levels(world: &mut TankWorld, left: f64, right: f64) {
    world.tank = TankSimulation::new(left as f32, right as f32);
}

#[when(regex = r"(\d+) seconds pass")]
async fn simulate_time_passes(world: &mut TankWorld, seconds: u64) {
    world.tank.advance(Duration::from_secs(seconds));
}

#[then(regex = r"the left tank should give (\w+) and the right tank (\w+) values according to the table")]
async fn check_overflow(world: &mut TankWorld, expected_left: String, expected_right: String) {
    let left_overflowed = world.tank.overflowed_left();
    let right_overflowed = world.tank.overflowed_right();
    assert_eq!(left_overflowed, expected_left == "true");
    assert_eq!(right_overflowed, expected_right == "true");
}

#[when(regex = r"the (left|right|none|both) tank reach (\d+\.\d+)")]
async fn simulate_overflow(world: &mut TankWorld, overflow: String, _overflow_level: f64) {
    match overflow.as_str() {
        "left" => world.tank.advance(Duration::from_secs(1)), // Simulate time to reach overflow
        "right" => world.tank.advance(Duration::from_secs(1)),
        "both" => world.tank.advance(Duration::from_secs(1)),
        "none" => world.tank.advance(Duration::from_secs(1)),
        _ => {}
    }
}

#[when(regex = r"the simulation time (\d+) seconds")]
async fn run_simulation(world: &mut TankWorld, simulation_time: u64) {
    world.tank.advance(Duration::from_secs(simulation_time));
}

#[then(regex = r"switching the valve to (\w+)")]
async fn check_valve_position(world: &mut TankWorld, expected_position: String) {
    let expected_valve = match expected_position.as_str() {
        "left" => Tank::Left,
        "right" => Tank::Right,
        _ => panic!("Unknown valve position: {}", expected_position),
    };
    assert_eq!(TankWrapper(world.tank.get_valve()), TankWrapper(expected_valve));
}

// DO NOT TOUCH THIS main FUNCTION
fn main() {
    futures::executor::block_on(TankWorld::run("tests/features/control_algorithm.feature"));
}
