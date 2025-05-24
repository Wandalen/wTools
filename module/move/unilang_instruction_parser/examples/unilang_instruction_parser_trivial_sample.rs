//! Basic usage example for the `unilang_instruction_parser` crate.
//!
//! This example demonstrates:
//! - Creating a `Parser` with custom options.
//! - Parsing a complex instruction string with command paths, positional, and named arguments.
//! - Parsing multiple instructions from a slice.
//! - Accessing parsed instruction details.

use unilang_instruction_parser::{
    Argument, GenericInstruction, Parser, SourceLocation, UnilangParserOptions,
};

fn main() {
    // 1. Create a parser with custom options
    // Set `error_on_positional_after_named` to false to allow positional arguments after named ones.
    let options = UnilangParserOptions {
        error_on_positional_after_named: false,
        ..Default::default()
    };
    let parser = Parser::new(options);

    // 2. Parse a single complex instruction string
    let input_single = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";
    println!("--- Parsing Single Instruction: \"{}\" ---", input_single);

    let instructions_single = parser.parse_single_str(input_single)
        .expect("Failed to parse single instruction");

    for instruction in instructions_single {
        print_instruction_details(&instruction);
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

    for (idx, instruction) in instructions_slice.iter().enumerate() {
        println!("\n--- Instruction #{} (from segment {}) ---", idx + 1,
            match instruction.overall_location {
                SourceLocation::SliceSegment { segment_index, .. } => segment_index.to_string(),
                _ => "N/A (StrSpan)".to_string(), // Should not happen for slice input
            }
        );
        print_instruction_details(instruction);
    }
}

/// Helper function to print details of a `GenericInstruction`.
fn print_instruction_details(instruction: &GenericInstruction) {
    println!("  Command Path: {:?}", instruction.command_path_slices);
    println!("  Overall Location: {:?}", instruction.overall_location);

    if instruction.help_requested {
        println!("  Help Requested: Yes");
    }

    if !instruction.positional_arguments.is_empty() {
        println!("  Positional Arguments:");
        for arg in &instruction.positional_arguments {
            print_argument_details(arg, "    ");
        }
    }

    if !instruction.named_arguments.is_empty() {
        println!("  Named Arguments:");
        for (name, arg) in &instruction.named_arguments {
            println!("    Name: \"{}\"", name);
            print_argument_details(arg, "      ");
        }
    }
}

/// Helper function to print details of an `Argument`.
fn print_argument_details(arg: &Argument, prefix: &str) {
    if let Some(name_loc) = &arg.name_location {
        println!("{}  Name Location: {:?}", prefix, name_loc);
    }
    println!("{}  Value: \"{}\"", prefix, arg.value);
    println!("{}  Value Location: {:?}", prefix, arg.value_location);
}