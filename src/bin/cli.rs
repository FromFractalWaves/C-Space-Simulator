// src/bin/cli.rs
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Welcome to the CS Simulator CLI!");
    println!("Type 'help' for a list of commands, or 'exit' to quit.");
    print_prompt();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            print_prompt();
            continue;
        }

        match input {
            "exit" => {
                println!("Goodbye!");
                break;
            }
            "help" => run_help(),
            "simulator" => run_simulator(),
            "start" => println!("Starting simulation... (integration TBD)"),
            "stop" => println!("Stopping simulation... (integration TBD)"),
            "status" => println!("Simulation status: TBD (integration pending)"),
            "reset" => println!("Resetting simulation... (integration TBD)"),
            _ => println!("Unknown command: '{}'. Type 'help' for available commands.", input),
        }
        print_prompt();
    }
}

fn print_prompt() {
    print!("cs_sim> ");
    io::stdout().flush().unwrap();
}

fn run_help() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "help"])
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Error running help: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("Failed to execute help command: {}", e),
    }
}

fn run_simulator() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "simulator"])
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Error running simulator: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("Failed to execute simulator command: {}", e),
    }
}