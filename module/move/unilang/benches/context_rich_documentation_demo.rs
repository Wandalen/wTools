//! Context-rich benchmark documentation demonstration
//!
//! Shows how to generate comprehensive benchmark reports with:

#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]
#![allow(clippy::manual_contains)]
//! - Measurement specifications clearly stated before results
//! - Before/After optimization comparisons  
//! - Key findings and insights included with results
//! - Actionable recommendations provided
//! - Environment specifications documented

#[ cfg( feature = "benchmarks" ) ]
use unilang::{
  ContextRichDocGenerator,
  BenchmarkMeasurementContext,
  EnvironmentContext,
  BeforeAfterComparison,
  OptimizationStatus,
  ComparativeBenchmark,
  BenchmarkDataSize,
  RealisticDataGenerator,
  MultiSizeComparison,
};

#[ cfg( feature = "benchmarks" ) ]
fn main()
{
  println!( "🎯 Generating Context-Rich Benchmark Documentation Demo" );
  
  // Set up environment context
  let environment = EnvironmentContext
  {
    cpu : "Intel i7-9700K (8 cores, 3.6GHz)".to_string(),
    ram : "32GB DDR4-3200".to_string(),
    storage : "NVMe SSD (Samsung 970 EVO)".to_string(),
    load_characteristics : "development environment, low background load".to_string(),
    notes : vec![ 
      "Results may vary under production load".to_string(),
      "CPU frequency scaling enabled".to_string(),
      "Measurements taken with cold CPU cache".to_string()
    ],
  };
  
  let mut doc_generator = ContextRichDocGenerator::new( environment );
  
  // Demo 1: Comparative benchmark with context
  generate_comparative_benchmark_example( &mut doc_generator );
  
  // Demo 2: Before/after optimization comparison
  generate_optimization_comparison_example( &mut doc_generator );
  
  // Generate final report
  let report = doc_generator.generate_report( "Unilang Performance Analysis Report" );
  println!( "\n{}", report );
  
  // Also write to file for documentation
  std::fs::write( "benchmark_report_example.md", &report )
    .expect( "Failed to write benchmark report" );
    
  println!( "📄 Context-rich documentation written to: benchmark_report_example.md" );
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_comparative_benchmark_example( doc_generator : &mut ContextRichDocGenerator )
{
  println!( "📊 Running comparative string processing benchmark..." );
  
  // Create realistic comparative benchmark
  let mut string_processing_comparison = ComparativeBenchmark::new(
    "String Processing Algorithms",
    "Performance comparison of string manipulation approaches in unilang CLI parsing"
  );
  
  // Algorithm 1: Standard string operations
  string_processing_comparison.add_algorithm( "standard_string_ops", | data : &Vec< String > | {
    for text in data {
      let _parts : Vec< &str > = text.split_whitespace().collect();
      let _uppercase = text.to_uppercase();
      let _length_check = text.len() > 10;
      let _contains_dot = text.contains( '.' );
    }
  } );
  
  // Algorithm 2: Optimized string operations with early returns
  string_processing_comparison.add_algorithm( "optimized_string_ops", | data : &Vec< String > | {
    for text in data {
      if text.len() <= 10 { continue; }
      
      let _parts : Vec< &str > = text.split_whitespace().collect();
      
      if !text.contains( '.' ) { continue; }
      
      let _uppercase = text.to_uppercase();
    }
  } );
  
  // Algorithm 3: SIMD-optimized approach (simulated)
  string_processing_comparison.add_algorithm( "simd_string_ops", | data : &Vec< String > | {
    // Simulate SIMD operations with batch processing
    let batch_size = 4;
    for chunk in data.chunks( batch_size ) {
      for text in chunk {
        let _fast_length = text.len(); // Simulated SIMD length check
        let _fast_contains = text.as_bytes().iter().any( | &b | b == b'.' ); // Simulated SIMD search
      }
    }
  } );
  
  // Set up test data
  let mut data_generator = RealisticDataGenerator::new();
  for size in [ BenchmarkDataSize::Medium, BenchmarkDataSize::Large ] {
    let count = size.value();
    let test_commands = data_generator.generate_command_names( count );
    string_processing_comparison.set_test_data( size, test_commands );
  }
  
  // Run the comparison for medium size
  let results = string_processing_comparison.run_comparison( BenchmarkDataSize::Medium, 100 );
  
  // Create measurement context
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : format!( 
      "String processing algorithms on {} realistic CLI command patterns", 
      BenchmarkDataSize::Medium.value() 
    ),
    how_to_measure : "cargo run --bin context_rich_documentation_demo --features benchmarks".to_string(),
    purpose : "Evaluate string processing optimizations for CLI command parsing performance".to_string(),
    environment : doc_generator.environment().clone(),
  };
  
  // Add to documentation
  doc_generator.add_comparative_results( context, &results );
}

#[ cfg( feature = "benchmarks" ) ]
fn generate_optimization_comparison_example( doc_generator : &mut ContextRichDocGenerator )
{
  println!( "🔧 Generating before/after optimization comparison..." );
  
  // Simulate before/after optimization measurements
  let comparisons = vec![
    BeforeAfterComparison
    {
      algorithm_name : "command_parsing".to_string(),
      before_nanos : 2_400_000.0,  // 2.4ms
      after_nanos : 1_800_000.0,   // 1.8ms
      status : OptimizationStatus::Optimized,
    },
    BeforeAfterComparison
    {
      algorithm_name : "argument_validation".to_string(),
      before_nanos : 850_000.0,    // 0.85ms
      after_nanos : 680_000.0,     // 0.68ms
      status : OptimizationStatus::ProductionReady,
    },
    BeforeAfterComparison
    {
      algorithm_name : "help_generation".to_string(),
      before_nanos : 1_200_000.0,  // 1.2ms
      after_nanos : 1_250_000.0,   // 1.25ms (slight regression)
      status : OptimizationStatus::NeedsWork,
    },
    BeforeAfterComparison
    {
      algorithm_name : "error_handling".to_string(),
      before_nanos : 450_000.0,    // 0.45ms
      after_nanos : 320_000.0,     // 0.32ms
      status : OptimizationStatus::ProductionReady,
    },
  ];
  
  // Create measurement context
  let context = BenchmarkMeasurementContext
  {
    what_is_measured : "CLI processing pipeline optimization impact on 1000 command executions".to_string(),
    how_to_measure : "cargo bench --bench cli_optimization --features benchmarks".to_string(),
    purpose : "Validate performance improvements from string interning and caching optimizations".to_string(),
    environment : doc_generator.environment().clone(),
  };
  
  // Add before/after comparison to documentation
  doc_generator.add_before_after_comparison(
    "CLI Processing Pipeline Optimization Results",
    context,
    &comparisons
  );
}

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main()
{
  eprintln!( "Error: Benchmarks not enabled!" );
  eprintln!( "Run with: cargo run --bin context_rich_documentation_demo --features benchmarks" );
  std::process::exit( 1 );
}