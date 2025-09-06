//! Tests for realistic test data generation functionality

#![ cfg( feature = "benchmarks" ) ]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_char_pattern)]

use unilang::{ RealisticDataGenerator, RealisticDataCache, BenchmarkDataSize };

#[ cfg( not( feature = "benchmarks" ) ) ]
fn main() {}

#[ test ]
fn test_realistic_command_generation()
{
  let mut generator = RealisticDataGenerator::new();
  let commands = generator.generate_command_names( 50 );
  
  // Verify count
  assert_eq!( commands.len(), 50 );
  
  // Verify realistic patterns
  assert!( commands.iter().any( | cmd | cmd.starts_with( ".config" ) ) );
  assert!( commands.iter().any( | cmd | cmd.contains( "." ) ) );
  
  // Verify reproducibility with same seed
  let mut generator2 = RealisticDataGenerator::with_seed( 12345 );
  let commands2 = generator2.generate_command_names( 50 );
  assert_eq!( commands, commands2 );
}

#[ test ]  
fn test_realistic_user_data()
{
  let mut generator = RealisticDataGenerator::new();
  let users = generator.generate_user_data( 10 );
  
  assert_eq!( users.len(), 10 );
  
  // Check realistic JSON structure
  for user in &users
  {
    assert!( user.contains( "\"id\":" ) );
    assert!( user.contains( "\"name\":" ) );
    assert!( user.contains( "\"email\":" ) );
    assert!( user.contains( "\"active\":" ) );
    assert!( user.contains( "\"department\":" ) );
  }
}

#[ test ]
fn test_json_scenario_generation()
{
  let mut generator = RealisticDataGenerator::new();
  
  for size in BenchmarkDataSize::all()
  {
    let json = generator.generate_json_scenarios( size );
    
    // Verify it's valid JSON-like structure
    assert!( json.contains( "{" ) );
    assert!( json.contains( "}" ) );
    assert!( json.len() > 10 );
    
    // Size-appropriate complexity
    match size
    {
      BenchmarkDataSize::Small => assert!( json.len() < 1000 ),
      BenchmarkDataSize::Medium => assert!( json.len() > 500 ),
      BenchmarkDataSize::Large => assert!( json.len() > 5000 ),
      BenchmarkDataSize::Huge => assert!( json.len() > 15000 ),
    }
  }
}

#[ test ]
fn test_realistic_data_cache()
{
  let cache = RealisticDataCache::new();
  
  // Verify pre-generated data exists
  for size in BenchmarkDataSize::all()
  {
    let count = size.value();
    
    assert!( cache.get_command_names( count ).is_some() );
    assert!( cache.get_user_data( count ).is_some() );
    assert!( cache.get_json_scenario( size ).is_some() );
  }
  
  // Verify command names are realistic
  let commands = cache.get_command_names( 100 ).unwrap();
  assert_eq!( commands.len(), 100 );
  assert!( commands.iter().any( | cmd | cmd.starts_with( ".system" ) || cmd.starts_with( ".config" ) ) );
}

#[ test ]
fn test_realistic_args_generation()
{
  let mut generator = RealisticDataGenerator::new();
  let args = generator.generate_realistic_args( ".deploy", 20 );
  
  assert_eq!( args.len(), 20 );
  
  // Check for realistic argument patterns
  assert!( args.iter().any( | arg | arg.contains( "--verbose" ) ) );
  assert!( args.iter().any( | arg | arg.contains( "--config" ) ) );
  assert!( args.iter().any( | arg | arg.contains( "--environment" ) ) );
  assert!( args.iter().any( | arg | arg.contains( "--timeout" ) ) );
}

#[ test ]  
fn test_reproducible_with_fixed_seed()
{
  let mut gen1 = RealisticDataGenerator::with_seed( 54321 );
  let mut gen2 = RealisticDataGenerator::with_seed( 54321 );
  
  let commands1 = gen1.generate_command_names( 25 );
  let commands2 = gen2.generate_command_names( 25 );
  
  assert_eq!( commands1, commands2, "Same seed should produce identical results" );
  
  let users1 = gen1.generate_user_data( 15 );
  let users2 = gen2.generate_user_data( 15 );
  
  assert_eq!( users1, users2, "Same seed should produce identical user data" );
}