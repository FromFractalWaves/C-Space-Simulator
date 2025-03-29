use eframe::egui;
use egui::{Color32, Pos2, RichText, Stroke, Vec2};

mod engine;
use engine::{CSpaceEngine, EngineParams, Vector2D};

// Application states for navigation
enum AppState {
    MainMenu,
    PlantCreator,
    Simulation,
}

// Parameter categories for organization
enum ParamTab {
    CSpace,
    Plant,
    Environment,
}

// Structure to hold all simulation parameters
#[derive(Clone)]
struct SimulationParams {
    // CSpace parameters
    alpha: f32,
    beta: f32,
    epsilon: f32,
    d_critical: f32,
    lambda: f32,
    
    // Plant parameters
    growth_rate: f32,
    growth_prob: f32,
    branch_prob: f32,
    max_nodes: usize,
    initial_energy: f32,
    
    // Environment parameters
    width: f32,
    height: f32,
    max_energy_distance: f32,
    num_light_sources: usize,
    light_intensity: f32,
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            // Default CSpace parameters
            alpha: 0.2,
            beta: 0.3,
            epsilon: 1e-9,
            d_critical: 15.0,
            lambda: 0.5,
            
            // Default Plant parameters
            growth_rate: 5.0,
            growth_prob: 0.3,
            branch_prob: 0.1,
            max_nodes: 500,
            initial_energy: 1.0,
            
            // Default Environment parameters
            width: 800.0,
            height: 600.0,
            max_energy_distance: 200.0,
            num_light_sources: 3,
            light_intensity: 1.0,
        }
    }
}

struct CSpaceApp {
    state: AppState,
    current_tab: ParamTab,
    params: SimulationParams,
    engine: Option<CSpaceEngine>,
    simulation_running: bool,
    simulation_speed: f32,
    current_step: i32,
    plant_position: Vec2,
    light_positions: Vec<Vec2>,
    param_valid: bool,
}

impl Default for CSpaceApp {
    fn default() -> Self {
        let mut app = Self {
            state: AppState::MainMenu,
            current_tab: ParamTab::CSpace,
            params: SimulationParams::default(),
            engine: None,
            simulation_running: false,
            simulation_speed: 1.0,
            current_step: 0,
            plant_position: Vec2::new(400.0, 500.0),
            light_positions: Vec::new(),
            param_valid: true,
        };
        
        // Generate some default light positions
        app.regenerate_light_positions();
        
        app
    }
}

impl CSpaceApp {
    fn regenerate_light_positions(&mut self) {
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
    
    fn initialize_simulation(&mut self) {
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
        
        self.engine = Some(engine);
        self.current_step = 0;
        self.simulation_running = false;
    }
    
    fn validate_params(&mut self) -> bool {
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
    
    fn render_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Add some space at the top
            ui.add_space(20.0);
            
            // Custom heading with larger text and styling
            let title = "C-SPACE SIMULATION VISUALIZER";
            let heading = RichText::new(title)
                .size(32.0)
                .strong()
                .color(Color32::from_rgb(65, 185, 255));
            ui.label(heading);
            
            // Add a separator line
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(30.0);
            
            // Main menu buttons
            let button_size = Vec2::new(220.0, 50.0);
            
            if ui.add_sized(button_size, egui::Button::new(
                RichText::new("🌱 Launch Plant Creator").size(18.0)
            )).clicked() {
                self.state = AppState::PlantCreator;
            }
            
            ui.add_space(15.0);
            
            if self.engine.is_some() {
                if ui.add_sized(button_size, egui::Button::new(
                    RichText::new("▶️ Continue Simulation").size(18.0)
                )).clicked() {
                    self.state = AppState::Simulation;
                }
                
                ui.add_space(15.0);
            }
            
            // Push the credit text to the bottom of the window
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("@FractalWaves");
                ui.add_space(5.0);
                ui.label("C-Space Framework Implementation");
            });
        });
    }
    
    fn render_plant_creator(&mut self, ui: &mut egui::Ui) {
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
                            ParamTab::CSpace => self.render_cspace_params(ui),
                            ParamTab::Plant => self.render_plant_params(ui),
                            ParamTab::Environment => self.render_environment_params(ui),
                        }
                    });
                
                ui.add_space(10.0);
                
                // Navigation and action buttons
                ui.horizontal(|ui| {
                    if ui.button("Back to Menu").clicked() {
                        self.state = AppState::MainMenu;
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
                            self.initialize_simulation();
                            self.state = AppState::Simulation;
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
    
    fn render_cspace_params(&mut self, ui: &mut egui::Ui) {
        ui.heading("C-Space Parameters");
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label("Alpha:");
            ui.add(egui::DragValue::new(&mut self.params.alpha)
                .speed(0.01)
                .clamp_range(0.01..=1.0)
            );
            ui.label("(coherence decay)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Beta:");
            ui.add(egui::DragValue::new(&mut self.params.beta)
                .speed(0.01)
                .clamp_range(0.01..=1.0)
            );
            ui.label("(temporal growth)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Epsilon:");
            ui.add(egui::DragValue::new(&mut self.params.epsilon)
                .speed(0.000000001)
                .clamp_range(1e-15..=1e-3)
                .prefix("1e-")
                .custom_formatter(|n, _| format!("{:e}", n))
            );
            ui.label("(singularity prevention)");
        });
        
        ui.horizontal(|ui| {
            ui.label("D-Critical:");
            ui.add(egui::DragValue::new(&mut self.params.d_critical)
                .speed(0.1)
                .clamp_range(1.0..=100.0)
            );
            ui.label("(singularity threshold)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Lambda:");
            ui.add(egui::DragValue::new(&mut self.params.lambda)
                .speed(0.05)
                .clamp_range(0.01..=5.0)
            );
            ui.label("(attention decay)");
        });
        
        ui.add_space(10.0);
        ui.label(RichText::new("These parameters affect the core C-Space mechanics.").italics());
    }
    
    fn render_plant_params(&mut self, ui: &mut egui::Ui) {
        ui.heading("Plant Parameters");
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label("Growth Rate:");
            ui.add(egui::DragValue::new(&mut self.params.growth_rate)
                .speed(0.1)
                .clamp_range(0.1..=20.0)
            );
            ui.label("(segment length)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Growth Probability:");
            ui.add(egui::DragValue::new(&mut self.params.growth_prob)
                .speed(0.01)
                .clamp_range(0.0..=1.0)
            );
        });
        
        ui.horizontal(|ui| {
            ui.label("Branch Probability:");
            ui.add(egui::DragValue::new(&mut self.params.branch_prob)
                .speed(0.01)
                .clamp_range(0.0..=1.0)
            );
        });
        
        ui.horizontal(|ui| {
            ui.label("Max Nodes:");
            ui.add(egui::DragValue::new(&mut self.params.max_nodes)
                .speed(1.0)
                .clamp_range(10..=10000)
            );
            ui.label("(complexity limit)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Initial Energy:");
            ui.add(egui::DragValue::new(&mut self.params.initial_energy)
                .speed(0.1)
                .clamp_range(0.1..=10.0)
            );
        });
        
        ui.add_space(10.0);
        ui.label(RichText::new("These parameters control the plant's growth behavior.").italics());
    }
    
    fn render_environment_params(&mut self, ui: &mut egui::Ui) {
        ui.heading("Environment Parameters");
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label("Width:");
            if ui.add(egui::DragValue::new(&mut self.params.width)
                .speed(10.0)
                .clamp_range(100.0..=2000.0)
            ).changed() {
                self.regenerate_light_positions();
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Height:");
            if ui.add(egui::DragValue::new(&mut self.params.height)
                .speed(10.0)
                .clamp_range(100.0..=2000.0)
            ).changed() {
                self.regenerate_light_positions();
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Max Energy Distance:");
            ui.add(egui::DragValue::new(&mut self.params.max_energy_distance)
                .speed(5.0)
                .clamp_range(10.0..=500.0)
            );
            ui.label("(light reach)");
        });
        
        ui.horizontal(|ui| {
            ui.label("Light Sources:");
            if ui.add(egui::DragValue::new(&mut self.params.num_light_sources)
                .speed(1.0)
                .clamp_range(1..=10)
            ).changed() {
                self.regenerate_light_positions();
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Light Intensity:");
            ui.add(egui::DragValue::new(&mut self.params.light_intensity)
                .speed(0.1)
                .clamp_range(0.1..=5.0)
            );
        });
        
        ui.add_space(10.0);
        ui.label(RichText::new("These parameters define the simulation environment.").italics());
    }
    
    fn render_simulation(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if let Some(engine) = &mut self.engine {
            // Update simulation if running
            if self.simulation_running {
                // Update speed controls how many steps per frame
                let steps = (self.simulation_speed * 1.0).round() as i32;
                for _ in 0..steps {
                    engine.update();
                    self.current_step += 1;
                }
            }
            
            // Top controls
            ui.horizontal(|ui| {
                // Back button
                if ui.button("⬅️ Back to Menu").clicked() {
                    self.state = AppState::MainMenu;
                }
                
                ui.add_space(20.0);
                
                // Play/pause button
                if ui.button(if self.simulation_running { "⏸ Pause" } else { "▶️ Play" }).clicked() {
                    self.simulation_running = !self.simulation_running;
                }
                
                // Step button (only when paused)
                if !self.simulation_running {
                    if ui.button("⏭️ Step").clicked() {
                        engine.update();
                        self.current_step += 1;
                    }
                }
                
                // Simulation speed slider
                ui.add_space(20.0);
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.simulation_speed, 0.1..=5.0)
                    .text("×")
                );
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Step: {}", self.current_step));
                    ui.label(format!("Nodes: {}", engine.get_nodes().len()));
                });
            });
            
            ui.separator();
            
            // Main view
            let available_size = ui.available_size();
            let (rect, _) = ui.allocate_exact_size(available_size, egui::Sense::click());
            
            if ui.is_rect_visible(rect) {
                let painter = ui.painter_at(rect);
                
                // Draw background
                painter.rect_filled(
                    rect,
                    0.0,
                    Color32::from_rgb(10, 10, 30)
                );
                
                // Calculate scaling factors to fit simulation in view
                let scale_x = rect.width() / self.params.width;
                let scale_y = rect.height() / self.params.height;
                let scale = scale_x.min(scale_y);
                
                // Center the view
                let offset_x = (rect.width() - self.params.width * scale) / 2.0 + rect.min.x;
                let offset_y = (rect.height() - self.params.height * scale) / 2.0 + rect.min.y;
                
                // Helper function to convert simulation coords to screen coords
                let to_screen = |pos: Vector2D| -> Pos2 {
                    Pos2::new(
                        pos.x * scale + offset_x,
                        pos.y * scale + offset_y
                    )
                };
                
                // Draw light sources
                for resource in engine.get_resources() {
                    if resource.r_type == "light" {
                        let screen_pos = to_screen(resource.position);
                        
                        // Draw light glow
                        painter.circle_filled(
                            screen_pos,
                            resource.intensity * 20.0 * scale,
                            Color32::from_rgba_premultiplied(255, 255, 150, 30)
                        );
                        
                        // Draw light source
                        painter.circle_filled(
                            screen_pos,
                            5.0 * scale,
                            Color32::from_rgb(255, 255, 150)
                        );
                    }
                }
                
                // Draw plant nodes and connections
                for node in engine.get_nodes() {
                    let screen_pos = to_screen(node.position);
                    
                    // Draw connection to parent if it exists
                    if let Some(parent_id) = node.parent {
                        // Find parent node
                        if let Some(parent) = engine.get_nodes().iter().find(|n| n.id == parent_id) {
                            let parent_pos = to_screen(parent.position);
                            
                            // Calculate color based on energy
                            let energy_color = Color32::from_rgb(
                                (50.0 + 100.0 * node.energy) as u8,
                                (180.0 + 50.0 * node.energy) as u8,
                                50
                            );
                            
                            // Draw branch
                            painter.line_segment(
                                [parent_pos, screen_pos],
                                Stroke::new(1.5 * scale * node.energy, energy_color)
                            );
                        }
                    }
                    
                    // Draw node
                    let node_radius = (2.0 + node.coherence * 2.0) * scale;
                    
                    // Node color based on its properties
                    let node_color = if node.coherence < 0.2 {
                        // Low coherence: reddish
                        Color32::from_rgb(200, 100, 50)
                    } else if node.distortion > self.params.d_critical * 0.7 {
                        // High distortion: purplish
                        Color32::from_rgb(180, 50, 200)
                    } else {
                        // Normal: greenish
                        Color32::from_rgb(50, 180, 50)
                    };
                    
                    painter.circle_filled(
                        screen_pos,
                        node_radius,
                        node_color
                    );
                }
            }
        }
    }
}

impl eframe::App for CSpaceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set app-wide visuals
        let mut style = (*ctx.style()).clone();
        style.visuals.window_rounding = 6.0.into();
        style.visuals.widgets.noninteractive.rounding = 2.0.into(); // Updated for newer egui
        style.visuals.widgets.inactive.rounding = 2.0.into();
        style.visuals.widgets.active.rounding = 2.0.into();
        style.visuals.widgets.hovered.rounding = 2.0.into();
        style.visuals.window_shadow.offset = Vec2::new(2.0, 8.0); // Use offset instead of extrusion
        ctx.set_style(style);
        
        // Define the UI layout based on current state
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::MainMenu => self.render_main_menu(ui),
                AppState::PlantCreator => self.render_plant_creator(ui),
                AppState::Simulation => self.render_simulation(ctx, ui),
            }
        });
        
        // Request continuous repaint if simulation is running
        if self.simulation_running {
            ctx.request_repaint();
        }
    }
}

fn main() {
    // Launch the native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]), // Larger default size
        ..Default::default()
    };
    eframe::run_native(
        "C-Space Simulation Visualizer", // Window title
        options,
        Box::new(|_cc| Box::new(CSpaceApp::default())), // Initialize our app
    ).expect("Failed to launch application");
}