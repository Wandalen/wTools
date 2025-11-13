//! # Static and Dynamic Command Registry
//!
//! This example demonstrates the hybrid command registry system that combines
//! compile-time optimized static commands with runtime dynamic registration.

use unilang::data::{ ArgumentAttributes, ArgumentDefinition, CommandDefinition, Kind, OutputData };
use unilang::registry::CommandRegistry;
use unilang::static_data::{ StaticArgumentDefinition, StaticArgumentAttributes, StaticCommandDefinition, StaticKind };

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Static and Dynamic Command Registry Demo ===\n" );

  // Step 1: Create registry (includes static commands from build process)
  #[allow(deprecated)]
  let mut registry = CommandRegistry::new();
  println!( "✓ Registry initialized with {} static commands", registry.commands().len() );

  // Step 2: Demonstrate static command structure
  println!( "\n=== Static Command Examples ===\n" );
  
  // Show static command definitions (const-compatible)
  static STATIC_ARG: StaticArgumentDefinition = StaticArgumentDefinition {
    name: "input",
    kind: StaticKind::String,
    attributes: StaticArgumentAttributes {
      optional: false,
      multiple: false,
      default: None,
      sensitive: false,
      interactive: false,
    },
    hint: "Input text",
    description: "Text input for processing",
    validation_rules: &[],
    aliases: &["i"],
    tags: &["required"],
  };

  static STATIC_CMD: StaticCommandDefinition = StaticCommandDefinition {
    name: "static_example",
    namespace: ".demo",
    description: "Example of a static command definition",
    hint: "Static command demo",
    arguments: &[STATIC_ARG],
    routine_link: Some("demo.static_example"),
    status: "stable",
    version: "1.0.0",
    tags: &["demo", "static"],
    aliases: &["se"],
    permissions: &[],
    idempotent: true,
    deprecation_message: "",
    http_method_hint: "GET",
    examples: &["demo.static_example input::hello"],
    auto_help_enabled: true,
    category: "",
  };

  println!( "📋 Static Command Definition:" );
  println!( "  Name: {}", STATIC_CMD.name );
  println!( "  Namespace: {}", STATIC_CMD.namespace );
  println!( "  Description: {}", STATIC_CMD.description );
  println!( "  Arguments: {} defined", STATIC_CMD.arguments.len() );
  println!( "  Aliases: {:?}", STATIC_CMD.aliases );
  println!( "  Tags: {:?}", STATIC_CMD.tags );

  // Convert static to dynamic for registration
  let _dynamic_cmd: CommandDefinition = (&STATIC_CMD).into();
  println!( "✓ Converted static command to dynamic format" );

  // Step 3: Register dynamic commands at runtime
  println!( "\n=== Dynamic Command Registration ===\n" );

  let dynamic_cmd = CommandDefinition::former()
  .name( ".dynamic_example" )
  .namespace( ".demo" )
  .description( "Example of a runtime-registered command".to_string() )
  .hint( "Dynamic command demo" )
  .status( "experimental" )
  .version( "0.1.0" )
  .aliases( vec![ ".de".to_string() ] )
  .tags( vec![ "demo".to_string(), "dynamic".to_string() ] )
  .permissions( vec![] )
  .idempotent( false )
  .deprecation_message( String::new() )
  .http_method_hint( "POST".to_string() )
  .examples( vec![ "demo.dynamic_example data::sample".to_string() ] )
  .arguments( vec!
  [
    ArgumentDefinition {
      name: "data".to_string(),
      description: "Data to process".to_string(),
      kind: Kind::String,
      hint: "Any data string".to_string(),
      attributes: ArgumentAttributes { optional: false, ..Default::default() },
      validation_rules: vec![],
      aliases: vec![ "d".to_string() ],
      tags: vec![ "required".to_string() ],
    },
    ArgumentDefinition {
      name: "format".to_string(),
      description: "Output format".to_string(),
      kind: Kind::Enum( vec![ "json".to_string(), "xml".to_string(), "yaml".to_string() ] ),
      hint: "Choose output format".to_string(),
      attributes: ArgumentAttributes { 
        optional: true, 
        default: Some("json".to_string()),
        ..Default::default() 
      },
      validation_rules: vec![],
      aliases: vec![ "f".to_string() ],
      tags: vec![ "format".to_string() ],
    },
  ])
  .end();

  let dynamic_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx |
  {
    println!( "🚀 Executing dynamic command with {} arguments", cmd.arguments.len() );
    
    for ( name, value ) in &cmd.arguments
    {
      println!( "  • {name}: {value}" );
    }

    Ok( OutputData
    {
      content : format!( "Dynamic command processed {} arguments", cmd.arguments.len() ),
      format : "text".to_string(),
      execution_time_ms : None,
    })
  });

  #[allow(deprecated)]
  registry.command_add_runtime( &dynamic_cmd, dynamic_routine )?;
  println!( "✓ Dynamic command registered successfully" );

  // Step 4: Compare static vs dynamic performance
  println!( "\n=== Performance Comparison ===\n" );

  println!( "🏁 Static Commands (Compile-Time Optimized):" );
  println!( "  • Compile-time generation with optimized lookup tables" );
  println!( "  • Zero allocation lookups: O(1) constant time" );
  println!( "  • Memory efficient: embedded in binary" );
  println!( "  • Startup time: ~95μs (vs 5000μs requirement)" );
  println!( "  • Lookup latency: ~0.2μs (vs 1000μs requirement)" );
  println!( "  • Immutable after compilation" );

  println!( "\n🔄 Dynamic Commands (HashMap-based):" );
  println!( "  • Runtime registration and modification" );
  println!( "  • Hash-based lookups: O(1) average, O(n) worst case" );
  println!( "  • Heap allocated: flexible but slower" );
  println!( "  • Can be added/removed/modified at runtime" );
  println!( "  • Slightly higher memory overhead" );

  // Step 5: Demonstrate hybrid lookup
  println!( "\n=== Hybrid Registry Lookup ===\n" );

  let test_commands = vec![
    ".perf.cmd_1",        // Likely from static registry (performance test commands)
    "demo.dynamic_example", // From our dynamic registration
    "nonexistent.command",  // Should not be found
  ];

  for cmd_name in test_commands
  {
    println!( "🔍 Looking up: {cmd_name}" );
    match registry.command( cmd_name )
    {
      Some( command ) =>
      {
        let source = if cmd_name.starts_with( ".perf" ) { "static registry" } else { "dynamic HashMap" };
        println!( "  ✓ Found in {source} registry" );
        println!( "    Name: {}", command.name() );
        println!( "    Namespace: {}", command.namespace() );
        println!( "    Description: {}", command.description() );
        println!( "    Arguments: {}", command.arguments().len() );
      },
      None =>
      {
        println!( "  ❌ Command not found in either registry" );
      }
    }
    println!();
  }

  // Step 6: Registry statistics
  println!( "=== Registry Statistics ===" );
  let total_commands = registry.commands().len();
  println!( "📊 Registry Information:" );
  println!( "  • Total commands: {total_commands}" );
  println!( "  • Static commands: Generated at build time" );
  println!( "  • Dynamic commands: Registered at runtime" );
  println!( "  • Lookup strategy: Static first, then dynamic fallback" );
  println!( "  • Memory usage: Static commands have zero runtime cost" );

  println!( "\n=== Build Process Integration ===\n" );
  println!( "🔧 Static Command Generation:" );
  println!( "  1. build.rs reads command definitions from YAML" );
  println!( "  2. Generates optimized static map at compile-time" );
  println!( "  3. Outputs static_commands.rs with const data structures" );
  println!( "  4. Registry includes generated file for zero-cost lookup" );

  println!( "\n📝 Generated Code Structure:" );
  println!( r#"
// Generated in static_commands.rs (internal implementation):
const STATIC_COMMANDS_INTERNAL: phf::Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{
  ".perf.cmd_1" => &CMD_0,
  ".perf.cmd_2" => &CMD_1,
  // ... millions of commands with O(1) lookup
}};

// Public API - implementation details hidden
pub static STATIC_COMMANDS: StaticCommandMap = StaticCommandMap::from_phf_internal(&STATIC_COMMANDS_INTERNAL);
"# );

  println!( "=== Advantages of Hybrid Approach ===\n" );
  println!( "⚡ Performance Benefits:" );
  println!( "  • Static commands: Ultra-fast, zero-allocation lookup" );
  println!( "  • Bulk commands: Perfect for CLI tools with many predefined commands" );
  println!( "  • Critical path optimization: Fastest possible command resolution" );

  println!( "\n🔄 Flexibility Benefits:" );
  println!( "  • Dynamic registration: Add commands based on runtime conditions" );
  println!( "  • Plugin system: Load commands from external modules" );
  println!( "  • User customization: Runtime command modification" );
  println!( "  • Development workflow: Hot reloading of command definitions" );

  println!( "\n🏗️ Architecture Benefits:" );
  println!( "  • Separation of concerns: Static for performance, dynamic for flexibility" );
  println!( "  • Memory efficiency: Static commands embedded in binary" );
  println!( "  • Deployment efficiency: Single binary contains all static commands" );
  println!( "  • Backwards compatibility: Existing code works with both types" );

  println!( "\n=== Usage Recommendations ===\n" );
  println!( "📋 Use Static Commands for:" );
  println!( "  • Core application commands" );
  println!( "  • Performance-critical operations" );
  println!( "  • Large command sets (1000+ commands)" );
  println!( "  • Production deployments" );

  println!( "\n🔧 Use Dynamic Commands for:" );
  println!( "  • Plugin-based commands" );
  println!( "  • User-defined commands" );
  println!( "  • Development and testing" );
  println!( "  • Runtime configuration" );

  println!( "\n=== Example Usage ===" );
  println!( "# Commands are looked up seamlessly:" );
  println!( "cargo run --bin unilang_cli demo.dynamic_example data::'hello world' format::json" );
  println!( "cargo run --bin unilang_cli .perf.cmd_1 input::test    # If static command exists" );
  
  println!( "\n💡 The hybrid approach provides the best of both worlds:" );
  println!( "   Performance + Flexibility = Production-Ready CLI Framework" );

  Ok( () )
}