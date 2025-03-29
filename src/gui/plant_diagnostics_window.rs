// src/gui/plant_diagnostics_window.rs
use eframe::egui;
use crate::plants::tropisms::Plant;
use std::sync::{Arc, Mutex};

pub struct PlantDiagnosticsWindow {
    plants: Arc<Mutex<Vec<Plant>>>, // Shared reference to the plants
    selected_plant: Option<usize>,  // Index of the selected plant
}

impl PlantDiagnosticsWindow {
    pub fn new(plants: Arc<Mutex<Vec<Plant>>>) -> Self {
        Self {
            plants,
            selected_plant: None,
        }
    }
}

impl eframe::App for PlantDiagnosticsWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let plants = self.plants.lock().unwrap();
        egui::Window::new("Plant Diagnostics")
            .id(egui::Id::new("plant_diagnostics"))
            .default_pos([900.0, 0.0])
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Plant Diagnostics");

                // Plant selection dropdown
                ui.horizontal(|ui| {
                    ui.label("Select Plant:");
                    egui::ComboBox::from_id_source("plant_selector")
                        .selected_text(
                            self.selected_plant
                                .map_or("None".to_string(), |i| format!("Plant {}", i))
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_plant, None, "None");
                            for (i, _) in plants.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_plant, Some(i), format!("Plant {}", i));
                            }
                        });
                });

                // Display selected plant details
                if let Some(idx) = self.selected_plant {
                    if let Some(plant) = plants.get(idx) {
                        ui.group(|ui| {
                            ui.label(format!("Position: {:?}", plant.pos));
                            ui.label(format!("Stem Direction: {:?}", plant.stem_dir));
                            ui.label(format!("Root Direction: {:?}", plant.root_dir));
                            ui.label(format!("Energy: {:.2}", plant.energy));
                            ui.label(format!("Coherence: {:.2}", plant.coherence));
                            ui.label(format!("Distortion: {:.2}", plant.distortion));
                            ui.label(format!("Temporal Complexity: {:.2}", plant.temporal_complexity));
                            ui.label(format!("Spatial Complexity: {:.2}", plant.spatial_complexity));
                        });

                        // Simple visualization
                        let painter = ui.painter();
                        let rect = ui.available_rect_before_wrap();
                        let scale = 20.0;
                        let center = rect.center();

                        let pos = center + egui::Vec2::new(plant.pos.x * scale, -plant.pos.y * scale);
                        let stem_end = pos + egui::Vec2::new(plant.stem_dir.x * scale, -plant.stem_dir.y * scale);
                        let root_end = pos + egui::Vec2::new(plant.root_dir.x * scale, -plant.root_dir.y * scale);

                        painter.line_segment([pos.into(), stem_end.into()], egui::Stroke::new(2.0, egui::Color32::GREEN));
                        painter.line_segment([pos.into(), root_end.into()], egui::Stroke::new(2.0, egui::Color32::BROWN));
                        painter.circle_filled(pos.into(), 3.0, egui::Color32::RED);
                    }
                } else {
                    ui.label("No plant selected.");
                }
            });

        ctx.request_repaint();
    }
}