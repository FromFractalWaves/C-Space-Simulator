use crate::engines::plant_engine::PlantEngine;
use crate::simulation::simulation_env::SimulationEnv;
use crate::plants::{Environment, Plant};
use glib::source::{idle_add, SourceId};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SimulationControl {
    engine: Arc<Mutex<PlantEngine>>,
    plants: Arc<Mutex<Vec<Plant>>>,
    environment: Arc<Mutex<Environment>>,
    logs: Arc<Mutex<Vec<String>>>,
    running: Arc<Mutex<bool>>,
    source_id: Arc<Mutex<Option<SourceId>>>,
}

impl SimulationControl {
    pub fn new(env: SimulationEnv) -> Self {
        let engine = Arc::new(Mutex::new(PlantEngine::new(env)));
        let plants = Arc::new(Mutex::new(engine.lock().unwrap().env.plants.clone()));
        let environment = Arc::new(Mutex::new(engine.lock().unwrap().env.environment.clone()));
        let logs = Arc::new(Mutex::new(Vec::new()));
        let running = Arc::new(Mutex::new(false));
        let source_id = Arc::new(Mutex::new(None));

        SimulationControl {
            engine,
            plants,
            environment,
            logs,
            running,
            source_id,
        }
    }

    // Rest of the code (start, stop, getters, etc.) remains unchanged
    pub fn start(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            return; // Already running
        }
        *running = true;

        let engine = self.engine.clone();
        let plants = self.plants.clone();
        let environment = self.environment.clone();
        let logs = self.logs.clone();
        let running_clone = self.running.clone();
        let source_id = self.source_id.clone();

        let id = idle_add(move || {
            let dt = 0.1;
            let mut engine = engine.lock().unwrap();
            let results = engine.update(dt);

            {
                let mut logs = logs.lock().unwrap();
                for plant_results in results {
                    for result in plant_results {
                        logs.push(result.log);
                        if logs.len() > 100 {
                            logs.remove(0);
                        }
                    }
                }
            }

            {
                let mut plants = plants.lock().unwrap();
                *plants = engine.env.plants.clone();
            }
            {
                let mut env = environment.lock().unwrap();
                *env = engine.env.environment.clone();
            }

            if *running_clone.lock().unwrap() {
                glib::ControlFlow::Continue
            } else {
                glib::ControlFlow::Break
            }
        });

        *self.source_id.lock().unwrap() = Some(id);
    }

    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        if !*running {
            return; // Already stopped
        }
        *running = false;

        if let Some(source_id) = self.source_id.lock().unwrap().take() {
            source_id.remove(); // Remove the idle_add source
        }
    }

    pub fn plants(&self) -> Arc<Mutex<Vec<Plant>>> {
        self.plants.clone()
    }

    pub fn environment(&self) -> Arc<Mutex<Environment>> {
        self.environment.clone()
    }

    pub fn logs(&self) -> Arc<Mutex<Vec<String>>> {
        self.logs.clone()
    }

    pub fn engine(&self) -> Arc<Mutex<PlantEngine>> {
        self.engine.clone()
    }
}

pub mod prelude {
    pub use super::SimulationControl;
}