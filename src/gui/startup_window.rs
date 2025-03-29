use gtk4::prelude::*;
use gtk4::Application;
use gtk4::{ApplicationWindow, Button, Box as GtkBox, Orientation};
use glib::source::idle_add;
use crate::engines::plant_engine::PlantEngine;
use crate::gui::{
    control_window, dev_window, environment_window, plant_diagnostics_window, simulation_window,
};
use std::sync::{Arc, Mutex};

pub fn launch() {
    let app = Application::new(Some("com.example.simulator"), Default::default());

    app.connect_activate(move |app| {
        let env = crate::simulation::simulation_env::SimulationEnv::new();
        let plants_shared = Arc::new(Mutex::new(env.plants.clone()));
        let environment_shared = Arc::new(Mutex::new(env.environment.clone()));
        let logs = Arc::new(Mutex::new(Vec::new()));
        let engine = Arc::new(Mutex::new(PlantEngine::new(env)));

        let app_clone = app.clone();

        let window = ApplicationWindow::builder()
            .application(&app_clone)
            .title("Simulator Startup")
            .default_width(300)
            .default_height(200)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 10);
        window.set_child(Some(&vbox));

        let control_btn = Button::with_label("Open Control Window");
        let dev_btn = Button::with_label("Open Dev Window");
        let env_btn = Button::with_label("Open Environment Window");
        let diag_btn = Button::with_label("Open Diagnostics Window");
        let sim_btn = Button::with_label("Open Simulation Window");

        vbox.append(&control_btn);
        vbox.append(&dev_btn);
        vbox.append(&env_btn);
        vbox.append(&diag_btn);
        vbox.append(&sim_btn);

        // Clone shared data for each closure
        let environment_shared_control = environment_shared.clone();
        let logs_dev = logs.clone();
        let environment_shared_env = environment_shared.clone();
        let plants_shared_diag = plants_shared.clone();
        let plants_shared_sim = plants_shared.clone();
        let logs_sim = logs.clone();
        let environment_shared_sim = environment_shared.clone();
        let engine_sim = engine.clone();
        let app_clone_control = app_clone.clone();
        let app_clone_dev = app_clone.clone();
        let app_clone_env = app_clone.clone();
        let app_clone_diag = app_clone.clone();
        let app_clone_sim = app_clone.clone();

        control_btn.connect_clicked(move |_| {
            let control_win = control_window::build_control_window(
                app_clone_control.clone(),
                environment_shared_control.clone(),
            );
            control_win.present();
        });

        dev_btn.connect_clicked(move |_| {
            let dev_win = dev_window::build_dev_window(
                app_clone_dev.clone(),
                logs_dev.clone(),
            );
            dev_win.present();
        });

        env_btn.connect_clicked(move |_| {
            let env_win = environment_window::build_environment_window(
                app_clone_env.clone(),
                environment_shared_env.clone(),
            );
            env_win.present();
        });

        diag_btn.connect_clicked(move |_| {
            let diag_win = plant_diagnostics_window::build_plant_diagnostics_window(
                app_clone_diag.clone(),
                plants_shared_diag.clone(),
            );
            diag_win.present();
        });

        sim_btn.connect_clicked(move |_| {
            let mut engine_locked = engine_sim.lock().unwrap();
            let sim_win = simulation_window::build_simulation_window(
                app_clone_sim.clone(),
                plants_shared_sim.clone(),
                &mut engine_locked,
                logs_sim.clone(),
                environment_shared_sim.clone(),
            );
            sim_win.present();
        });

        let engine_idle = engine.clone();
        let plants_shared_idle = plants_shared.clone();
        let environment_shared_idle = environment_shared.clone();
        let logs_idle = logs.clone();

        idle_add(move || {
            let dt = 0.1;
            let mut engine = engine_idle.lock().unwrap();
            let results = engine.update(dt);

            {
                let mut logs = logs_idle.lock().unwrap();
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
                let mut plants = plants_shared_idle.lock().unwrap();
                *plants = engine.env.plants.clone();
            }
            {
                let mut env = environment_shared_idle.lock().unwrap();
                *env = engine.env.environment.clone();
            }

            glib::ControlFlow::Continue
        });

        window.present();
    });

    app.run();
}