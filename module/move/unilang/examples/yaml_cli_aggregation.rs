//! YAML-Based CLI Aggregation: Complete Workflow Example
//!
//! This example demonstrates the complete workflow for aggregating multiple CLI tools
//! using YAML command definitions and compile-time PHF map generation.
//!
//! ## Workflow Demonstrated
//!
//! 1. Define individual CLI modules in separate YAML files
//! 2. Use build.rs to aggregate YAML files at compile-time
//! 3. Generate unified PHF map for zero-cost lookups
//! 4. Execute unified CLI with namespace isolation
//!
//! ## Real-World Application
//!
//! This pattern is ideal for organizations that want to:
//! - Maintain individual CLI tools as separate YAML files
//! - Version control each tool independently
//! - Aggregate tools into umbrella applications
//! - Maintain zero-cost lookup performance

use unilang::prelude::*;

fn main() -> Result< (), unilang::Error >
{
  println!( "=== YAML-Based CLI Aggregation Demo ===" );
  println!( "Demonstrating multi-YAML file aggregation workflow" );
  println!();

  // Demonstrate the YAML workflow
  demonstrate_yaml_workflow();

  // Show actual aggregation with existing static commands
  demonstrate_static_aggregation();

  Ok( () )
}

fn demonstrate_yaml_workflow()
{
  println!( "=== YAML CLI Aggregation Workflow ===" );
  println!();

  println!( "Step 1: Individual CLI Module YAML Files" );
  println!( "├── database/commands.yaml" );
  println!( "│   ├── migrate command" );
  println!( "│   ├── backup command" );
  println!( "│   └── restore command" );
  println!( "├── filesystem/commands.yaml" );
  println!( "│   ├── copy command" );
  println!( "│   ├── move command" );
  println!( "│   └── delete command" );
  println!( "└── network/commands.yaml" );
  println!( "    ├── ping command" );
  println!( "    ├── trace command" );
  println!( "    └── scan command" );
  println!();

  println!( "Step 2: build.rs Configuration" );
  println!( "```rust" );
  println!( "// build.rs" );
  println!( "use unilang::multi_yaml::MultiYamlAggregator;" );
  println!();
  println!( "fn main() {{" );
  println!( "  let aggregator = MultiYamlAggregator::new()" );
  println!( "    .add_yaml_module(\"database\", \"database/commands.yaml\", \"db\")" );
  println!( "    .add_yaml_module(\"filesystem\", \"filesystem/commands.yaml\", \"fs\")" );
  println!( "    .add_yaml_module(\"network\", \"network/commands.yaml\", \"net\")" );
  println!( "    .detect_conflicts(true)" );
  println!( "    .validate_schemas(true);" );
  println!();
  println!( "  // Generate unified PHF map at compile-time" );
  println!( "  let output_path = env::var(\"OUT_DIR\").unwrap() + \"/aggregated_commands.rs\";" );
  println!( "  aggregator.generate_static_map(&output_path)?;" );
  println!( "}}" );
  println!( "```" );
  println!();

  println!( "Step 3: Generated Static Command Map" );
  println!( "```rust" );
  println!( "// Generated in OUT_DIR/aggregated_commands.rs" );
  println!( "use phf::{{phf_map, Map}};" );
  println!( "use unilang::static_data::StaticCommandDefinition;" );
  println!();
  println!( "pub static AGGREGATED_COMMANDS: Map<&'static str, &'static StaticCommandDefinition> = phf_map! {{" );
  println!( "  \".db.migrate\" => &DB_MIGRATE_CMD," );
  println!( "  \".db.backup\" => &DB_BACKUP_CMD," );
  println!( "  \".fs.copy\" => &FS_COPY_CMD," );
  println!( "  \".net.ping\" => &NET_PING_CMD," );
  println!( "  // ... all commands with prefixes applied" );
  println!( "}};" );
  println!( "```" );
  println!();

  println!( "Step 4: Application Integration" );
  println!( "```rust" );
  println!( "// main.rs" );
  println!( "include!(concat!(env!(\"OUT_DIR\"), \"/aggregated_commands.rs\"));" );
  println!();
  println!( "fn main() -> Result<(), unilang::Error> {{" );
  println!( "  let registry = StaticCommandRegistry::from_phf(&AGGREGATED_COMMANDS);" );
  println!( "  let pipeline = Pipeline::new(registry);" );
  println!();
  println!( "  // Zero-cost command execution" );
  println!( "  pipeline.process_command_simple(\".db.migrate direction::up\")?;" );
  println!( "  pipeline.process_command_simple(\".fs.copy src::data dest::backup\")?;" );
  println!( "  Ok(())" );
  println!( "}}" );
  println!( "```" );
  println!();
}

fn demonstrate_static_aggregation()
{
  println!( "=== Static Aggregation Demo (Using Current Commands) ===" );
  println!();

  // Include the generated static commands
  // NOTE: Temporarily commented out due to build script integration issues
  // include!( concat!( env!( "OUT_DIR" ), "/static_commands.rs" ) );

  // Create registry from existing static commands
  // NOTE: Temporarily using new registry instead of static commands
  #[allow(deprecated)]
  let registry = CommandRegistry::new();
  let pipeline = Pipeline::new( registry );

  println!( "Available static commands: (placeholder - static commands temporarily disabled)" );
  // for ( name, cmd ) in STATIC_COMMANDS.entries()
  // {
  //   println!( "  {} - {}", name, cmd.description );
  // }
  println!();

  // Demonstrate aggregation patterns
  println!( "Aggregation Patterns Demonstrated:" );
  println!();

  println!( "1. **Namespace Isolation**" );
  println!( "   - Each module maintains its own command namespace" );
  println!( "   - Prefix application prevents naming conflicts" );
  println!( "   - Clear hierarchical organization" );
  println!();

  println!( "2. **Performance Characteristics**" );
  println!( "   - O(1) lookup regardless of module count" );
  println!( "   - Zero runtime aggregation overhead" );
  println!( "   - Single PHF map for all aggregated commands" );
  println!();

  // Test some commands to show aggregation in action
  println!( "3. **Unified Command Execution**" );
  let test_commands = vec!
  [
  ".greet name::Alice",
  ".greet name::Bob",
 ];

  for cmd_str in test_commands
  {
  println!( "   Executing: {}", cmd_str );
  let result = pipeline.process_command_simple( cmd_str );

  if result.success
  {
  println!( "   Result: ✅ {}", result.outputs[ 0 ].content );
 }
  else if let Some( error ) = result.error
  {
  println!( "   Result: ❌ {}", error );
 }
 }
  println!();

  println!( "4. **Development Workflow Benefits**" );
  println!( "   ✅ Individual YAML files for each CLI module" );
  println!( "   ✅ Version control isolation for independent development" );
  println!( "   ✅ Compile-time conflict detection and validation" );
  println!( "   ✅ Zero-cost runtime aggregation with PHF maps" );
  println!( "   ✅ Unified help system across all aggregated tools" );
  println!( "   ✅ Type safety and validation for all commands" );
  println!();

  println!( "5. **Organizational Benefits**" );
  println!( "   • **Team Independence**: Each team maintains their own YAML files" );
  println!( "   • **Consistent Interface**: Unified command syntax across all tools" );
  println!( "   • **Easy Onboarding**: Single CLI to learn instead of many" );
  println!( "   • **Centralized Documentation**: All commands discoverable through help system" );
  println!( "   • **Performance**: No runtime overhead for command lookup or namespace resolution" );
  println!();

  println!( "=== Example YAML Command Definition ===" );
  println!( "```yaml" );
  println!( "# database/commands.yaml" );
  println!( "- name: \"migrate\"" );
  println!( "  namespace: \"\"  # Will become .db.migrate with prefix" );
  println!( "  description: \"Run database migrations\"" );
  println!( "  arguments:" );
  println!( "    - name: \"direction\"" );
  println!( "      kind: \"String\"" );
  println!( "      attributes:" );
  println!( "        optional: true" );
  println!( "        default: \"up\"" );
  println!( "      validation_rules:" );
  println!( "        - Pattern: \"^(up|down|redo)$\"" );
  println!( "  tags: [\"database\", \"migration\"]" );
  println!( "  permissions: [\"database_admin\"]" );
  println!( "  examples:" );
  println!( "    - \"migrate direction::up\"" );
  println!( "    - \"migrate direction::down\"" );
  println!( "```" );
  println!();

  println!( "Run this example:" );
  println!( "```bash" );
  println!( "cargo run --example yaml_cli_aggregation" );
  println!( "```" );
}

