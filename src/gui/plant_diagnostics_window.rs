use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, ComboBoxText, DrawingArea, Label, Orientation};
use crate::plants::tropisms::Plant;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::RefCell;

pub fn build_plant_diagnostics_window(
    app: gtk4::Application,
    plants: Arc<Mutex<Vec<Plant>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(&app);
    window.set_title(Some("Plant Diagnostics"));
    window.set_default_size(300, 400);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);

    let combo = ComboBoxText::new();
    // ... combo setup ...

    let details = GtkBox::new(Orientation::Vertical, 5);
    let drawing_area = Rc::new(RefCell::new(DrawingArea::new()));
    drawing_area.borrow().set_size_request(200, 200);

    container.append(&details);
    container.append(&*drawing_area.borrow()); // Dereference Ref to get &DrawingArea

    let plants_clone = plants.clone();
    let drawing_area_clone = drawing_area.clone();
    combo.connect_changed(move |combo| {
        let idx = combo.active().unwrap_or(0) as usize;
        let plants = plants_clone.lock().unwrap();
        
        while let Some(child) = details.first_child() {
            details.remove(&child);
        }
        
        if idx > 0 && idx - 1 < plants.len() {
            if let Some(plant) = plants.get(idx - 1) {
                details.append(&Label::new(Some(&format!("Position: {:?}", plant.pos))));
                // ... other append calls ...
            }
        } else {
            details.append(&Label::new(Some("No plant selected.")));
        }
        drawing_area_clone.borrow().queue_draw();
    });

    let plants_clone = plants.clone();
    let drawing_area_clone = drawing_area.clone();
    drawing_area.borrow().set_draw_func(move |_area, cr, width, height| {
        let plants = plants_clone.lock().unwrap();
        let idx = combo.active().unwrap_or(0) as usize;
        if idx > 0 && idx - 1 < plants.len() {
            if let Some(plant) = plants.get(idx - 1) {
                let scale = 20.0;
                let center_x = width as f64 / 2.0;
                let center_y = height as f64 / 2.0;
                // ... drawing logic ...
            }
        }
    });

    window.set_child(Some(&container));
    window
}