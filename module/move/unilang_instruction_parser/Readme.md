# `unilang_instruction_parser`

`unilang_instruction_parser` is a Rust crate for parsing `unilang` CLI-like instruction strings into structured `GenericInstruction` objects. It provides a robust and configurable parser with detailed error reporting.

## Features

*   **Command Path Parsing**: Handles single or multi-segment command paths, including `.` and `/` as path separators (e.g., `command.sub.command`, `path/to/cmd`).
*   **Argument Types**: Supports positional arguments and named arguments (e.g., `name::value`).
*   **Quoting & Escaping**: Parses quoted values (`"value with spaces"`, `'another value'`) and handles standard escape sequences (`\\`, `\"`, `\'`, `\n`, `\t`).
*   **Help Operator**: Recognizes the `?` operator for requesting help on a command.
*   **Multiple Instructions**: Parses multiple instructions separated by `;;` from a single input.
*   **Detailed Error Reporting**: Provides `ParseError` with `ErrorKind` and `SourceLocation` to pinpoint syntax issues.
*   **Configurable Behavior**: Allows customization of parsing rules via `UnilangParserOptions` (e.g., behavior for duplicate named arguments, allowing positional arguments after named ones).
*   **`no_std` Support**: Can be used in `no_std` environments via a feature flag.

## Installation

Add `unilang_instruction_parser` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
unilang_instruction_parser = { path = "path/to/unilang_instruction_parser" } # Or version = "x.y.z" if published
```

(Adjust the path or version as necessary.)

## Basic Usage

```rust
use unilang_instruction_parser::{Parser, UnilangParserOptions, GenericInstruction, Argument, SourceLocation};

let options = UnilangParserOptions { error_on_positional_after_named: false, ..Default::default() };
let parser = Parser::new(options);
let input = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose ;; system.info ?";

let instructions = parser.parse_single_str(input).expect("Failed to parse valid input");

for instruction in instructions {
    println!("Command Path: {:?}", instruction.command_path_slices);

    if instruction.help_requested {
        println!("Help was requested for this command.");
    }

    println!("Positional Arguments:");
    for pos_arg in &instruction.positional_arguments {
        println!("  - Value: '{}' (at {:?})", pos_arg.value, pos_arg.value_location);
    }

    println!("Named Arguments:");
    for (name, named_arg) in &instruction.named_arguments {
        println!("  - {}: '{}' (name at {:?}, value at {:?})",
            name,
            named_arg.value,
            named_arg.name_location,
            named_arg.value_location
        );
    }
    println!("---");
}

// For error handling, you would typically use a match statement:
// match parser.parse_single_str("invalid input") {
//     Ok(_) => { /* handle success */ },
//     Err(e) => { eprintln!("Parse error: {}", e); },
// }
```

## Specification

This parser aims to strictly adhere to the (conceptual) `unilang` command language specification, which would typically be detailed in a document like `unilang/spec.md`. Key aspects include the structure of commands, argument types, quoting rules, and error conditions.

## License

This crate is licensed under the terms of the [Apache License 2.0](LICENSE) or the [MIT License](LICENSE), at your option.
