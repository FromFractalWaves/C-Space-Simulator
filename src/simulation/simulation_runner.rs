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
    Reset,
}

pub struct SimulationRunner {
    control: Arc<SimulationControl>,
    command_receiver: Receiver<ControlCommand>,
    command_sender: Sender<ControlCommand>,
    log_sender: Sender<Vec<Vec<TropismResult>>>,
    running: Arc<Mutex<bool>>,
    plant_engine: Arc<Mutex<PlantEngine>>,
    cspace_engine: Arc<Mutex<CSpaceEngine>>,
}

impl SimulationRunner {
    pub fn new(control: Arc<SimulationControl>) -> (Self, Sender<ControlCommand>, Receiver<Vec<Vec<TropismResult>>>) {
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
            let _elapsed = now.duration_since(last_update); // Unused for now

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
                        self.log_sender.send(vec![vec![TropismResult {
                            growth_delta: nalgebra::Vector3::zeros(),
                            rho_c: 0.0,
                            log: format!("Status: {}", status),
                        }]]).unwrap();
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
                    let plants_guard = self.control.plants();
                    let mut plants = plants_guard.lock().unwrap();
                    *plants = plant_engine.env.plants.clone();
                }
                {
                    let env_guard = self.control.environment();
                    let mut env = env_guard.lock().unwrap();
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