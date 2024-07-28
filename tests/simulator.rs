// DO NOT CHANGE THIS FILE //

use cucumber::World;
use fuel_control::simulator::{ConstControlAlgorithm, TankSimulation};
use std::time::Duration;

use cucumber::{given, then, when};
use fuel_control::{Tank, TankSystem};

#[given(expr = "the {tank} tank is empty")]
fn set_tank_empty(world: &mut TankWorld, tank: Tank) {
    set_tank_level(world, tank, 0.0)
}

#[given(expr = "the {tank} tank is at {float} liter")]
fn set_tank_level(world: &mut TankWorld, tank: Tank, fill: f32) {
    match tank {
        Tank::Left => world.tank = TankSimulation::new(fill, world.tank.level_right()),
        Tank::Right => world.tank = TankSimulation::new(world.tank.level_left(), fill),
    }
}

#[given(expr = "the valve position is {tank}")]
fn set_valve_position(world: &mut TankWorld, tank: Tank) {
    let control_algorithm = ConstControlAlgorithm::new(tank);
    world.tank.set_control_algorithm(control_algorithm);
}

#[when(expr = "{int} seconds pass")]
fn time_passed(world: &mut TankWorld, seconds: u64) {
    world.tank.advance(Duration::from_secs(seconds));
}

#[then(expr = "the {tank} tank should not be empty")]
fn tank_is_not_empty(world: &mut TankWorld, tank: Tank) {
    match tank {
        Tank::Left => assert_ne!(world.tank.level_left(), 0.0),
        Tank::Right => assert_ne!(world.tank.level_right(), 0.0),
    }
}

#[then(expr = "the {tank} tank should be empty")]
fn tank_is_empty(world: &mut TankWorld, tank: Tank) {
    match tank {
        Tank::Left => assert_eq!(world.tank.level_left(), 0.0),
        Tank::Right => assert_eq!(world.tank.level_right(), 0.0),
    }
}

#[then(expr = "the {tank} tank should overflow")]
fn tank_overflowed(world: &mut TankWorld, tank: Tank) {
    match tank {
        Tank::Left => assert!(world.tank.overflowed_left()),
        Tank::Right => assert!(world.tank.overflowed_right()),
    }
}

#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
}

// DO NOT TOUCH THIS main FUNCTION
fn main() {
    futures::executor::block_on(TankWorld::run("tests/features/simulator.feature"));
}
