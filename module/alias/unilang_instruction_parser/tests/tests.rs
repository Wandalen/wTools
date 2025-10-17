//! Test reuse for `unilang_instruction_parser` alias crate.
//!
//! This alias crate inherits all tests from the core `unilang_parser` implementation.
//! Following the wTools test reuse pattern used by `meta_tools` and `test_tools`.

#[ allow(unused_imports) ]
use unilang_instruction_parser as the_module;
#[ allow(unused_imports) ]
use test_tools :: *;

// Include all test modules from the core unilang_parser crate using full module path
#[ path = "../../../../module/move/unilang_parser/tests/parser_config_entry_tests.rs" ]
mod parser_config_entry_tests;

#[ path = "../../../../module/move/unilang_parser/tests/command_parsing_tests.rs" ]
mod command_parsing_tests;

#[ path = "../../../../module/move/unilang_parser/tests/syntactic_analyzer_command_tests.rs" ]
mod syntactic_analyzer_command_tests;

#[ path = "../../../../module/move/unilang_parser/tests/argument_parsing_tests.rs" ]
mod argument_parsing_tests;

#[ path = "../../../../module/move/unilang_parser/tests/comprehensive_tests.rs" ]
mod comprehensive_tests;

#[ path = "../../../../module/move/unilang_parser/tests/error_reporting_tests.rs" ]
mod error_reporting_tests;

#[ path = "../../../../module/move/unilang_parser/tests/spec_adherence_tests.rs" ]
mod spec_adherence_tests;
