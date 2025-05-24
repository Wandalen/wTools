# `unilang_instruction_parser`

A Rust crate for parsing CLI-like instruction strings into structured `GenericInstruction` objects, providing a configurable parser with detailed error reporting.

## Features

*   **Command Paths**: Supports single/multi-segment paths (e.g., `cmd.sub`, `path/to/cmd`).
*   **Arguments**: Parses positional and named arguments (`name::value`).
*   **Quoting & Escaping**: Handles quoted values (`"val"`, `'val'`) and standard escape sequences.
*   **Help Operator**: Recognizes `?` for help requests.
*   **Multiple Instructions**: Parses `;;`-separated instructions.
*   **Error Reporting**: Provides `ParseError` with `ErrorKind` and `SourceLocation`.
*   **Configurable**: Customizes parsing rules via `UnilangParserOptions`.
*   **`no_std` Support**: Available via a feature flag.

## Installation

Add `unilang_instruction_parser` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
unilang_instruction_parser = { path = "path/to/unilang_instruction_parser" } # Or version = "x.y.z" if published
```

(Adjust the path or version as necessary.)

## Basic Usage

```rust
use unilang_instruction_parser::{Parser, UnilangParserOptions};

let options = UnilangParserOptions::default();
let parser = Parser::new(options);
let input = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";

match parser.parse_single_str(input) {
    Ok(instructions) => {
        for instruction in instructions {
            println!("Parsed Instruction: {:?}", instruction);
            // Access instruction.command_path_slices, instruction.named_arguments, etc.
        }
    },
    Err(e) => {
        eprintln!("Parse error: {}", e);
    },
}
```

## Specification

This parser aims to strictly adhere to the (conceptual) `unilang` command language specification, which would typically be detailed in a document like `unilang/spec.md`.

## License

This crate is licensed under the terms of the [Apache License 2.0](LICENSE) or the [MIT License](LICENSE), at your option.
