//! Comparative parsing benchmark demonstrating side-by-side algorithm performance analysis
//!
//! Implements benchkit usage.md "Write Comparative Benchmarks" section requirements:

#![ allow( clippy::uninlined_format_args ) ]
#![ allow( unused_imports ) ]
//! - Side-by-side algorithm comparisons
//! - Baseline establishment (1.00x reference point)  
//! - Relative performance calculations and reporting
//! - Clear performance comparison tables generated

#[ cfg( feature = "benchmarks" ) ]
use unilang::{
  comparative_benchmark_structure::ComparativeBenchmark,
  Pipeline,
  CommandRegistry,
  prelude::*
};

#[ cfg( feature = "benchmarks" ) ]
use std::time::Instant;

#[ cfg( feature = "benchmarks" ) ]
fn run_parsing_comparison_benchmark()
{
  use std::time::Duration;

  println!( "🚀 Running Comparative Parsing Benchmark" );

  // Create parsing algorithms comparison
  let parsing_comparison = ComparativeBenchmark::new(
    "Command Parsing Algorithms",
    "Comparison of different command parsing approaches in unilang"
  );

  // Test data: sample commands
  let test_commands = vec![
    ".greet name::Alice".to_string(),
    ".calculate x::10 y::20".to_string(),
    ".help".to_string(),
    ".status verbose::true".to_string(),
    ".test data::sample".to_string(),
  ];

  // Add algorithm 1: Pipeline-based parsing
  let parsing_comparison = parsing_comparison.add_algorithm( "pipeline_parsing", move |data: &Vec<String>| {
    let start = Instant::now();

    #[ allow( deprecated ) ]
    let mut registry = CommandRegistry::new();

    // Add basic commands for parsing context
    for i in 0..5 {
      let cmd = CommandDefinition {
        name: format!( "cmd_{}", i ),
        namespace: ".test".to_string(),
        description: format!( "Test command {}", i ),
        hint: "Test command".to_string(),
        arguments: vec![],
        routine_link: None,
        status: "stable".to_string(),
        version: "1.0.0".to_string(),
        tags: vec![],
        aliases: vec![],
        permissions: vec![],
        idempotent: true,
        deprecation_message: String::new(),
        http_method_hint: String::new(),
        examples: vec![],
        auto_help_enabled: false,
      };
      registry.register( cmd );
    }

    let pipeline = Pipeline::new( registry );

    // Parse all commands
    for command in data {
      let _ = pipeline.process_command_simple( command );
    }

    start.elapsed()
  } );

  // Add algorithm 2: String-based parsing
  let parsing_comparison = parsing_comparison.add_algorithm( "string_parsing", move |data: &Vec<String>| {
    let start = Instant::now();

    // Naive string-based parsing simulation
    for command in data {
      let _parts: Vec< &str > = command.split_whitespace().collect();
      let _namespace_check = command.starts_with( '.' );
      let _arg_count = command.matches( "::" ).count();

      // Simulate validation work
      for part in command.split_whitespace() {
        let _len = part.len();
        let _has_special = part.contains( "::" ) || part.contains( "--" );
      }
    }

    start.elapsed()
  } );

  // Set baseline to pipeline parsing
  let parsing_comparison = parsing_comparison.set_baseline( "pipeline_parsing" );

  // Run comparison
  let results = parsing_comparison.run_comparison( &test_commands );

  println!( "📊 Parsing Algorithm Comparison Results:" );
  println!( "{}", results.performance_summary() );

  for (name, time) in results.ranked_algorithms() {
    if let Some( relative ) = results.get_relative_performance( name ) {
      println!( "  {}: {:.2}μs ({:.2}x relative to baseline)",
        name,
        time.as_nanos() as f64 / 1000.0,
        relative
      );
    }
  }
}


#[ cfg( feature = "benchmarks" ) ]
fn main()
{
  run_parsing_comparison_benchmark();
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  eprintln!( "Error: Benchmarks not enabled!" );
  eprintln!( "Run with: cargo bench --bench comparative_parsing_benchmark --features benchmarks" );
  std::process::exit( 1 );
}