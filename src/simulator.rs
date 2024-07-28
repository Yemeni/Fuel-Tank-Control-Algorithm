//! Implementation of the tank system simulation. DO NOT TOUCH.

use std::{
    f32::consts::{E, PI},
    time::Duration,
};

use crate::{ControlAlgorithm, Tank, TankSystem};

/// Simulation state container
///
/// Contains the current state of the simulation
#[derive(Debug, Default)]
pub struct TankSimulation {
    /// Fill level of the left tank in liter
    pub level_left: f32,

    /// Fill level of the right tank in liter
    pub level_right: f32,

    /// Position of the valve
    position: Tank,

    /// Time passed so far in this simulation
    time: Duration,

    /// If the left tank had an overflow
    left_overflowed: bool,

    /// If the right tank had an overflow
    right_overflowed: bool,

    control_algorithm: Option<Box<dyn ControlAlgorithm>>,
}

impl TankSystem for TankSimulation {
    fn get_valve(&self) -> Tank {
        self.position
    }

    fn level_left(&self) -> f32 {
        self.level_left
    }

    fn level_right(&self) -> f32 {
        self.level_right
    }

    fn fuel_capacity_left(&self) -> f32 {
        200.0
    }

    fn fuel_capacity_right(&self) -> f32 {
        250.0
    }
}

impl TankSimulation {
    const TIME_STEP: Duration = Duration::from_millis(1);

    /// Create new TankSimulation with starting fuel level in liter for left and right tank
    pub fn new(level_left: f32, level_right: f32) -> Self {
        Self {
            level_left,
            level_right,
            ..Default::default()
        }
    }

    /// Set the control algorithm of the simulator
    pub fn set_control_algorithm(&mut self, control_algorithm: impl ControlAlgorithm) {
        self.control_algorithm = Some(Box::new(control_algorithm));
    }

    /// Returns true if the right tank overflowed
    pub fn overflowed_right(&self) -> bool {
        self.right_overflowed
    }

    /// Returns true if the left tank overflowed
    pub fn overflowed_left(&self) -> bool {
        self.left_overflowed
    }

    /// Advance the simulation by a given [Duration]
    pub fn advance(&mut self, mut dur: Duration) {
        while dur > Self::TIME_STEP {
            self.advance_step(Self::TIME_STEP);
            dur = dur.saturating_sub(Self::TIME_STEP)
        }
        self.advance_step(dur);
    }

    /// Advance the simulation by a single (small) step
    ///
    /// [Self::advance] uses this function internally in a loop to simulate for [Duration]s longer
    /// than [Self::TIME_STEP].
    fn advance_step(&mut self, step: Duration) {
        self.set_valve_pos();
        self.fill_step(step);
        self.consume_step(step);
        self.overflow_check();
        self.clamp_fuel_level();

        self.time = self.time.saturating_add(step);
    }

    /// Set the valve position to the position specified by the control algorithm
    fn set_valve_pos(&mut self) {
        let Some(mut control) = self.control_algorithm.take() else {
            return;
        };
        self.position = control.control(Box::new(self));
        self.control_algorithm = Some(control);
    }

    /// Fill all tanks for a given step duration
    fn fill_step(&mut self, step: Duration) {
        self.level_left += Self::flow_left_function(self.time) * step.as_secs_f32();
        self.level_right += Self::flow_right_function(self.time) * step.as_secs_f32();
    }

    /// Consume from the connected tank for a given step duration
    fn consume_step(&mut self, step: Duration) {
        match self.position {
            Tank::Left => self.level_left -= self.current_consumption() * step.as_secs_f32(),
            Tank::Right => self.level_right -= self.current_consumption() * step.as_secs_f32(),
        }
    }

    /// clamp negative fuel level to 0.0 and overfull fuel level to the max capacity
    fn clamp_fuel_level(&mut self) {
        if self.level_left < 0.0 {
            self.level_left = 0.0;
        }
        if self.level_left > self.fuel_capacity_left() {
            self.level_left = self.fuel_capacity_left();
        }
        if self.level_right < 0.0 {
            self.level_right = 0.0;
        }
        if self.level_right > self.fuel_capacity_right() {
            self.level_right = self.fuel_capacity_right();
        }
    }

    /// Check if any tank overflowed and mark it as such
    fn overflow_check(&mut self) {
        if self.level_left > self.fuel_capacity_left() {
            self.left_overflowed = true;
        }
        if self.level_right > self.fuel_capacity_right() {
            self.right_overflowed = true;
        }
    }

    /// Calculate the in-flow for the right tank
    fn flow_right_function(t: Duration) -> f32 {
        let t = t.as_secs_f32();
        1.0 - E.powf(-t) * t.cos()
            + (1.0 - E.powf(-t / 10.0)) * (t * 2.0 / 3.0).cos()
            + 5.0 * (1.0 - E.powf(-t / 10.0))
    }

    /// Apply the current in-flow on the right tank
    fn flow_left_function(t: Duration) -> f32 {
        let t = t.as_secs_f32();
        (30.0 * (1.0 - E.powf(-t / 10.0)) + PI).cos() + 1.0 + 3.0 * (1.0 - E.powf(-t))
    }

    /// Calculate the out-flow for both tanks, considering the current valve position
    fn current_consumption(&self) -> f32 {
        let level_relative = match self.position {
            Tank::Left => self.level_left / self.fuel_capacity_left(),
            Tank::Right => self.level_right / self.fuel_capacity_right(),
        };
        level_relative * 10.0 + 10.0
    }
}

/// Constant Control Algorithm which always returns the same valve position
#[derive(Debug)]
pub struct ConstControlAlgorithm {
    position: Tank,
}

impl ConstControlAlgorithm {
    /// Create a new Const Control Algorithm with a given FIXED valve position
    pub fn new(pos: Tank) -> Self {
        Self { position: pos }
    }
}

impl ControlAlgorithm for ConstControlAlgorithm {
    fn control(&mut self, _system: Box<&dyn TankSystem>) -> Tank {
        self.position
    }
}
