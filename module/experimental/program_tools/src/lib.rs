//! Rust script runner — compile and execute Rust files as scripts with output capture.
//!
//! Provides a three-level builder hierarchy for constructing execution plans
//! (`Source`, `Program`, `Plan`), execution entry points (`run`, `run_file`,
//! `run_source`, `run_project`), and a captured output type with assertion
//! methods for use in Rust test functions.

#[ cfg( feature = "enabled" ) ]
mod private
{
}

#[ cfg( feature = "enabled" ) ]
mod_interface::mod_interface!
{
  /// Builder types for program representation.
  layer program;
  /// Execution configuration options.
  layer run_options;
  /// Captured execution output with assertion methods.
  layer output;
  /// Script execution runner.
  layer runner;
}
