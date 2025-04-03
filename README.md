# Space engine

**Space engine** is a game engine focused on creating and managing space games. The project includes various modules (crates) for handling assets handling, camera management and the simulation.


## Requirements
- Rust 2021 edition or later. You can install Rust using [rustup](https://rustup.rs/).
- **Toolchains:** stable, nightly

## Usage
You can run example applications to see the engine in action. Each example showcases different capabilities of the engine.

**core:**

``
cargo run --example pack_objects
``

``
cargo run --example streaming
``

**space:**

``
cargo run --example orbit_viewer
``

``
cargo run --example two_body
``


# Project Modules
#### core modules
- **core_assets:** Manages loading, caching, and handling of game assets.
- **core_camera:** Provides functionalities for camera controls and rendering perspectives.

#### Modules

- **engine:** export of all features. Acting as an entry point
- **engine_core:** Provides essential utilities and core functions for the engine's operation.
Simulation and Physics
- **space:** Manages physics, rendering, and functionality related to 2D space environment.
- **utils:** Shared utility functions across the engine modules