use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, Label, Orientation, Align};
use vte4::Terminal as VteTerminal;
use vte4::TerminalExt;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::plants::tropisms::TropismResult;
use crate::simulation::simulation_runner::ControlCommand;

pub fn build_dev_window(
    app: gtk4::Application,
    logs: Arc<Mutex<Vec<String>>>,
    log_receiver: Receiver<Vec<Vec<TropismResult>>>,
    command_sender: Sender<ControlCommand>,
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

    // Initial prompt
    terminal.feed(b"CLI Commands: start, stop, status, reset\n> ");

    // Log buffer to limit terminal content
    let log_buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    const MAX_LINES: usize = 100;

    // Handle logs in a separate thread to avoid GTK main thread dependency
    let terminal_clone = terminal.clone();
    let log_buffer_clone = log_buffer.clone();
    std::thread::spawn(move || {
        while let Ok(results_vec) = log_receiver.recv() {
            let mut buffer = log_buffer_clone.lock().unwrap();
            for plant_results in &results_vec {
                for result in plant_results {
                    buffer.push(result.log.clone());
                    if buffer.len() > MAX_LINES {
                        buffer.remove(0);
                    }
                }
            }
            let text = buffer.join("\n");
            glib::idle_add_once(move || {
                terminal_clone.reset(true, false);
                terminal_clone.feed(format!("{}\n> ", text).as_bytes());
            });
        }
    });

    // Handle user input
    let command_sender_clone = command_sender.clone();
    terminal.connect_char_event(move |terminal, char| {
        if char == '\r' { // Enter key
            let contents = terminal.get_text(None, None, |_| true).unwrap_or_default().to_string();
            let command = contents
                .lines()
                .last()
                .unwrap_or("")
                .trim_start_matches('>')
                .trim();
            match command {
                "start" => {
                    if command_sender_clone.send(ControlCommand::Start).is_ok() {
                        terminal.feed(b"\nSimulation starting...\n> ");
                    }
                }
                "stop" => {
                    if command_sender_clone.send(ControlCommand::Stop).is_ok() {
                        terminal.feed(b"\nSimulation stopping...\n> ");
                    }
                }
                "status" => {
                    if command_sender_clone.send(ControlCommand::Status).is_ok() {
                        terminal.feed(b"\nChecking status...\n> ");
                    }
                }
                "reset" => {
                    if command_sender_clone.send(ControlCommand::Reset).is_ok() {
                        terminal.feed(b"\nSimulation resetting...\n> ");
                        terminal.reset(true, true);
                    }
                }
                cmd => {
                    if !cmd.is_empty() {
                        terminal.feed(format!("\nUnknown command: {}\n> ", cmd).as_bytes());
                    }
                }
            }
        }
        glib::Propagation::Proceed
    });

    window.set_child(Some(&container));
    window
}