//! Semantic Domain Tests
//!
//! All tests related to semantic analysis: command validation, argument binding,
//! type checking, and multiple parameter collection.

mod semantic {
  mod argument_binding;
  mod command_validation;
  mod multiple_parameters;
  mod parser_semantic;
  mod unknown_parameters;
  mod unknown_parameters_edge_cases;
  mod parameter_storage_validation;
}