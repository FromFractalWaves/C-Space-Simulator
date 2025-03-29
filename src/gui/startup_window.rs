use gtk4::prelude::*;
use gtk4::Application;
use glib::ControlFlow;
use glib::timeout_add;
use crate::engines::plant_engine::PlantEngine;
use crate::gui::{
    control_window, dev_window, environment_window, plant_diagnostics_window, simulation_window,
};
use std::sync::{Arc, Mutex};
use std::time::Duration; // Added for Duration

pub fn launch() {
    let app = Application::new(Some("com.example.simulator"), Default::default());
    
    app.connect_activate(move |app| {
        let env = crate::simulation::simulation_env::SimulationEnv::new();
        let plants_shared = Arc::new(Mutex::new(env.plants.clone()));
        let environment_shared = Arc::new(Mutex::new(env.environment.clone()));
        let logs = Arc::new(Mutex::new(Vec::new()));
        let mut engine = PlantEngine::new(env);
        
        let control_win = control_window::build_control_window(app.clone(), environment_shared.clone());
        let dev_win = dev_window::build_dev_window(app, logs.clone()); // Changed to app (reference)
        let env_win = environment_window::build_environment_window(app.clone(), environment_shared.clone());
        let diag_win = plant_diagnostics_window::build_plant_diagnostics_window(app.clone(), plants_shared.clone());
        let sim_win = simulation_window::build_simulation_window(app.clone(), plants_shared.clone());
        
        timeout_add(Duration::from_millis(100), move || { // Changed 100 to Duration::from_millis(100)
            let dt = 0.1;
            let results = engine.update(dt);
            
            {
                let mut logs = logs.lock().unwrap();
                for plant_results in results {
                    for result in plant_results {
                        logs.push(result.log);
                        if logs.len() > 100 {
                            logs.remove(0);
                        }
                    }
                }
            }
            
            {
                let mut plants = plants_shared.lock().unwrap();
                *plants = engine.env.plants.clone();
            }
            {
                let mut env = environment_shared.lock().unwrap();
                *env = engine.env.environment.clone();
            }
            
            ControlFlow::Continue
        });
        
        control_win.present();
        dev_win.present();
        env_win.present();
        diag_win.present();
        sim_win.present();
    });
    
    app.run();
}