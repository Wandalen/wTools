//! Parser Domain Tests
//!
//! All tests related to parsing functionality: tokenization, argument parsing,
//! SIMD parsing, string interning, and related data structures.

mod parser {
  mod argument_parsing;
  mod command_sequence_scalability;
  mod edge_case_handling;
  mod extended_ascii_safety;
  mod file_path_parsing;
  mod quoted_values;
  mod simd_json;
  mod simd_tokenization;
  mod static_data_structures;
  mod string_interning;
  mod unicode_safety;
}