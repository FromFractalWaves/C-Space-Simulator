// File: src/ui/plant.rs
use eframe::egui::{self, Color32, Pos2, RichText, Stroke, Vec2};

use crate::engine::{CSpaceEngine, EngineParams};
use super::common::{SimulationParams, ParamTab, draw_cspace_params, draw_plant_params, draw_environment_params};

pub struct PlantCreatorApp {
    current_tab: ParamTab,
    params: SimulationParams,
    plant_position: Vec2,
    light_positions: Vec<Vec2>,
    param_valid: bool,
}

impl Default for PlantCreatorApp {
    fn default() -> Self {
        let mut app = Self {
            current_tab: ParamTab::CSpace,
            params: SimulationParams::default(),
            plant_position: Vec2::new(400.0, 500.0),
            light_positions: Vec::new(),
            param_valid: true,
        };
        
        // Generate some default light positions
        app.regenerate_light_positions();
        
        app
    }
}

impl PlantCreatorApp {
    pub fn regenerate_light_positions(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.light_positions.clear();
        for _ in 0..self.params.num_light_sources {
            self.light_positions.push(Vec2::new(
                rng.gen_range(0.0..self.params.width),
                rng.gen_range(0.0..self.params.height / 2.0), // Lights mostly at the top
            ));
        }
    }
    
    pub fn initialize_simulation(&self) -> CSpaceEngine {
        // Create engine parameters from the UI params
        let engine_params = EngineParams {
            alpha: self.params.alpha,
            beta: self.params.beta,
            epsilon: self.params.epsilon,
            d_critical: self.params.d_critical,
            lambda: self.params.lambda,
            growth_rate: self.params.growth_rate,
            growth_prob: self.params.growth_prob,
            branch_prob: self.params.branch_prob,
            max_nodes: self.params.max_nodes,
            initial_energy: self.params.initial_energy,
            max_energy_distance: self.params.max_energy_distance,
        };

        let mut engine = CSpaceEngine::new_with_params(
            self.params.width, 
            self.params.height,
            engine_params
        );
        
        // Initialize plant at the chosen position
        engine.initialize_plant(
            (self.plant_position.x, self.plant_position.y),
            self.params.initial_energy
        );
        
        // Add light resources
        for pos in &self.light_positions {
            engine.add_resource(
                (pos.x, pos.y),
                self.params.light_intensity,
                "light"
            );
        }
        
        engine
    }
    
    pub fn validate_params(&mut self) -> bool {
        // Check for valid parameter ranges
        let valid = 
            self.params.alpha > 0.0 && 
            self.params.beta > 0.0 && 
            self.params.epsilon > 0.0 && 
            self.params.d_critical > 0.0 &&
            self.params.growth_rate > 0.0 &&
            self.params.growth_prob >= 0.0 && self.params.growth_prob <= 1.0 &&
            self.params.branch_prob >= 0.0 && self.params.branch_prob <= 1.0 &&
            self.params.max_nodes > 0 &&
            self.params.width > 0.0 &&
            self.params.height > 0.0 &&
            self.params.max_energy_distance > 0.0;
        
        self.param_valid = valid;
        valid
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, on_back: &mut dyn FnMut(), on_start: &mut dyn FnMut(CSpaceEngine)) {
        ui.horizontal(|ui| {
            // Left panel for parameter tabs and settings
            ui.vertical(|ui| {
                ui.add_space(5.0);
                ui.heading("Plant Creator");
                ui.separator();
                
                // Tab selection
                ui.horizontal(|ui| {
                    if ui.selectable_label(matches!(self.current_tab, ParamTab::CSpace), "C-Space").clicked() {
                        self.current_tab = ParamTab::CSpace;
                    }
                    if ui.selectable_label(matches!(self.current_tab, ParamTab::Plant), "Plant").clicked() {
                        self.current_tab = ParamTab::Plant;
                    }
                    if ui.selectable_label(matches!(self.current_tab, ParamTab::Environment), "Environment").clicked() {
                        self.current_tab = ParamTab::Environment;
                    }
                });
                
                ui.add_space(10.0);
                
                // Parameters for the selected tab
                egui::Frame::group(ui.style())
                    .stroke(Stroke::new(1.0, Color32::from_gray(160)))
                    .show(ui, |ui| {
                        ui.set_min_width(250.0);
                        match self.current_tab {
                            ParamTab::CSpace => draw_cspace_params(ui, &mut self.params),
                            ParamTab::Plant => draw_plant_params(ui, &mut self.params),
                            ParamTab::Environment => {
                                // Create a local copy of the regenerate function to avoid
                                // borrowing self.params more than once
                                let params_copy = &mut self.params.clone();
                                let mut regenerate = || self.regenerate_light_positions();
                                draw_environment_params(ui, params_copy, &mut regenerate);
                                // Update the original params with the modified copy
                                self.params = params_copy.clone();
                            },
                        }
                    });
                
                ui.add_space(10.0);
                
                // Navigation and action buttons
                ui.horizontal(|ui| {
                    if ui.button("Back to Menu").clicked() {
                        on_back();
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if !self.param_valid {
                            ui.label(RichText::new("Invalid parameters!").color(Color32::RED));
                        }
                        
                        let start_button = ui.add_enabled(
                            self.validate_params(),
                            egui::Button::new("Start Simulation")
                        );
                        
                        if start_button.clicked() {
                            on_start(self.initialize_simulation());
                        }
                    });
                });
            });
            
            // Right panel for visualization
            ui.vertical(|ui| {
                ui.add_space(5.0);
                ui.heading("Preview");
                ui.separator();
                
                // Canvas for preview visualization
                let (rect, _) = ui.allocate_exact_size(
                    Vec2::new(self.params.width / 2.0, self.params.height / 2.0),
                    egui::Sense::click_and_drag()
                );
                
                if ui.is_rect_visible(rect) {
                    let painter = ui.painter_at(rect);
                    
                    // Draw background
                    painter.rect_filled(
                        rect,
                        0.0,
                        Color32::from_rgb(20, 20, 40)
                    );
                    
                    // Draw light sources
                    for pos in &self.light_positions {
                        let screen_pos = Pos2::new(
                            rect.min.x + pos.x / 2.0,
                            rect.min.y + pos.y / 2.0
                        );
                        
                        // Draw glow
                        painter.circle_filled(
                            screen_pos,
                            20.0,
                            Color32::from_rgba_premultiplied(255, 255, 150, 30)
                        );
                        
                        // Draw light source
                        painter.circle_filled(
                            screen_pos,
                            5.0,
                            Color32::from_rgb(255, 255, 150)
                        );
                    }
                    
                    // Draw plant seed position
                    let seed_pos = Pos2::new(
                        rect.min.x + self.plant_position.x / 2.0,
                        rect.min.y + self.plant_position.y / 2.0
                    );
                    
                    painter.circle_filled(
                        seed_pos,
                        8.0,
                        Color32::from_rgb(50, 180, 50)
                    );
                    
                    painter.circle_stroke(
                        seed_pos,
                        10.0,
                        Stroke::new(1.0, Color32::from_rgb(100, 255, 100))
                    );
                    
                    // Draw plant position label
                    painter.text(
                        seed_pos + Vec2::new(0.0, -20.0),
                        egui::Align2::CENTER_CENTER,
                        "Plant Seed",
                        egui::FontId::proportional(12.0),
                        Color32::WHITE
                    );
                    
                    // Handle clicks to place the plant
                    if ui.rect_contains_pointer(rect) {
                        if ui.input(|i| i.pointer.primary_clicked()) {
                            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                                // Convert to simulation coordinates (x2 because we're showing at half scale)
                                self.plant_position = Vec2::new(
                                    (pos.x - rect.min.x) * 2.0,
                                    (pos.y - rect.min.y) * 2.0
                                );
                            }
                        }
                    }
                }
                
                ui.add_space(10.0);
                ui.label("Click to place plant seed position");
                
                // Generate new light positions button
                if ui.button("Randomize Light Positions").clicked() {
                    self.regenerate_light_positions();
                }
            });
        });
    }
}

// Implementation for when PlantCreatorApp is used as a standalone app
impl eframe::App for PlantCreatorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Set app-wide visuals
        let mut style = (*ctx.style()).clone();
        style.visuals.window_rounding = 6.0.into();
        style.visuals.widgets.noninteractive.rounding = 2.0.into();
        style.visuals.widgets.inactive.rounding = 2.0.into();
        style.visuals.widgets.active.rounding = 2.0.into();
        style.visuals.widgets.hovered.rounding = 2.0.into();
        style.visuals.window_shadow.offset = Vec2::new(2.0, 8.0);
        ctx.set_style(style);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // In standalone mode, quitting is the only "back" option
            let mut on_back = || {
                frame.close = true;
            };
            
            // In standalone mode, we can't actually start the simulation,
            // but we can show a message that we would have
            let mut on_start = |_engine: CSpaceEngine| {
                // Show a popup or message indicating success
                // In a real app, we might save the configuration to a file
            };
            
            self.render(ui, &mut on_back, &mut on_start);
        });
    }
}