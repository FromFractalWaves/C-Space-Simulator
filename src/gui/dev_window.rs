use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, Orientation, Align};
use vte4::Terminal as VteTerminal;
use vte4::TerminalExt; // Import for `feed`
use vte4::TerminalExtManual; // Import for `spawn_async`
use vte4::PtyFlags;
use gtk4::gio;
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

    let terminal = VteTerminal::new();
    terminal.set_vexpand(true);
    container.append(&terminal);

    // Spawn a process to stream logs into the terminal
    let logs_clone = logs.clone();
    terminal.spawn_async(
        PtyFlags::DEFAULT,
        None, // Working directory
        &["/bin/sh", "-c", "while true; do echo 'Log update'; sleep 1; done"], // Example command
        &[], // Environment variables
        glib::SpawnFlags::DEFAULT,
        || {},
        -1, // Timeout
        None::<&gio::Cancellable>, // Use gio::Cancellable
        move |result| {
            if let Ok(_pid) = result {
                // Optionally handle the process ID
            }
        },
    );

    // Feed logs into the terminal
    glib::source::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let logs = logs_clone.lock().unwrap();
        let text = logs.join("\n");
        terminal.feed(text.as_bytes(), text.len() as isize); // Now works with TerminalExt in scope
        glib::ControlFlow::Continue
    });

    window.set_child(Some(&container));
    window
}