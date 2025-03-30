mod control;
mod engines;
mod gui;
mod plants;
mod simulation;

use crate::gui::startup_window::launch_with_runner;
use std::sync::mpsc::channel;

fn main() {
    println!("Starting main function...");

    // Create channels for communication
    let (command_sender, command_receiver) = channel();
    let (_log_sender, log_receiver) = channel();

    println!("Launching GTK application...");
    // Launch GTK app with channels; SimulationRunner will be started later via dev_window
    launch_with_runner(command_sender, log_receiver);

    println!("GTK application exited.");
}