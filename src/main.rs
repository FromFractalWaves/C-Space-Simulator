// File: src/main.rs
use eframe::egui;

mod engine;
mod ui;
use ui::app::CSpaceApp;

fn main() {
    // Launch the native window with the main app
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "C-Space Simulation Visualizer",
        options,
        Box::new(|_cc| Box::new(CSpaceApp::default())),
    ).expect("Failed to launch application");
}