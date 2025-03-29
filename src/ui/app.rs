// File: src/ui/app.rs
use eframe::egui;
use crate::engine::CSpaceEngine;
use super::main_menu;
use super::plant::PlantCreatorApp;
use super::simulation::SimulationView;
use std::rc::Rc;
use std::cell::RefCell;  // Add these imports at the top

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
                    // CHANGE HERE: Use Rc<RefCell<>> to allow multiple mutable borrows
                    let state = Rc::new(RefCell::new(&mut self.state));
                    let state_clone1 = state.clone();
                    let state_clone2 = state.clone();
                    
                    let mut on_plant_creator = move || {
                        **state_clone1.borrow_mut() = AppState::PlantCreator;
                    };
                    
                    let mut on_continue = move || {
                        **state_clone2.borrow_mut() = AppState::Simulation;
                    };
                    
                    main_menu::render_main_menu(ui, &mut on_plant_creator, &mut on_continue, self.simulation.is_some());
                },
                AppState::PlantCreator => {
                    // CHANGE HERE: Use Rc<RefCell<>> for the second set of closures
                    let state = Rc::new(RefCell::new(&mut self.state));
                    let state_clone1 = state.clone();
                    let state_clone2 = state.clone();
                    let simulation_ref = Rc::new(RefCell::new(&mut self.simulation));
                    
                    let mut back_to_main = move || {
                        **state_clone1.borrow_mut() = AppState::MainMenu;
                    };
                    
                    let mut launch_simulation = move |engine: CSpaceEngine| {
                        **simulation_ref.borrow_mut() = Some(SimulationView::new(engine));
                        **state_clone2.borrow_mut() = AppState::Simulation;
                    };
                    
                    // Render the plant creator
                    self.plant_creator.render(ui, &mut back_to_main, &mut launch_simulation);
                },
                AppState::Simulation => {
                    if let Some(simulation) = &mut self.simulation {
                        // CHANGE HERE: For consistency, use the same pattern
                        let state = Rc::new(RefCell::new(&mut self.state));
                        let state_clone = state.clone();
                        
                        let mut on_back = move || {
                            **state_clone.borrow_mut() = AppState::MainMenu;
                        };
                        
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