// src/gui/dev_window.rs
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
    log_receiver: Receiver<Vec<TropismResult>>,
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
    const MAX_LINES: usize = 100; // Limit to 100 lines

    // Handle logs
    let command_sender_clone = command_sender.clone();
    let terminal_clone = terminal.clone();
    let log_buffer_clone = log_buffer.clone();
    gtk4::glib::source::timeout_add_local(std::time::Duration::from_millis(100), move || {
        if let Ok(results_vec) = log_receiver.try_recv() {
            let mut buffer = log_buffer_clone.lock().unwrap();
            for plant_results in &results_vec {
                for result in plant_results {
                    buffer.push(result.log.clone());
                    if buffer.len() > MAX_LINES {
                        buffer.remove(0); // Remove oldest log
                    }
                }
            }
            // Refresh terminal with current buffer
            terminal_clone.reset(true, false); // Clear without resetting state
            let text = buffer.join("\n");
            terminal_clone.feed(text.as_bytes());
            terminal_clone.feed(b"\n> ");
        }
        gtk4::glib::ControlFlow::Continue
    });

    // Handle user input
    terminal.connect_char_event({
        let command_sender = command_sender_clone.clone();
        let terminal_clone = terminal.clone();
        let mut input_buffer = String::new();
        move |terminal, ch, _mods| {
            if ch == '\r' || ch == '\n' { // Enter key
                match input_buffer.trim() {
                    "start" => {
                        if command_sender.send(ControlCommand::Start).is_ok() {
                            terminal.feed(b"\nSimulation starting...\n> ");
                        }
                    }
                    "stop" => {
                        if command_sender.send(ControlCommand::Stop).is_ok() {
                            terminal.feed(b"\nSimulation stopping...\n> ");
                        }
                    }
                    "status" => {
                        if command_sender.send(ControlCommand::Status).is_ok() {
                            terminal.feed(b"\nChecking status...\n> ");
                        }
                    }
                    "reset" => {
                        if command_sender.send(ControlCommand::Reset).is_ok() {
                            terminal.feed(b"\nSimulation resetting...\n> ");
                            terminal_clone.reset(true, true); // Full reset
                        }
                    }
                    cmd => {
                        terminal.feed(format!("\nUnknown command: {}\n> ", cmd).as_bytes());
                    }
                }
                input_buffer.clear();
            } else if ch.is_ascii() && !ch.is_control() {
                input_buffer.push(ch);
                terminal.feed(&[ch as u8]); // Echo input
            } else if ch == 8 || ch == 127 { // Backspace
                if input_buffer.pop().is_some() {
                    terminal.feed(b"\x08 \x08"); // Erase last char
                }
            }
            gtk4::Inhibit(false)
        }
    });

    window.set_child(Some(&container));
    window
}