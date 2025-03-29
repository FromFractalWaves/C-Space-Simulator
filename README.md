cs_simulator/
├── src/
│   ├── main.rs                # Main application entry point
│   ├── plant_main.rs          # Standalone plant creator entry point
│   ├── engine.rs              # C-Space simulation engine
│   └── ui/                    # UI components
│       ├── mod.rs             # Module declarations
│       ├── app.rs             # Main application container
│       ├── main_menu.rs       # Main menu screen
│       ├── plant.rs           # Plant creator component
│       ├── simulation.rs      # Simulation viewer component
│       └── common.rs          # Shared UI utilities and parameter definitions
├── Cargo.toml                 # Project configuration with multiple binaries
└── README.md                  # This file