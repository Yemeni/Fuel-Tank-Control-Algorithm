//! Simulation of a simple two-tank system
use cucumber::Parameter;
use std::str::FromStr;

pub mod control_algorithm;
pub mod simulator;

/// Identifies the tanks of the system
#[derive(PartialEq,Debug, Default, Clone, Copy, Parameter)]
#[param(name = "tank", regex = "left|right")]
pub enum Tank {
    /// The left tank
    #[default]
    Left,
    /// The right tank
    Right,
}

impl FromStr for Tank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "left" => Self::Left,
            "right" => Self::Right,
            invalid => return Err(format!("Invalid `Valve`: {invalid}")),
        })
    }
}

/// Interface for control algorithms
pub trait ControlAlgorithm: std::fmt::Debug + 'static {
    /// return the valve position for a given tank system
    fn control(&mut self, system: Box<&dyn TankSystem>) -> Tank;
}

/// Interface for a system of two tanks
pub trait TankSystem {
    /// Get the current position of the valve
    fn get_valve(&self) -> Tank;

    /// Get the fill level of the left tank in liter
    fn level_left(&self) -> f32;

    /// Get the fill level of the right tank in liter
    fn level_right(&self) -> f32;

    /// Get the capacity of the left tank in liter
    fn fuel_capacity_left(&self) -> f32;

    /// Get the capacity of the right tank in liter
    fn fuel_capacity_right(&self) -> f32;
}
