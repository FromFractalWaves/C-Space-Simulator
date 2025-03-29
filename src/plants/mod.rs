// src/plants/mod.rs
pub mod tropisms;

// Re-export Plant and other necessary types from tropisms
pub use tropisms::{Plant, Environment, TropismResult, Tropisms};