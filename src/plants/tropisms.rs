// src/engines/tropisms.rs
use nalgebra::{Vector3, Norm}; // For 3D vector operations
use rand::Rng; // For environmental noise or variability
use std::f32;

/// Hypothetical Plant struct (assumed from plant_engine.rs)
#[derive(Debug)]
pub struct Plant {
    pub pos: Vector3<f32>,      // Position in 3D space
    pub stem_dir: Vector3<f32>, // Direction of stem growth
    pub root_dir: Vector3<f32>, // Direction of root growth
    pub energy: f32,            // Energy level for growth
    pub coherence: f32,         // Structural stability (H)
}

/// Hypothetical Environment struct (assumed from simulation_env.rs)
#[derive(Debug)]
pub struct Environment {
    pub light_pos: Vector3<f32>,  // Position of light source
    pub water_pos: Vector3<f32>,  // Position of water source
    pub gravity: Vector3<f32>,    // Gravity direction (e.g., [0, -1, 0])
    pub obstacles: Vec<Vector3<f32>>, // Positions of obstacles
    pub light_intensity: f32,     // Strength of light
    pub water_level: f32,         // Availability of water
}

/// Tropism results for logging and rendering
#[derive(Debug)]
pub struct TropismResult {
    pub growth_delta: Vector3<f32>, // Change in position or direction
    pub energy_change: f32,         // Impact on plant energy
    pub log: String,                // Description for dev_window
}

pub struct Tropisms;

impl Tropisms {
    /// Phototropism: Growth toward light source
    pub fn phototropism(plant: &mut Plant, env: &Environment) -> TropismResult {
        let light_dir = (env.light_pos - plant.pos).normalize();
        let intensity_factor = env.light_intensity * 0.1; // Scale growth by light strength
        let growth = light_dir * intensity_factor;

        // Update plant stem direction and position
        plant.stem_dir = (plant.stem_dir + growth).normalize();
        plant.pos += growth;
        plant.energy += intensity_factor * 0.5; // Light boosts energy
        plant.coherence -= 0.01; // Slight cost to structural stability

        TropismResult {
            growth_delta: growth,
            energy_change: intensity_factor * 0.5,
            log: format!(
                "Phototropism: Grew {:.2} toward light at {:?}",
                growth.norm(), env.light_pos
            ),
        }
    }

    /// Gravitropism: Stems grow up, roots grow down relative to gravity
    pub fn gravitropism(plant: &mut Plant, env: &Environment) -> TropismResult {
        let gravity_dir = env.gravity.normalize();
        let stem_growth = -gravity_dir * 0.05; // Stems grow opposite gravity
        let root_growth = gravity_dir * 0.03;  // Roots grow with gravity

        // Update directions
        plant.stem_dir = (plant.stem_dir + stem_growth).normalize();
        plant.root_dir = (plant.root_dir + root_growth).normalize();
        plant.pos += stem_growth; // Stem movement affects position
        plant.energy -= 0.02;     // Energy cost for growth
        plant.coherence += 0.01;  // Gravity alignment improves stability

        TropismResult {
            growth_delta: stem_growth,
            energy_change: -0.02,
            log: format!(
                "Gravitropism: Stem grew up {:.2}, roots down {:.2}",
                stem_growth.norm(), root_growth.norm()
            ),
        }
    }

    /// Hydrotropism: Roots grow toward water source
    pub fn hydrotropism(plant: &mut Plant, env: &Environment) -> TropismResult {
        let water_dir = (env.water_pos - plant.pos).normalize();
        let water_factor = env.water_level * 0.08; // Scale by water availability
        let growth = water_dir * water_factor;

        // Update root direction and position
        plant.root_dir = (plant.root_dir + growth).normalize();
        plant.pos += growth * 0.5; // Roots pull plant slightly
        plant.energy += water_factor * 0.3; // Water boosts energy
        plant.coherence -= 0.01; // Minor stability cost

        TropismResult {
            growth_delta: growth,
            energy_change: water_factor * 0.3,
            log: format!(
                "Hydrotropism: Roots grew {:.2} toward water at {:?}",
                growth.norm(), env.water_pos
            ),
        }
    }

    /// Thigmotropism: Growth response to physical contact (e.g., wrapping around obstacles)
    pub fn thigmotropism(plant: &mut Plant, env: &Environment) -> TropismResult {
        let mut closest_obstacle = None;
        let mut min_dist = f32::MAX;

        // Find nearest obstacle
        for obstacle in &env.obstacles {
            let dist = (obstacle - plant.pos).norm();
            if dist < min_dist && dist < 1.0 { // Threshold for contact
                min_dist = dist;
                closest_obstacle = Some(obstacle);
            }
        }

        if let Some(obstacle) = closest_obstacle {
            let contact_dir = (obstacle - plant.pos).normalize();
            let tangent = Vector3::new(-contact_dir.y, contact_dir.x, 0.0).normalize(); // Perpendicular in 2D plane
            let growth = tangent * 0.04; // Wrap around obstacle

            plant.stem_dir = (plant.stem_dir + growth).normalize();
            plant.pos += growth;
            plant.energy -= 0.03; // Energy cost for bending
            plant.coherence += 0.02; // Structural adaptation improves coherence

            TropismResult {
                growth_delta: growth,
                energy_change: -0.03,
                log: format!(
                    "Thigmotropism: Wrapped {:.2} around obstacle at {:?}",
                    growth.norm(), obstacle
                ),
            }
        } else {
            TropismResult {
                growth_delta: Vector3::zeros(),
                energy_change: 0.0,
                log: "Thigmotropism: No obstacles in range".to_string(),
            }
        }
    }

    /// Apply all tropisms with randomness for natural variation
    pub fn apply_all(plant: &mut Plant, env: &Environment) -> Vec<TropismResult> {
        let mut rng = rand::thread_rng();
        let mut results = Vec::new();

        // Apply each tropism with slight random noise
        results.push(Self::phototropism(plant, env));
        results.push(Self::gravitropism(plant, env));
        results.push(Self::hydrotropism(plant, env));
        results.push(Self::thigmotropism(plant, env));

        // Add small random perturbation to simulate natural irregularity
        let noise = Vector3::new(
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-0.01..0.01),
        );
        plant.pos += noise;

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
        };
        let env = Environment {
            light_pos: Vector3::new(5.0, 5.0, 0.0),
            water_pos: Vector3::new(2.0, -2.0, 0.0),
            gravity: Vector3::new(0.0, -1.0, 0.0),
            obstacles: vec![Vector3::new(1.0, 0.0, 0.0)],
            light_intensity: 1.0,
            water_level: 1.0,
        };
        (plant, env)
    }

    #[test]
    fn test_phototropism() {
        let (mut plant, env) = setup();
        let result = Tropisms::phototropism(&mut plant, &env);
        assert!(result.growth_delta.norm() > 0.0);
        assert!(plant.energy > 10.0); // Energy increases
        assert!(plant.coherence < 1.0); // Coherence decreases slightly
    }

    #[test]
    fn test_thigmotropism_contact() {
        let (mut plant, env) = setup();
        plant.pos = Vector3::new(0.5, 0.0, 0.0); // Near obstacle
        let result = Tropisms::thigmotropism(&mut plant, &env);
        assert!(result.growth_delta.norm() > 0.0);
        assert!(result.log.contains("Wrapped"));
    }
}