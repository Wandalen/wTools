//! # Example 005: Debug Variants
//!
//! This example demonstrates the debug variants of assertion macros.
//! Debug variants show values even when assertions succeed, making them
//! perfect for development and troubleshooting.
//!
//! ## What you'll learn:
//! - Debug variants: `a_dbg_true!`, `a_dbg_false!`, `a_dbg_id!`, `a_dbg_not_id!`
//! - When to use debug variants vs regular variants
//! - Development workflow integration
//! - Visibility into successful assertions
//!
//! ## Run this example:
//! ```bash
//! cargo run --example 005_debug_variants
//! ```

use diagnostics_tools::*;

#[ derive( Debug, PartialEq ) ]
struct ProcessingResult
{
  processed_items : usize,
  success_rate : f64, 
  error_count : usize,
}

fn main()
{
  println!( "🔧 Demonstrating debug assertion variants" );
  println!( "Debug variants show values even when assertions succeed!\n" );

  // ✅ Regular vs Debug variants comparison
  println!( "1. Regular vs Debug variants:" );
  
  let value = 42;
  
  // Regular variant: only shows output on failure
  a_true!( value > 0 );
  println!( "   Regular a_true!: Silent when successful" );
  
  // Debug variant: shows the values even on success  
  a_dbg_true!( value > 0, "Value should be positive" );
  println!( "   ↑ Debug variant shows the actual value and result\n" );

  // ✅ Debug comparisons
  println!( "2. Debug value comparisons:" );
  
  let expected = "Hello";
  let actual = "Hello";
  
  // Regular comparison (silent on success)
  a_id!( actual, expected );
  println!( "   Regular a_id!: Silent when values match" );
  
  // Debug comparison (shows values even on success)
  a_dbg_id!( actual, expected, "Greeting should match" );
  println!( "   ↑ Debug variant shows both values for verification\n" );

  // ✅ Complex data debugging
  demonstrate_complex_debugging();
  
  // ✅ Development workflow examples
  demonstrate_development_workflow();
  
  // ✅ Troubleshooting scenarios
  demonstrate_troubleshooting();

  println!( "\n🎉 All debug assertions completed!" );
  println!( "\n💡 When to use debug variants:" );
  println!( "   • During active development to see intermediate values" );
  println!( "   • When troubleshooting complex logic" );
  println!( "   • To verify calculations are working correctly" );
  println!( "   • In temporary debugging code that will be removed" );
  println!( "\n💡 When to use regular variants:" );
  println!( "   • In production code that should be silent on success" );
  println!( "   • In tests where you only care about failures" );  
  println!( "   • When you want minimal output for performance" );
  println!( "\n➡️  Next: Run example 006 for real-world usage scenarios!" );
}

fn demonstrate_complex_debugging()
{
  println!( "3. Debugging complex data structures:" );
  
  let result = ProcessingResult
  {
    processed_items : 150,
    success_rate : 0.95,
    error_count : 7,
  };
  
  // Debug variants let you see the actual values during development
  a_dbg_true!( result.processed_items > 100, "Should process many items" );
  a_dbg_true!( result.success_rate > 0.9, "Should have high success rate" );
  a_dbg_true!( result.error_count < 10, "Should have few errors" );
  
  // You can also compare entire structures
  let expected_range = ProcessingResult
  {
    processed_items : 140, // Close but not exact
    success_rate : 0.94,   // Close but not exact
    error_count : 8,       // Close but not exact
  };
  
  // This will show both structures so you can see the differences
  a_dbg_not_id!( result, expected_range, "Results should differ from template" );
  
  println!( "   ✓ Complex structure debugging completed\n" );
}

fn demonstrate_development_workflow()
{
  println!( "4. Development workflow integration:" );
  
  // Simulate a calculation function you're developing
  let input_data = vec![ 1.0, 2.5, 3.7, 4.2, 5.1 ];
  let processed = process_data( &input_data );
  
  // During development, you want to see intermediate values
  println!( "   Debugging data processing pipeline:" );
  a_dbg_true!( processed.len() == input_data.len(), "Output length should match input" );
  a_dbg_true!( processed.iter().all( |&x| x > 0.0 ), "All outputs should be positive" );
  
  let sum : f64 = processed.iter().sum();
  a_dbg_true!( sum > 0.0, "Sum should be positive" );
  
  // Check specific calculations
  let first_result = processed[ 0 ];
  a_dbg_id!( first_result, 2.0, "First calculation should double the input" );
  
  println!( "   ✓ Development debugging workflow completed\n" );
}

fn demonstrate_troubleshooting()
{
  println!( "5. Troubleshooting scenarios:" );
  
  // Scenario: You're debugging a configuration issue
  let config = load_config();
  
  println!( "   Debugging configuration loading:" );
  a_dbg_true!( !config.database_url.is_empty(), "Database URL should be configured" );
  a_dbg_true!( config.max_connections > 0, "Max connections should be positive" );
  a_dbg_true!( config.timeout_ms >= 1000, "Timeout should be at least 1 second" );
  
  // Scenario: You're debugging calculation logic
  let calculation_input = 15.5;
  let result = complex_calculation( calculation_input );
  
  println!( "   Debugging calculation logic:" );
  a_dbg_true!( result.is_finite(), "Result should be a finite number" );
  a_dbg_true!( result > calculation_input, "Result should be greater than input" );
  
  // Show the intermediate steps
  let step1 = calculation_input * 2.0;
  let step2 = step1 + 10.0;
  a_dbg_id!( result, step2, "Result should match expected calculation" );
  
  println!( "   ✓ Troubleshooting scenarios completed\n" );
}

// Simulated functions for examples

fn process_data( input : &[ f64 ] ) -> Vec< f64 >
{
  input.iter().map( |x| x * 2.0 ).collect()
}

#[ derive( Debug ) ]
struct AppConfig
{
  database_url : String,
  max_connections : u32,
  timeout_ms : u64,
}

fn load_config() -> AppConfig
{
  AppConfig
  {
    database_url : "postgresql://localhost:5432/myapp".to_string(),
    max_connections : 50,
    timeout_ms : 5000,
  }
}

fn complex_calculation( input : f64 ) -> f64
{
  input * 2.0 + 10.0
}

// Examples of different assertion patterns
#[ allow( dead_code ) ]
fn assertion_pattern_comparison()
{
  let value = 42;
  let name = "Alice";
  
  // Pattern 1: Silent success (production code)
  a_true!( value > 0 );
  a_id!( name.len(), 5 );
  
  // Pattern 2: Visible success (development/debugging)  
  a_dbg_true!( value > 0, "Checking if value is positive" );
  a_dbg_id!( name.len(), 5, "Verifying name length" );
  
  // Pattern 3: Mixed approach
  a_true!( value > 0 );  // Silent for basic checks
  a_dbg_id!( calculate_complex_result( value ), 84, "Verifying complex calculation" ); // Visible for complex logic
}

fn calculate_complex_result( input : i32 ) -> i32
{
  input * 2  // Simplified for example
}