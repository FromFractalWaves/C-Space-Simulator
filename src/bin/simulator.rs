// src/bin/simulator.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: simulator <command>");
        println!("Commands: start, stop, status, reset");
        println!("Run 'simulator --help' for more information.");
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            println!("Simulator Command Help:");
            println!("Usage: simulator <command>");
            println!("\nCommands:");
            println!("  start  - Start the simulation");
            println!("  stop   - Stop the simulation");
            println!("  status - Check simulation status");
            println!("  reset  - Reset the simulation to initial state");
        }
        "start" => println!("Starting simulation... (placeholder)"),
        "stop" => println!("Stopping simulation... (placeholder)"),
        "status" => println!("Simulation status: Running (placeholder)"),
        "reset" => println!("Resetting simulation... (placeholder)"),
        cmd => println!("Unknown simulator command: '{}'. Use 'simulator --help' for usage.", cmd),
    }
}