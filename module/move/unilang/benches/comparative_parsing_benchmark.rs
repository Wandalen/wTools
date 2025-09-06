//! Comparative parsing benchmark demonstrating side-by-side algorithm performance analysis
//!
//! Implements benchkit usage.md "Write Comparative Benchmarks" section requirements:

#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]
//! - Side-by-side algorithm comparisons
//! - Baseline establishment (1.00x reference point)  
//! - Relative performance calculations and reporting
//! - Clear performance comparison tables generated

#[ cfg( feature = "benchmarks" ) ]
use unilang::{ 
  ComparativeBenchmark, 
  BenchmarkDataSize, 
  RealisticDataGenerator,
  Pipeline,
  CommandRegistry,
  prelude::*
};

#[ cfg( feature = "benchmarks" ) ]
use std::time::Instant;

#[ cfg( feature = "benchmarks" ) ]
fn run_parsing_comparison_benchmark()
{
  println!( "🚀 Running Comparative Parsing Benchmark" );
  
  // Create realistic test data for different sizes
  let mut data_generator = RealisticDataGenerator::new();
  
  // Create parsing algorithms comparison
  let mut parsing_comparison = ComparativeBenchmark::new(
    "Command Parsing Algorithms",
    "Comparison of different command parsing approaches in unilang"
  );
  
  // Algorithm 1: Pipeline-based parsing (current optimized approach)
  parsing_comparison.add_algorithm( "pipeline_parsing", | commands : &Vec< String > | {
    let mut registry = CommandRegistry::new();
    
    // Add some basic commands for parsing context
    for i in 0..10 {
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
      };
      registry.register( cmd );
    }
    
    let pipeline = Pipeline::new( registry );
    
    // Parse all commands
    for command in commands {
      let _ = pipeline.process_command_simple( command );
    }
  } );
  
  // Algorithm 2: Direct registry lookup (basic approach)
  parsing_comparison.add_algorithm( "direct_registry", | commands : &Vec< String > | {
    let mut registry = CommandRegistry::new();
    
    // Add basic commands
    for i in 0..10 {
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
      };
      registry.register( cmd );
    }
    
    // Direct lookup without pipeline optimizations
    for command in commands {
      // Simulate basic parsing by looking up command existence
      let parts: Vec< &str > = command.split_whitespace().collect();
      if let Some( cmd_part ) = parts.first() {
        let _ = registry.command( cmd_part );
      }
    }
  } );
  
  // Algorithm 3: String-based parsing (naive approach)
  parsing_comparison.add_algorithm( "string_parsing", | commands : &Vec< String > | {
    // Naive string-based parsing simulation
    for command in commands {
      // Simple string operations to simulate parsing overhead
      let _parts: Vec< &str > = command.split_whitespace().collect();
      let _namespace_check = command.starts_with( '.' );
      let _arg_count = command.matches( "::" ).count();
      
      // Simulate validation work
      for part in command.split_whitespace() {
        let _len = part.len();
        let _has_special = part.contains( "::" ) || part.contains( "--" );
      }
    }
  } );
  
  // Set up test data for each size category
  for size in BenchmarkDataSize::all() {
    let count = size.value();
    let commands = data_generator.generate_command_names( count );
    parsing_comparison.set_test_data( size, commands );
  }
  
  // Run comprehensive comparison across all sizes
  println!( "📊 Running comparisons across all data sizes..." );
  
  let mut comprehensive_comparison = unilang::MultiSizeComparison::new( parsing_comparison );
  comprehensive_comparison.run_all_sizes( 50 ); // 50 iterations per algorithm per size
  
  // Generate and display the comparison report
  let report = comprehensive_comparison.generate_comprehensive_report();
  println!( "{}", report );
  
  // Also benchmark JSON parsing comparisons
  run_json_parsing_comparison( &mut data_generator );
}

#[ cfg( feature = "benchmarks" ) ]
fn run_json_parsing_comparison( data_generator : &mut RealisticDataGenerator )
{
  println!( "\n🔄 Running JSON Parsing Comparison..." );
  
  let mut json_comparison = ComparativeBenchmark::new(
    "JSON Parsing Algorithms",
    "Comparison of JSON parsing approaches for different payload sizes"
  );
  
  // Algorithm 1: Standard serde_json
  json_comparison.add_algorithm( "serde_json", | json_data : &String | {
    let _: Result< serde_json::Value, _ > = serde_json::from_str( json_data );
  } );
  
  // Algorithm 2: SIMD JSON (if available)
  #[ cfg( feature = "simd" ) ]
  json_comparison.add_algorithm( "simd_json", | json_data : &String | {
    use unilang::SIMDJsonParser;
    let _result = SIMDJsonParser::parse_to_serde_value( json_data );
  } );
  
  // Set up JSON test data for different sizes
  for size in BenchmarkDataSize::all() {
    let json_payload = data_generator.generate_json_scenarios( size );
    json_comparison.set_test_data( size, json_payload );
  }
  
  // Run JSON parsing comparison  
  let mut json_multi_comparison = unilang::MultiSizeComparison::new( json_comparison );
  json_multi_comparison.run_all_sizes( 100 ); // More iterations for JSON parsing
  
  let json_report = json_multi_comparison.generate_comprehensive_report();
  println!( "{}", json_report );
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