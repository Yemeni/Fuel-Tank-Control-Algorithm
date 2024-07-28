use cucumber::{World, given, when, then};
use fuel_control::simulator::TankSimulation;
use fuel_control::{Tank, TankSystem};
use std::time::Duration;
use fuel_control::control_algorithm::SimpleControlAlgorithm;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
    algorithm: Option<SimpleControlAlgorithm>, // Store the control algorithm
}

#[derive(Debug, Default, World)]
pub struct FuelTestWorld {
    tank: TankSimulation,
    algorithm: Option<SimpleControlAlgorithm>, // Store the control algorithm
}

#[derive(Debug, Clone, Copy)]
pub struct TankWrapper(Tank);

impl PartialEq for TankWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 as u8 == other.0 as u8
    }
}

impl From<Tank> for TankWrapper {
    fn from(tank: Tank) -> Self {
        TankWrapper(tank)
    }
}

static FUEL_VALUES: Lazy<Mutex<Vec<(f64, f64)>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn print_fuel_values() {
    let fuel_values = FUEL_VALUES.lock().unwrap();
    for (expected, actual) in fuel_values.iter() {
        println!("Expected fuel left around: {}, Actual fuel left: {}", expected, actual);
    }
}
// TankWorld step definitions

#[given(regex = r"the fuel level in the left tank is (\d+\.\d+)L and in the right tank is (\d+\.\d+)L")]
async fn set_initial_fuel_levels(world: &mut TankWorld, left: f64, right: f64) {
    world.tank = TankSimulation::new(left as f32, right as f32);
}

#[when(regex = r"(\d+) seconds pass")]
async fn simulate_time_passes(world: &mut TankWorld, seconds: u64) {
    world.tank.advance(Duration::from_secs(seconds));
}

#[then(regex = r"the left tank should give (\w+) and (\w+) values according to the table")]
async fn check_overflow(world: &mut TankWorld, expected_left: String, expected_right: String) {
    let left_overflowed = world.tank.overflowed_left();
    let right_overflowed = world.tank.overflowed_right();
    assert_eq!(left_overflowed, expected_left == "true");
    assert_eq!(right_overflowed, expected_right == "true");
}

#[when(regex = r"the (left|right|none|both) tank reach (\d+\.\d+)")]
async fn simulate_overflow(world: &mut TankWorld, _overflow: String, _overflow_level: f64) {
    // This step is effectively a no-op since we advance the simulation in the next step
}

#[when(regex = r"the simulation time (\d+) seconds")]
async fn run_simulation(world: &mut TankWorld, simulation_time: u64) {
    world.tank.advance(Duration::from_secs(simulation_time));
}

#[then(regex = r"the first valve switch should be to (\w+)")]
async fn check_first_valve_switch(world: &mut TankWorld, expected_position: String) {
    let expected_valve = match expected_position.as_str() {
        "left" => Tank::Left,
        "right" => Tank::Right,
        _ => panic!("Unknown valve position: {}", expected_position),
    };
    if let Some(algorithm) = &world.algorithm {
        let first_switch = algorithm.get_first_valve_switch();
        assert_eq!(first_switch, Some(expected_valve));
    }
}

// FuelTestWorld step definitions

#[given(regex = r"the fuel level in the left tank is (\d+\.\d+)L and in the right tank is (\d+\.\d+)L")]
async fn set_initial_fuel_levels_fuel(world: &mut FuelTestWorld, left: f64, right: f64) {
    world.tank = TankSimulation::new(left as f32, right as f32);
    let algorithm = SimpleControlAlgorithm::new();
    world.tank.set_control_algorithm(algorithm);
    world.algorithm = Some(SimpleControlAlgorithm::new());
    // println!("Set initial fuel levels: left = {}, right = {}", left, right);
}

#[when(regex = r"the simulation runs for (\d+) seconds")]
async fn run_simulation_fuel(world: &mut FuelTestWorld, simulation_time: u64) {
    // println!("Running simulation for {} seconds", simulation_time);
    world.tank.advance(Duration::from_secs(simulation_time));
}

#[then(regex = r"the expected total fuel left should be around (\d+\.\d+)L")]
async fn check_expected_total_fuel_left_fuel(world: &mut FuelTestWorld, expected_fuel_left: f64) {
    let total_fuel_left = world.tank.level_left() + world.tank.level_right();
    // println!("Expected fuel left: {}, Actual fuel left: {}", expected_fuel_left, total_fuel_left);

    let tolerance = 10.0;
    let lower_bound = expected_fuel_left - tolerance;
    let upper_bound = expected_fuel_left + tolerance;



    assert!(total_fuel_left >= lower_bound as f32 && total_fuel_left <= upper_bound as f32,
            "Actual fuel left {} is not within the range {} - {}", total_fuel_left, lower_bound, upper_bound);


    // println!("Expected fuel left: {}, Actual fuel left: {}", expected_fuel_left, total_fuel_left); // Debug logging
    FUEL_VALUES.lock().unwrap().push((expected_fuel_left, total_fuel_left as f64));
}

// Add a final step to print all the fuel values
// Add a final step to print all the fuel values


// DO NOT TOUCH THIS main FUNCTION
fn main() {
    futures::executor::block_on(async {
        TankWorld::run("tests/features/control_algorithm.feature").await;
        FuelTestWorld::run("tests/features/control_algorithm_tank.feature").await;
    });
    print_fuel_values();
}
