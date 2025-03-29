// File: src/ui/app.rs
use eframe::egui;

use crate::engine::CSpaceEngine;
use super::main_menu;
use super::plant::PlantCreatorApp;
use super::simulation::SimulationView;

// Application states for navigation
pub enum AppState {
    MainMenu,
    PlantCreator,
    Simulation,
}

pub struct CSpaceApp {
    state: AppState,
    plant_creator: PlantCreatorApp,
    simulation: Option<SimulationView>,
}

impl Default for CSpaceApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            plant_creator: PlantCreatorApp::default(),
            simulation: None,
        }
    }
}

impl eframe::App for CSpaceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set app-wide visuals
        let mut style = (*ctx.style()).clone();
        style.visuals.window_rounding = 6.0.into();
        style.visuals.widgets.noninteractive.rounding = 2.0.into();
        style.visuals.widgets.inactive.rounding = 2.0.into();
        style.visuals.widgets.active.rounding = 2.0.into();
        style.visuals.widgets.hovered.rounding = 2.0.into();
        style.visuals.window_shadow.offset = egui::Vec2::new(2.0, 8.0);
        ctx.set_style(style);
        
        // Define the UI layout based on current state
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::MainMenu => {
                    let mut on_plant_creator = || self.state = AppState::PlantCreator;
                    let mut on_continue = || self.state = AppState::Simulation;
                    main_menu::render_main_menu(ui, &mut on_plant_creator, &mut on_continue, self.simulation.is_some());
                },
                AppState::PlantCreator => {
                    // Handle navigation from plant creator
                    let mut back_to_main = || self.state = AppState::MainMenu;
                    let mut launch_simulation = |engine: CSpaceEngine| {
                        self.simulation = Some(SimulationView::new(engine));
                        self.state = AppState::Simulation;
                    };
                    
                    // Render the plant creator
                    self.plant_creator.render(ui, &mut back_to_main, &mut launch_simulation);
                },
                AppState::Simulation => {
                    if let Some(simulation) = &mut self.simulation {
                        let mut on_back = || self.state = AppState::MainMenu;
                        simulation.render(ctx, ui, &mut on_back);
                    } else {
                        // Fallback if simulation is not available
                        self.state = AppState::MainMenu;
                    }
                },
            }
        });
        
        // Request continuous repaint if simulation is running
        if let Some(simulation) = &self.simulation {
            if simulation.simulation_running {
                ctx.request_repaint();
            }
        }
    }
}