// src/engines/plant_engine.rs
use crate::simulation::simulation_env::SimulationEnv;
use crate::plants::tropisms::{Tropisms, TropismResult};

pub struct PlantEngine {
    pub env: SimulationEnv,
}

impl PlantEngine {
    pub fn new(env: SimulationEnv) -> Self {
        Self { env }
    }

    pub fn update(&mut self, dt: f32) -> Vec<Vec<TropismResult>> {
        let mut results = Vec::new();
        for plant in &mut self.env.plants {
            let plant_results = Tropisms::apply_all(plant, &mut self.env.environment, dt);
            results.push(plant_results);
        }
        self.env.update_time(dt);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plant_engine_update() {
        let env = SimulationEnv::new();
        let mut engine = PlantEngine::new(env);
        let initial_pos = engine.env.plants[0].pos;

        let results = engine.update(1.0);
        assert_eq!(results.len(), 1); // One plant
        assert_eq!(results[0].len(), 4); // Four tropisms applied
        assert_ne!(engine.env.plants[0].pos, initial_pos); // Position changed
        assert_eq!(engine.env.time, 1.0); // Time incremented
    }
}