// src/gui/mod.rs
pub mod control_window;
pub mod dev_window;
pub mod environment_window;
pub mod plant_diagnostics_window;
pub mod simulation_window;
pub mod startup_window;

pub use control_window::build_control_window;
pub use dev_window::build_dev_window;
pub use environment_window::build_environment_window;
pub use plant_diagnostics_window::build_plant_diagnostics_window;
pub use simulation_window::build_simulation_window;
pub use startup_window::launch;