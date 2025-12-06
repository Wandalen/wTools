//! YAML-Based CLI Aggregation: Complete Workflow Example
//!
//! This example demonstrates the complete workflow for aggregating multiple CLI tools
//! using YAML command definitions and compile-time static map generation.
//!
//! ## Workflow Demonstrated
//!
//! 1. Define individual CLI modules in separate YAML files
//! 2. Use build.rs to aggregate YAML files at compile-time
//! 3. Generate unified static command map for zero-cost lookups
//! 4. Execute unified CLI with namespace isolation
//!
//! ## Real-World Application
//!
//! This pattern is ideal for organizations that want to:
//! - Maintain individual CLI tools as separate YAML files
//! - Version control each tool independently
//! - Aggregate tools into umbrella applications
//! - Maintain zero-cost lookup performance

use unilang::multi_yaml::{ MultiYamlAggregator, AggregationConfig, ModuleConfig, ConflictResolutionStrategy, NamespaceIsolation };

fn main()
{
  println!( "=== YAML-Based CLI Aggregation Demo ===" );
  println!( "Demonstrating multi-YAML file aggregation workflow" );
  println!();

  // Demonstrate the YAML workflow
  demonstrate_yaml_workflow();

  // Show actual aggregation with existing static commands
  demonstrate_static_aggregation();
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
  println!( "  // Generate unified static map at compile-time" );
  println!( "  let output_path = env::var(\"OUT_DIR\").unwrap() + \"/aggregated_commands.rs\";" );
  println!( "  aggregator.generate_static_map(&output_path)?;" );
  println!( "}}" );
  println!( "```" );
  println!();

  println!( "Step 3: Generated Static Command Map" );
  println!( "```rust" );
  println!( "// Generated in OUT_DIR/aggregated_commands.rs" );
  println!( "// Internal implementation uses optimized static maps for O(1) lookup." );
  println!( "// Users interact with the public API only:" );
  println!( "use unilang::static_data::{{StaticCommandMap, StaticCommandDefinition}};" );
  println!();
  println!( "/// Aggregated commands with O(1) lookup" );
  println!( "pub static AGGREGATED_COMMANDS: StaticCommandMap = StaticCommandMap::from_definitions(&[" );
  println!( "  DB_MIGRATE_CMD,  // .svc1.migrate" );
  println!( "  DB_BACKUP_CMD,   // .svc1.backup" );
  println!( "  FS_COPY_CMD,     // .cmd2.copy" );
  println!( "  NET_PING_CMD,    // .net.ping" );
  println!( "  // ... all commands with prefixes applied" );
  println!( "]);" );
  println!( "```" );
  println!();

  println!( "Step 4: Application Integration" );
  println!( "```rust" );
  println!( "// main.rs" );
  println!( "include!(concat!(env!(\"OUT_DIR\"), \"/aggregated_commands.rs\"));" );
  println!();
  println!( "fn main() -> Result<(), unilang::Error> {{" );
  println!( "  let registry = StaticCommandRegistry::from_commands(&AGGREGATED_COMMANDS);" );
  println!( "  let pipeline = Pipeline::new(registry);" );
  println!();
  println!( "  // Zero-cost command execution with proper prefixes" );
  println!( "  pipeline.process_command_simple(\".tool.svc1.migrate direction::up\")?;" );
  println!( "  pipeline.process_command_simple(\".tool.cmd2.copy src::data dest::backup\")?;" );
  println!( "  pipeline.process_command_simple(\".tool.net.ping host::example.com\")?;" );
  println!( "  Ok(())" );
  println!( "}}" );
  println!( "```" );
  println!();
}

#[allow(clippy::too_many_lines)]
fn demonstrate_static_aggregation()
{
  println!( "=== Static Aggregation Demo (Using MultiYamlAggregator) ===" );
  println!();

  // Demonstrate the actual MultiYamlAggregator API
  let config = AggregationConfig
  {
    base_dir: std::path::PathBuf::from( "examples" ),
    modules: vec![
      ModuleConfig
      {
        name: "database".to_string(),
        yaml_path: "database/commands.yaml".to_string(),
        prefix: Some( "db".to_string() ),
        enabled: true,
      },
      ModuleConfig
      {
        name: "filesystem".to_string(),
        yaml_path: "filesystem/commands.yaml".to_string(),
        prefix: Some( "cmd2".to_string() ),
        enabled: true,
      },
      ModuleConfig
      {
        name: "network".to_string(),
        yaml_path: "network/commands.yaml".to_string(),
        prefix: Some( "net".to_string() ),
        enabled: true,
      },
    ],
    global_prefix: Some( "tool".to_string() ),
    detect_conflicts: true,
    env_overrides: std::collections::HashMap::new(),
    conflict_resolution: ConflictResolutionStrategy::Fail,
    auto_discovery: false,
    discovery_patterns: vec![ "*.yaml".to_string() ],
    namespace_isolation: NamespaceIsolation
    {
      enabled: true,
      separator: ".".to_string(),
      strict_mode: false,
    },
  };

  let aggregator = MultiYamlAggregator::new( config );

  println!( "MultiYamlAggregator Configuration:" );
  println!( "  Base directory: {}", aggregator.config().base_dir.display() );
  println!( "  Modules count: {}", aggregator.config().modules.len() );
  println!( "  Global prefix: {:?}", aggregator.config().global_prefix );
  println!( "  Conflict detection: {}", aggregator.config().detect_conflicts );
  println!();

  // Demonstrate aggregation patterns
  println!( "Aggregation Patterns Demonstrated:" );
  println!();

  println!( "1. **Namespace Isolation**" );
  println!( "   - Each module maintains its own command namespace" );
  println!( "   - Prefix application prevents naming conflicts" );
  println!( "   - Clear hierarchical organization" );
  println!( "   - Global prefix: .tool applied to all commands" );
  println!();

  println!( "2. **Performance Characteristics**" );
  println!( "   - O(1) lookup regardless of module count" );
  println!( "   - Zero runtime aggregation overhead" );
  println!( "   - Single static map for all aggregated commands" );
  println!( "   - Commands resolved at compile-time via build.rs" );
  println!();

  println!( "3. **YAML File Processing**" );
  println!( "   Expected command structure after aggregation:" );
  println!( "   - .tool.svc1.migrate  (from database/commands.yaml)" );
  println!( "   - .tool.svc1.backup   (from database/commands.yaml)" );
  println!( "   - .tool.cmd2.copy     (from filesystem/commands.yaml)" );
  println!( "   - .tool.cmd2.move     (from filesystem/commands.yaml)" );
  println!( "   - .tool.net.ping    (from network/commands.yaml)" );
  println!( "   - .tool.net.trace   (from network/commands.yaml)" );
  println!();

  println!( "4. **Development Workflow Benefits**" );
  println!( "   ✅ Individual YAML files for each CLI module" );
  println!( "   ✅ Version control isolation for independent development" );
  println!( "   ✅ Compile-time conflict detection and validation" );
  println!( "   ✅ Zero-cost runtime aggregation with static maps" );
  println!( "   ✅ Unified help system across all aggregated tools" );
  println!( "   ✅ Type safety and validation for all commands" );
  println!( "   ✅ Environment variable overrides support" );
  println!( "   ✅ Automatic YAML file discovery" );
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
  println!( "  namespace: \"\"  # Will become .svc1.migrate with prefix" );
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

