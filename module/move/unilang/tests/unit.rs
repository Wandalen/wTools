//! Unit tests for Unilang framework components
//!
//! This module aggregates all unit tests organized by component.

// Semantic analysis unit tests
#[path = "unit/semantic/multiple_parameters.rs"]
pub mod multiple_parameters;

#[path = "unit/semantic/command_validation.rs"]
pub mod command_validation;

// Parser unit tests
#[path = "unit/parser/quoted_values.rs"]
pub mod quoted_values;

#[path = "unit/parser/argument_parsing.rs"]
pub mod argument_parsing;

#[path = "unit/parser/file_path_parsing.rs"]
pub mod file_path_parsing;

// Help system unit tests
#[path = "unit/help/conventions.rs"]
pub mod help_conventions;

#[path = "unit/help/formatting.rs"]
pub mod help_formatting;

#[path = "unit/help/generation.rs"]
pub mod help_generation;

#[path = "unit/help/operator.rs"]
pub mod help_operator;

#[path = "unit/help/enforcement.rs"]
pub mod help_enforcement;

// Data structures unit tests
#[path = "unit/data/error_handling.rs"]
pub mod error_handling;

#[path = "unit/data/types.rs"]
pub mod data_types;

#[path = "unit/data/api_consistency.rs"]
pub mod api_consistency;

#[path = "unit/data/loader.rs"]
pub mod data_loader;

#[path = "unit/data/static_data.rs"]
pub mod static_data;

// Registry unit tests
#[path = "unit/registry/debug.rs"]
pub mod registry_debug;

#[path = "unit/registry/phf_map_functionality.rs"]
pub mod phf_map_functionality;

// Performance unit tests
#[path = "unit/performance/benchmark_config.rs"]
pub mod benchmark_config;

// Build system unit tests
#[path = "unit/build/compile_time_debug.rs"]
pub mod compile_time_debug;

// CLI unit tests
#[path = "unit/cli/verbosity_control.rs"]
pub mod verbosity_control;