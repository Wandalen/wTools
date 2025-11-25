//! Project templates for cargo_unilang
//!
//! Contains template generators for creating correct unilang project structure.

mod cargo_toml;
mod main_rs;
mod commands_yaml;

pub use cargo_toml::*;
pub use main_rs::*;
pub use commands_yaml::*;
