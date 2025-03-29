// File: src/lib.rs
pub mod ui;
pub mod engine;

// Re-export main components for easy importing
pub use crate::ui::app::CSpaceApp;
pub use crate::ui::plant::PlantCreatorApp;
pub use crate::engine::CSpaceEngine;