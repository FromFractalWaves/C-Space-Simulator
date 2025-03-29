use eframe::egui;
use crate::plants::tropisms::Plant;
use std::sync::{Arc, Mutex};

pub struct SimulationWindow {
    plants: Arc<Mutex<Vec<Plant>>>, // Shared reference to the plants
}

impl SimulationWindow {
    pub fn new(plants: Arc<Mutex<Vec<Plant>>>) -> Self {
        Self { plants }
    }
}

impl eframe::App for SimulationWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let plants = self.plants.lock().unwrap();
        egui::Window::new("Simulation View")
            .id(egui::Id::new("simulation_view"))
            .default_pos([300.0, 0.0])
            .default_size([400.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Plant Simulation");

                let painter = ui.painter();
                let rect = ui.available_rect_before_wrap();
                let scale = 20.0;
                let center = rect.center();

                for plant in plants.iter() {
                    let pos = center + egui::Vec2::new(
                        plant.pos.x * scale,
                        -plant.pos.y * scale,
                    );

                    let stem_end = pos + egui::Vec2::new(
                        plant.stem_dir.x * scale,
                        -plant.stem_dir.y * scale,
                    );
                    painter.line_segment(
                        [pos.into(), stem_end.into()],
                        egui::Stroke::new(2.0, egui::Color32::GREEN),
                    );

                    let root_end = pos + egui::Vec2::new(
                        plant.root_dir.x * scale,
                        -plant.root_dir.y * scale,
                    );
                    painter.line_segment(
                        [pos.into(), root_end.into()],
                        egui::Stroke::new(2.0, egui::Color32::BROWN),
                    );

                    painter.circle_filled(pos.into(), 3.0, egui::Color32::RED);
                }
            });

        ctx.request_repaint();
    }
}