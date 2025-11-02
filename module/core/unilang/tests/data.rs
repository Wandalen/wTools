//! Data Domain Tests
//!
//! All tests related to data models: serialization, validation,
//! error handling, and type systems.

mod data {
  mod api_consistency;
  mod command_definition_v2;
  mod error_handling;
  mod loader;
  mod static_data;
  mod types;
  mod validated_newtypes;
}