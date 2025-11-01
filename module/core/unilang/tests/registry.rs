//! Registry Domain Tests
//!
//! All tests related to registry management: static/dynamic registry,
//! command lookup, and performance metrics.

mod registry {
  mod debug;
  mod phf_map_functionality;
  mod registry_basic;
  mod static_registry;
  mod duplicate_detection;
  mod registration_error_handling;
  mod validation_enforcement;  // Phase 1 validation tests
}