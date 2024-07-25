use cucumber::{World, given, when, then};
use fuel_control::simulator::TankSimulation;
use std::time::Duration;

// Define your "given", "when", "then" functions here

#[given(regex = r"the fuel level in the left tank is (\d+\.\d+)L and in the right tank is (\d+\.\d+)L")]
async fn set_initial_fuel_levels(world: &mut TankWorld, left: f64, right: f64) {
    world.tank.set_fuel_levels(left, right);
}

#[when(regex = r"the (\w+) tank reach (\d+\.\d+)")]
async fn simulate_tank_overflow(world: &mut TankWorld, overflow: String, over_flow_level: f64) {
    world.tank.simulate_overflow(overflow, over_flow_level);
}

#[when(regex = r"the simulation time (\d+) seconds")]
async fn run_simulation(world: &mut TankWorld, simulation_time: u64) {
    world.tank.run_simulation(Duration::new(simulation_time, 0)).await;
}

#[then(regex = r"switching the valve to (\w+)")]
async fn check_valve_position(world: &mut TankWorld, value_position: String) {
    assert_eq!(world.tank.get_valve_position(), value_position);
}

// NO CHANGES NEED TO BE DONE TO THIS WORLD OBJECT (but you can if you want)
#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
}

// DO NOT TOUCH THIS main FUNCTION
fn main() {
    futures::executor::block_on(TankWorld::run("tests/features/control_algorithm.feature"));
}
