// src/main.rs
mod gui;         // Already present
mod plants;      // Declare the plants module
mod engines;     // Declare the engines module
mod simulation;  // Declare the simulation module

fn main() -> Result<(), eframe::Error> {
    gui::launch()
}