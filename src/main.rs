// src/main.rs
mod control; // Add this line
mod engines;
mod gui;
mod plants;
mod simulation;

use crate::gui::startup_window::launch;

fn main() {
    launch();
}