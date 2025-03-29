use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DrawingArea};
use crate::plants::tropisms::Plant;
use crate::engines::plant_engine::PlantEngine;
use std::sync::{Arc, Mutex};

pub fn build_simulation_window(
    app: gtk4::Application,
    plants: Arc<Mutex<Vec<Plant>>>,
    engine: &mut PlantEngine, // Pass engine reference to access updated state
    logs: Arc<Mutex<Vec<String>>>, // For logging if needed
    environment: Arc<Mutex<crate::plants::tropisms::Environment>>, // Environment reference
) -> ApplicationWindow {
    let window = ApplicationWindow::new(&app);
    window.set_title(Some("Simulation View"));
    window.set_default_size(400, 400);

    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(400, 400);

    // Set up the drawing function
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let plants = plants.lock().unwrap();
        let env = environment.lock().unwrap();
        let scale = 20.0;
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;

        // Draw plants
        for plant in plants.iter() {
            let pos_x = center_x + plant.pos.x as f64 * scale;
            let pos_y = center_y - plant.pos.y as f64 * scale;

            let stem_x = pos_x + plant.stem_dir.x as f64 * scale;
            let stem_y = pos_y - plant.stem_dir.y as f64 * scale;
            let root_x = pos_x + plant.root_dir.x as f64 * scale;
            let root_y = pos_y - plant.root_dir.y as f64 * scale;

            cr.set_source_rgb(0.0, 1.0, 0.0); // Green for stem
            cr.move_to(pos_x, pos_y);
            cr.line_to(stem_x, stem_y);
            cr.stroke().unwrap();

            cr.set_source_rgb(0.65, 0.16, 0.16); // Brown for roots
            cr.move_to(pos_x, pos_y);
            cr.line_to(root_x, root_y);
            cr.stroke().unwrap();

            cr.set_source_rgb(1.0, 0.0, 0.0); // Red dot for plant base
            cr.arc(pos_x, pos_y, 3.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill().unwrap();
        }

        // Optionally draw environment elements (e.g., light, water)
        let light_x = center_x + env.light_pos.x as f64 * scale;
        let light_y = center_y - env.light_pos.y as f64 * scale;
        cr.set_source_rgb(1.0, 1.0, 0.0); // Yellow for light
        cr.arc(light_x, light_y, 5.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();

        let water_x = center_x + env.water_pos.x as f64 * scale;
        let water_y = center_y - env.water_pos.y as f64 * scale;
        cr.set_source_rgb(0.0, 0.0, 1.0); // Blue for water
        cr.arc(water_x, water_y, 5.0, 0.0, 2.0 * std::f64::consts::PI);
        cr.fill().unwrap();
    });

    // Connect to queue_draw when the window is shown to ensure continuous updates
    window.connect_show(move |_| {
        drawing_area.queue_draw();
    });

    window.set_child(Some(&drawing_area));
    window
}