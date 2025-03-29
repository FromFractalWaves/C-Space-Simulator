
// File: src/ui/simulation.rs
use eframe::egui::{self, Color32, Pos2, RichText, Stroke, Vec2};
use crate::engine::{CSpaceEngine, Vector2D};

pub struct SimulationView {
    engine: CSpaceEngine,
    simulation_running: bool,
    simulation_speed: f32,
    current_step: i32,
}

impl SimulationView {
    pub fn new(engine: CSpaceEngine) -> Self {
        Self {
            engine,
            simulation_running: false,
            simulation_speed: 1.0,
            current_step: 0,
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, on_back: &mut dyn FnMut()) {
        // Update simulation if running
        if self.simulation_running {
            // Update speed controls how many steps per frame
            let steps = (self.simulation_speed * 1.0).round() as i32;
            for _ in 0..steps {
                self.engine.update();
                self.current_step += 1;
            }
        }
        
        // Top controls
        ui.horizontal(|ui| {
            // Back button
            if ui.button("⬅️ Back to Menu").clicked() {
                on_back();
            }
            
            ui.add_space(20.0);
            
            // Play/pause button
            if ui.button(if self.simulation_running { "⏸ Pause" } else { "▶️ Play" }).clicked() {
                self.simulation_running = !self.simulation_running;
            }
            
            // Step button (only when paused)
            if !self.simulation_running {
                if ui.button("⏭️ Step").clicked() {
                    self.engine.update();
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
                ui.label(format!("Nodes: {}", self.engine.get_nodes().len()));
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
            let scale_x = rect.width() / self.engine.width;
            let scale_y = rect.height() / self.engine.height;
            let scale = scale_x.min(scale_y);
            
            // Center the view
            let offset_x = (rect.width() - self.engine.width * scale) / 2.0 + rect.min.x;
            let offset_y = (rect.height() - self.engine.height * scale) / 2.0 + rect.min.y;
            
            // Helper function to convert simulation coords to screen coords
            let to_screen = |pos: Vector2D| -> Pos2 {
                Pos2::new(
                    pos.x * scale + offset_x,
                    pos.y * scale + offset_y
                )
            };
            
            // Draw light sources
            for resource in self.engine.get_resources() {
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
            for node in self.engine.get_nodes() {
                let screen_pos = to_screen(node.position);
                
                // Draw connection to parent if it exists
                if let Some(parent_id) = node.parent {
                    // Find parent node
                    if let Some(parent) = self.engine.get_nodes().iter().find(|n| n.id == parent_id) {
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
                } else if node.distortion > 10.5 { // Use a reasonable d_critical value
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
        
        // Request continuous repaint if simulation is running
        if self.simulation_running {
            ctx.request_repaint();
        }
    }
}