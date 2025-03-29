// src/simulation/simulation_runner.rs
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Duration, Instant};
use crate::control::SimulationControl;
use crate::engines::cspace_engine::CSpaceEngine;
use crate::engines::plant_engine::PlantEngine;
use crate::plants::tropisms::TropismResult;

pub enum ControlCommand {
    Start,
    Stop,
    Status,
    Reset, // Added for future flexibility
}

pub struct SimulationRunner {
    control: Arc<SimulationControl>,
    command_receiver: Receiver<ControlCommand>,
    command_sender: Sender<ControlCommand>,
    log_sender: Sender<Vec<TropismResult>>,
    running: Arc<Mutex<bool>>,
    plant_engine: Arc<Mutex<PlantEngine>>,
    cspace_engine: Arc<Mutex<CSpaceEngine>>,
}

impl SimulationRunner {
    pub fn new(control: Arc<SimulationControl>) -> (Self, Sender<ControlCommand>, Receiver<Vec<TropismResult>>) {
        let (command_sender, command_receiver) = channel();
        let (log_sender, log_receiver) = channel();
        let running = Arc::new(Mutex::new(false));
        
        let plant_engine = control.engine().clone();
        let cspace_engine = Arc::new(Mutex::new(CSpaceEngine::new(
            plant_engine.lock().unwrap().env.plants.clone(),
            plant_engine.lock().unwrap().env.environment.clone(),
        )));

        let runner = SimulationRunner {
            control,
            command_receiver,
            command_sender: command_sender.clone(),
            log_sender,
            running,
            plant_engine,
            cspace_engine,
        };
        (runner, command_sender, log_receiver)
    }

    pub fn run(&self) {
        let target_dt = Duration::from_millis(100); // 10 FPS
        let mut last_update = Instant::now();
        println!("SimulationRunner event loop started.");

        loop {
            let now = Instant::now();
            let elapsed = now.duration_since(last_update);

            // Process commands
            while let Ok(command) = self.command_receiver.try_recv() {
                match command {
                    ControlCommand::Start => {
                        let mut running = self.running.lock().unwrap();
                        if !*running {
                            *running = true;
                            println!("Simulation started.");
                        }
                    }
                    ControlCommand::Stop => {
                        let mut running = self.running.lock().unwrap();
                        if *running {
                            *running = false;
                            println!("Simulation stopped.");
                        }
                    }
                    ControlCommand::Status => {
                        let running = *self.running.lock().unwrap();
                        let status = if running { "Running" } else { "Stopped" };
                        println!("Simulation status: {}", status);
                        self.log_sender.send(vec![TropismResult {
                            growth_delta: nalgebra::Vector3::zeros(),
                            rho_c: 0.0,
                            log: format!("Status: {}", status),
                        }]).unwrap();
                    }
                    ControlCommand::Reset => {
                        let mut running = self.running.lock().unwrap();
                        *running = false;
                        let mut plant_engine = self.plant_engine.lock().unwrap();
                        let mut cspace_engine = self.cspace_engine.lock().unwrap();
                        plant_engine.env = crate::simulation::simulation_env::SimulationEnv::new();
                        cspace_engine.plants = plant_engine.env.plants.clone();
                        cspace_engine.environment = plant_engine.env.environment.clone();
                        println!("Simulation reset.");
                    }
                }
            }

            // Update simulation if running
            if *self.running.lock().unwrap() {
                let dt = 0.1; // Fixed timestep
                let mut plant_engine = self.plant_engine.lock().unwrap();
                let mut cspace_engine = self.cspace_engine.lock().unwrap();

                // Step 1: Update PlantEngine (tropisms)
                let results = plant_engine.update(dt);

                // Step 2: Update CSpaceEngine (manifold properties)
                cspace_engine.plants = plant_engine.env.plants.clone();
                cspace_engine.environment = plant_engine.env.environment.clone();
                cspace_engine.update(dt);

                // Send logs
                if let Err(e) = self.log_sender.send(results.clone()) {
                    eprintln!("Failed to send simulation results: {}", e);
                    break;
                }

                // Update shared state in SimulationControl
                {
                    let mut plants = self.control.plants().lock().unwrap();
                    *plants = plant_engine.env.plants.clone();
                }
                {
                    let mut env = self.control.environment().lock().unwrap();
                    *env = plant_engine.env.environment.clone();
                }
            }

            // Timing control
            let frame_time = now.elapsed();
            if frame_time < target_dt {
                std::thread::sleep(target_dt - frame_time);
            }
            last_update = now;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::simulation_env::SimulationEnv;

    #[test]
    fn test_simulation_runner() {
        let env = SimulationEnv::new();
        let control = Arc::new(SimulationControl::new(env));
        let (runner, command_sender, log_receiver) = SimulationRunner::new(control.clone());

        let runner_handle = std::thread::spawn(move || {
            runner.run();
        });

        // Test start
        command_sender.send(ControlCommand::Start).unwrap();
        std::thread::sleep(Duration::from_millis(300));
        let results = log_receiver.recv_timeout(Duration::from_millis(500)).unwrap();
        assert_eq!(results.len(), 1); // One plant
        assert_eq!(results[0].len(), 4); // Four tropisms

        // Test reset
        command_sender.send(ControlCommand::Reset).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        let plants = control.plants().lock().unwrap();
        assert_eq!(plants[0].pos, nalgebra::Vector3::new(0.0, 0.0, 0.0)); // Reset to initial position

        // Test stop
        command_sender.send(ControlCommand::Stop).unwrap();
        std::thread::sleep(Duration::from_millis(100));

        // Test status
        command_sender.send(ControlCommand::Status).unwrap();
        let status = log_receiver.recv_timeout(Duration::from_millis(500)).unwrap();
        assert!(status[0].log.contains("Stopped"));

        // Note: Runner runs indefinitely; in production, add a clean shutdown mechanism
    }
}