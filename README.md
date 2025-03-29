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




control window must set up simulation_env

To refactor simulation_runner.rs into an event loop that operates independently of GTK, managing the engines, control, and simulation, we'll need to:

    Redesign SimulationRunner as a standalone event loop: Replace the GTK idle_add and thread-based approach with a custom loop using Rust's standard library or a lightweight async framework (e.g., tokio or a simple loop with timing control).
    Integrate engines and control: Ensure the loop interacts with PlantEngine, CSpaceEngine, and SimulationControl in a cohesive manner.
    Enable CLI control via dev_window: Use the vte4 terminal in dev_window.rs to accept commands (e.g., start, stop, status) and communicate them to the runner via a channel.

Below, I'll outline the steps and provide modified code for simulation_runner.rs and dev_window.rs to achieve this.

How It Works

    Runner Event Loop:
        Runs in a separate thread, independent of GTK.
        Listens for ControlCommands via a channel and updates the simulation accordingly.
        Sends TropismResults back via another channel.
    CLI Control via Dev Window:
        The VTE terminal in dev_window displays logs and accepts commands.
        Typing start, stop, or status sends the corresponding ControlCommand to the runner.
    Integration:
        main.rs spawns the runner and passes the command sender and log receiver to the GUI.
        Other windows (e.g., simulation_window) continue to visualize the shared state.

to do

launch startup --> open dev_window --> get simulation status