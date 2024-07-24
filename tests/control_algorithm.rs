use cucumber::{given, then, when, World};
use fuel_control::simulator::TankSimulation;
use fuel_control::Tank;

// Add your "given", "when", "then" functions here

#[given(expr = "the left tank is at {float} liter")]
fn set_left_tank(world: &mut TankWorld, level: f32) {
    world.tank.level_left = level;
}

#[given(expr = "the right tank is at {float} liter")]
fn set_right_tank(world: &mut TankWorld, level: f32) {
    world.tank.level_right = level;
}

#[when(expr = "{int} seconds pass")]
fn pass_time(world: &mut TankWorld, seconds: u64) {
    world.tank.advance(std::time::Duration::from_secs(seconds));
}

#[then(expr = "the left tank should not overflow")]
fn check_left_overflow(world: &mut TankWorld) {
    assert!(!world.tank.overflowed_left(), "Left tank overflowed");
}

#[then(expr = "the right tank should not overflow")]
fn check_right_overflow(world: &mut TankWorld) {
    assert!(!world.tank.overflowed_right(), "Right tank overflowed");
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
