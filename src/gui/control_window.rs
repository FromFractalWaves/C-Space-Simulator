// src/gui/control_window.rs
use eframe::egui;
use nalgebra::Vector3;
use crate::plants::tropisms::Environment;

pub struct ControlWindow {
    pub environment: Environment,
}

impl ControlWindow {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Control Panel")
            .default_pos([0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("Environment Controls");

                // Light Position
                ui.horizontal(|ui| {
                    ui.label("Light Pos X:");
                    ui.add(egui::DragValue::new(&mut self.environment.light_pos.x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut self.environment.light_pos.y).speed(0.1));
                });

                // Water Position
                ui.horizontal(|ui| {
                    ui.label("Water Pos X:");
                    ui.add(egui::DragValue::new(&mut self.environment.water_pos.x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut self.environment.water_pos.y).speed(0.1));
                });

                // Light Intensity and Water Level
                ui.add(egui::Slider::new(&mut self.environment.light_intensity, 0.0..=2.0).text("Light Intensity"));
                ui.add(egui::Slider::new(&mut self.environment.water_level, 0.0..=2.0).text("Water Level"));

                // Add Obstacle (simple example)
                if ui.button("Add Obstacle").clicked() {
                    self.environment.obstacles.push(Vector3::new(1.0, 1.0, 0.0));
                }

                ui.label(format!("Obstacles: {}", self.environment.obstacles.len()));
            });
    }
}