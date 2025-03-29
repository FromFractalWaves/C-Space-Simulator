// src/simulation/simulation_env.rs
use nalgebra::{Vector3, Matrix3};
use crate::plants::tropisms::{Plant, Environment};

pub struct SimulationEnv {
    pub plants: Vec<Plant>,
    pub environment: Environment,
    pub time: f32, // Tracks simulation time
}

impl SimulationEnv {
    pub fn new() -> Self {
        let plants = vec![
            Plant {
                pos: Vector3::new(0.0, 0.0, 0.0),
                stem_dir: Vector3::new(0.0, 1.0, 0.0), // Upward
                root_dir: Vector3::new(0.0, -1.0, 0.0), // Downward
                energy: 10.0,
                coherence: 1.0,
                distortion: 0.0,
                temporal_complexity: 0.0,
                spatial_complexity: 0.0,
            },
        ];
        let environment = Environment {
            light_pos: Vector3::new(5.0, 5.0, 0.0),
            water_pos: Vector3::new(2.0, -2.0, 0.0),
            gravity: Vector3::new(0.0, -1.0, 0.0),
            obstacles: vec![Vector3::new(1.0, 0.0, 0.0)],
            light_intensity: 1.0,
            water_level: 1.0,
            metric_tensor: Matrix3::identity(),
            d_critical: 10.0,
        };
        Self {
            plants,
            environment,
            time: 0.0,
        }
    }

    pub fn update_time(&mut self, dt: f32) {
        self.time += dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_env_init() {
        let env = SimulationEnv::new();
        assert_eq!(env.plants.len(), 1);
        assert_eq!(env.environment.light_pos, Vector3::new(5.0, 5.0, 0.0));
        assert_eq!(env.time, 0.0);
    }
}