use eframe::{egui, App};
use crate::engines::plant_engine::PlantEngine;
use crate::gui::control_window::ControlWindow;
use crate::gui::simulation_window::SimulationWindow;
use crate::gui::environment_window::EnvironmentWindow;
use crate::gui::plant_diagnostics_window::PlantDiagnosticsWindow;
use crate::gui::dev_window::DevWindow; // New
use crate::plants::tropisms::{Environment, Plant};
use std::sync::{Arc, Mutex};

pub struct SimulatorLauncher {
    engine: PlantEngine,
    logs: Vec<String>,
    control_window: Option<ControlWindow>,
    simulation_window: Option<SimulationWindow>,
    environment_window: Option<EnvironmentWindow>,
    plant_diagnostics_window: Option<PlantDiagnosticsWindow>,
    dev_window: Option<DevWindow>, // New
    environment_shared: Arc<Mutex<Environment>>,
    plants_shared: Arc<Mutex<Vec<Plant>>>,
    egui_ctx: egui::Context,
    show_control: bool,
    show_simulation: bool,
    show_environment: bool,
    show_diagnostics: bool,
    show_dev: bool, // New
}

impl SimulatorLauncher {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let env = crate::simulation::simulation_env::SimulationEnv::new();
        let plants_shared = Arc::new(Mutex::new(env.plants.clone()));
        let environment_shared = Arc::new(Mutex::new(env.environment.clone()));

        Self {
            engine: PlantEngine::new(env),
            logs: Vec::new(),
            control_window: None,
            simulation_window: None,
            environment_window: None,
            plant_diagnostics_window: None,
            dev_window: None, // New
            environment_shared,
            plants_shared,
            egui_ctx: cc.egui_ctx.clone(),
            show_control: false,
            show_simulation: false,
            show_environment: false,
            show_diagnostics: false,
            show_dev: false, // New
        }
    }

    fn spawn_viewport(&self, ctx: &egui::Context, id: egui::ViewportId, title: &str, pos: [f32; 2], size: [f32; 2]) {
        ctx.show_viewport_deferred(
            id,
            egui::ViewportBuilder::default()
                .with_title(title)
                .with_inner_size(size)
                .with_position(pos),
            move |ctx, _class| {},
        );
    }
}

impl eframe::App for SimulatorLauncher {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let dt = 0.1;
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

        // Sync shared state
        {
            let mut plants = self.plants_shared.lock().unwrap();
            *plants = self.engine.env.plants.clone();
        }
        {
            let mut env = self.environment_shared.lock().unwrap();
            *env = self.engine.env.environment.clone();
        }

        // Initialize windows if requested
        if self.show_control && self.control_window.is_none() {
            self.control_window = Some(ControlWindow::new(self.environment_shared.clone()));
            self.spawn_viewport(ctx, egui::ViewportId::from_hash_of("control_window"), "Control Panel", [0.0, 0.0], [300.0, 400.0]);
        }
        if self.show_simulation && self.simulation_window.is_none() {
            self.simulation_window = Some(SimulationWindow::new(self.plants_shared.clone()));
            self.spawn_viewport(ctx, egui::ViewportId::from_hash_of("simulation_window"), "Simulation View", [300.0, 0.0], [400.0, 400.0]);
        }
        if self.show_environment && self.environment_window.is_none() {
            self.environment_window = Some(EnvironmentWindow::new(self.environment_shared.clone()));
            self.spawn_viewport(ctx, egui::ViewportId::from_hash_of("environment_window"), "Environment Overview", [600.0, 0.0], [300.0, 400.0]);
        }
        if self.show_diagnostics && self.plant_diagnostics_window.is_none() {
            self.plant_diagnostics_window = Some(PlantDiagnosticsWindow::new(self.plants_shared.clone()));
            self.spawn_viewport(ctx, egui::ViewportId::from_hash_of("plant_diagnostics_window"), "Plant Diagnostics", [900.0, 0.0], [300.0, 400.0]);
        }
        if self.show_dev && self.dev_window.is_none() {
            self.dev_window = Some(DevWindow::new(self.logs.clone()));
            self.spawn_viewport(ctx, egui::ViewportId::from_hash_of("dev_window"), "Development Logs", [1200.0, 0.0], [300.0, 400.0]);
        }

        // Render active windows
        if let Some(window) = &mut self.control_window {
            window.update(ctx, frame);
        }
        if let Some(window) = &mut self.simulation_window {
            window.update(ctx, frame);
        }
        if let Some(window) = &mut self.environment_window {
            window.update(ctx, frame);
        }
        if let Some(window) = &mut self.plant_diagnostics_window {
            window.update(ctx, frame);
        }
        if let Some(window) = &mut self.dev_window {
            window.logs = self.logs.clone(); // Sync logs
            window.update(ctx, frame);
        }

        // Launcher UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("C-Space Simulator Launcher");
            ui.label("Launch simulation windows:");

            ui.horizontal(|ui| {
                if ui.button("Control Panel").clicked() {
                    self.show_control = true;
                }
                if ui.button("Simulation View").clicked() {
                    self.show_simulation = true;
                }
                if ui.button("Environment Overview").clicked() {
                    self.show_environment = true;
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Plant Diagnostics").clicked() {
                    self.show_diagnostics = true;
                }
                if ui.button("Development Logs").clicked() {
                    self.show_dev = true;
                }
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