//! YAML Build-Time Processing
//!
//! Demonstrates processing YAML command definitions at compile-time.
//! The build.rs script converts YAML definitions into PHF maps for optimal performance.
//!
//! ## Build Process Flow
//!
//! 1. build.rs reads unilang.commands.yaml
//! 2. Parses and validates YAML command definitions
//! 3. Generates static_commands.rs with PHF maps
//! 4. Compile-time validation ensures correctness
//! 5. Zero runtime overhead for command lookups
//!
//! ## Performance Characteristics
//!
//! - Static lookup: O(1) PHF access (~1-3 CPU cycles)
//! - No hash computation overhead
//! - No memory allocations during lookup
//! - Better CPU cache locality

use unilang::prelude::*;
use unilang::static_data::StaticCommandDefinition;

// This include! brings in the PHF map generated from YAML
include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

fn main() -> Result< (), unilang::Error >
{
  println!( "=== YAML Build-Time Processing Demo ===" );
  println!();

  // Demonstrate compile-time validation benefits
  println!( "Commands validated at compile-time:" );
  for ( name, cmd ) in STATIC_COMMANDS.entries()
  {
    println!( "  {} v{} - {}", name, cmd.version, cmd.description );

    // Show compile-time argument validation
    if !cmd.arguments.is_empty()
    {
      println!( "    Arguments:" );
      for arg in cmd.arguments
      {
        let optional_marker = if arg.attributes.optional
        {
          " (optional)"
        }
        else
        {
          ""
        };
        println!( "      {} - {}{}", arg.name, arg.description, optional_marker );
      }
    }
  }
  println!();

  // Create zero-overhead registry
  let registry = StaticCommandRegistry::from_phf( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Demonstrate YAML-defined command execution
  println!( "Executing YAML-defined commands..." );

  let commands = vec!
  [
    ".greet name::Alice",
    ".greet name::Bob",
    ".greet",  // Uses default value
  ];

  for cmd_str in commands
  {
    println!( "Executing: {}", cmd_str );
    let result = pipeline.process_command_simple( cmd_str );

    if result.success
    {
      println!( "  Success: {}", result.outputs[ 0 ].content );
    }
    else if let Some( error ) = result.error
    {
      println!( "  Error: {}", error );
    }
  }

  // Performance demonstration
  println!();
  println!( "Performance characteristics:" );

  let start_time = std::time::Instant::now();
  for _i in 0..10000
  {
    // Each lookup is O(1) PHF access - no hashing overhead
    let _cmd = STATIC_COMMANDS.get( ".greet" );
  }
  let static_duration = start_time.elapsed();

  println!(
    "10,000 static lookups completed in {:?} (~{} ns per lookup)",
    static_duration,
    static_duration.as_nanos() / 10000
  );

  println!();
  println!( "Build-time validation benefits:" );
  println!( "- All command definitions checked for correctness" );
  println!( "- Argument types validated against supported kinds" );
  println!( "- Namespace conflicts detected early" );
  println!( "- Missing required fields caught at build time" );

  Ok( () )
}