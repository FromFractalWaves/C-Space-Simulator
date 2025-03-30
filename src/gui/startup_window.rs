use gtk4::prelude::*;
use gtk4::Application;
use gtk4::{ApplicationWindow, Button, Box as GtkBox, Orientation};
use crate::control::SimulationControl;
use crate::gui::{
    control_window, dev_window, environment_window, plant_diagnostics_window, simulation_window,
};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use crate::plants::tropisms::TropismResult;
use crate::simulation::simulation_runner::ControlCommand;

pub fn launch_with_runner(command_sender: Sender<ControlCommand>, log_receiver: Receiver<Vec<Vec<TropismResult>>>) {
    let app = Application::new(Some("com.example.simulator"), Default::default());
    let log_receiver = Arc::new(Mutex::new(log_receiver)); // Wrap in Arc<Mutex>

    app.connect_activate(move |app| {
        let control = Arc::new(SimulationControl::new(crate::simulation::simulation_env::SimulationEnv::new()));
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

        let control_control = control.clone();
        let control_dev = control.clone();
        let control_env = control.clone();
        let control_diag = control.clone();
        let control_sim = control.clone();
        let app_clone_control = app_clone.clone();
        let app_clone_dev = app_clone.clone();
        let app_clone_env = app_clone.clone();
        let app_clone_diag = app_clone.clone();
        let app_clone_sim = app_clone.clone();
        let command_sender_control = command_sender.clone();
        let command_sender_dev = command_sender.clone();
        let log_receiver_dev = log_receiver.clone(); // Clone the Arc

        control_btn.connect_clicked(move |_| {
            let control_win = control_window::build_control_window(
                app_clone_control.clone(),
                control_control.environment(),
                control_control.clone(),
            );
            control_win.present();
            command_sender_control.send(ControlCommand::Start).unwrap();
        });

        dev_btn.connect_clicked(move |_| {
            let dev_win = dev_window::build_dev_window(
                app_clone_dev.clone(),
                control_dev.logs(),
                log_receiver_dev.clone(), // Pass only log_receiver
            );
            dev_win.present();
        });

        env_btn.connect_clicked(move |_| {
            let env_win = environment_window::build_environment_window(
                app_clone_env.clone(),
                control_env.environment(),
            );
            env_win.present();
        });

        diag_btn.connect_clicked(move |_| {
            let diag_win = plant_diagnostics_window::build_plant_diagnostics_window(
                app_clone_diag.clone(),
                control_diag.plants(),
            );
            diag_win.present();
        });

        sim_btn.connect_clicked(move |_| {
            let engine_lock = control_sim.engine();
            let mut engine = engine_lock.lock().unwrap();
            let sim_win = simulation_window::build_simulation_window(
                app_clone_sim.clone(),
                control_sim.plants(),
                &mut engine,
                control_sim.logs(),
                control_sim.environment(),
            );
            sim_win.present();
        });

        window.present();
    });

    app.run();
}