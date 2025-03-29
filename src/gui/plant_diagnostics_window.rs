use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, ComboBoxText, DrawingArea, Label};
use crate::plants::tropisms::Plant;
use std::sync::{Arc, Mutex};

pub fn build_plant_diagnostics_window(
    app: &gtk::Application,
    plants: Arc<Mutex<Vec<Plant>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Plant Diagnostics"));
    window.set_default_size(300, 400);

    let container = GtkBox::new(gtk::Orientation::Vertical, 10);
    container.set_margin_all(10);

    let combo = ComboBoxText::new();
    combo.append_text("None");
    {
        let plants = plants.lock().unwrap();
        for i in 0..plants.len() {
            combo.append_text(&format!("Plant {}", i));
        }
    }
    combo.set_active(Some(0));
    container.append(&Label::new(Some("Select Plant:")));
    container.append(&combo);

    let details = GtkBox::new(gtk::Orientation::Vertical, 5);
    let drawing_area = DrawingArea::new();
    drawing_area.set_size_request(200, 200);

    container.append(&details);
    container.append(&drawing_area);

    let plants_clone = plants.clone();
    combo.connect_changed(move |combo| {
        let idx = combo.active().unwrap_or(0) as usize;
        let plants = plants_clone.lock().unwrap();
        details.foreach(|child| details.remove(child));

        if idx > 0 && idx - 1 < plants.len() {
            if let Some(plant) = plants.get(idx - 1) {
                details.append(&Label::new(Some(&format!("Position: {:?}", plant.pos))));
                details.append(&Label::new(Some(&format!("Stem Direction: {:?}", plant.stem_dir))));
                details.append(&Label::new(Some(&format!("Root Direction: {:?}", plant.root_dir))));
                details.append(&Label::new(Some(&format!("Energy: {:.2}", plant.energy))));
                details.append(&Label::new(Some(&format!("Coherence: {:.2}", plant.coherence))));
                details.append(&Label::new(Some(&format!("Distortion: {:.2}", plant.distortion))));
                details.append(&Label::new(Some(&format!(
                    "Temporal Complexity: {:.2}",
                    plant.temporal_complexity
                ))));
                details.append(&Label::new(Some(&format!(
                    "Spatial Complexity: {:.2}",
                    plant.spatial_complexity
                ))));
            }
        } else {
            details.append(&Label::new(Some("No plant selected.")));
        }
        drawing_area.queue_draw();
    });

    let plants_clone = plants.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let plants = plants_clone.lock().unwrap();
        let idx = combo.active().unwrap_or(0) as usize;
        if idx > 0 && idx - 1 < plants.len() {
            if let Some(plant) = plants.get(idx - 1) {
                let scale = 20.0;
                let center_x = width as f64 / 2.0;
                let center_y = height as f64 / 2.0;

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

                cr.set_source_rgb(0.65, 0.16, 0.16); // Brown for root
                cr.move_to(pos_x, pos_y);
                cr.line_to(root_x, root_y);
                cr.stroke().unwrap();

                cr.set_source_rgb(1.0, 0.0, 0.0); // Red for position
                cr.arc(pos_x, pos_y, 3.0, 0.0, 2.0 * std::f64::consts::PI);
                cr.fill().unwrap();
            }
        }
    });

    window.set_child(Some(&container));
    window
}