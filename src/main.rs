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
            ui.vertical_centered(|ui| {
                // Add some space at the top
                ui.add_space(20.0);
                
                // Custom heading with larger text and styling
                let title = "C-SPACE SIMULATION VISUALIZER";
                let heading = egui::RichText::new(title)
                    .size(32.0)
                    .strong()
                    .color(egui::Color32::from_rgb(65, 185, 255));
                ui.label(heading);
                
                // Add a separator line
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(16.0);
                
                ui.label("Welcome! This is the starting point for your simulation visualizer.");
                ui.label("Buttons and features will be added here soon.");
                
                // Push the credit text to the bottom of the window
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label("@FractalWaves");
                    ui.add_space(5.0);
                });
            });
        });
    }
}