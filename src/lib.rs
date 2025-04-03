//! This library serves as a foundational engine built on top of the Bevy framework.
//! It provides various abstractions and functionalities that facilitate the development
//! of games and applications. The core components of the engine are organized into
//! modules that encapsulate specific functionalities, making it easier to manage and
//! extend the engine's capabilities.
//!
//! ## Modules Overview
//!
//! - **[space](engine::space)**: This module offers two body implementations that support orbital transfers,
//!   allowing for realistic space simulations and mechanics.
//!
//! - **[camera management](engine::core::camera)**: This module provides abstractions for managing camera behaviors,
//!   enabling smoother transitions between scenes with different camera configurations.
//!
//! - **[assets](engine::core::assets)**: This module abstracts asset management, simplifying the loading
//!   and handling of various resources within the engine, ensuring efficient asset usage.


#[doc(inline)]
pub use engine;


#[doc(hidden)]
pub mod prelude;