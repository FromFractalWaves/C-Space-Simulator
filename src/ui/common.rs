// File: src/ui/common.rs
use eframe::egui::{self, RichText};

#[derive(Clone)]
pub struct SimulationParams {
    // CSpace parameters
    pub alpha: f32,
    pub beta: f32,
    pub epsilon: f32,
    pub d_critical: f32,
    pub lambda: f32,
    
    // Plant parameters
    pub growth_rate: f32,
    pub growth_prob: f32,
    pub branch_prob: f32,
    pub max_nodes: usize,
    pub initial_energy: f32,
    
    // Environment parameters
    pub width: f32,
    pub height: f32,
    pub max_energy_distance: f32,
    pub num_light_sources: usize,
    pub light_intensity: f32,
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

// Parameter categories for organization
pub enum ParamTab {
    CSpace,
    Plant,
    Environment,
}

// Helper functions for UI components
pub fn draw_cspace_params(ui: &mut egui::Ui, params: &mut SimulationParams) {
    ui.heading("C-Space Parameters");
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Alpha:");
        ui.add(egui::DragValue::new(&mut params.alpha)
            .speed(0.01)
            .clamp_range(0.01..=1.0)
        );
        ui.label("(coherence decay)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Beta:");
        ui.add(egui::DragValue::new(&mut params.beta)
            .speed(0.01)
            .clamp_range(0.01..=1.0)
        );
        ui.label("(temporal growth)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Epsilon:");
        ui.add(egui::DragValue::new(&mut params.epsilon)
            .speed(0.000000001)
            .clamp_range(1e-15..=1e-3)
            .prefix("1e-")
            .custom_formatter(|n, _| format!("{:e}", n))
        );
        ui.label("(singularity prevention)");
    });
    
    ui.horizontal(|ui| {
        ui.label("D-Critical:");
        ui.add(egui::DragValue::new(&mut params.d_critical)
            .speed(0.1)
            .clamp_range(1.0..=100.0)
        );
        ui.label("(singularity threshold)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Lambda:");
        ui.add(egui::DragValue::new(&mut params.lambda)
            .speed(0.05)
            .clamp_range(0.01..=5.0)
        );
        ui.label("(attention decay)");
    });
    
    ui.add_space(10.0);
    ui.label(RichText::new("These parameters affect the core C-Space mechanics.").italics());
}

pub fn draw_plant_params(ui: &mut egui::Ui, params: &mut SimulationParams) {
    ui.heading("Plant Parameters");
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Growth Rate:");
        ui.add(egui::DragValue::new(&mut params.growth_rate)
            .speed(0.1)
            .clamp_range(0.1..=20.0)
        );
        ui.label("(segment length)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Growth Probability:");
        ui.add(egui::DragValue::new(&mut params.growth_prob)
            .speed(0.01)
            .clamp_range(0.0..=1.0)
        );
    });
    
    ui.horizontal(|ui| {
        ui.label("Branch Probability:");
        ui.add(egui::DragValue::new(&mut params.branch_prob)
            .speed(0.01)
            .clamp_range(0.0..=1.0)
        );
    });
    
    ui.horizontal(|ui| {
        ui.label("Max Nodes:");
        ui.add(egui::DragValue::new(&mut params.max_nodes)
            .speed(1.0)
            .clamp_range(10..=10000)
        );
        ui.label("(complexity limit)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Initial Energy:");
        ui.add(egui::DragValue::new(&mut params.initial_energy)
            .speed(0.1)
            .clamp_range(0.1..=10.0)
        );
    });
    
    ui.add_space(10.0);
    ui.label(RichText::new("These parameters control the plant's growth behavior.").italics());
}

pub fn draw_environment_params(ui: &mut egui::Ui, params: &mut SimulationParams, regenerate_light: &mut dyn FnMut()) {
    ui.heading("Environment Parameters");
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("Width:");
        if ui.add(egui::DragValue::new(&mut params.width)
            .speed(10.0)
            .clamp_range(100.0..=2000.0)
        ).changed() {
            regenerate_light();
        }
    });
    
    ui.horizontal(|ui| {
        ui.label("Height:");
        if ui.add(egui::DragValue::new(&mut params.height)
            .speed(10.0)
            .clamp_range(100.0..=2000.0)
        ).changed() {
            regenerate_light();
        }
    });
    
    ui.horizontal(|ui| {
        ui.label("Max Energy Distance:");
        ui.add(egui::DragValue::new(&mut params.max_energy_distance)
            .speed(5.0)
            .clamp_range(10.0..=500.0)
        );
        ui.label("(light reach)");
    });
    
    ui.horizontal(|ui| {
        ui.label("Light Sources:");
        if ui.add(egui::DragValue::new(&mut params.num_light_sources)
            .speed(1.0)
            .clamp_range(1..=10)
        ).changed() {
            regenerate_light();
        }
    });
    
    ui.horizontal(|ui| {
        ui.label("Light Intensity:");
        ui.add(egui::DragValue::new(&mut params.light_intensity)
            .speed(0.1)
            .clamp_range(0.1..=5.0)
        );
    });
    
    ui.add_space(10.0);
    ui.label(RichText::new("These parameters define the simulation environment.").italics());
}