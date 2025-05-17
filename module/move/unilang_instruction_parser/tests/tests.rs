// Main test harness for unilang_instruction_parser

// Individual test files are included as modules
#[path = "parser_config_entry_tests.rs"]
mod parser_config_entry_tests;

// Add other test modules here as they are created, e.g.:
#[path = "syntactic_analyzer_command_tests.rs"]
mod syntactic_analyzer_command_tests;

#[path = "argument_parsing_tests.rs"]
mod argument_parsing_tests;
