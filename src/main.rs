use eframe::egui;

fn main() {
    // Launch the native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]), // Width: 800px, Height: 600px
        ..Default::default()
    };
    
    eframe::run_native(
        "C-Space Simulation Visualizer", // Window title
        options,
        Box::new(|_cc| Box::new(CSpaceApp::default())), // Initialize our app
    ).expect("Failed to launch application");
}

// Define the app state and GUI logic
#[derive(Default)]
struct CSpaceApp {}

impl eframe::App for CSpaceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Define the UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("C-Space Simulation Visualizer");
            ui.label("Welcome! This is the starting point for your simulation visualizer.");
            ui.label("Buttons and features will be added here soon.");
        });
    }
}