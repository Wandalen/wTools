//! Integration tests for Unilang framework
//!
//! This module aggregates all integration tests that verify component interactions.

// Core workflow integration tests
#[path = "integration/end_to_end.rs"]
pub mod end_to_end;

#[path = "integration/parser_semantic.rs"]
pub mod parser_semantic;

#[path = "integration/comprehensive_workflow.rs"]
pub mod comprehensive_workflow;

// Performance integration tests
#[path = "integration/performance.rs"]
pub mod performance;

#[path = "integration/performance_analysis.rs"]
pub mod performance_analysis;

#[path = "integration/json_performance.rs"]
pub mod json_performance;

// API compatibility integration tests
#[path = "integration/api_compatibility.rs"]
pub mod api_compatibility;

#[path = "integration/cli_builder_api.rs"]
pub mod cli_builder_api;

#[path = "integration/ergonomic_apis.rs"]
pub mod ergonomic_apis;

#[path = "integration/external_usage.rs"]
pub mod external_usage;

// Registry integration tests
#[path = "integration/registry_basic.rs"]
pub mod registry_basic;

#[path = "integration/static_registry.rs"]
pub mod static_registry;

#[path = "integration/phf_map_generation.rs"]
pub mod phf_map_generation;

// Data structure integration tests
#[path = "integration/static_data_structures.rs"]
pub mod static_data_structures;

#[path = "integration/string_interning.rs"]
pub mod string_interning;

#[path = "integration/simd_json.rs"]
pub mod simd_json;

// System integration tests
#[path = "integration/multi_yaml_system.rs"]
pub mod multi_yaml_system;