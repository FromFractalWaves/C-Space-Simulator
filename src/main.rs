// src/main.rs
mod gui;

fn main() -> Result<(), eframe::Error> {
    gui::launch() // Call launch and propagate its Result
}