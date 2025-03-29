use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, ScrolledWindow, TextView, Orientation, Align};
use glib::ControlFlow;
use glib::timeout_add;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn build_dev_window(
    app: &gtk4::Application, // Fixed syntax from >k4::Application
    logs: Arc<Mutex<Vec<String>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
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

    timeout_add(Duration::from_millis(100), glib::clone!(@weak text_view => move || {
        let logs = logs.lock().unwrap();
        let text = logs.join("\n");
        text_view.buffer().set_text(&text);
        ControlFlow::Continue // Ensure no semicolon here
    }));

    window.set_child(Some(&container));
    window
}