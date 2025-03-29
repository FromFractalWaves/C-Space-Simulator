use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box as GtkBox, Orientation};
use std::sync::{Arc, Mutex};
use crate::control::SimulationControl;
use crate::plants::tropisms::Environment;

pub fn build_control_window(
    app: Application,
    environment: Arc<Mutex<Environment>>,
    control: Arc<SimulationControl>, // Change from &SimulationControl to Arc<SimulationControl>
) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(&app)
        .title("Control Window")
        .default_width(400)
        .default_height(300)
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    window.set_child(Some(&vbox));

    let start_btn = Button::with_label("Start Simulation");
    let stop_btn = Button::with_label("Stop Simulation");
    vbox.append(&start_btn);
    vbox.append(&stop_btn);

    let control_start = control.clone();
    let control_stop = control.clone();

    start_btn.connect_clicked(move |_| {
        control_start.start();
    });

    stop_btn.connect_clicked(move |_| {
        control_stop.stop();
    });

    window
}