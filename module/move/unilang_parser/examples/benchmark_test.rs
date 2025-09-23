//! Simple benchmark test for zero-copy token parsing.

use std ::time ::Instant;
use unilang_parser :: { Parser, UnilangParserOptions };

fn main()
{
  println!( "=== unilang_parser Zero-Copy Benchmark ===" );
  
  let test_input = "command.sub_command arg1 ::value1 arg2 \"quoted value\" 123 end?";
  let parser = Parser ::new( UnilangParserOptions ::default() );
  
  // Warmup
  for _ in 0..1000
  {
  let _ = parser.parse_single_instruction( test_input );
 }
  
  // Benchmark current implementation
  let start = Instant ::now();
  let iterations = 10000;
  
  for _ in 0..iterations
  {
  let result = parser.parse_single_instruction( test_input );
  assert!( result.is_ok() );
 }
  
  let elapsed = start.elapsed();
  let avg_time = elapsed / iterations;
  
  println!( "Test input: {test_input}" );
  println!( "Iterations: {iterations}" );
  println!( "Total time: {elapsed:?}" );
  println!( "Average time per parse: {avg_time:?}" );
  println!( "Parsing rate: {:.0} commands/sec", 1_000_000_000.0 / avg_time.as_nanos() as f64 );
  
  // Test the instruction result
  let result = parser.parse_single_instruction( test_input ).unwrap();
  println!( "\nParsed instruction: " );
  println!( "  Command path: {:?}", result.command_path_slices );
  println!( "  Named args: {}", result.named_arguments.len() );
  println!( "  Positional args: {}", result.positional_arguments.len() );
  println!( "  Help requested: {}", result.help_requested );
}