// src/main.rs
mod gui;
use gui::startup_window::launch;

fn main() -> Result<(), eframe::Error> {
    startup_window::launch() // Call launch and propagate its Result
}