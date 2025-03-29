
// File: src/ui/main_menu.rs
use eframe::egui::{self, Color32, RichText, Vec2};

pub fn render_main_menu(ui: &mut egui::Ui, on_plant_creator: &mut dyn FnMut(), on_continue: &mut dyn FnMut(), has_simulation: bool) {
    ui.vertical_centered(|ui| {
        // Add some space at the top
        ui.add_space(20.0);
        
        // Custom heading with larger text and styling
        let title = "C-SPACE SIMULATION VISUALIZER";
        let heading = RichText::new(title)
            .size(32.0)
            .strong()
            .color(Color32::from_rgb(65, 185, 255));
        ui.label(heading);
        
        // Add a separator line
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(30.0);
        
        // Main menu buttons
        let button_size = Vec2::new(220.0, 50.0);
        
        if ui.add_sized(button_size, egui::Button::new(
            RichText::new("🌱 Launch Plant Creator").size(18.0)
        )).clicked() {
            on_plant_creator();
        }
        
        ui.add_space(15.0);
        
        if has_simulation {
            if ui.add_sized(button_size, egui::Button::new(
                RichText::new("▶️ Continue Simulation").size(18.0)
            )).clicked() {
                on_continue();
            }
            
            ui.add_space(15.0);
        }
        
        // Push the credit text to the bottom of the window
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            ui.label("@FractalWaves");
            ui.add_space(5.0);
            ui.label("C-Space Framework Implementation");
        });
    });
}