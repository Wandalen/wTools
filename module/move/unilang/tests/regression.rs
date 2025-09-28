//! Regression tests for Unilang framework
//!
//! This module aggregates all regression tests that prevent previously fixed bugs.

// Task 024 regression prevention
#[path = "regression/parameter_collection.rs"]
pub mod parameter_collection;

// Command registration regression prevention
#[path = "regression/command_registration.rs"]
pub mod command_registration;

// Dot command panic regression prevention
#[path = "regression/dot_command_panic.rs"]
pub mod dot_command_panic;