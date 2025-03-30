// src/engines/cspace_engine.rs
use nalgebra::Matrix3;
use crate::plants::tropisms::{Plant, Environment};

pub struct CSpaceEngine {
    pub plants: Vec<Plant>,
    pub environment: Environment,
}

impl CSpaceEngine {
    pub fn new(plants: Vec<Plant>, environment: Environment) -> Self {
        Self { plants, environment }
    }

    pub fn update(&mut self, dt: f32) {
        for plant in &mut self.plants {
            // Pass plant data directly instead of borrowing self
            let rho_c = Self::compute_complex_density(plant);
            let metric = Self::compute_metric_tensor(plant.energy, plant.distortion);
            self.environment.metric_tensor = metric; // Update environment's metric tensor
            println!(
                "CSpace Update: Plant at {:?}, ρ_c={:.2}, metric={:?}",
                plant.pos, rho_c, metric
            );
        }
    }

    fn compute_complex_density(plant: &Plant) -> f32 {
        (plant.spatial_complexity.powi(2) + plant.temporal_complexity.powi(2)).sqrt() * plant.energy
    }

    fn compute_metric_tensor(energy: f32, distortion: f32) -> Matrix3<f32> {
        let epsilon = 1e-6;
        Matrix3::new(
            1.0 / (energy * energy), 0.0, 0.0,
            0.0, 1.0 / energy, 0.0,
            0.0, 0.0, 1.0 / (distortion + epsilon),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_cspace_engine_update() {
        let plant = Plant {
            pos: Vector3::new(0.0, 0.0, 0.0),
            stem_dir: Vector3::new(0.0, 1.0, 0.0),
            root_dir: Vector3::new(0.0, -1.0, 0.0),
            energy: 10.0,
            coherence: 1.0,
            distortion: 0.0,
            temporal_complexity: 0.0,
            spatial_complexity: 0.0,
        };
        let env = Environment {
            light_pos: Vector3::new(5.0, 5.0, 0.0),
            water_pos: Vector3::new(2.0, -2.0, 0.0),
            gravity: Vector3::new(0.0, -1.0, 0.0),
            obstacles: vec![],
            light_intensity: 1.0,
            water_level: 1.0,
            metric_tensor: Matrix3::identity(),
            d_critical: 10.0,
        };
        let mut engine = CSpaceEngine::new(vec![plant], env);
        engine.update(1.0);
        // Check console output manually for now; add assertions later as logic expands
    }
}