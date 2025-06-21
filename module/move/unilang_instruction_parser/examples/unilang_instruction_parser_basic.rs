//! Basic usage example for the `unilang_instruction_parser` crate.
//!
//! This example demonstrates:
//! - Creating a `Parser` with default options.
//! - Parsing a single complex instruction string.
//! - Parsing multiple instructions from a slice.
//! - Printing the parsed `GenericInstruction` objects.

use unilang_instruction_parser::{Parser, UnilangParserOptions};

fn main() {
    // 1. Create a parser with default options
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);

    // 2. Parse a single complex instruction string
    let input_single = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";
    println!("--- Parsing Single Instruction: \"{}\" ---", input_single);

    let instructions_single = parser.parse_single_str(input_single)
        .expect("Failed to parse single instruction");

    for instruction in instructions_single {
        println!("  Parsed Instruction: {:?}", instruction);
    }

    // 3. Parse multiple instructions from a slice
    let input_slice: &[&str] = &[
        "system.info ?",
        "file.read path::\"/etc/hosts\" --binary",
        "user.add 'John Doe' email::john.doe@example.com"
    ];
    println!("\n--- Parsing Multiple Instructions from Slice: {:?} ---", input_slice);

    let instructions_slice = parser.parse_slice(input_slice)
        .expect("Failed to parse slice instructions");

    for instruction in instructions_slice {
        println!("  Parsed Instruction: {:?}", instruction);
    }
}