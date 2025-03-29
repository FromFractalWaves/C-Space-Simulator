// File: src/bin/plant_main.rs
use eframe::egui;
use eframe::epaint::Vec2;
use cs_simulator::ui::plant::PlantCreatorApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::ViewportBuilder::default()
            .with_inner_size(Vec2::new(1200.0, 800.0))
            .with_min_inner_size(Vec2::new(800.0, 600.0))
            .with_centered(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "C-Space Plant Simulator",
        options,
        Box::new(|_cc| Box::new(PlantCreatorApp::default()))
    )
}