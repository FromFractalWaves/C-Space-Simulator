// src/gui/startup_window.rs
use eframe::egui;

pub struct SimulatorLauncher {}

impl SimulatorLauncher {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for SimulatorLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("@FractalWaves");
                ui.add_space(5.0);
                ui.label("Computational Manifold Framework");
            });
        });
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