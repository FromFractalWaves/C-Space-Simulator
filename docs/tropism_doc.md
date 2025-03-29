# Tropisms in the Computational Spacetime Framework

**File**: `src/engines/tropisms.rs`  
**Date**: March 29, 2025  
**Purpose**: Models plant tropisms as navigational processes within the Computational Spacetime (C-Space) Framework, integrating geometric computation, coherence dynamics, and perpendicularity mechanics for the C-Plants simulation.

## Overview

The `tropisms.rs` module implements plant growth behaviors (phototropism, gravitropism, hydrotropism, and thigmotropism) as computational navigation through a geometric **computational manifold** (\(\mathcal{M}\)) defined by the C-Space Framework. Plants are entities optimizing their growth paths—geodesics—within this manifold, influenced by a **metric tensor** (\(g\)), **complex density** (\(\rho_c\)), **coherence** (\(H\)), **distortion** (\(D\)), and **emergent time** (\(T\)). The module bridges theoretical constructs from `01Computational-SpaceTime.md`, `02Time-Defined-Energy.md`, `03Perpendicularity-Mechanics.md`, and `04Hierarchical-Infinity.md` with practical simulation logic for the C-Plants project.

This implementation:
- Represents plants as computational entities navigating \(\mathcal{M}\).
- Applies tropisms as dynamic updates to plant state, reflecting energy-coherence interactions.
- Logs results for debugging and visualization, aligning with the C-Plants modular design.

## Key Components

### `Plant` Struct
- **Purpose**: Represents a plant as a computational entity in \(\mathcal{M}\).
- **Fields**:
  - `pos: Vector3<f32>`: Position in 3D space, mapped to manifold coordinates.
  - `stem_dir: Vector3<f32>`: Stem direction, aligned with coherence (\(H\)).
  - `root_dir: Vector3<f32>`: Root direction, aligned with emergent time (\(T\)).
  - `energy: f32`: Computational energy \(E(p)\), influenced by resources.
  - `coherence: f32`: \(H\), structural organization.
  - `distortion: f32`: \(D\), instability from environment or growth.
  - `temporal_complexity: f32`: \(T\), emergent time from growth processes.
  - `spatial_complexity: f32`: \(S\), structural complexity in space.

### `Environment` Struct
- **Purpose**: Defines the computational manifold \(\mathcal{M}\) and its environmental factors.
- **Fields**:
  - `light_pos`, `water_pos`, `gravity`, `obstacles`: Physical influences on tropisms.
  - `light_intensity`, `water_level`: Energy contributions to \(E(p)\).
  - `metric_tensor: Matrix3<f32>`: \(g\), defines manifold geometry (basis: \(\{dE, dH, dD\}\)).
  - `d_critical: f32`: Threshold for computational singularities.

### `TropismResult` Struct
- **Purpose**: Captures outcomes of tropism computations for logging and rendering.
- **Fields**:
  - `growth_delta: Vector3<f32>`: Change in position or direction.
  - `rho_c: f32`: Complex density (\(\rho_c = \sqrt{S^2 + T^2} \cdot E\)).
  - `log: String`: Descriptive output for `dev_window.rs`.

### `Tropisms` Implementation
- **Methods**:
  - `compute_metric_tensor`: Constructs \(g = \begin{pmatrix} \frac{1}{E^2} & 0 & 0 \\ 0 & \frac{1}{E} & 0 \\ 0 & 0 & \frac{1}{D + \epsilon} \end{pmatrix}\).
  - `compute_complex_density`: Calculates \(\rho_c = \sqrt{S^2 + T^2} \cdot E\).
  - `update_dynamics`: Updates \(H\), \(D\), and \(T\) using:
    - \(\frac{dH}{dt} = -\alpha \left( \frac{D}{H + \epsilon} + \nabla S \right)\)
    - \(\frac{dD}{dt} = \beta \cdot \log(1 + |\Delta H| \cdot E)\)
    - \(\frac{dT}{dt} = \beta \cdot \tanh(|\Delta H| \cdot E) \cdot \text{sign}(H)\)
  - `phototropism`, `gravitropism`, `hydrotropism`, `thigmotropism`: Specific tropism behaviors.
  - `apply_all`: Combines all tropisms with noise, simulating manifold navigation.

## Theoretical Integration

### Computational Manifold (\(\mathcal{M}\))
- Tropisms adjust `stem_dir` and `root_dir` as geodesics in \(\mathcal{M}\), shaped by the `metric_tensor`. Growth reflects path optimization per `01Computational-SpaceTime.md`.

### Complex Density (\(\rho_c\))
- Computed as \(\rho_c = \sqrt{S^2 + T^2} \cdot E\), unifying spatial (\(S\)), temporal (\(T\)), and energy (\(E\)) factors, consistent with `02Time-Defined-Energy.md`.

### Coherence Dynamics
- Implements coupled evolution of \(H\) and \(D\), with \(T\) as an orthogonal projection, per `03Perpendicularity-Mechanics.md`. Singularities (\(D > D_{\text{critical}}\)) trigger Pure Time States (\(\rho_c = T \cdot E\)).

### Perpendicularity Mechanics
- `stem_dir` (\(H\)) and `root_dir` (\(T\)) evolve orthogonally, coupled via energy and distortion, aligning with non-dual dynamics.

### Hierarchical Infinity
- Noise and unbounded \(T\) growth hint at recursive complexity, a precursor to hierarchical space generation from `04Hierarchical-Infinity.md`.

### Pure Time States
- At singularities, \(S \to 0\), collapsing to \(\rho_c = T \cdot E\), preserving information as per `02Time-Defined-Energy.md`.

## Usage in C-Plants

- **Plant Engine**: Call `Tropisms::apply_all` in `plant_engine.rs` with a time step (`dt`) to simulate growth.
- **Logging**: Pass `TropismResult::log` to `dev_window.rs` for debugging.
- **Rendering**: Use `growth_delta` and `rho_c` in `renderer.rs` to visualize navigation and complexity.
- **Control**: Modify `Environment` parameters via `control_window.rs` to tune \(\mathcal{M}\).

## Assumptions and Simplifications

- \(\nabla S\) is approximated as average resource distance, omitting neighbor interactions for simplicity.
- Hierarchical Infinity is partially represented (no explicit space branching); extendable in future iterations.
- Constants (\(\alpha = 0.05\), \(\beta = 0.1\)) are placeholders—tune for simulation scale.

## Future Enhancements

- Integrate recursive Hilbert spaces for full Hierarchical Infinity support.
- Refine `metric_tensor` influence on growth trajectories.
- Add energy-time compression (ETC) for resource encoding.

This module transforms plant tropisms into a computational navigation paradigm, showcasing the C-Space Framework’s power in a biological context.