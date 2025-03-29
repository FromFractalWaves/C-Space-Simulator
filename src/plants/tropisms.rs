// src/engines/tropisms.rs
use nalgebra::{Vector3, Matrix3}; // For vector and matrix operations
use rand::Rng; // For variability in environmental responses


/// Represents a plant as a computational entity in the C-Space manifold
#[derive(Debug)]
pub struct Plant {
    pub pos: Vector3<f32>,      // Position in 3D space (mapped to manifold coordinates)
    pub stem_dir: Vector3<f32>, // Stem direction (coherence projection)
    pub root_dir: Vector3<f32>, // Root direction (time projection)
    pub energy: f32,            // Computational energy E(p)
    pub coherence: f32,         // H: Structural organization
    pub distortion: f32,        // D: Instability or environmental chaos
    pub temporal_complexity: f32, // T: Emergent time from growth processes
    pub spatial_complexity: f32,  // S: Structural complexity in space
}

/// Represents the environment as a computational manifold
#[derive(Debug)]
pub struct Environment {
    pub light_pos: Vector3<f32>,  // Light source position
    pub water_pos: Vector3<f32>,  // Water source position
    pub gravity: Vector3<f32>,    // Gravity direction (e.g., [0, -1, 0])
    pub obstacles: Vec<Vector3<f32>>, // Obstacle positions
    pub light_intensity: f32,     // Energy contribution from light
    pub water_level: f32,         // Energy contribution from water
    pub metric_tensor: Matrix3<f32>, // g: Defines manifold geometry
    pub d_critical: f32,          // Critical distortion threshold
}

/// Tropism result for logging and rendering
#[derive(Debug)]
pub struct TropismResult {
    pub growth_delta: Vector3<f32>, // Change in position or direction
    pub rho_c: f32,                 // Complex density after tropism
    pub log: String,                // Description for dev_window
}

pub struct Tropisms;

impl Tropisms {
    /// Computes the metric tensor based on energy and distortion
    fn compute_metric_tensor(energy: f32, distortion: f32) -> Matrix3<f32> {
        let epsilon = 1e-6;
        Matrix3::new(
            1.0 / (energy * energy), 0.0, 0.0,
            0.0, 1.0 / energy, 0.0,
            0.0, 0.0, 1.0 / (distortion + epsilon),
        )
    }

    /// Computes complex density: ρ_c = sqrt(S^2 + T^2) * E
    fn compute_complex_density(spatial: f32, temporal: f32, energy: f32) -> f32 {
        (spatial * spatial + temporal * temporal).sqrt() * energy
    }

    /// Updates coherence and distortion dynamics
    fn update_dynamics(plant: &mut Plant, env: &Environment, dt: f32) {
        let alpha = 0.05; // Scaling coefficient from coherence dynamics
        let beta = 0.1;   // Scaling coefficient from distortion dynamics
        let epsilon = 1e-6;

        // Simplified spatial complexity gradient (distance to nearest resource)
        let light_dist = (env.light_pos - plant.pos).norm();
        let water_dist = (env.water_pos - plant.pos).norm();
        let grad_s = (light_dist + water_dist) / 2.0;

        // Coherence evolution: dH/dt = -α (D/(H+ε) + ∇S)
        let d_h_dt = -alpha * (plant.distortion / (plant.coherence + epsilon) + grad_s);
        let delta_h = d_h_dt * dt;

        // Distortion evolution: dD/dt = β * log(1 + |ΔH| * E)
        let d_d_dt = beta * (1.0 + (delta_h.abs() * plant.energy)).ln();
        plant.distortion += d_d_dt * dt;
        plant.coherence += delta_h;

        // Emergent time evolution (Perpendicularity Mechanics): dT/dt = β * tanh(|ΔH| * E) * sign(H)
        let d_t_dt = beta * (delta_h.abs() * plant.energy).tanh() * plant.coherence.signum();
        plant.temporal_complexity += d_t_dt * dt;

        // Update spatial complexity based on growth extent
        plant.spatial_complexity = (plant.stem_dir.norm() + plant.root_dir.norm()) / 2.0;

        // Singularity check
        if plant.distortion > env.d_critical {
            plant.coherence = 0.0; // Collapse to Pure Time State
            plant.temporal_complexity = plant.energy * plant.temporal_complexity.signum();
            plant.spatial_complexity = 0.0;
        }
    }

    /// Phototropism: Growth toward light, optimizing geodesic path
    pub fn phototropism(plant: &mut Plant, env: &Environment, dt: f32) -> TropismResult {
        let light_dir = (env.light_pos - plant.pos).normalize();
        let intensity_factor = env.light_intensity * 0.1;
        let growth = light_dir * intensity_factor * dt;

        plant.stem_dir = (plant.stem_dir + growth).normalize();
        plant.pos += growth;
        plant.energy += intensity_factor * 0.5 * dt;

        Self::update_dynamics(plant, env, dt);
        let rho_c = Self::compute_complex_density(plant.spatial_complexity, plant.temporal_complexity, plant.energy);

        TropismResult {
            growth_delta: growth,
            rho_c,
            log: format!(
                "Phototropism: Grew {:.2} toward light, ρ_c={:.2}, H={:.2}, D={:.2}, T={:.2}",
                growth.norm(), rho_c, plant.coherence, plant.distortion, plant.temporal_complexity
            ),
        }
    }

    /// Gravitropism: Stems up, roots down, aligning with manifold curvature
    pub fn gravitropism(plant: &mut Plant, env: &Environment, dt: f32) -> TropismResult {
        let gravity_dir = env.gravity.normalize();
        let stem_growth = -gravity_dir * 0.05 * dt;
        let root_growth = gravity_dir * 0.03 * dt;

        plant.stem_dir = (plant.stem_dir + stem_growth).normalize();
        plant.root_dir = (plant.root_dir + root_growth).normalize();
        plant.pos += stem_growth;
        plant.energy -= 0.02 * dt;

        Self::update_dynamics(plant, env, dt);
        let rho_c = Self::compute_complex_density(plant.spatial_complexity, plant.temporal_complexity, plant.energy);

        TropismResult {
            growth_delta: stem_growth,
            rho_c,
            log: format!(
                "Gravitropism: Stem up {:.2}, roots down {:.2}, ρ_c={:.2}, H={:.2}, D={:.2}, T={:.2}",
                stem_growth.norm(), root_growth.norm(), rho_c, plant.coherence, plant.distortion, plant.temporal_complexity
            ),
        }
    }

    /// Hydrotropism: Roots toward water, navigating energy gradients
    pub fn hydrotropism(plant: &mut Plant, env: &Environment, dt: f32) -> TropismResult {
        let water_dir = (env.water_pos - plant.pos).normalize();
        let water_factor = env.water_level * 0.08 * dt;
        let growth = water_dir * water_factor;

        plant.root_dir = (plant.root_dir + growth).normalize();
        plant.pos += growth * 0.5;
        plant.energy += water_factor * 0.3;

        Self::update_dynamics(plant, env, dt);
        let rho_c = Self::compute_complex_density(plant.spatial_complexity, plant.temporal_complexity, plant.energy);

        TropismResult {
            growth_delta: growth,
            rho_c,
            log: format!(
                "Hydrotropism: Roots grew {:.2} toward water, ρ_c={:.2}, H={:.2}, D={:.2}, T={:.2}",
                growth.norm(), rho_c, plant.coherence, plant.distortion, plant.temporal_complexity
            ),
        }
    }

    /// Thigmotropism: Wrapping around obstacles, adapting manifold topology
    pub fn thigmotropism(plant: &mut Plant, env: &Environment, dt: f32) -> TropismResult {
        let mut closest_obstacle = None;
        let mut min_dist = f32::MAX;

        for obstacle in &env.obstacles {
            let dist = (obstacle - plant.pos).norm();
            if dist < min_dist && dist < 1.0 {
                min_dist = dist;
                closest_obstacle = Some(obstacle);
            }
        }

        let result = if let Some(obstacle) = closest_obstacle {
            let contact_dir = (obstacle - plant.pos).normalize();
            let tangent = Vector3::new(-contact_dir.y, contact_dir.x, 0.0).normalize();
            let growth = tangent * 0.04 * dt;

            plant.stem_dir = (plant.stem_dir + growth).normalize();
            plant.pos += growth;
            plant.energy -= 0.03 * dt;

            Self::update_dynamics(plant, env, dt);
            let rho_c = Self::compute_complex_density(plant.spatial_complexity, plant.temporal_complexity, plant.energy);

            TropismResult {
                growth_delta: growth,
                rho_c,
                log: format!(
                    "Thigmotropism: Wrapped {:.2} around obstacle, ρ_c={:.2}, H={:.2}, D={:.2}, T={:.2}",
                    growth.norm(), rho_c, plant.coherence, plant.distortion, plant.temporal_complexity
                ),
            }
        } else {
            TropismResult {
                growth_delta: Vector3::zeros(),
                rho_c: Self::compute_complex_density(plant.spatial_complexity, plant.temporal_complexity, plant.energy),
                log: "Thigmotropism: No obstacles in range".to_string(),
            }
        };

        result
    }

    /// Apply all tropisms, simulating navigation through the manifold
    pub fn apply_all(plant: &mut Plant, env: &mut Environment, dt: f32) -> Vec<TropismResult> {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();

        results.push(Self::phototropism(plant, env, dt));
        results.push(Self::gravitropism(plant, env, dt));
        results.push(Self::hydrotropism(plant, env, dt));
        results.push(Self::thigmotropism(plant, env, dt));

        // Update metric tensor based on current state
        env.metric_tensor = Self::compute_metric_tensor(plant.energy, plant.distortion);

        // Add noise to simulate manifold fluctuations
        let noise = Vector3::new(
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-0.01..0.01),
        );
        plant.pos += noise * dt;

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Plant, Environment) {
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
            obstacles: vec![Vector3::new(1.0, 0.0, 0.0)],
            light_intensity: 1.0,
            water_level: 1.0,
            metric_tensor: Matrix3::identity(),
            d_critical: 10.0,
        };
        (plant, env)
    }

    #[test]
    fn test_phototropism() {
        let (mut plant, env) = setup();
        let result = Tropisms::phototropism(&mut plant, &env, 1.0);
        assert!(result.growth_delta.norm() > 0.0);
        assert!(plant.energy > 10.0);
        assert!(result.rho_c > 0.0);
    }

    #[test]
    fn test_singularity() {
        let (mut plant, mut env) = setup();
        plant.distortion = 15.0; // Exceed d_critical
        let results = Tropisms::apply_all(&mut plant, &mut env, 1.0);
        assert_eq!(plant.coherence, 0.0); // Pure Time State
        assert_eq!(plant.spatial_complexity, 0.0);
    }
}