use cucumber::World;
use fuel_control::simulatorca::TankSimulation;

// Add your "given", "when", "then" functions here


// NO CHANGES NEED TO BE DONE TO THIS WORLD OBJECT (but you can if you want)
#[derive(Debug, Default, World)]
pub struct TankWorld {
    tank: TankSimulation,
}

// DO NOT TOUCH THIS main FUNCTION
fn main() {
    futures::executor::block_on(TankWorld::run("tests/features/control_algorithm.feature"));
}
