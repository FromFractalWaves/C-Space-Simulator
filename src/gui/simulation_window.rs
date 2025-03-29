use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DrawingArea};
use glib::ControlFlow;
use crate::plants::tropisms::Plant;
use std::sync::{Arc, Mutex};

pub fn build_simulation_window(
    app: gtk4::Application,
    plants: Arc<Mutex<Vec<Plant>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title("Simulation View");
    window.set_default_size(400, 400);

    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(400, 400);

    let plants_clone = plants.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let plants = plants_clone.lock().unwrap();
        let scale = 20.0;
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        for plant in plants.iter() {
            let pos_x = center_x + plant.pos.x as f64 * scale;
            let pos_y = center_y - plant.pos.y as f64 * scale;

            let stem_x = pos_x + plant.stem_dir.x as f64 * scale;
            let stem_y = pos_y - plant.stem_dir.y as f64 * scale;
            let root_x = pos_x + plant.root_dir.x as f64 * scale;
            let root_y = pos_y - plant.root_dir.y as f64 * scale;

            cr.set_source_rgb(0.0, 1.0, 0.0);
            cr.move_to(pos_x, pos_y);
            cr.line_to(stem_x, stem_y);
            cr.stroke().unwrap();

            cr.set_source_rgb(0.65, 0.16, 0.16);
            cr.move_to(pos_x, pos_y);
            cr.line_to(root_x, root_y);
            cr.stroke().unwrap();

            cr.set_source_rgb(1.0, 0.0, 0.0);
            cr.arc(pos_x, pos_y, 3.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill().unwrap();
        }
    });

    gtk4::timeout_add(16, move || {
        drawing_area.queue_draw();
        ControlFlow::Continue
    });

    window.set_child(Some(&drawing_area));
    window
}