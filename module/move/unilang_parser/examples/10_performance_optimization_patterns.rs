//! Performance Optimization Patterns Example
//!
//! This example demonstrates:
//! - Parser instance reuse for better performance
//! - Efficient batch processing techniques
//! - Memory usage optimization patterns
//! - Performance measurement examples

use unilang_parser::{ Parser, UnilangParserOptions };
use std::time::Instant;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "=== Performance Optimization Patterns ===" );

  // Pattern 1: Reuse parser instance for better performance
  println!( "\n1. Parser Instance Reuse:" );
  let parser = Parser::new( UnilangParserOptions::default() );

  let commands = vec!
  [
    "system.status",
    "user.list active::true",
    "report.generate format::pdf output::\"/tmp/report.pdf\"",
    "backup.create name::daily compress::true",
    "notify.send \"Operation complete\" priority::high",
    "log.rotate max_files::10 max_size::100MB",
    "cache.clear namespace::user_data",
    "service.restart name::web_server graceful::true",
    "db.optimize table::users analyze::true",
    "monitoring.check service::all alert::true",
  ];

  let start = Instant::now();
  let mut successful_parses = 0;
  let mut _total_instructions = 0;

  for cmd_str in &commands
  {
    match parser.parse_single_instruction( cmd_str )
    {
      Ok( instruction ) =>
      {
        successful_parses += 1;
        _total_instructions += 1;

        // Process instruction efficiently
        let command_name = instruction.command_path_slices.join( "." );
        let arg_count = instruction.positional_arguments.len() + instruction.named_arguments.len();

        if successful_parses <= 3
        { // Only print first few for brevity
          println!( "  ✓ {}: {} args", command_name, arg_count );
        }
      },
      Err( e ) =>
      {
        eprintln!( "  ✗ Parse error in '{}': {}", cmd_str, e );
      }
    }
  }

  let duration = start.elapsed();
  println!
  (
    "  Processed {} commands in {:?} ({:.2} μs/command)",
    successful_parses,
    duration,
    duration.as_micros() as f64 / successful_parses as f64
  );

  // Pattern 2: Batch processing with pre-validation
  println!( "\n2. Efficient Batch Processing:" );

  // Pre-validate commands before processing
  let batch_input = "user.create name::alice email::alice@test.com ;; \
                       user.update id::123 name::\"Alice Smith\" ;; \
                       user.delete id::456 ;; \
                       user.list active::true limit::50";

  let batch_start = Instant::now();
  match parser.parse_multiple_instructions( batch_input )
  {
    Ok( instructions ) =>
    {
      let parse_duration = batch_start.elapsed();
      println!( "  Parsed {} instructions in {:?}", instructions.len(), parse_duration );

      // Process with minimal allocations
      let process_start = Instant::now();
      for ( i, instruction ) in instructions.iter().enumerate()
      {
        // Simulate processing without unnecessary allocations
        let command_segments = &instruction.command_path_slices;
        let arg_count = instruction.positional_arguments.len() + instruction.named_arguments.len();

        if i < 2
        { // Only print first couple
          println!
          (
            "    Instruction {}: {:?} ({} args)",
            i + 1,
            command_segments,
            arg_count
          );
        }
      }
      let process_duration = process_start.elapsed();
      println!( "  Processed in {:?} (total: {:?})", process_duration, parse_duration + process_duration );
    }
    Err( e ) => eprintln!( "  Batch parse error: {}", e ),
  }

  // Pattern 3: Memory-efficient streaming for large inputs
  println!( "\n3. Memory-Efficient Processing:" );

  // Simulate processing large number of commands without storing all results
  let large_command_set = vec!
  [
    "log.write level::info message::\"System started\"",
    "metrics.record cpu::85.2 memory::67.8 disk::45.1",
    "alert.check threshold::95 service::database",
    "backup.verify checksum::abc123 size::1024MB",
    "security.scan type::vulnerability target::web_app",
  ];

  let streaming_start = Instant::now();
  let mut processed_count = 0;
  let mut total_args = 0;

  // Process one at a time to minimize memory usage
  for cmd in large_command_set.iter().cycle().take( 1000 )
  {
    match parser.parse_single_instruction( cmd )
    {
      Ok( instruction ) =>
      {
        processed_count += 1;
        total_args += instruction.positional_arguments.len() + instruction.named_arguments.len();

        // Process immediately without storing
        // In real application, you'd execute the command here
      }
      Err( _ ) =>
      {
        // Handle error without breaking the stream
        continue;
      }
    }
  }

  let streaming_duration = streaming_start.elapsed();
  println!
  (
    "  Streamed {} commands in {:?} ({:.2} μs/command)",
    processed_count,
    streaming_duration,
    streaming_duration.as_micros() as f64 / processed_count as f64
  );
  println!
  (
    "  Average arguments per command: {:.1}",
    total_args as f64 / processed_count as f64
  );

  // Pattern 4: Error handling optimization
  println!( "\n4. Optimized Error Handling:" );

  let mixed_commands = vec!
  [
    "valid.command arg::value",
    "invalid..command", // This will fail
    "another.valid cmd::test",
    "malformed arg:::bad", // This will fail
    "good.command final::ok",
  ];

  let error_start = Instant::now();
  let mut success_count = 0;
  let mut error_count = 0;

  for cmd in mixed_commands
  {
    match parser.parse_single_instruction( cmd )
    {
      Ok( _ ) =>
      {
        success_count += 1;
        // Fast path for successful parsing
      }
      Err( _ ) =>
      {
        error_count += 1;
        // Minimal error handling for performance
      }
    }
  }

  let error_duration = error_start.elapsed();
  println!
  (
    "  Processed mixed input: {} success, {} errors in {:?}",
    success_count, error_count, error_duration
  );

  // Pattern 5: Configuration optimization
  println!( "\n5. Configuration Optimization:" );

  // Use default options for maximum performance
  let fast_parser = Parser::new( UnilangParserOptions::default() );

  // For strict validation (slower but more thorough)
  let strict_parser = Parser::new( UnilangParserOptions
  {
    main_delimiters : vec![ " ", "." ],
    operators : vec![ "::", "?", "!" ],
    whitespace_is_separator : true,
    error_on_positional_after_named : true,
    error_on_duplicate_named_arguments : true,
    quote_pairs : vec![ ( '"', '"' ), ( '\'', '\'' ) ],
    verbosity : 0,
  });

  let test_cmd = "test.command pos1 pos2 name::value";

  // Compare performance
  let fast_start = Instant::now();
  for _ in 0..1000
  {
    let _ = fast_parser.parse_single_instruction( test_cmd );
  }
  let fast_duration = fast_start.elapsed();

  let strict_start = Instant::now();
  for _ in 0..1000
  {
    let _ = strict_parser.parse_single_instruction( test_cmd );
  }
  let strict_duration = strict_start.elapsed();

  println!( "  Default config: {:?} for 1000 parses", fast_duration );
  println!( "  Strict config:  {:?} for 1000 parses", strict_duration );
  println!
  (
    "  Performance ratio: {:.2}x",
    strict_duration.as_nanos() as f64 / fast_duration.as_nanos() as f64
  );

  // Pattern 6: Best practices summary
  println!( "\n=== Performance Best Practices ===" );
  println!( "  ✓ Reuse Parser instances across multiple operations" );
  println!( "  ✓ Use default configuration when strict validation isn't needed" );
  println!( "  ✓ Process commands immediately rather than accumulating results" );
  println!( "  ✓ Handle errors efficiently without complex diagnostics in hot paths" );
  println!( "  ✓ Prefer batch parsing for multiple instructions" );
  println!( "  ✓ Avoid unnecessary string allocations in processing loops" );

  println!( "\n✓ Performance optimization patterns demonstration complete!" );
  Ok( () )
}