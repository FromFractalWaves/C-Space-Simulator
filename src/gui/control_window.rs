use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Button, Label, Scale, Orientation};
use nalgebra::Vector3;
use crate::plants::tropisms::Environment;
use std::sync::{Arc, Mutex};

pub fn build_control_window(
    app: gtk4::Application,
    environment: Arc<Mutex<Environment>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title("Control Panel");
    window.set_default_size(300, 400);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_all(10);

    let light_x = Scale::with_range(Orientation::Horizontal, -10.0, 10.0, 0.1);
    light_x.set_value(environment.lock().unwrap().light_pos.x as f64);
    container.append(&Label::new(Some("Light Pos X:")));
    container.append(&light_x);

    let light_y = Scale::with_range(Orientation::Horizontal, -10.0, 10.0, 0.1);
    light_y.set_value(environment.lock().unwrap().light_pos.y as f64);
    container.append(&Label::new(Some("Light Pos Y:")));
    container.append(&light_y);

    let water_x = Scale::with_range(Orientation::Horizontal, -10.0, 10.0, 0.1);
    water_x.set_value(environment.lock().unwrap().water_pos.x as f64);
    container.append(&Label::new(Some("Water Pos X:")));
    container.append(&water_x);

    let water_y = Scale::with_range(Orientation::Horizontal, -10.0, 10.0, 0.1);
    water_y.set_value(environment.lock().unwrap().water_pos.y as f64);
    container.append(&Label::new(Some("Water Pos Y:")));
    container.append(&water_y);

    let light_intensity = Scale::with_range(Orientation::Horizontal, 0.0, 2.0, 0.01);
    light_intensity.set_value(environment.lock().unwrap().light_intensity as f64);
    container.append(&Label::new(Some("Light Intensity:")));
    container.append(&light_intensity);

    let water_level = Scale::with_range(Orientation::Horizontal, 0.0, 2.0, 0.01);
    water_level.set_value(environment.lock().unwrap().water_level as f64);
    container.append(&Label::new(Some("Water Level:")));
    container.append(&water_level);

    let add_obstacle = Button::with_label("Add Obstacle");
    container.append(&add_obstacle);

    let env = environment.clone();
    light_x.connect_value_changed(move |scale| {
        env.lock().unwrap().light_pos.x = scale.value() as f32;
    });
    let env = environment.clone();
    light_y.connect_value_changed(move |scale| {
        env.lock().unwrap().light_pos.y = scale.value() as f32;
    });
    let env = environment.clone();
    water_x.connect_value_changed(move |scale| {
        env.lock().unwrap().water_pos.x = scale.value() as f32;
    });
    let env = environment.clone();
    water_y.connect_value_changed(move |scale| {
        env.lock().unwrap().water_pos.y = scale.value() as f32;
    });
    let env = environment.clone();
    light_intensity.connect_value_changed(move |scale| {
        env.lock().unwrap().light_intensity = scale.value() as f32;
    });
    let env = environment.clone();
    water_level.connect_value_changed(move |scale| {
        env.lock().unwrap().water_level = scale.value() as f32;
    });
    let env = environment.clone();
    add_obstacle.connect_clicked(move |_| {
        env.lock().unwrap().obstacles.push(Vector3::new(1.0, 1.0, 0.0));
    });

    window.set_child(Some(&container));
    window
}