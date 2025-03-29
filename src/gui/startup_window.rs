// src/gui/startup_window.rs
use eframe::egui;
use crate::engines::plant_engine::PlantEngine;
use crate::gui::control_window::ControlWindow;
use crate::gui::simulation_window::SimulationWindow;

pub struct SimulatorLauncher {
    engine: PlantEngine,
    logs: Vec<String>,
    control_window: ControlWindow,
    simulation_window: SimulationWindow,
}

impl SimulatorLauncher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let env = crate::simulation::simulation_env::SimulationEnv::new();
        let plants = env.plants.clone();
        let environment = env.environment.clone();
        Self {
            engine: PlantEngine::new(env),
            logs: Vec::new(),
            control_window: ControlWindow::new(environment),
            simulation_window: SimulationWindow::new(plants),
        }
    }
}

impl eframe::App for SimulatorLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = 0.1; // Time step
        let results = self.engine.update(dt);

        // Update logs
        for plant_results in results {
            for result in plant_results {
                self.logs.push(result.log);
                if self.logs.len() > 100 {
                    self.logs.remove(0);
                }
            }
        }

        // Update simulation window with current plant states
        self.simulation_window.update_plants(self.engine.env.plants.clone());

        // Show windows
        self.control_window.show(ctx);
        self.simulation_window.show(ctx);

        // Main window with logs
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("C-Space Simulator Logs");

            ui.label(format!("Time: {:.2}s", self.engine.env.time));
            ui.label(format!("Plants: {}", self.engine.env.plants.len()));

            ui.group(|ui| {
                ui.label("Tropism Logs");
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        for log in &self.logs {
                            ui.label(log);
                        }
                    });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("@FractalWaves");
                ui.add_space(5.0);
                ui.label("Computational Manifold Framework");
            });
        });

        ctx.request_repaint();
    }
}

pub fn launch() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([640.0, 480.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "C-Space Simulator",
        options,
        Box::new(|cc| Box::new(SimulatorLauncher::new(cc))),
    )
}