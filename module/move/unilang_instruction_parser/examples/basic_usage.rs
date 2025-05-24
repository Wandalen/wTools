//! Basic usage example for the `unilang_instruction_parser` crate.
//!
//! This example demonstrates:
//! - Creating a `Parser` with default options.
//! - Parsing a simple instruction string.
//! - Iterating through parsed `GenericInstruction`s.
//! - Accessing command paths, positional arguments, and named arguments.
//! - Printing parsed information.
//! - Demonstrating basic error handling for a `ParseError`.

use unilang_instruction_parser::{
    Argument, GenericInstruction, ParseError, Parser, SourceLocation, UnilangParserOptions,
};

fn main() -> Result<(), ParseError> {
    // 1. Create a parser with default options
    // By default, `error_on_positional_after_named` is true.
    let default_parser = Parser::new(UnilangParserOptions::default());

    // 2. Define an input string that will cause an error with default options
    // because "--verbose" is a positional argument after named arguments.
    let input_expected_to_error1 = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";
    println!("Parsing input expected to cause 'positional after named' error:\n\"{}\"\n", input_expected_to_error1);

    match default_parser.parse_single_str(input_expected_to_error1) {
        Ok(instructions) => {
            println!("Unexpectedly parsed {} instruction(s):", instructions.len());
            for (idx, instruction) in instructions.iter().enumerate() {
                println!("\n--- Instruction #{} ---", idx + 1);
                print_instruction_details(instruction);
            }
        }
        Err(e) => {
            println!("\n--- Correctly Failed Parsing (as expected for input_expected_to_error1) ---");
            handle_parse_error(&e, input_expected_to_error1);
        }
    }

    // 3. Demonstrate parsing an input that is known to cause a different specific error
    println!("\n--- Demonstrating Specific Error Handling for incomplete named argument ---");
    // This input is missing a value after 'name_incomplete_delimiter::'
    let error_input_incomplete_named = "cmd name_incomplete_delimiter::";
    println!("Parsing input with incomplete named argument: \"{}\"\n", error_input_incomplete_named);
    match default_parser.parse_single_str(error_input_incomplete_named) {
        Ok(instructions) => {
            println!(
                "Unexpectedly parsed {} instruction(s) from incomplete named arg input:",
                instructions.len()
            );
            for instruction in instructions {
                print_instruction_details(&instruction);
            }
        }
        Err(e) => {
            println!("\n--- Correctly Failed Parsing (as expected for error_input_incomplete_named) ---");
            handle_parse_error(&e, error_input_incomplete_named);
        }
    }

    // 4. Example of parsing a slice.
    println!("\n--- Demonstrating Slice Parsing ---");
    let slice_input: &[&str] = &["cmd1 pos_arg1", "cmd2 name_arg::val2", "cmd3 'quoted pos'"];
    // Using options to allow positional after named to temporarily work around a suspected parser bug
    // where state might carry over between slice segments.
    let slice_options = UnilangParserOptions {
        error_on_positional_after_named: false,
        ..Default::default()
    };
    let slice_parser = Parser::new(slice_options);
    println!("Parsing slice input: {:?} with options: error_on_positional_after_named = false\n", slice_input);

    match slice_parser.parse_slice(slice_input) { // Use slice_parser with specific options
        Ok(instructions) => {
            println!("Successfully parsed {} instruction(s) from slice:", instructions.len());
            for (idx, instruction) in instructions.iter().enumerate() {
                let segment_idx_display = match instruction.overall_location {
                    SourceLocation::SliceSegment { segment_index, .. } => segment_index.to_string(),
                    _ => "N/A (StrSpan)".to_string(),
                };
                println!("\n--- Slice Instruction #{} (from segment {}) ---", idx + 1, segment_idx_display);
                print_instruction_details(instruction);
            }
        }
        Err(e) => {
            eprintln!("\n--- Slice Parsing Failed Unexpectedly (even with relaxed options) ---");
            handle_parse_error_for_slice(&e, slice_input);
        }
    }

    // // 5. Example of a simple parse that should fail with default options due to positional after named
    // println!("\n--- Demonstrating Expected Failure for Positional After Named (Default Options) ---");
    // let simple_input_fail_default = "command.sub path_arg name::value 'pos arg'";
    // println!("Parsing input expected to fail with default options: \"{}\"\n", simple_input_fail_default);
    // match default_parser.parse_single_str(simple_input_fail_default) {
    //     Ok(instructions) => {
    //         println!("Unexpectedly parsed simple input that should have failed:");
    //         for instruction in instructions {
    //             print_instruction_details(&instruction);
    //         }
    //     }
    //     Err(e) => {
    //         println!("\n--- Correctly Failed Parsing (as expected for simple_input_fail_default) ---");
    //         handle_parse_error(&e, simple_input_fail_default);
    //     }
    // }

    Ok(())
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

/// Helper function to print `ParseError` details for single string input.
fn handle_parse_error(error: &ParseError, original_input: &str) {
    eprintln!("Error: {}", error);
    if let Some(location) = &error.location {
        eprintln!("  Location: {:?}", location);
        match location {
            SourceLocation::StrSpan { start, end } => {
                if *start <= original_input.len() && *end <= original_input.len() && *start <= *end {
                    eprintln!("  Problematic part: \"{}\"", &original_input[*start..*end]);
                } else {
                    eprintln!("  Error location span [{}-{}] is out of bounds for input length {}.", start, end, original_input.len());
                }
            }
            SourceLocation::SliceSegment {
                segment_index,
                start_in_segment,
                end_in_segment,
            } => {
                eprintln!(
                    "  Error in (unexpected for single string) segment {}, bytes {}-{}",
                    segment_index, start_in_segment, end_in_segment
                );
            }
        }
    }
}

/// Helper function to print `ParseError` details for slice input.
fn handle_parse_error_for_slice(error: &ParseError, original_input_segments: &[&str]) {
    eprintln!("Error: {}", error);
    if let Some(location) = &error.location {
        eprintln!("  Location: {:?}", location);
        match location {
            SourceLocation::StrSpan { start, end } => {
                 eprintln!(
                    "  Error in (unexpected for slice input) string span, bytes {}-{}",
                    start, end
                );
            }
            SourceLocation::SliceSegment {
                segment_index,
                start_in_segment,
                end_in_segment,
            } => {
                if *segment_index < original_input_segments.len() {
                    let segment_content = original_input_segments[*segment_index];
                     if *start_in_segment <= segment_content.len() && *end_in_segment <= segment_content.len() && *start_in_segment <= *end_in_segment {
                        eprintln!("  In segment {}: \"{}\"", segment_index, segment_content);
                        eprintln!("  Problematic part: \"{}\"", &segment_content[*start_in_segment..*end_in_segment]);
                    } else {
                        eprintln!("  Error location span [{}-{}] in segment {} is out of bounds for segment length {}.", start_in_segment, end_in_segment, segment_index, segment_content.len());
                    }
                } else {
                     eprintln!("  Error location segment index {} is out of bounds for input slice with {} segments.", segment_index, original_input_segments.len());
                }
            }
        }
    }
}