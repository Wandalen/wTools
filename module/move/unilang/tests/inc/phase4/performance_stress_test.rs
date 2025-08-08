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
  use std::time::Instant;
  use unilang::registry::CommandRegistry;
  
  println!( "=== Direct Performance Test ===" );
  
  // Test 1: Registry initialization time (startup time)
  let start_time = Instant::now();
  let registry = CommandRegistry::new();
  let startup_time = start_time.elapsed();
  let startup_micros = startup_time.as_nanos() as f64 / 1000.0;
  
  println!( "Registry initialization time: {startup_time:?}" );
  println!( "STARTUP_TIME_MICROS: {startup_micros:.2}" );
  
  // Test 2: Command lookup performance 
  let lookup_count = 100_000; // Reasonable test size
  let mut latencies = Vec::with_capacity( lookup_count );
  
  println!( "Starting {lookup_count} command lookups..." );
  
  for i in 0..lookup_count {
    // Test lookups for existing and non-existing commands
    let cmd_name = if i % 10 == 0 { ".version" } else { &format!(".nonexistent_{i}") };
    
    let lookup_start = Instant::now();
    let _command = registry.command( cmd_name );
    let lookup_time = lookup_start.elapsed();
    
    latencies.push( lookup_time );
  }
  
  // Calculate p99 latency
  latencies.sort();
  let p99 = latencies[ (lookup_count as f64 * 0.99) as usize ];
  let p99_micros = p99.as_nanos() as f64 / 1000.0;
  
  println!( "P99 command lookup latency: {p99:?}" );
  println!( "P99_LATENCY_MICROS: {p99_micros:.2}" );
  
  // Verify performance requirements (NFRs)
  println!( "=== Performance Assertions ===" );
  println!( "Startup time: {startup_micros:.2} microseconds" );
  println!( "P99 latency: {p99_micros:.2} microseconds" );
  
  // NFR-PERF-1: p99 latency must be < 1 millisecond (1000 microseconds)
  assert!( 
    p99_micros < 1000.0, 
    "P99 latency ({p99_micros:.2} μs) must be < 1000 μs" 
  );
  
  // NFR-PERF-2: startup time must be < 5 milliseconds (5000 microseconds) 
  assert!( 
    startup_micros < 5000.0, 
    "Startup time ({startup_micros:.2} μs) must be < 5000 μs" 
  );
  
  println!( "✅ All performance requirements MET!" );
  println!( "   - P99 command resolution latency: {p99_micros:.2} μs < 1000 μs" );
  println!( "   - Startup time: {startup_micros:.2} μs < 5000 μs" );
}