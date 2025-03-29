// src/gui/simulation_window.rs
use eframe::egui;
use nalgebra::Vector3;
use crate::plants::tropisms::Plant;

pub struct SimulationWindow {
    plants: Vec<Plant>,
}

impl SimulationWindow {
    pub fn new(plants: Vec<Plant>) -> Self {
        Self { plants }
    }

    pub fn update_plants(&mut self, plants: Vec<Plant>) {
        self.plants = plants;
    }

    pub fn show(&self, ctx: &egui::Context) {
        egui::Window::new("Simulation View")
            .default_pos([300.0, 0.0])
            .default_size([400.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Plant Simulation");

                let painter = ui.painter();
                let rect = ui.available_rect_before_wrap();

                // Scale factor to map plant positions to window size
                let scale = 20.0;
                let center = rect.center();

                // Draw each plant
                for plant in &self.plants {
                    let pos = center + egui::Vec2::new(
                        plant.pos.x * scale,
                        -plant.pos.y * scale, // Flip Y for screen coords
                    );

                    // Stem (green line upward)
                    let stem_end = pos + egui::Vec2::new(
                        plant.stem_dir.x * scale,
                        -plant.stem_dir.y * scale,
                    );
                    painter.line_segment(
                        [pos.into(), stem_end.into()],
                        egui::Stroke::new(2.0, egui::Color32::GREEN),
                    );

                    // Root (brown line downward)
                    let root_end = pos + egui::Vec2::new(
                        plant.root_dir.x * scale,
                        -plant.root_dir.y * scale,
                    );
                    painter.line_segment(
                        [pos.into(), root_end.into()],
                        egui::Stroke::new(2.0, egui::Color32::BROWN),
                    );

                    // Plant base (small circle)
                    painter.circle_filled(pos.into(), 3.0, egui::Color32::RED);
                }
            });
    }
}