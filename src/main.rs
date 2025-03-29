use eframe::egui::{self, Color32, RichText, Vec2, Align2};

// Import the plant simulator modules
mod ui;
mod engine;
use crate::ui::plant::PlantCreatorApp;

// Application launcher for C-Space simulators
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])  // Replaces initial_window_size
            .with_min_inner_size([640.0, 480.0]), // Replaces min_window_size
        centered: true,
        ..Default::default()
    };
    
    eframe::run_native(
        "C-Space Simulators",
        options,
        Box::new(|cc| Box::new(SimulatorLauncher::new(cc)))
    )
}

// Struct to manage available simulators
struct SimulatorLauncher {
    available_simulators: Vec<SimulatorInfo>,
}

// Information about each simulator
struct SimulatorInfo {
    name: String,
    description: String,
    icon: char,
    launch_fn: fn() -> Result<(), eframe::Error>,
}

impl SimulatorLauncher {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let available_simulators = vec![
            SimulatorInfo {
                name: "Plant Growth Simulator".to_string(),
                description: "Simulate plant growth using C-Space principles, modeling coherence and distortion dynamics.".to_string(),
                icon: '🌱',
                launch_fn: || {
                    let options = eframe::NativeOptions {
                        viewport: egui::ViewportBuilder::default()
                            .with_inner_size([1200.0, 800.0])  // Replaces initial_window_size
                            .with_min_inner_size([800.0, 600.0]), // Replaces min_window_size
                        centered: true,
                        ..Default::default()
                    };
                    
                    eframe::run_native(
                        "C-Space Plant Simulator",
                        options,
                        Box::new(|cc| Box::new(PlantCreatorApp::default()))
                    )
                },
            },
        ];
        
        Self {
            available_simulators,
        }
    }
}

impl eframe::App for SimulatorLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_rounding = 6.0.into();
        style.visuals.widgets.noninteractive.rounding = 2.0.into();
        style.visuals.widgets.inactive.rounding = 2.0.into();
        style.visuals.widgets.active.rounding = 2.0.into();
        style.visuals.widgets.hovered.rounding = 2.0.into();
        style.visuals.window_shadow.offset = egui::Vec2::new(2.0, 8.0);
        ctx.set_style(style);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                let title = "C-SPACE SIMULATION FRAMEWORK";
                let heading = RichText::new(title)
                    .size(32.0)
                    .strong()
                    .color(Color32::from_rgb(65, 185, 255));
                ui.label(heading);
                
                ui.add_space(10.0);
                ui.label("Select a simulator to launch:");
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(30.0);
                
                egui::Grid::new("simulators_grid")
                    .num_columns(1)
                    .spacing([20.0, 20.0])
                    .show(ui, |ui| {
                        for simulator in &self.available_simulators {
                            ui.push_id(&simulator.name, |ui| {
                                let button_height = 100.0;
                                let available_width = ui.available_width();
                                
                                let response = ui.add_sized(
                                    [available_width, button_height],
                                    egui::Button::new(RichText::new("").size(0.1))
                                );
                                
                                if response.rect.width() > 0.0 {
                                    let rect = response.rect;
                                    
                                    ui.painter().text(
                                        rect.left_center() + Vec2::new(35.0, 0.0),
                                        Align2::CENTER_CENTER,
                                        &simulator.icon.to_string(),
                                        egui::FontId::proportional(48.0),
                                        ui.visuals().strong_text_color(),
                                    );
                                    
                                    ui.painter().text(
                                        rect.left_center() + Vec2::new(120.0, -15.0),
                                        Align2::LEFT_CENTER,
                                        &simulator.name,
                                        egui::FontId::proportional(20.0),
                                        ui.visuals().text_color(),
                                    );
                                    
                                    ui.painter().text(
                                        rect.left_center() + Vec2::new(120.0, 15.0),
                                        Align2::LEFT_CENTER,
                                        &simulator.description,
                                        egui::FontId::proportional(14.0),
                                        Color32::from_rgb(200, 200, 200),
                                    );
                                }
                                
                                if response.clicked() {
                                    let launch = simulator.launch_fn;
                                    std::thread::spawn(move || {
                                        let _ = launch();
                                    });
                                }
                            });
                            ui.end_row();
                        }
                    });
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label("@FractalWaves");
                    ui.add_space(5.0);
                    ui.label("C-Space Framework Implementation");
                });
            });
        });
    }
}