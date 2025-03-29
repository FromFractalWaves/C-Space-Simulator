// src/main.rs
mod control;
mod engines;
mod gui;
mod plants;
mod simulation;

use crate::gui::startup_window::launch_with_runner;
use crate::simulation::simulation_runner::SimulationRunner;
use crate::simulation::simulation_env::SimulationEnv;
use std::sync::Arc;

fn main() {
    let env = SimulationEnv::new();
    let control = Arc::new(control::SimulationControl::new(env));
    let (runner, command_sender, log_receiver) = SimulationRunner::new(control.clone());

    // Spawn the runner in a separate thread
    std::thread::spawn(move || {
        runner.run();
    });

    // Launch GUI with command sender and log receiver
    launch_with_runner(command_sender, log_receiver);
}