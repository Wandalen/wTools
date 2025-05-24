<!-- {{# generate.module_header{} #}} -->

# Module :: unilang_instruction_parser
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_instruction_parser_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_unilang_instruction_parser_push.yml) [![docs.rs](https://img.shields.io/docsrs/unilang_instruction_parser?color=e3e8f0&logo=docs.rs)](https://docs.rs/unilang_instruction_parser) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fmove%2Funilang_instruction_parser%2Fexamples%2Funilang_instruction_parser_trivial.rs,RUN_POSTFIX=--example%20module%2Fmove%2Funilang_instruction_parser%2Fexamples%2Funilang_instruction_parser_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

`unilang_instruction_parser` is a Rust crate designed to parse `unilang` CLI-like instruction strings. It transforms raw text input into structured `GenericInstruction` objects, capable of handling complex command paths, named and positional arguments, quoted strings with escapes, and provides detailed, location-aware error reporting.

The parser is configurable and aims to adhere to the (hypothetical) `unilang/spec.md` for syntax rules.

## Key Features

*   **Structured Output**: Parses input into `Vec<GenericInstruction>`, where each instruction contains:
    *   `command_path_slices`: A `Vec<String>` for multi-segment command paths (e.g., `git remote add`).
    *   `positional_arguments`: A `Vec<Argument>` for ordered arguments.
    *   `named_arguments`: A `HashMap<String, Argument>` for arguments like `name::value`.
    *   `help_requested`: A boolean flag for the `?` operator.
    *   `overall_location`: A `SourceLocation` spanning the entire instruction.
*   **Argument Types**: Handles unquoted, double-quoted (`"`), and single-quoted (`'`) arguments.
*   **Escape Sequences**: Supports common escapes (`\\`, `\"`, `\'`, `\n`, `\t`) within quoted strings and reports errors for invalid sequences.
*   **Instruction Separation**: Parses multiple instructions separated by `;;`.
*   **Configurable Behavior**: `UnilangParserOptions` allows customization, such as:
    *   Error handling for duplicate named arguments.
    *   Rules for positional arguments appearing after named arguments.
    *   Definition of quote pairs and primary delimiters.
*   **Detailed Error Reporting**: `ParseError` provides an `ErrorKind` and an optional `SourceLocation` to pinpoint syntax issues in the input string or slice segments.
*   **`no_std` Support**: Can be used in `no_std` environments via a feature flag.

## Basic Usage

```rust
use unilang_instruction_parser::{Parser, UnilangParserOptions, GenericInstruction, Argument, SourceLocation, ParseError};

fn main() -> Result<(), ParseError> {
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);
    let input = "module.install path::\"C:/Program Files/My App\" version::1.2.3 --force ;; list.items --sort name";

    let instructions = parser.parse_single_str(input)?;

    for instruction in instructions {
        println!("Command Path: {:?}", instruction.command_path_slices);

        if instruction.help_requested {
            println!("Help was requested for this command.");
        }

        println!("Positional Arguments:");
        for pos_arg in &instruction.positional_arguments { // Added &
            println!("  - Value: '{}' (at {:?})", pos_arg.value, pos_arg.value_location);
        }

        println!("Named Arguments:");
        for (name, named_arg) in &instruction.named_arguments { // Added &
            println!("  - {}: '{}' (name at {:?}, value at {:?})",
                name,
                named_arg.value,
                named_arg.name_location,
                named_arg.value_location
            );
        }
        println!("---");
    }
    Ok(())
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unilang_instruction_parser = "0.1.0" # Replace with the desired version
```
Or use `cargo add`:
```sh
cargo add unilang_instruction_parser
```

## Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
# To run the example (once created in examples/basic_usage.rs):
# cargo run --example basic_usage -p unilang_instruction_parser
```
(Note: The `trivial` example mentioned in the original boilerplate might need to be updated or replaced by `basic_usage.rs` as planned in Increment 8.)

<!-- {{# generate.module_footer{} #}} -->
