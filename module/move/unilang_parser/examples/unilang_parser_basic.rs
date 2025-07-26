//! Basic usage example for the `unilang_parser` crate.
//!
//! This example demonstrates:
//! - Creating a `Parser` with default options.
//! - Parsing a single complex instruction string.
//! - Printing the parsed `GenericInstruction` objects.

use unilang_parser::{Parser, UnilangParserOptions};

fn main() {
  // 1. Create a parser with default options
  let options = UnilangParserOptions::default();
  let parser = Parser::new(options); // Use new_with_options for custom options

  // 2. Parse a single complex instruction string
  let input_single = "log.level severity::\"debug\" message::'Hello, Unilang!' --verbose";
  println!("--- Parsing Single Instruction: \"{}\" ---", input_single);

  let instruction_single = parser.parse_single_instruction(input_single) // Renamed and returns single instruction
        .expect("Failed to parse single instruction");

  println!("  Parsed Instruction: {:?}", instruction_single);

  // 3. Parse multiple instructions from a string with ';;' delimiter
  // Note: The `parse_slice` method is no longer available.
  // To parse multiple instructions, use `parse_single_instruction` on a string
  // containing `;;` delimiters, which will return a Vec<GenericInstruction>.
  let input_multiple = "system.info ?;;file.read path::\"/etc/hosts\" --binary;;user.add 'John Doe' email::john.doe@example.com";
  println!(
    "\n--- Parsing Multiple Instructions from String with ';;': \"{}\" ---",
    input_multiple
  );

  let instructions_multiple = parser
    .parse_multiple_instructions(input_multiple)
    .expect("Failed to parse multiple instructions");

  for instruction in instructions_multiple {
    println!("  Parsed Instruction: {:?}", instruction);
  }
}
