//! Multi-Module Static Aggregation
//!
//! Demonstrates combining commands from multiple modules at compile-time.
//! Features namespace management, conflict detection, and modular design.
//!
//! ## Architecture Benefits
//!
//! - Modular command organization
//! - Compile-time conflict detection
//! - Namespace isolation and prefixing
//! - Zero runtime aggregation overhead
//! - Conditional module loading via feature flags
//!
//! ## Build Integration
//!
//! NOTE: Temporarily commented out due to API mismatches with current implementation.

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!("This example is temporarily disabled due to API mismatches.");
  Ok(())
}

/*
//!
//! This example shows how the CliBuilder can aggregate multiple YAML files
//! into a single PHF map at compile-time, maintaining O(1) lookup performance
//! while supporting complex organizational structures.

use unilang::multi_yaml::CliBuilder;
use unilang::prelude::*;

fn main() -> Result< (), unilang::Error >
{
  println!( "=== Multi-Module Static Aggregation ===" );
  println!();

  // Note: In a real application, this would be done in build.rs
  // Here we demonstrate the API for educational purposes

  println!( "Building aggregated registry from multiple modules..." );

  // Build aggregated registry from multiple YAML sources
  let builder = CliBuilder::new()
  .static_module( "file_ops", "commands/file_commands.yaml" )
  .static_module( "network", "commands/network_commands.yaml" )
  .static_module( "database", "commands/db_commands.yaml" )
  .global_prefix( Some( "myapp".to_string() ) )
  .detect_conflicts( true )
  .validate_dependencies( true );

  // In practice, this would generate PHF maps at compile-time
  println!( "CliBuilder configuration:" );
  println!( "  - File operations module: commands/file_commands.yaml" );
  println!( "  - Network module: commands/network_commands.yaml" );
  println!( "  - Database module: commands/db_commands.yaml" );
  println!( "  - Global prefix: myapp" );
  println!( "  - Conflict detection: enabled" );
  println!( "  - Dependency validation: enabled" );
  println!();

  // For demonstration, we'll use the existing static commands
  // In a real implementation, this would be the aggregated result
  let registry = CommandRegistry::from_phf( &STATIC_COMMANDS );
  let pipeline = Pipeline::new( registry );

  // Demonstrate namespace organization
  println!( "Namespace organization benefits:" );
  println!();

  // Show how commands would be organized in a multi-module setup
  let example_modules = vec!
  [
  ( "file_ops", vec![ "list", "create", "delete", "copy", "move" ] ),
  ( "network", vec![ "ping", "download", "upload", "status" ] ),
  ( "database", vec![ "query", "migrate", "backup", "restore" ] ),
 ];

  for ( module_name, commands ) in example_modules
  {
  println!( "Module: {}", module_name );
  for cmd in commands
  {
  println!( "  .myapp.{}.{}", module_name, cmd );
 }
  println!();
 }

  // Demonstrate conflict detection
  println!( "Conflict Detection Benefits:" );
  println!( "- Duplicate command names detected at build time" );
  println!( "- Namespace collisions prevented" );
  println!( "- Alias conflicts identified early" );
  println!( "- Clear error messages for resolution" );
  println!();

  // Show modular command execution patterns
  println!( "Executing modular commands..." );
  let example_commands = vec!
  [
  ".greet name::Alice",  // Using existing command for demo
  ".greet name::Bob",
 ];

  for cmd_str in example_commands
  {
  println!( "Executing: {}", cmd_str );
  let result = pipeline.process_command_simple( cmd_str );

  if result.success
  {
  println!( "  Success: Command resolved with zero lookup cost" );
  println!( "  Output: {}", result.outputs[ 0 ].content );
 }
  else if let Some( error ) = result.error
  {
  println!( "  Error: {}", error );
 }
 }

  println!();
  println!( "Advanced Aggregation Features:" );
  println!();

  // Conditional module loading
  println!( "Conditional Module Loading:" );
  println!( "```rust" );
  println!( "CliBuilder::new()" );
  println!( "  .add_conditional_module(" );
  println!( "    \"database\"," );
  println!( "    \"commands/db.yaml\"," );
  println!( "    &[\"feature_database\"]" );
  println!( " )" );
  println!( "  .build_static()" );
  println!( "```" );
  println!();

  // Dependency management
  println!( "Dependency Validation:" );
  println!( "```rust" );
  println!( "CliBuilder::new()" );
  println!( "  .add_static_module(\"auth\", \"auth.yaml\")" );
  println!( "  .add_static_module(\"user\", \"user.yaml\")" );
  println!( "  .validate_dependencies(true)  // Ensures auth commands exist" );
  println!( "  .build_static()" );
  println!( "```" );
  println!();

  // Performance characteristics
  println!( "Performance Characteristics:" );
  println!( "- Single PHF map for all modules" );
  println!( "- O(1) lookup regardless of module count" );
  println!( "- Zero runtime aggregation overhead" );
  println!( "- Namespace resolution at compile-time" );
  println!( "- Dead code elimination per module" );

  println!();
  println!( "Best Practices:" );
  println!( "1. Organize commands by functional domain" );
  println!( "2. Use consistent naming conventions across modules" );
  println!( "3. Enable conflict detection in build process" );
  println!( "4. Leverage conditional loading for optional features" );
  println!( "5. Document module dependencies clearly" );

  Ok( () )
}

*/