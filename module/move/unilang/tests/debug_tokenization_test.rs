//! Debug test to understand tokenization

use unilang_parser::*;

#[test]
fn debug_tokenization()
{
  let parser = Parser::new(UnilangParserOptions::default());
  let input = ".run_file file::./examples/rust_learning.yaml";
  
  println!("Input: {}", input);
  
  let result = parser.parse_single_instruction(input);
  match result {
    Ok(instruction) => {
      println!("Successfully parsed!");
      println!("Command path: {:?}", instruction.command_path_slices);
      println!("Named args: {:?}", instruction.named_arguments.keys().collect::<Vec<_>>());
      for (k, v) in &instruction.named_arguments {
        println!("  {}: '{}'", k, v.value);
      }
      println!("Positional args: {:?}", instruction.positional_arguments);
    }
    Err(e) => {
      println!("Parse error: {:?}", e);
    }
  }
}