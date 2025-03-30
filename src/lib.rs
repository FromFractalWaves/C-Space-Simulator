// src/lib.rs
pub mod control;
pub mod engines;
pub mod gui;
pub mod plants;
pub mod simulation;

pub use gui::startup_window::launch_with_runner;
pub use simulation::simulation_runner::SimulationRunner;