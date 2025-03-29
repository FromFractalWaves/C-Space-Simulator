C-Space Simulation Visualizer (CSSV)
A modular application for visualizing the Computational Spacetime Framework.


C-Plants is an interactive simulation that models plant growth using principles from the Computational Spacetime (C-Space) Framework. This application demonstrates how plants can be modeled as navigators through a geometric computational manifold, optimizing their growth patterns based on resources, constraints, and environmental factors.

The simulation visualizes how plants use various tropisms (directional growth responses) to navigate their environment, seeking light, water, and structural support while avoiding obstacles - all framed within the theoretical C-Space paradigm.

## Key Concepts

### Plants as C-Space Navigators

C-Plants represents plants as entities that:

1. **Navigate a Computational Manifold**: Plants move through a geometric space where distances, paths, and complexity are defined by mathematical relations.

2. **Follow Tropism Principles**: Plants exhibit:
   - **Phototropism**: Growth toward light sources
   - **Gravitropism**: Response to gravity (stems grow up, roots down)
   - **Hydrotropism**: Growth toward water
   - **Thigmotropism**: Response to physical touch (wrapping around supports)

3. **Balance Energy-Coherence Dynamics**: Plants maintain structural integrity (coherence) while optimizing for energy acquisition, managing:
   - **Coherence (H)**: Structural organization and stability
   - **Distortion (D)**: Environmental or internal instability
   - **Temporal Complexity (T)**: Rate of growth or procedural evolution
   - **Spatial Complexity (S)**: Structural organization in 3D space

   C-Plants README
Overview

C-Plants is an interactive Rust-based simulation that models plant growth using principles from the Computational Spacetime (C-Space) Framework. Porting a 1700-line Python project (game/plant_app/), this application demonstrates how plants can be modeled as navigators through a geometric computational manifold, optimizing their growth patterns based on resources, constraints, and environmental factors. Built with eframe and egui for GUI, it leverages Rust’s static typing and ownership for a robust, modular simulator.

The simulation visualizes how plants use various tropisms (directional growth responses) to navigate their environment, seeking light, water, and structural support while avoiding obstacles—all framed within the theoretical C-Space paradigm. Results are logged for debugging, rendered visually, and controlled indirectly via a multi-window interface.

Current structure (as of March 29, 2025):
text
cs_simulator/
├── Cargo.toml
├── data/                  # Runtime assets (e.g., plant configs)
├── docs/
│   └── plan.md
└── src/
    ├── control/
    │   ├── objects.rs     # Control entities
    │   └── tables.rs      # Control mappings
    ├── engines/
    │   ├── cspace_engine.rs  # C-Space framework
    │   └── plant_engine.rs   # Plant simulation logic
    ├── gui/
    │   ├── control_window.rs  # Control UI
    │   ├── dev_window.rs      # Dev tools UI
    │   ├── mod.rs             # GUI module root
    │   ├── simulation_window.rs  # Sim display
    │   └── startup_window.rs     # App launcher
    ├── main.rs                # Entry point
    └── simulation/
        ├── asset_handler.rs   # Asset management
        ├── renderer.rs        # Visual rendering
        └── simulation_env.rs  # Sim state
Goals

    Port Python’s plant_app/ (controls, cspace_engine, plant_engine, effects, renderer, main) into Rust as "C-Plants."
    Model plant growth as C-Space navigation, logging results to dev_window and rendering via renderer.
    Maintain modularity with indirect global control through GUI and simulation state.

Key Concepts
Plants as C-Space Navigators

C-Plants represents plants as entities that:

    Navigate a Computational Manifold: Plants move through a geometric space where distances, paths, and complexity are defined by mathematical relations, implemented in cspace_engine.rs.
    Follow Tropism Principles: Plants exhibit:
        Phototropism: Growth toward light sources.
        Gravitropism: Response to gravity (stems up, roots down).
        Hydrotropism: Growth toward water.
        Thigmotropism: Response to physical touch (e.g., wrapping around supports).
    Balance Energy-Coherence Dynamics: Plants optimize energy acquisition while maintaining structural integrity, managing:
        Coherence (H): Structural organization and stability.
        Distortion (D): Environmental or internal instability.
        Temporal Complexity (T): Rate of growth or procedural evolution.
        Spatial Complexity (S): Structural organization in 3D space.

These concepts drive plant_engine.rs, with cspace_engine.rs providing the mathematical backbone for manifold navigation and tropism calculations.
Core Assumptions

    Simulation State: simulation_env.rs holds plants and environment data (e.g., light, water levels).
    Plant Engine: plant_engine.rs computes growth based on simulation_env and C-Space principles, producing loggable results.
    Logging: dev_window.rs displays debug output (e.g., tropism effects, growth rates).
    Rendering: renderer.rs visualizes plants using egui in startup_window.rs.
    Control: control/ manages user input, indirectly influencing all modules via simulation_env.
    Assets: data/ provides initial conditions (e.g., plants.json), loaded by asset_handler.rs.

Strategy
1. Simulation Environment (simulation_env.rs)

    Role: Central state holder—owns plants and env variables (e.g., Vec<Plant>, time: f32).
    Plan: Struct with methods like add_plant(x, y) and update_time(dt). Passed as &mut to plant_engine, & to renderer.
    Why: Rust’s ownership ensures one source of truth, aligning with C-Space’s structured navigation.

2. Plant Engine (plant_engine.rs)

    Role: Simulates plant growth using simulation_env and C-Space tropisms, producing results (e.g., growth delta, logs).
    Plan: Takes &mut SimulationEnv, uses cspace_engine for manifold math, returns results for dev_window and renderer.
    Why: Keeps logic stateless, outputs data for downstream use, reflects C-Plants’ navigation paradigm.

3. Developer Window (dev_window.rs)

    Role: Displays logs (e.g., "Plant grew 0.5 via phototropism").
    Plan: Function draw_dev(ui, logs) called from startup_window.rs, with logs stored in SimulatorLauncher.
    Why: Separates debug UI, supports C-Space result analysis.

4. Renderer (renderer.rs)

    Role: Draws plants from simulation_env using egui.
    Plan: Takes &SimulationEnv, renders plants (e.g., shapes for stems, roots) based on tropism and complexity.
    Why: Read-only access ensures safe visualization of C-Space navigation.

5. Global Control (Indirect via control/ and gui/)

    Role: User input (e.g., "Add Plant") influences simulation_env, affecting all modules.
    Plan:
        control/objects.rs: Input logic (e.g., process_input(env, "P")).
        control/tables.rs: Key-to-action maps.
        control_window.rs: UI buttons call control functions.
        SimulatorLauncher: Checks inputs via ctx.input().
    Why: Indirect control via simulation_env keeps modules independent, mimicking C-Plants’ environmental response.

6. Asset Handling (asset_handler.rs)

    Role: Loads data/ (e.g., plants.json) into simulation_env at startup.
    Plan: Struct AssetHandler with new() and load_into_env(&mut SimulationEnv), using serde.
    Why: Decouples data from logic, supports initial C-Space conditions.

7. GUI Orchestration (startup_window.rs)

    Role: Runs the app, integrates modules.
    Plan: SimulatorLauncher holds SimulationEnv, logs, and window flags; update() calls engine, renderer, and UI.
    Why: Centralizes egui loop, reflects C-Plants’ cohesive simulation.