use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, ScrolledWindow, TextView, Orientation, Align};
use glib::source::idle_add;
use std::sync::{Arc, Mutex};

pub fn build_dev_window(
    app: gtk4::Application,
    logs: Arc<Mutex<Vec<String>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(&app);
    window.set_title(Some("Development Logs"));
    window.set_default_size(300, 400);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);

    let header = Label::new(Some("Tropism Logs"));
    header.set_halign(Align::Start);
    container.append(&header);

    let text_view = TextView::new();
    let scroll = ScrolledWindow::new();
    scroll.set_child(Some(&text_view));
    scroll.set_vexpand(true);
    container.append(&scroll);

    // Use glib::clone! to safely capture a weak reference to text_view
    let text_view_weak = text_view.downgrade(); // Create a weak reference to avoid ownership issues
    glib::source::timeout_add_local(std::time::Duration::from_millis(100),move || {
        // Upgrade the weak reference to a strong one, if the widget still exists
        if let Some(text_view) = text_view_weak.upgrade() {
            let logs = logs.lock().unwrap();
            let text = logs.join("\n");
            text_view.buffer().set_text(&text);
        }
        glib::ControlFlow::Continue
    });

    window.set_child(Some(&container));
    window
}