use gtk::prelude::*;
use gtk::{ApplicationWindow, Box as GtkBox, Label, ScrolledWindow, TextView};

pub fn build_dev_window(app: &gtk::Application, logs: Arc<Mutex<Vec<String>>>) -> ApplicationWindow {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Development Logs"));
    window.set_default_size(300, 400);

    let container = GtkBox::new(gtk::Orientation::Vertical, 10);
    container.set_margin_all(10);

    let header = Label::new(Some("Tropism Logs"));
    header.set_halign(gtk::Align::Start);
    container.append(&header);

    let text_view = TextView::new();
    let scroll = ScrolledWindow::new();
    scroll.set_child(Some(&text_view));
    scroll.set_vexpand(true);
    container.append(&scroll);

    // Update logs dynamically
    let buffer = text_view.buffer();
    gtk::timeout_add(100, move || {
        let logs = logs.lock().unwrap();
        let text = logs.join("\n");
        buffer.set_text(&text);
        Continue(true)
    });

    window.set_child(Some(&container));
    window
}