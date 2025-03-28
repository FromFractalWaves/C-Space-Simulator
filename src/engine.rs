use rand::Rng;
use std::collections::HashMap;

// Struct to hold all engine parameters
#[derive(Clone, Debug)]
pub struct EngineParams {
    // C-Space parameters
    pub alpha: f32,
    pub beta: f32,
    pub epsilon: f32,
    pub d_critical: f32,
    pub lambda: f32,
    
    // Plant parameters
    pub growth_rate: f32,
    pub growth_prob: f32,
    pub branch_prob: f32,
    pub max_nodes: usize,
    pub initial_energy: f32,
    
    // Environment parameters
    pub max_energy_distance: f32,
}

impl Default for EngineParams {
    fn default() -> Self {
        Self {
            alpha: 0.2,
            beta: 0.3,
            epsilon: 1e-9,
            d_critical: 15.0,
            lambda: 0.5,
            growth_rate: 5.0,
            growth_prob: 0.3,
            branch_prob: 0.1,
            max_nodes: 500,
            initial_energy: 1.0,
            max_energy_distance: 200.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self { x: self.x / mag, y: self.y / mag }
        } else {
            Self { x: 0.0, y: -1.0 }
        }
    }
}

impl std::ops::Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::ops::Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl std::ops::Mul<f32> for Vector2D {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

#[derive(Clone, Debug)]
pub struct ResourcePoint {
    pub position: Vector2D,
    pub intensity: f32,
    pub r_type: String,
}

#[derive(Clone, Debug)]
pub struct PlantNode {
    pub id: usize,
    pub position: Vector2D,
    pub energy: f32,
    pub coherence: f32,
    pub distortion: f32,
    pub temporal_complexity: f32,
    pub spatial_complexity: f32,
    pub parent: Option<usize>,
    pub age: i32,
    pub c_t: Vector2D,
}

pub struct CSpaceEngine {
    pub width: f32,
    pub height: f32,
    nodes: Vec<PlantNode>,
    resources: Vec<ResourcePoint>,
    paths: HashMap<usize, Vec<PlantNode>>,
    time: i32,
    params: EngineParams,
}

impl CSpaceEngine {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            nodes: Vec::new(),
            resources: Vec::new(),
            paths: HashMap::new(),
            time: 0,
            params: EngineParams::default(),
        }
    }
    
    pub fn new_with_params(width: f32, height: f32, params: EngineParams) -> Self {
        Self {
            width,
            height,
            nodes: Vec::new(),
            resources: Vec::new(),
            paths: HashMap::new(),
            time: 0,
            params,
        }
    }

    pub fn initialize_plant(&mut self, start_pos: (f32, f32), initial_energy: f32) {
        let seed = PlantNode {
            id: 0,
            position: Vector2D::new(start_pos.0, start_pos.1),
            energy: initial_energy,
            coherence: 1.0,
            distortion: 0.0,
            temporal_complexity: 0.0,
            spatial_complexity: 0.5,
            parent: None,
            age: 0,
            c_t: Vector2D::new(0.0, 0.0),
        };
        self.nodes = vec![seed];
        self.paths.clear();
    }

    pub fn add_resource(&mut self, pos: (f32, f32), intensity: f32, r_type: &str) {
        self.resources.push(ResourcePoint {
            position: Vector2D::new(pos.0, pos.1),
            intensity,
            r_type: r_type.to_string(),
        });
    }
    
    pub fn update_params(&mut self, params: EngineParams) {
        self.params = params;
    }

    fn calculate_energy(&self, pos: Vector2D) -> f32 {
        let mut energy = 0.3;
        for resource in &self.resources {
            if resource.r_type == "light" {
                let distance = pos.distance(&resource.position);
                if distance < self.params.max_energy_distance {
                    energy += resource.intensity * (1.0 - distance / self.params.max_energy_distance) * 2.0;
                }
            }
        }
        energy.clamp(0.1, 1.5)
    }

    fn compute_attention(&self, node: &PlantNode) -> Vector2D {
        let mut attentions = Vec::new();
        for resource in &self.resources {
            let dist = node.position.distance(&resource.position);
            let g11 = 1.0 / (node.energy.powi(2));
            let exp_term = (-self.params.lambda * dist * g11).exp();
            let weight = exp_term * (node.coherence / node.distortion.max(self.params.epsilon));
            let direction = (resource.position - node.position).normalize();
            attentions.push((weight, direction));
        }
        let total_weight: f32 = attentions.iter().map(|(w, _)| *w).sum();
        let total_weight = total_weight.max(self.params.epsilon);
        attentions.iter().fold(Vector2D::new(0.0, 0.0), |acc, (w, d)| {
            acc + (*w / total_weight * *d)
        })
    }

    fn grow_node(&self, node: &PlantNode) -> Option<PlantNode> {
        if node.coherence <= 0.1 || node.age >= 20 {
            return None;
        }
        let direction = self.compute_attention(node);
        let growth = self.params.growth_rate * (0.5 + node.energy);
        let mut new_pos = node.position + (direction * growth);
        new_pos.x = new_pos.x.clamp(0.0, self.width - 1.0);
        new_pos.y = new_pos.y.clamp(0.0, self.height - 1.0);

        Some(PlantNode {
            id: self.nodes.len(),
            position: new_pos,
            energy: self.calculate_energy(new_pos),
            coherence: (node.coherence * 0.9).max(0.5),
            distortion: node.distortion * 0.5,
            temporal_complexity: 0.0,
            spatial_complexity: 0.5,
            parent: Some(node.id),
            age: 0,
            c_t: node.c_t + node.position,
        })
    }
    
    // Create a branch with variations in properties
    fn branch_from_node(&self, node: &PlantNode) -> Option<PlantNode> {
        if node.coherence <= 0.2 || node.age >= 5 {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        
        // Create a perpendicular direction with some randomness
        let direction = Vector2D::new(
            angle.cos(),
            angle.sin(),
        );
        
        let growth = self.params.growth_rate * (0.3 + node.energy * 0.7);
        let mut new_pos = node.position + (direction * growth);
        new_pos.x = new_pos.x.clamp(0.0, self.width - 1.0);
        new_pos.y = new_pos.y.clamp(0.0, self.height - 1.0);

        Some(PlantNode {
            id: self.nodes.len(),
            position: new_pos,
            energy: self.calculate_energy(new_pos) * 0.8, // Branches have slightly less energy
            coherence: node.coherence * 0.8,
            distortion: node.distortion * 0.8,
            temporal_complexity: node.temporal_complexity + 0.1,
            spatial_complexity: node.spatial_complexity + 0.2,
            parent: Some(node.id),
            age: 0,
            c_t: node.c_t + node.position,
        })
    }
    
    // Update a specific node's properties based on C-Space dynamics
    fn update_node_properties(&self, node: &mut PlantNode) {
        // Calculate spatial complexity (distance to target resources)
        let mut spatial_complexity = 0.5;
        if let Some(closest_resource) = self.resources.iter()
            .filter(|r| r.r_type == "light")
            .min_by(|a, b| {
                let dist_a = a.position.distance(&node.position);
                let dist_b = b.position.distance(&node.position);
                dist_a.partial_cmp(&dist_b).unwrap()
            }) 
        {
            let dist = node.position.distance(&closest_resource.position);
            spatial_complexity = (dist / self.params.max_energy_distance).min(1.0);
        }
        
        // Update coherence using C-Space dynamics
        let dH = -self.params.alpha * (node.distortion / (node.coherence + self.params.epsilon) + spatial_complexity);
        node.distortion += self.params.beta * (1.0 + dH.abs() * node.energy).ln();
        node.temporal_complexity += self.params.beta * (dH.abs() * node.energy).tanh() * node.coherence.signum();
        node.coherence += dH;
        
        // Clamp values
        node.coherence = node.coherence.max(0.0).min(1.0);
        node.distortion = node.distortion.max(0.0);
        node.temporal_complexity = node.temporal_complexity.max(0.0);
        node.spatial_complexity = spatial_complexity;
        
        // Handle singularity if needed
        if node.distortion > self.params.d_critical {
            // Reset node to "pure time state"
            node.coherence = 0.1;
            node.distortion = 0.0;
            node.temporal_complexity = node.energy * (node.c_t.magnitude() + self.params.epsilon).ln();
        }
    }

    pub fn update(&mut self) {
        self.time += 1;
        let mut new_nodes = Vec::new();
        let mut rng = rand::thread_rng();

        for node in &self.nodes {
            let mut updated_node = node.clone();
            updated_node.energy = self.calculate_energy(node.position);
            updated_node.age += 1;
            
            // Update node properties according to C-Space dynamics
            self.update_node_properties(&mut updated_node);
            
            new_nodes.push(updated_node);

            // Check for growth
            if rng.gen::<f32>() < self.params.growth_prob && self.nodes.len() < self.params.max_nodes {
                if let Some(new_node) = self.grow_node(node) {
                    new_nodes.push(new_node);
                }
            }
            
            // Check for branching
            if rng.gen::<f32>() < self.params.branch_prob && self.nodes.len() < self.params.max_nodes {
                if let Some(branch_node) = self.branch_from_node(node) {
                    new_nodes.push(branch_node);
                }
            }
        }

        self.nodes = new_nodes;
        self.paths = self.nodes.iter()
            .filter(|n| n.parent.is_some())
            .fold(HashMap::new(), |mut acc, n| {
                acc.entry(n.parent.unwrap()).or_insert(Vec::new()).push(n.clone());
                acc
            });
    }

    pub fn get_nodes(&self) -> &Vec<PlantNode> {
        &self.nodes
    }

    pub fn get_resources(&self) -> &Vec<ResourcePoint> {
        &self.resources
    }
    
    pub fn get_paths(&self) -> &HashMap<usize, Vec<PlantNode>> {
        &self.paths
    }
    
    pub fn get_time(&self) -> i32 {
        self.time
    }
}