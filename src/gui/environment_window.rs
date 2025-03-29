use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, Orientation};
use nalgebra::Matrix3;
use crate::plants::tropisms::Environment;
use std::sync::{Arc, Mutex};

pub fn build_environment_window(
    app: gtk4::Application,
    environment: Arc<Mutex<Environment>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(&app);
    window.set_title(Some("Environment Overview"));
    window.set_default_size(300, 400);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);

    let env = environment.lock().unwrap();
    container.append(&Label::new(Some("Environment Metrics")));
    container.append(&Label::new(Some(&format!("Light Position: {:?}", env.light_pos))));
    container.append(&Label::new(Some(&format!("Water Position: {:?}", env.water_pos))));
    container.append(&Label::new(Some(&format!("Gravity: {:?}", env.gravity))));
    container.append(&Label::new(Some(&format!("Light Intensity: {:.2}", env.light_intensity))));
    container.append(&Label::new(Some(&format!("Water Level: {:.2}", env.water_level))));
    container.append(&Label::new(Some(&format!("Obstacles: {}", env.obstacles.len()))));

    let tensor = env.metric_tensor;
    container.append(&Label::new(Some("Metric Tensor")));
    container.append(&Label::new(Some(&format!(
        "{:>8.4} {:>8.4} {:>8.4}",
        tensor[(0, 0)], tensor[(0, 1)], tensor[(0, 2)]
    ))));
    container.append(&Label::new(Some(&format!(
        "{:>8.4} {:>8.4} {:>8.4}",
        tensor[(1, 0)], tensor[(1, 1)], tensor[(1, 2)]
    ))));
    container.append(&Label::new(Some(&format!(
        "{:>8.4} {:>8.4} {:>8.4}",
        tensor[(2, 0)], tensor[(2, 1)], tensor[(2, 2)]
    ))));
    container.append(&Label::new(Some(&format!("Critical Distortion: {:.2}", env.d_critical))));

    window.set_child(Some(&container));
    window
}