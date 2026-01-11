//! Regression Domain Tests
//!
//! All tests related to bug prevention: critical bug reproduction tests,
//! regression prevention, and historical issue coverage.

mod regression {
  mod command_registration;
  mod dot_command_panic;
  mod example_yaml_discovery_bug;
  mod parameter_collection;
}