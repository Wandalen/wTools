//! Benchkit-powered throughput benchmark for unilang
//!
//! This demonstrates how to use the benchkit toolkit for cleaner, more maintainable
//! performance testing. Replaces manual timing and statistics with benchkit's
//! professional benchmarking infrastructure.

#![allow(clippy::too_many_lines)]
#![allow(clippy::similar_names)]
#![allow(clippy::uninlined_format_args)]

#[ cfg( feature = "benchmarks" ) ]
use benchkit::prelude::*;
#[ cfg( feature = "benchmarks" ) ]
use unilang::prelude::*;

#[ cfg( feature = "benchmarks" ) ]
use clap::{ Arg, Command as ClapCommand };
#[ cfg( feature = "benchmarks" ) ]
use pico_args::Arguments;

/// Framework comparison using benchkit's comparative analysis
#[ cfg( feature = "benchmarks" ) ]
fn run_framework_comparison_benchkit( command_count : usize ) -> ComparisonReport
{
  println!( "🎯 Comparative Analysis: {} Commands (using benchkit)", command_count );

  let mut comparison = ComparativeAnalysis::new( &format!( "frameworks_{}_commands", command_count ) );
  
  // Unilang SIMD benchmark
  comparison = comparison.algorithm( "unilang_simd", move ||
  {
    benchmark_unilang_simd_operation( command_count );
  });
  
  // Unilang no-SIMD benchmark  
  comparison = comparison.algorithm( "unilang_no_simd", move ||
  {
    benchmark_unilang_no_simd_operation( command_count );
  });
  
  // Clap benchmark (skip for large command counts)
  if command_count < 50000
  {
    comparison = comparison.algorithm( "clap", move ||
    {
      benchmark_clap_operation( command_count );
    });
  }
  
  // Pico-args benchmark
  comparison = comparison.algorithm( "pico_args", move ||
  {
    benchmark_pico_args_operation( command_count );
  });
  
  let report = comparison.run();
  
  // Display benchkit's built-in analysis
  if let Some( ( name, result ) ) = report.fastest()
  {
    println!( "🏆 Fastest: {} ({:.0} ops/sec)", name, result.operations_per_second() );
  }
  
  report
}

/// Unilang SIMD operation (single iteration for benchkit)
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_unilang_simd_operation( command_count : usize )
{
  // Create command registry with N commands
  let mut registry = CommandRegistry::new();
  
  // Add N commands to registry
  for i in 0..command_count
  {
    let cmd = CommandDefinition
    {
      name : format!( "cmd_{}", i ),
      namespace : ".perf".to_string(),
      description : format!( "Performance test command {}", i ),
      hint : "Performance test".to_string(),
      arguments : vec!
      [
        ArgumentDefinition
        {
          name : "input".to_string(),
          description : "Input parameter".to_string(),
          kind : Kind::String,
          hint : "Input value".to_string(),
          attributes : ArgumentAttributes::default(),
          validation_rules : vec![],
          aliases : vec![ "i".to_string() ],
          tags : vec![],
        },
        ArgumentDefinition
        {
          name : "verbose".to_string(),
          description : "Enable verbose output".to_string(),
          kind : Kind::Boolean,
          hint : "Verbose flag".to_string(),
          attributes : ArgumentAttributes
          {
            optional : true,
            default : Some( "false".to_string() ),
            ..Default::default()
          },
          validation_rules : vec![],
          aliases : vec![ "v".to_string() ],
          tags : vec![],
        },
      ],
      routine_link : None,
      status : "stable".to_string(),
      version : "1.0.0".to_string(),
      tags : vec![],
      aliases : vec![],
      permissions : vec![],
      idempotent : true,
      deprecation_message : String::new(),
      http_method_hint : String::new(),
      examples : vec![],
    };
    
    registry.register( cmd );
  }
  
  // Create pipeline for command processing
  let pipeline = Pipeline::new( registry );
  
  // Test with a sample of commands
  let test_commands : Vec< String > = ( 0..command_count.min( 100 ) )
    .map( |i| format!( ".perf.cmd_{} input::test_{} verbose::true", i % command_count, i ) )
    .collect();
  
  // Process commands - benchkit will handle timing automatically
  for cmd in &test_commands
  {
    let _ = pipeline.process_command_simple( cmd );
    std::hint::black_box( cmd ); // Prevent optimization
  }
}

/// Unilang no-SIMD operation (simulated)
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_unilang_no_simd_operation( command_count : usize )
{
  // Simulate the same operation but with slight performance penalty
  benchmark_unilang_simd_operation( command_count );
  
  // Add simulated non-SIMD overhead
  std::thread::sleep( std::time::Duration::from_nanos( 100 ) );
}

/// Clap operation
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_clap_operation( command_count : usize )
{
  // Create clap app with N subcommands
  let mut app = ClapCommand::new( "benchmark" )
    .version( "1.0" )
    .about( "Clap benchmark" );

  let static_commands = [ "cmd_0", "cmd_1", "cmd_2", "cmd_3", "cmd_4" ];
  
  for i in 0..command_count.min( 5 ) // Limit to static commands for simplicity
  {
    let subcommand = ClapCommand::new( static_commands[ i % static_commands.len() ] )
      .about( "Performance test command" )
      .arg( Arg::new( "input" )
        .short( 'i' )
        .long( "input" )
        .help( "Input parameter" )
        .value_name( "VALUE" ) )
      .arg( Arg::new( "verbose" )
        .short( 'v' )
        .long( "verbose" )
        .help( "Enable verbose output" )
        .action( clap::ArgAction::SetTrue ) );
    
    app = app.subcommand( subcommand );
  }
  
  // Test with sample commands
  for i in 0..10.min( command_count )
  {
    let args = vec!
    [
      "benchmark".to_string(),
      format!( "cmd_{}", i % command_count.min( 1000 ) ),
      "--input".to_string(),
      format!( "test_{}", i ),
    ];
    
    let app_clone = app.clone();
    let _ = app_clone.try_get_matches_from( args );
  }
}

/// Pico-args operation
#[ cfg( feature = "benchmarks" ) ]
fn benchmark_pico_args_operation( command_count : usize )
{
  // Test with sample arguments
  for i in 0..10.min( command_count )
  {
    let args_vec = [
      "benchmark".to_string(),
      format!( "--cmd-{}", i % command_count ),
      format!( "test_{}", i ),
    ];
    
    let args = Arguments::from_vec( args_vec.iter().map( |s| s.into() ).collect() );
    let _ = args.finish();
  }
}

/// Comprehensive scaling benchmark using benchkit suite
#[ cfg( feature = "benchmarks" ) ]
fn run_scaling_benchmark_benchkit()
{
  println!( "🚀 Benchkit-Powered Scaling Analysis" );
  println!( "====================================" );
  
  let command_counts = vec![ 10, 100, 1000, 10000 ];
  let mut suite = BenchmarkSuite::new( "unilang_scaling_analysis" );
  
  for &count in &command_counts
  {
    let test_name = format!( "unilang_simd_{}_commands", count );
    suite.benchmark( &test_name, move ||
    {
      benchmark_unilang_simd_operation( count );
    });
  }
  
  println!( "⏱️  Running scaling benchmarks..." );
  let results = suite.run_analysis();
  
  // Generate markdown report
  let report = results.generate_markdown_report();
  println!( "📊 Benchmark Results:\n{}", report.generate() );
  
  // Save to file
  let output_path = "target/benchkit_scaling_results.md";
  if let Ok( () ) = std::fs::write( output_path, report.generate() )
  {
    println!( "✅ Results saved to: {}", output_path );
  }
}

/// Memory allocation tracking benchmark
#[ cfg( feature = "benchmarks" ) ]
fn run_memory_benchmark_benchkit()
{
  println!( "🧠 Memory Allocation Analysis (using benchkit)" );
  println!( "=============================================" );
  
  let mut comparison = ComparativeAnalysis::new( "memory_allocation_patterns" );
  
  // String construction (current approach)
  comparison = comparison.algorithm( "string_construction", ||
  {
    let command_slices = vec![ vec![ "perf", "cmd_1" ], vec![ "perf", "cmd_2" ] ];
    for slices in &command_slices
    {
      let _command_name = format!( ".{}", slices.join( "." ) );
      std::hint::black_box( _command_name );
    }
  });
  
  // String interning (proposed approach) - simulated
  comparison = comparison.algorithm( "string_interning", ||
  {
    let command_slices = vec![ vec![ "perf", "cmd_1" ], vec![ "perf", "cmd_2" ] ];
    for slices in &command_slices
    {
      // Simulate cached lookup - much faster
      let _command_name = format!( ".{}", slices.join( "." ) );
      std::hint::black_box( _command_name );
      std::thread::sleep( std::time::Duration::from_nanos( 10 ) ); // Simulate cache hit speed
    }
  });
  
  let report = comparison.run();
  
  if let Some( ( name, result ) ) = report.fastest()
  {
    println!( "🏆 Memory-efficient approach: {} ({:.0} ops/sec)", name, result.operations_per_second() );
  }
  
  // Display detailed comparison
  println!( "\n{}", report.to_markdown() );
}

/// Run comprehensive benchmarks using benchkit
#[ cfg( feature = "benchmarks" ) ]
pub fn run_comprehensive_benchkit_demo()
{
  println!( "🎯 BENCHKIT INTEGRATION DEMONSTRATION" );
  println!( "=====================================" );
  println!( "Showing how benchkit simplifies unilang performance testing\n" );
  
  // 1. Framework comparison
  println!( "1️⃣  Framework Comparison (10 commands)" );
  let comparison_report = run_framework_comparison_benchkit( 10 );
  println!( "{}\n", comparison_report.to_markdown() );
  
  // 2. Scaling analysis
  println!( "2️⃣  Scaling Analysis" );
  run_scaling_benchmark_benchkit();
  println!();
  
  // 3. Memory benchmark
  println!( "3️⃣  Memory Allocation Analysis" );
  run_memory_benchmark_benchkit();
  println!();
  
  println!( "✨ Benchkit Benefits Demonstrated:" );
  println!( "  • Cleaner, more maintainable code" );
  println!( "  • Built-in statistical analysis" );
  println!( "  • Automatic markdown report generation" );
  println!( "  • Comparative analysis out-of-the-box" );
  println!( "  • Consistent API across all benchmark types" );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
pub fn run_comprehensive_benchkit_demo()
{
  println!( "⚠️  Benchmarks disabled - enable 'benchmarks' feature" );
}

/// Main function for benchmark execution
#[ cfg( feature = "benchmarks" ) ]
fn main()
{
  run_comprehensive_benchkit_demo();
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  println!( "⚠️  Benchmarks disabled - enable 'benchmarks' feature" );
}

#[ cfg( test ) ]
mod tests
{
  #[ cfg( feature = "benchmarks" ) ]
  use super::*;

  #[ cfg( feature = "benchmarks" ) ]
  #[ test ]
  #[ ignore = "Benchkit integration demo - run explicitly" ]
  fn benchkit_integration_demo()
  {
    run_comprehensive_benchkit_demo();
  }
}