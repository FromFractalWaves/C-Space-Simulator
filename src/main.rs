// src/main.rs
mod engines;
mod gui;
mod plants;
mod simulation;

use crate::gui::startup_window::launch;

fn main() {
    launch();
}