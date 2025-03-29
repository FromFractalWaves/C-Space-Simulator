// File: src/plant_main.rs
use eframe::egui;

mod engine;
mod ui;
use ui::plant::PlantCreatorApp;

fn main() {
    // Launch the native window with just the plant creator
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("C-Space Plant Creator"),
        ..Default::default()
    };

    eframe::run_native(
        "C-Space Plant Creator",
        options,
        Box::new(|cc| {
            // You could restore state here if needed
            let mut app = PlantCreatorApp::default();
            
            // Here you can configure the standalone app
            // For example, you might load saved parameters
            
            Box::new(app)
        }),
    ).expect("Failed to launch Plant Creator application");
}