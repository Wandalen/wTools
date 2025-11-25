#![allow(clippy::all)]
//! # YAML and JSON Command Loading
//!
//! This example demonstrates how to load command definitions from external
//! YAML and JSON files, allowing for declarative command specification.

use unilang::registry::CommandRegistry;
use unilang::help::HelpGenerator;

#[allow(clippy::too_many_lines)]
fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== YAML and JSON Command Loading Demo ===\n" );

  // Step 1: Define commands in YAML format
  let yaml_commands = r#"
- name: "backup"
  namespace: ".system"
  description: "Creates a backup of specified files and directories"
  hint: "Backup utility with compression"
  status: "stable"
  version: "3.2.1"
  aliases: ["bak", "archive"]
  tags: ["filesystem", "backup", "compression"]
  permissions: ["read_file", "write_file"]
  idempotent: false
  deprecation_message: ""
  http_method_hint: "POST"
  examples:
    - "system.backup source::/home/user destination::/backup/user.tar.gz"
    - "bak source::~/documents destination::backup.zip compress::gzip"
  arguments:
    - name: "source"
      description: "Source file or directory to backup"
      kind: "Path"
      hint: "Path to backup source"
      attributes:
        optional: false
        multiple: false
        is_default_arg: false
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: null
      aliases: ["s", "src"]
      tags: ["required", "input"]
    - name: "destination"
      description: "Destination path for the backup archive"
      kind: "Path"
      hint: "Backup archive location"
      attributes:
        optional: false
        multiple: false
        is_default_arg: false
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: null
      aliases: ["d", "dest", "output"]
      tags: ["required", "output"]
    - name: "compress"
      description: "Compression algorithm to use"
      kind: "Enum([\"none\", \"gzip\", \"bzip2\", \"xz\"])"
      hint: "Compression method"
      attributes:
        optional: true
        multiple: false
        is_default_arg: true
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: "gzip"
      aliases: ["c", "compression"]
      tags: ["compression"]
    - name: "exclude"
      description: "Patterns to exclude from backup"
      kind: "List(String,|)"
      hint: "Pipe-separated exclusion patterns"
      attributes:
        optional: true
        multiple: false
        is_default_arg: false
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: null
      aliases: ["x", "ignore"]
      tags: ["filtering"]

- name: "restore"
  namespace: ".system"
  description: "Restores files from a backup archive"
  hint: "Restore from backup archives"
  status: "beta"
  version: "2.1.0"
  aliases: ["unpack", "extract"]
  tags: ["filesystem", "backup", "restoration"]
  permissions: ["read_file", "write_file"]
  idempotent: false
  deprecation_message: ""
  http_method_hint: "POST"
  examples:
    - "system.restore archive::backup.tar.gz target::/restore/location"
    - "restore archive::~/backup.zip target::. verify::true"
  arguments:
    - name: "archive"
      description: "Backup archive to restore from"
      kind: "File"
      hint: "Path to backup archive"
      attributes:
        optional: false
        multiple: false
        is_default_arg: false
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: null
      aliases: ["a", "backup", "file"]
      tags: ["required", "input"]
    - name: "target"
      description: "Target directory for restoration"
      kind: "Directory"
      hint: "Restore destination"
      attributes:
        optional: true
        multiple: false
        is_default_arg: true
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: "."
      aliases: ["t", "dest", "destination"]
      tags: ["output"]
    - name: "verify"
      description: "Verify archive integrity before restoration"
      kind: "Boolean"
      hint: "Enable integrity verification"
      attributes:
        optional: true
        multiple: false
        is_default_arg: true
        interactive: false
        sensitive: false
      validation_rules: []
      default_value: "true"
      aliases: ["v", "check"]
      tags: ["verification", "integrity"]
"#;

  // Step 2: Define commands in JSON format
  let json_commands = r#"
[
  {
    "name": "monitor",
    "namespace": ".system",
    "description": "Monitors system resources and performance metrics",
    "hint": "Real-time system monitoring",
    "status": "experimental",
    "version": "0.5.2",
    "aliases": ["watch", "track", "observe"],
    "tags": ["monitoring", "performance", "system"],
    "permissions": ["read_system"],
    "idempotent": true,
    "deprecation_message": "",
    "http_method_hint": "GET",
    "examples": [
      "system.monitor interval::5 metrics::cpu,memory",
      "monitor interval::1 metrics::all format::json"
    ],
    "arguments": [
      {
        "name": "interval",
        "description": "Monitoring interval in seconds",
        "kind": "Integer",
        "hint": "Seconds between updates",
        "attributes": {
          "optional": true,
          "multiple": false,
          "is_default_arg": true,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": ["min:1", "max:3600"],
        "default_value": "10",
        "aliases": ["i", "freq", "frequency"],
        "tags": ["timing"]
      },
      {
        "name": "metrics",
        "description": "Metrics to monitor",
        "kind": "List(String,,)",
        "hint": "Comma-separated metric names",
        "attributes": {
          "optional": true,
          "multiple": false,
          "is_default_arg": true,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": ["min_length:1"],
        "default_value": "cpu,memory,disk",
        "aliases": ["m", "stats"],
        "tags": ["monitoring"]
      },
      {
        "name": "format",
        "description": "Output format for metrics",
        "kind": "Enum([\"table\", \"json\", \"csv\", \"xml\"])",
        "hint": "Data presentation format",
        "attributes": {
          "optional": true,
          "multiple": false,
          "is_default_arg": true,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "default_value": "table",
        "aliases": ["f", "fmt"],
        "tags": ["formatting"]
      },
      {
        "name": "alert_thresholds",
        "description": "Alert thresholds for metrics",
        "kind": "Map(String,Float,;,:)",
        "hint": "metric:threshold;metric2:threshold2",
        "attributes": {
          "optional": true,
          "multiple": false,
          "is_default_arg": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "default_value": null,
        "aliases": ["alerts", "thresholds"],
        "tags": ["alerting"]
      }
    ]
  },
  {
    "name": "deploy",
    "namespace": ".app",
    "description": "Deploys applications to various environments",
    "hint": "Application deployment utility",
    "status": "stable",
    "version": "4.1.0",
    "aliases": ["release", "publish"],
    "tags": ["deployment", "devops", "automation"],
    "permissions": ["deploy_app", "modify_environment"],
    "idempotent": false,
    "deprecation_message": "",
    "http_method_hint": "POST",
    "examples": [
      "app.deploy env::production version::2.1.0",
      "deploy env::staging version::latest rollback-on-failure::true"
    ],
    "arguments": [
      {
        "name": "environment",
        "description": "Target deployment environment",
        "kind": "Enum([\"development\", \"staging\", \"production\"])",
        "hint": "Deployment target",
        "attributes": {
          "optional": false,
          "multiple": false,
          "is_default_arg": false,
          "interactive": true,
          "sensitive": false
        },
        "validation_rules": [],
        "default_value": null,
        "aliases": ["e", "env", "target"],
        "tags": ["required", "environment"]
      },
      {
        "name": "version",
        "description": "Application version to deploy",
        "kind": "String",
        "hint": "Version tag or identifier",
        "attributes": {
          "optional": false,
          "multiple": false,
          "is_default_arg": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": ["regex:^[0-9]+\\.[0-9]+\\.[0-9]+.*$"],
        "default_value": null,
        "aliases": ["v", "ver", "tag"],
        "tags": ["required", "versioning"]
      },
      {
        "name": "rollback_on_failure",
        "description": "Automatically rollback on deployment failure",
        "kind": "Boolean",
        "hint": "Enable automatic rollback",
        "attributes": {
          "optional": true,
          "multiple": false,
          "is_default_arg": true,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "default_value": "true",
        "aliases": ["rollback", "safe"],
        "tags": ["safety", "rollback"]
      }
    ]
  }
]
"#;

  // Step 3: Load commands from YAML
  println!( "📋 Loading commands from YAML..." );
  let yaml_registry = CommandRegistry::builder()
  .load_from_yaml_str( yaml_commands )?
  .build();

  println!( "✓ Loaded {} commands from YAML", yaml_registry.commands().len() );

  // Step 4: Load commands from JSON
  println!( "\n📋 Loading commands from JSON..." );
  let json_registry = CommandRegistry::builder()
  .load_from_json_str( json_commands )?
  .build();

  println!( "✓ Loaded {} commands from JSON", json_registry.commands().len() );

  // Step 5: Combine both registries
  println!( "\n🔗 Combining registries..." );
  #[allow(deprecated)]
  let mut combined_registry = CommandRegistry::new();

  // Add YAML commands
  for ( _name, command ) in yaml_registry.commands()
  {
    combined_registry.register( command ).expect( "Valid commands should register successfully" );
  }

  // Add JSON commands
  for ( _name, command ) in json_registry.commands()
  {
    combined_registry.register( command ).expect( "Valid commands should register successfully" );
  }

  println!( "✓ Combined registry has {} total commands", combined_registry.commands().len() );

  // Step 6: Display help for loaded commands
  let help_generator = HelpGenerator::new( &combined_registry );

  println!( "\n=== Commands Loaded from External Files ===" );
  println!( "{}", help_generator.list_commands() );

  // Step 7: Show detailed help for specific commands
  println!( "\n=== YAML-Loaded Command Details ===" );
  if let Some( backup_help ) = help_generator.command( "system.backup" )
  {
    println!( "{backup_help}" );
  }

  println!( "\n=== JSON-Loaded Command Details ===" );
  if let Some( monitor_help ) = help_generator.command( "system.monitor" )
  {
    println!( "{monitor_help}" );
  }

  println!( "\n=== External Definition Benefits ===" );
  println!( "✨ Loading from YAML/JSON provides:" );
  println!( "  • Separation of command definitions from code" );
  println!( "  • Easy configuration management" );
  println!( "  • Version control for command specs" );
  println!( "  • Non-programmer friendly editing" );
  println!( "  • Dynamic command loading" );
  println!( "  • Easier maintenance of large command sets" );
  println!( "  • Consistent structure validation" );

  println!( "\n=== File Format Comparison ===" );
  println!( "YAML advantages:" );
  println!( "  • Human-readable and editable" );
  println!( "  • Supports comments" );
  println!( "  • Less verbose than JSON" );
  println!( "  • Better for complex configurations" );

  println!( "\nJSON advantages:" );
  println!( "  • Ubiquitous format support" );
  println!( "  • Strict syntax validation" );
  println!( "  • Better tooling support" );
  println!( "  • Faster parsing performance" );

  println!( "\n=== Usage Examples ===" );
  println!( "# Test the loaded commands:" );
  println!( "cargo run --bin unilang_cli system.backup --help" );
  println!( "cargo run --bin unilang_cli system.monitor --help" );
  println!( "cargo run --bin unilang_cli app.deploy --help" );

  println!( "\n# Using aliases:" );
  println!( "cargo run --bin unilang_cli bak --help" );
  println!( "cargo run --bin unilang_cli watch --help" );
  println!( "cargo run --bin unilang_cli release --help" );

  println!( "\n💡 Note: Since these commands were loaded without routine_link," );
  println!( "   they use placeholder routines. In a real application, you would" );
  println!( "   specify routine_link values to connect to actual implementations." );

  Ok(())
}