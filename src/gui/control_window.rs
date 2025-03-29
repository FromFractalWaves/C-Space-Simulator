use eframe::egui;
use nalgebra::Vector3;
use crate::plants::tropisms::Environment;
use std::sync::{Arc, Mutex};

pub struct ControlWindow {
    environment: Arc<Mutex<Environment>>, // Shared reference to the environment
}

impl ControlWindow {
    pub fn new(environment: Arc<Mutex<Environment>>) -> Self {
        Self { environment }
    }
}

impl eframe::App for ControlWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut env = self.environment.lock().unwrap();
        egui::Window::new("Control Panel")
            .id(egui::Id::new("control_panel")) // Unique ID within this viewport
            .default_pos([0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("Environment Controls");

                ui.horizontal(|ui| {
                    ui.label("Light Pos X:");
                    ui.add(egui::DragValue::new(&mut env.light_pos.x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut env.light_pos.y).speed(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Water Pos X:");
                    ui.add(egui::DragValue::new(&mut env.water_pos.x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut env.water_pos.y).speed(0.1));
                });

                ui.add(egui::Slider::new(&mut env.light_intensity, 0.0..=2.0).text("Light Intensity"));
                ui.add(egui::Slider::new(&mut env.water_level, 0.0..=2.0).text("Water Level"));

                if ui.button("Add Obstacle").clicked() {
                    env.obstacles.push(Vector3::new(1.0, 1.0, 0.0));
                }

                ui.label(format!("Obstacles: {}", env.obstacles.len()));
            });

        ctx.request_repaint();
    }
}