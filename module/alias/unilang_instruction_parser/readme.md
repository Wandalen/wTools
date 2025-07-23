# unilang_instruction_parser

[![Crates.io](https://img.shields.io/crates/v/unilang_instruction_parser.svg)](https://crates.io/crates/unilang_instruction_parser)
[![Documentation](https://docs.rs/unilang_instruction_parser/badge.svg)](https://docs.rs/unilang_instruction_parser)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Alias crate for `unilang_parser`. Re-exports `unilang_parser` for backward compatibility.

## overview

This crate serves as a compatibility alias for the core `unilang_parser` library, which provides syntactic analysis for CLI-like instruction strings within the Unilang Framework. It enables parsing of command strings into structured `GenericInstruction` objects.

## key_features

- **command_path_parsing**: Multi-segment command paths (`namespace.command`)
- **argument_processing**: Both positional and named arguments (`key::value`)
- **quoting_support**: Single and double quotes with escape sequences
- **help_operator**: Built-in `?` help request handling
- **multiple_instructions**: Sequence parsing with `;;` separator
- **robust_error_reporting**: Detailed parse errors with source locations

## usage

```rust
use unilang_instruction_parser::{Parser, UnilangParserOptions};

let parser = Parser::new(UnilangParserOptions::default());
let input = "log.level severity::debug message::'Hello World!'";

match parser.parse_single_instruction(input) {
    Ok(instruction) => {
        println!("Command: {:?}", instruction.command_path_slices);
        println!("Named args: {:?}", instruction.named_arguments);
    },
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## migration_notice

This is an alias crate that re-exports `unilang_parser`. For new projects, consider using `unilang_parser` directly. This crate exists to maintain backward compatibility for existing code.

## documentation

For complete documentation and examples, see:
- [api_documentation](https://docs.rs/unilang_instruction_parser)
- [core_parser_documentation](https://docs.rs/unilang_parser)

## license

MIT License. See LICENSE file for details.