//! Basic usage example for the `unilang_instruction_parser` crate.
//!
//! This example demonstrates:
//! 1. Creating a parser with default options.
//! 2. Parsing a string containing multiple instructions.
//! 3. Iterating through parsed instructions and their components.
//! 4. Basic error handling for parse failures.

use unilang_instruction_parser::{
    Argument, ErrorKind, GenericInstruction, ParseError, Parser, SourceLocation, UnilangParserOptions,
};

fn main() -> Result<(), ParseError> {
    // 1. Create a parser with default options.
    let options = UnilangParserOptions::default();
    let parser = Parser::new(options);

    // 2. Define an input string with multiple instructions and various features.
    let input = r#"
        system.info --verbose ;;
        file.copy path::"source dir/file.txt" target::"/dest/dir/file.txt" ;;
        user.add name::'John "The Admin" Doe' age::30 roles::"admin,user" ;;
        config.set key::my.setting value::"complex \"value\" with escapes \\n and \\t" ;;
        broken.command name_only_no_delimiter_then_value ;;
        another.cmd ?
    "#;

    println!("Parsing input string:\n{}\n", input.trim());

    // 3. Parse the input string.
    let instructions_result = parser.parse_single_str(input);

    match instructions_result {
        Ok(instructions) => {
            println!("Successfully parsed {} instructions:\n", instructions.len());
            for (i, instruction) in instructions.iter().enumerate() {
                println!("--- Instruction #{} ---", i + 1);
                print_instruction_details(instruction);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse input string fully due to an error in one of the instructions.");
            handle_parse_error(&e, input); // Pass original input for context if needed
            return Err(e); // Propagate the error
        }
    }

    println!("\n--- Demonstrating Error Handling ---");
    // 4. Demonstrate parsing an input that causes a ParseError.
    let error_input = "cmd name_only_no_delimiter then_value ::trailing_delimiter";
    println!("\nParsing potentially erroneous input: '{}'", error_input);
    match parser.parse_single_str(error_input) {
        Ok(instrs) => {
            println!(
                "Error demonstration unexpectedly parsed OK. Parsed {} instructions.",
                instrs.len()
            );
            for (i, instruction) in instrs.iter().enumerate() {
                println!("--- Erroneous Input - Instruction #{} ---", i + 1);
                print_instruction_details(instruction);
            }
        }
        Err(e) => {
            println!("Successfully caught expected parse error for input '{}':", error_input);
            handle_parse_error(&e, error_input);
        }
    }

    let error_input_invalid_escape = "cmd arg::\"bad\\xescape\"";
    println!("\nParsing input with invalid escape: '{}'", error_input_invalid_escape);
    match parser.parse_single_str(error_input_invalid_escape) {
        Ok(instrs) => {
             println!(
                "Error demonstration for invalid escape unexpectedly parsed OK. Parsed {} instructions.",
                instrs.len()
            );
        }
        Err(e) => {
            println!("Successfully caught expected parse error for input '{}':", error_input_invalid_escape);
            handle_parse_error(&e, error_input_invalid_escape);
        }
    }


    Ok(())
}

/// Helper function to print details of a GenericInstruction.
fn print_instruction_details(instruction: &GenericInstruction) {
    println!("  Command Path: {:?}", instruction.command_path_slices);
    println!("  Overall Location: {:?}", instruction.overall_location);

    if instruction.help_requested {
        println!("  Help Requested: Yes");
    }

    if !instruction.positional_arguments.is_empty() {
        println!("  Positional Arguments:");
        for (idx, pos_arg) in instruction.positional_arguments.iter().enumerate() {
            println!(
                "    {}: Value: '{}', Location: {:?}",
                idx, pos_arg.value, pos_arg.value_location
            );
        }
    }

    if !instruction.named_arguments.is_empty() {
        println!("  Named Arguments:");
        for (name, named_arg) in &instruction.named_arguments {
            println!(
                "    {}: Value: '{}', Name Loc: {:?}, Value Loc: {:?}",
                name,
                named_arg.value,
                named_arg.name_location,
                named_arg.value_location
            );
        }
    }
}

/// Helper function to print ParseError details.
fn handle_parse_error(error: &ParseError, original_input_for_context: &str) {
    eprintln!("  Error Kind: {:?}", error.kind);
    if let Some(location) = &error.location {
        eprintln!("  Location: {:?}", location);
        // Example of how to use location to show context (simplified)
        match location {
            SourceLocation::StrSpan { start, end } => {
                let s = std::cmp::max(0, *start as isize -10) as usize;
                let e = std::cmp::min(original_input_for_context.len(), *end + 10);
                let context_start = original_input_for_context.get(s..*start).unwrap_or("...");
                let error_span = original_input_for_context.get(*start..*end).unwrap_or("ERROR");
                let context_end = original_input_for_context.get(*end..e).unwrap_or("...");
                eprintln!("  Context: {}{}{}", context_start, error_span, context_end);
                eprintln!("           {}^-- HERE", " ".repeat(context_start.chars().count()));

            }
            SourceLocation::SliceSegment { segment_index, start_in_segment, end_in_segment } => {
                // For slice segment, you'd need access to the original input_segments array
                // to provide similar context. This example doesn't have it directly.
                eprintln!("  (Error in input slice segment {}, bytes {}-{})", segment_index, start_in_segment, end_in_segment);
            }
        }
    } else {
        eprintln!("  Location: Not available");
    }
    eprintln!("  Full Error: {}", error);
}