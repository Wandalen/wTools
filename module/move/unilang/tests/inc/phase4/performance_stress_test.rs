//!
//! Performance stress test for static command registry.
//! 
//! This test verifies the NFR-Performance requirement by generating
//! 1000+ static commands and measuring command resolution latency.
//!

use std::env;
use std::fs;
use std::path::Path;

/// Generates a YAML string with the specified number of unique command definitions.
/// 
/// Each command will have basic metadata and a few arguments to test realistic scenarios.
#[must_use] pub fn generate_stress_yaml( count : usize ) -> String
{
  let mut yaml = String::new();
  yaml.push_str( "---\n" );

  for i in 0..count
  {
    yaml.push_str( &format!( r#"
- name: "cmd_{i}"
  namespace: ".perf"
  description: "Performance test command {i}"
  hint: "Command for performance testing"
  arguments:
    - name: "arg1"
      description: "First argument"
      kind: "String"
      hint: "String argument"
      attributes:
        optional: false
        multiple: false
        default: null
        sensitive: false
        interactive: false
      validation_rules: []
      aliases: []
      tags: []
    - name: "arg2"
      description: "Second argument"
      kind: "Integer"
      hint: "Integer argument"
      attributes:
        optional: true
        multiple: false
        default: "0"
        sensitive: false
        interactive: false
      validation_rules: []
      aliases: []
      tags: []
  routine_link: null
  status: "stable"
  version: "1.0.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
"# ) );
  }

  yaml
}

#[ test ]
fn test_stress_yaml_generation()
{
  let yaml = generate_stress_yaml( 10 );
  assert!( yaml.contains( "cmd_0" ) );
  assert!( yaml.contains( "cmd_9" ) );
  assert!( yaml.len() > 1000 ); // Should be substantial content
}

#[ test ]
fn test_performance_stress_setup()
{
  // This test sets up the stress test environment
  let test_count = 1_000_000;
  
  // Set environment variable for custom commands path
  let out_dir = env::var( "OUT_DIR" ).unwrap_or_else( |_| "/tmp".to_string() );
  let stress_yaml_path = Path::new( &out_dir ).join( "stress_commands.yaml" );
  
  // Generate the large YAML file
  let yaml_content = generate_stress_yaml( test_count );
  fs::write( &stress_yaml_path, yaml_content ).expect( "Failed to write stress test YAML" );
  
  // Set the environment variable so build.rs uses our stress commands
  env::set_var( "UNILANG_STATIC_COMMANDS_PATH", stress_yaml_path.to_str().unwrap() );
  
  println!( "Generated {test_count} commands for stress testing" );
  println!( "Stress commands written to: {}", stress_yaml_path.display() );
  
  // Verify the file was created
  assert!( stress_yaml_path.exists() );
  let content = fs::read_to_string( &stress_yaml_path ).unwrap();
  assert!( content.contains( "cmd_0" ) );
  assert!( content.contains( &format!( "cmd_{}", test_count - 1 ) ) );
}

#[ test ]
#[ ignore ] // This test should be run manually or in CI due to its intensive nature
fn test_performance_stress_full()
{
  use std::process::Command;
  use std::time::Instant;
  
  // Generate stress test environment
  let test_count = 1_000_000;
  let temp_dir = env::temp_dir();
  let stress_yaml_path = temp_dir.join( "unilang_stress_commands.yaml" );
  
  // Generate the large YAML file
  let yaml_content = generate_stress_yaml( test_count );
  fs::write( &stress_yaml_path, yaml_content ).expect( "Failed to write stress test YAML" );
  
  println!( "Generated {test_count} commands for performance test" );
  
  // Run the stress test binary with the custom command set
  let start_time = Instant::now();
  
  let output = Command::new( "cargo" )
    .args( [ "run", "--bin", "stress_test_bin" ] )
    .env( "UNILANG_STATIC_COMMANDS_PATH", stress_yaml_path.to_str().unwrap() )
    .output()
    .expect( "Failed to execute stress test binary" );
  
  let total_execution_time = start_time.elapsed();
  
  // Parse the output
  let stdout = String::from_utf8_lossy( &output.stdout );
  let stderr = String::from_utf8_lossy( &output.stderr );
  
  println!( "=== Stress Test Output ===" );
  println!( "{stdout}" );
  if !stderr.is_empty()
  {
    println!( "=== Stderr ===" );
    println!( "{stderr}" );
  }
  
  // Verify the binary executed successfully
  assert!( output.status.success(), "Stress test binary failed to execute successfully" );
  
  // Verify the output contains "Ready" indicating completion
  assert!( stdout.contains( "Ready" ), "Stress test binary did not complete properly" );
  
  // Parse and verify performance metrics
  let p99_line = stdout.lines()
    .find( |line| line.starts_with( "P99_LATENCY_MICROS:" ) )
    .expect( "Could not find P99_LATENCY_MICROS in output" );
  
  let p99_micros: f64 = p99_line
    .split( ':' )
    .nth( 1 )
    .expect( "Could not parse P99 latency value" )
    .trim()
    .parse()
    .expect( "Could not parse P99 latency as number" );
  
  // Verify performance requirements
  println!( "=== Performance Assertions ===" );
  println!( "Total execution time: {total_execution_time:?}" );
  println!( "P99 latency: {p99_micros:.2} microseconds" );
  
  // NFR-Performance: p99 latency must be < 1 millisecond (1000 microseconds)
  assert!( 
    p99_micros < 1000.0, 
    "Performance requirement FAILED: p99 latency ({p99_micros:.2} μs) >= 1000 μs (1ms)" 
  );
  
  // Additional startup time check - total execution should be reasonable
  assert!( 
    total_execution_time.as_millis() < 10000, 
    "Startup time too high: {total_execution_time:?} > 10 seconds" 
  );
  
  println!( "✅ All performance requirements MET!" );
  println!( "   - P99 command resolution latency: {p99_micros:.2} μs < 1000 μs" );
  println!( "   - Total execution time: {total_execution_time:?} < 10s" );
  
  // Clean up
  let _ = fs::remove_file( stress_yaml_path );
}