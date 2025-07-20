//! ## Test Matrix for `unilang_instruction_parser` Test Suite
//!
//! This matrix provides an overview of the main test modules included in this test suite
//! and their primary testing focus.
//!
//! **Test Factors:**
//! - Included Module: Name of the test module
//! - Purpose: High-level description of what the module tests
//!
//! ---
//!
//! **Test Combinations:**
//!
//! | ID | Included Module | Purpose |
//! |---|---|---|
//! | T7.1 | `parser_config_entry_tests` | Tests parser entry points and basic configuration. |
//! | T7.2 | `command_parsing_tests` | Tests various command path parsing scenarios. |
//! | T7.3 | `syntactic_analyzer_command_tests` | Tests syntactic analysis of commands, arguments, and operators. |
//! | T7.4 | `argument_parsing_tests` | Tests detailed argument parsing logic. |
//! | T7.5 | `comprehensive_tests` | Comprehensive test suite covering various instruction structures and error conditions. |
//! | T7.6 | `error_reporting_tests` | Tests error reporting and source location accuracy. |
//! | T7.7 | `spec_adherence_tests` | Tests adherence to the Unilang specification rules. |
//! | T7.8 | `temp_unescape_test` | Temporary test for `strs_tools` unescaping behavior. |

// Main test harness for unilang_instruction_parser
//
// Individual test files are included as modules
#[path = "parser_config_entry_tests.rs"]
mod parser_config_entry_tests;

// Add other test modules here as they are created, e.g.:
#[path = "command_parsing_tests.rs"]
mod command_parsing_tests;
#[path = "syntactic_analyzer_command_tests.rs"]
mod syntactic_analyzer_command_tests;

#[path = "argument_parsing_tests.rs"]
mod argument_parsing_tests;

#[path = "comprehensive_tests.rs"]
mod comprehensive_tests;

#[path = "error_reporting_tests.rs"]
mod error_reporting_tests;

#[path = "spec_adherence_tests.rs"]
mod spec_adherence_tests;

#[path = "temp_unescape_test.rs"]
mod temp_unescape_test;

mod inc;
