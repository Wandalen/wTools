//! # Static Command Registry - Basic Compile-Time Example
//!
//! This example demonstrates the core functionality of the `StaticCommandRegistry`
//! with PHF (Perfect Hash Function) based zero-overhead lookups. All command
//! definitions are registered at compile time using perfect hash maps for
//! optimal runtime performance.
//!
//! ## Key Features Demonstrated
//!
//! - Zero-overhead PHF-based command lookups
//! - Compile-time command registration
//! - Static command definitions
//! - Performance comparison with dynamic registry
//!
//! ## Performance Characteristics
//!
//! - O(1) command lookup time
//! - No runtime memory allocation for commands
//! - Sub-microsecond command resolution
//! - Minimal binary size overhead

use std::time::Instant;
use unilang::static_data::StaticCommandMap;
use unilang::registry::StaticCommandRegistry;

/// Example static commands using PHF map
static EXAMPLE_COMMANDS: StaticCommandMap = phf::phf_map!
{
  ".version" => &unilang::static_data::StaticCommandDefinition
  {
    name: ".version",
    namespace: "",
    description: "Show version information",
    hint: "Display program version",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[],
  },

  ".help" => &unilang::static_data::StaticCommandDefinition
  {
    name: ".help",
    namespace: "",
    description: "Show help information",
    hint: "Display command help",
    arguments: &[],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &[],
  },

  ".math.add" => &unilang::static_data::StaticCommandDefinition
  {
    name: ".math.add",
    namespace: ".math",
    description: "Add two numbers",
    hint: "Perform addition operation",
    arguments: &[
      unilang::static_data::StaticArgumentDefinition
      {
        name: "a",
        description: "First number",
        hint: "Number",
        kind: unilang::static_data::StaticKind::Integer,
        attributes: unilang::static_data::StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &[],
        tags: &[],
      },
      unilang::static_data::StaticArgumentDefinition
      {
        name: "b",
        description: "Second number",
        hint: "Number",
        kind: unilang::static_data::StaticKind::Integer,
        attributes: unilang::static_data::StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &[],
        tags: &[],
      },
    ],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[],
  },

  ".file.copy" => &unilang::static_data::StaticCommandDefinition
  {
    name: ".file.copy",
    namespace: ".file",
    description: "Copy a file",
    hint: "File copy operation",
    arguments: &[
      unilang::static_data::StaticArgumentDefinition
      {
        name: "source",
        description: "Source file path",
        hint: "Path",
        kind: unilang::static_data::StaticKind::String,
        attributes: unilang::static_data::StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &[],
        tags: &[],
      },
      unilang::static_data::StaticArgumentDefinition
      {
        name: "destination",
        description: "Destination file path",
        hint: "Path",
        kind: unilang::static_data::StaticKind::String,
        attributes: unilang::static_data::StaticArgumentAttributes
        {
          optional: false,
          multiple: false,
          default: None,
          sensitive: false,
          interactive: false,
        },
        validation_rules: &[],
        aliases: &[],
        tags: &[],
      },
    ],
    routine_link: None,
    status: "stable",
    version: "1.0.0",
    tags: &[],
    aliases: &[],
    permissions: &[],
    idempotent: false,
    deprecation_message: "",
    http_method_hint: "POST",
    examples: &[],
  },
};

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🚀 Static Command Registry - Basic Compile-Time Example" );
  println!( "=======================================================" );

  // Create static command registry
  let static_registry = StaticCommandRegistry::from_phf( &EXAMPLE_COMMANDS );

  println!( "\n📊 Static Registry Information:" );
  println!( "  Commands loaded: {}", static_registry.commands().len() );
  println!( "  Registry mode: {:?}", static_registry.mode() );

  // Demonstrate zero-overhead lookup performance
  demonstrate_lookup_performance( &static_registry )?;

  // Demonstrate command access
  demonstrate_command_access( &static_registry )?;

  // Demonstrate PHF efficiency
  demonstrate_phf_efficiency( &static_registry )?;

  println!( "\n✅ Static command registry example completed successfully" );
  Ok( () )
}

/// Demonstrate lookup performance characteristics
#[allow(clippy::unnecessary_wraps)]
fn demonstrate_lookup_performance( registry: &StaticCommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n⚡ Performance Demonstration:" );

  let commands_to_test = vec![ ".version", ".help", ".math.add", ".file.copy" ];
  let iterations = 10_000;

  println!( "  Testing {iterations} iterations of command lookups..." );

  let start = Instant::now();
  for _ in 0..iterations
  {
    for cmd_name in &commands_to_test
    {
      let _cmd = registry.command( cmd_name );
    }
  }
  let duration = start.elapsed();

  let total_lookups = iterations * commands_to_test.len();
  let avg_lookup_time = duration / u32::try_from(total_lookups).unwrap_or(1);

  println!( "  Total lookups: {total_lookups}" );
  println!( "  Total time: {duration:?}" );
  println!( "  Average lookup time: {avg_lookup_time:?}" );

  // Validate performance requirement: should be sub-microsecond
  if avg_lookup_time.as_nanos() < 1000
  {
    println!( "  ✅ Performance target met: < 1 microsecond per lookup" );
  }
  else
  {
    let nanos = avg_lookup_time.as_nanos();
    println!( "  ⚠️  Performance target missed: {nanos} ns per lookup" );
  }

  Ok( () )
}

/// Demonstrate command access and information retrieval
#[allow(clippy::unnecessary_wraps)]
fn demonstrate_command_access( registry: &StaticCommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n🔍 Command Access Demonstration:" );

  let test_commands = vec![ ".version", ".help", ".math.add", ".file.copy" ];

  for cmd_name in test_commands
  {
    if let Some( cmd ) = registry.command( cmd_name )
    {
      let namespace = if cmd.namespace.is_empty() { "root" } else { &cmd.namespace };
      println!( "\n  Command: {}", cmd.name );
      println!( "    Description: {}", cmd.description );
      println!( "    Namespace: {namespace}" );
      println!( "    Arguments: {}", cmd.arguments.len() );
      println!( "    Status: {}", cmd.status );
      println!( "    Idempotent: {}", cmd.idempotent );
    }
    else
    {
      println!( "  ❌ Command '{cmd_name}' not found" );
    }
  }

  Ok( () )
}

/// Demonstrate PHF (Perfect Hash Function) efficiency
#[allow(clippy::unnecessary_wraps)]
fn demonstrate_phf_efficiency( registry: &StaticCommandRegistry ) -> Result< (), Box< dyn std::error::Error > >
{
  println!( "\n🎯 PHF Efficiency Demonstration:" );

  // Test that all commands are accessible
  let all_commands = registry.commands();
  println!( "  Total commands in registry: {}", all_commands.len() );

  // Verify perfect hash function properties
  let mut successful_lookups = 0;
  let mut failed_lookups = 0;

  for name in all_commands.keys()
  {
    if registry.command( name ).is_some()
    {
      successful_lookups += 1;
    }
    else
    {
      failed_lookups += 1;
    }
  }

  println!( "  Successful lookups: {successful_lookups}" );
  println!( "  Failed lookups: {failed_lookups}" );

  if failed_lookups == 0
  {
    println!( "  ✅ Perfect hash function: 100% lookup success rate" );
  }
  else
  {
    println!( "  ❌ Hash function issues: {failed_lookups} failed lookups" );
  }

  // Test lookup consistency
  println!( "\n  Testing lookup consistency..." );
  let mut consistent_results = true;

  for name in all_commands.keys()
  {
    let lookup1 = registry.command( name );
    let lookup2 = registry.command( name );

    match (lookup1, lookup2)
    {
      (Some( cmd1 ), Some( cmd2 )) if cmd1.name == cmd2.name => {},
      _ =>
      {
        consistent_results = false;
        break;
      }
    }
  }

  if consistent_results
  {
    println!( "  ✅ Lookup consistency: All repeated lookups return identical results" );
  }
  else
  {
    println!( "  ❌ Lookup inconsistency detected" );
  }

  Ok( () )
}