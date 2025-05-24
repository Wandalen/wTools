# `unilang_instruction_parser`

`unilang_instruction_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings. It transforms raw string input into structured `GenericInstruction` objects, which represent a command and its associated arguments. The parser is built to be robust, provide detailed error reporting with source locations, and is configurable.

This parser is intended to be a core component for any application that needs to interpret `unilang` command syntax, as specified in `unilang/spec.md` (conceptual).

## Features

*   **Command Path Parsing**: Handles single or multi-segment command paths (e.g., `command.sub_command`).
*   **Argument Types**: Supports positional arguments and named arguments (e.g., `name::value`).
*   **Quoting & Escaping**: Parses quoted values (`"value with spaces"`, `'another value'`) and handles standard escape sequences (`\\`, `\"`, `\'`, `\n`, `\t`) within them.
*   **Help Operator**: Recognizes the `?` operator for requesting help on a command.
*   **Multiple Instructions**: Can parse multiple instructions separated by `;;` from a single input.
*   **Detailed Error Reporting**: Provides `ParseError` with `ErrorKind` and `SourceLocation` to pinpoint syntax issues in the input.
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
use unilang_instruction_parser::{Parser, UnilangParserOptions, GenericInstruction, Argument, SourceLocation, ParseError};

fn main() -> Result<(), ParseError> {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let input = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose ;; system.info ?";

    match parser.parse_single_str(input) {
        Ok(instructions) => {
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
        }
        Err(e) => {
            eprintln!("Failed to parse input: {}", e);
            if let Some(location) = e.location {
                eprintln!("Error location: {:?}", location);
                // Example: Highlighting the error in the original input (simplified)
                // This requires access to the original input string and logic to map SourceLocation
                // (StrSpan or SliceSegment) back to the string.
                match location {
                    SourceLocation::StrSpan { start, end } => {
                        if end <= input.len() {
                            eprintln!("Problematic part: \"{}\"", &input[start..end]);
                        }
                    }
                    SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => {
                        // For slice input, you'd need the original slice segments.
                        eprintln!("Problem in segment {}, bytes {}-{}", segment_index, start_in_segment, end_in_segment);
                    }
                }
            }
        }
    }

    Ok(())
}
```

## Specification

This parser aims to strictly adhere to the (conceptual) `unilang` command language specification, which would typically be detailed in a document like `unilang/spec.md`. Key aspects include the structure of commands, argument types, quoting rules, and error conditions.

## License

This crate is licensed under the terms of the [Apache License 2.0](LICENSE) or the [MIT License](LICENSE), at your option.
