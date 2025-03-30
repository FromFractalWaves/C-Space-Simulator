// src/gui/dev_window.rs
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, Orientation, Align};
use gtk4::gio; // For gio::Cancellable
use vte4::Terminal as VteTerminal;
use vte4::{TerminalExt, TerminalExtManual}; // For spawn_async
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::plants::tropisms::TropismResult;

pub fn build_dev_window(
    app: gtk4::Application,
    logs: Arc<Mutex<Vec<String>>>,
    log_receiver: Arc<Mutex<Receiver<Vec<Vec<TropismResult>>>>>,
) -> ApplicationWindow {
    let window = ApplicationWindow::new(&app);
    window.set_title(Some("Development Logs & CLI"));
    window.set_default_size(400, 500);

    let container = GtkBox::new(Orientation::Vertical, 10);
    container.set_margin_start(10);
    container.set_margin_end(10);
    container.set_margin_top(10);
    container.set_margin_bottom(10);

    let header = Label::new(Some("Tropism Logs & CLI Control"));
    header.set_halign(Align::Start);
    container.append(&header);

    let terminal = VteTerminal::new();
    terminal.set_vexpand(true);
    container.append(&terminal);

    // Spawn the CLI session
    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None,
        &["cargo", "run", "--bin", "cli"],
        &[],
        glib::SpawnFlags::DEFAULT,
        || {},
        -1,
        None::<&gio::Cancellable>,
        |result| {
            if let Err(e) = result {
                eprintln!("Failed to spawn CLI session: {}", e);
            } else {
                println!("CLI session spawned successfully.");
            }
        },
    );

    // Monitor CLI subprocess exit
    terminal.connect_child_exited(|_terminal, status| {
        eprintln!("CLI subprocess exited with status: {}", status);
    });

    // Log buffer
    let log_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    const MAX_LINES: usize = 100;

    // Handle logs
    let terminal_clone = terminal.clone();
    let log_buffer_clone = log_buffer.clone();
    let log_receiver_clone = log_receiver.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let receiver = log_receiver_clone.lock().unwrap();
        let mut new_logs = Vec::new();
        while let Ok(results_vec) = receiver.try_recv() {
            for plant_results in &results_vec {
                for result in plant_results {
                    new_logs.push(result.log.clone());
                }
            }
        }
        if !new_logs.is_empty() {
            let mut buffer = log_buffer_clone.lock().unwrap();
            buffer.extend(new_logs);
            while buffer.len() > MAX_LINES {
                buffer.remove(0);
            }
            let text = buffer.join("\n");
            terminal_clone.feed(format!("\n{}", text).as_bytes());
        }
        glib::ControlFlow::Continue
    });

    window.set_child(Some(&container));
    window
}