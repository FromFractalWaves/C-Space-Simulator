// src/bin/cli.rs
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    println!("CLI Session Started. Available commands: start, stop, status, reset, exit");
    print!("> ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        if let Some(Ok(line)) = lines.next() {
            let command = line.trim().to_lowercase();
            match command.as_str() {
                "start" => println!("Simulation starting..."),
                "stop" => println!("Simulation stopping..."),
                "status" => println!("Simulation status: Running"), // Placeholder
                "reset" => println!("Simulation resetting..."),
                "exit" => {
                    println!("Exiting CLI session...");
                    process::exit(0);
                }
                "" => {}
                _ => println!("Unknown command: {}", command),
            }
            print!("> ");
            io::stdout().flush().unwrap();
        } else {
            break;
        }
    }
}