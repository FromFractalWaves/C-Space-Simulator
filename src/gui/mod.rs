// src/gui/mod.rs
pub mod control_window;
pub mod simulation_window;
pub mod startup_window;
pub mod environment_window;      // New
pub mod plant_diagnostics_window; // New

pub use startup_window::launch;