//! # Comprehensive Loader Demonstration
//!
//! This example demonstrates all aspects of command loading from YAML/JSON files,
//! including error handling, complex argument types, and routine resolution.

use unilang::registry::CommandRegistry;
use unilang::loader::{ load_command_definitions_from_yaml_str, load_command_definitions_from_json_str };
use unilang::help::HelpGenerator;

fn main() -> Result< (), unilang::error::Error >
{
  println!( "=== Comprehensive Command Loader Demo ===\n" );

  // Step 1: Demonstrate YAML loading with all features
  demonstrate_yaml_loading()?;

  // Step 2: Demonstrate JSON loading with all features
  demonstrate_json_loading()?;

  // Step 3: Error handling scenarios
  demonstrate_error_handling()?;

  // Step 4: Complex validation and types
  demonstrate_complex_features()?;

  println!( "\n=== Loader Best Practices ===\n" );
  display_best_practices();

  Ok( () )
}

fn demonstrate_yaml_loading() -> Result< (), unilang::error::Error >
{
  println!( "=== YAML Loading Demonstration ===\n" );

  let comprehensive_yaml = r#"
# Complete command definition showcasing all available fields
- name: "process_data"
  namespace: ".analytics"
  description: "Processes analytical data with comprehensive options"
  hint: "Data processing pipeline with validation"
  status: "stable"
  version: "3.1.2"
  tags:
    - "analytics"
    - "data"
    - "processing"
    - "ml"
  aliases:
    - "proc"
    - "analyze"
    - "process"
  permissions:
    - "read_data"
    - "write_results"
    - "access_analytics"
  idempotent: true
  deprecation_message: ""
  http_method_hint: "POST"
  examples:
    - "analytics.process_data input::/data/raw.csv output::/results/processed.json"
    - "proc input::data.csv algorithm::svm parameters::gamma=0.1,C=1.0"
    - "analyze input::large_dataset.parquet format::json validate::true"
  arguments:
    # String argument with pattern validation
    - name: "input"
      kind: "String"
      description: "Input data file path or URL"
      hint: "Path to input data file"
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["i", "source", "data"]
      tags: ["required", "input"]

    # File path argument with existence validation
    - name: "output"
      kind: "File"
      description: "Output file for processed results"
      hint: "Result file path"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "results.json"
      validation_rules: []
      aliases: ["o", "dest", "target"]
      tags: ["output", "file"]

    # Enum argument with predefined choices
    - name: "algorithm"
      kind: "Enum([\"linear\", \"svm\", \"random_forest\", \"neural_network\"])"
      description: "Machine learning algorithm to use"
      hint: "Choose processing algorithm"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "linear"
      validation_rules: []
      aliases: ["a", "algo", "method"]
      tags: ["algorithm", "ml"]

    # Map argument for algorithm parameters
    - name: "parameters"
      kind: "Map(String,Float,;,=)"
      description: "Algorithm-specific parameters"
      hint: "Format: param=value;param2=value2"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["p", "params", "config"]
      tags: ["configuration", "tuning"]

    # List argument for feature selection
    - name: "features"
      kind: "List(String,|)"
      description: "Features to include in processing"
      hint: "Pipe-separated feature names"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["f", "cols", "columns"]
      tags: ["features", "selection"]

    # Boolean flag for validation
    - name: "validate"
      kind: "Boolean"
      description: "Enable data validation before processing"
      hint: "Perform input validation"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "true"
      validation_rules: []
      aliases: ["v", "check"]
      tags: ["validation", "quality"]

    # Integer with range validation
    - name: "threads"
      kind: "Integer"
      description: "Number of processing threads"
      hint: "Thread count (1-32)"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "4"
      validation_rules: []
      aliases: ["t", "workers"]
      tags: ["performance", "parallelism"]

    # Float with precision requirements
    - name: "threshold"
      kind: "Float"
      description: "Confidence threshold for results"
      hint: "Minimum confidence (0.0-1.0)"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "0.85"
      validation_rules: []
      aliases: ["th", "confidence"]
      tags: ["filtering", "quality"]

    # DateTime for time-based filtering
    - name: "start_date"
      kind: "DateTime"
      description: "Start date for time-based filtering"
      hint: "ISO 8601 format"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["start", "from"]
      tags: ["temporal", "filtering"]

    # URL for remote data sources
    - name: "remote_source"
      kind: "Url"
      description: "Remote data source URL"
      hint: "HTTP/HTTPS data endpoint"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["url", "endpoint"]
      tags: ["remote", "api"]

    # Pattern for data filtering
    - name: "filter_pattern"
      kind: "Pattern"
      description: "Regex pattern for data filtering"
      hint: "Regular expression"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["regex", "pattern"]
      tags: ["filtering", "regex"]

    # JSON configuration object
    - name: "advanced_config"
      kind: "Object"
      description: "Advanced configuration as JSON object"
      hint: "Complex configuration parameters"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["config", "settings"]
      tags: ["advanced", "json"]

    # Sensitive API key (interactive)
    - name: "api_key"
      kind: "String"
      description: "API key for remote access"
      hint: "Secret API key"
      attributes:
        optional: true
        multiple: false
        interactive: true
        sensitive: true
      validation_rules: []
      aliases: ["key", "token"]
      tags: ["security", "auth"]

  routine_link: "analytics.process_data_routine"

# Second command demonstrating minimal required fields
- name: "simple_task"
  namespace: ".util"
  description: "Simple utility task with minimal configuration"
  hint: "Basic utility"
  status: "experimental"
  version: "0.1.0"
  tags: []
  aliases: []
  permissions: []
  idempotent: true
  deprecation_message: ""
  http_method_hint: "GET"
  examples: []
  arguments: []
  routine_link: null
"#;

  println!( "📋 Loading comprehensive YAML definitions..." );
  match load_command_definitions_from_yaml_str( comprehensive_yaml )
  {
    Ok( commands ) =>
    {
      println!( "✅ Successfully loaded {} commands from YAML", commands.len() );

      for cmd in &commands
      {
        println!( "\n🎯 Command: {}.{}", cmd.namespace, cmd.name );
        println!( "   Description: {}", cmd.description );
        println!( "   Status: {} (v{})", cmd.status, cmd.version );
        println!( "   Arguments: {} defined", cmd.arguments.len() );
        println!( "   Aliases: {:?}", cmd.aliases );
        println!( "   Tags: {:?}", cmd.tags );

        if !cmd.arguments.is_empty()
        {
          println!( "   🔧 Arguments:" );
          for arg in &cmd.arguments
          {
            let interactive = if arg.attributes.interactive { " (interactive)" } else { "" };
            let sensitive = if arg.attributes.sensitive { " (sensitive)" } else { "" };
            let optional = if arg.attributes.optional { " (optional)" } else { " (required)" };
            println!( "     • {}: {:?}{}{}{}", arg.name, arg.kind, optional, interactive, sensitive );
          }
        }
      }
    },
    Err( error ) =>
    {
      println!( "❌ YAML loading failed: {error}" );
      return Err( error );
    }
  }

  Ok( () )
}

fn demonstrate_json_loading() -> Result< (), unilang::error::Error >
{
  println!( "\n=== JSON Loading Demonstration ===\n" );

  let comprehensive_json = r#"[
  {
    "name": "deploy_service",
    "namespace": ".devops",
    "description": "Deploys microservices with comprehensive deployment options",
    "hint": "Production deployment tool",
    "status": "stable",
    "version": "2.5.1",
    "tags": ["devops", "deployment", "kubernetes", "docker"],
    "aliases": ["deploy", "release", "ship"],
    "permissions": ["deploy", "k8s_access", "docker_push"],
    "idempotent": false,
    "deprecation_message": "",
    "http_method_hint": "POST",
    "examples": [
      "devops.deploy_service service::api-gateway version::v1.2.3 env::production",
      "deploy service::user-service replicas::3 resources::cpu=500m,memory=1Gi"
    ],
    "arguments": [
      {
        "name": "service",
        "kind": "String",
        "description": "Service name to deploy",
        "hint": "Kubernetes service identifier",
        "attributes": {
          "optional": false,
          "multiple": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "aliases": ["s", "name"],
        "tags": ["required", "service"]
      },
      {
        "name": "version",
        "kind": "String",
        "description": "Service version/tag to deploy",
        "hint": "Docker image tag",
        "attributes": {
          "optional": false,
          "multiple": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "aliases": ["v", "tag"],
        "tags": ["required", "version"]
      },
      {
        "name": "environment",
        "kind": "Enum([\"development\", \"staging\", \"production\"])",
        "description": "Target deployment environment",
        "hint": "Choose target environment",
        "attributes": {
          "optional": true,
          "multiple": false,
          "interactive": false,
          "sensitive": false,
          "default": "staging"
        },
        "validation_rules": [],
        "aliases": ["env", "e"],
        "tags": ["environment"]
      },
      {
        "name": "replicas",
        "kind": "Integer",
        "description": "Number of service replicas",
        "hint": "Pod replica count",
        "attributes": {
          "optional": true,
          "multiple": false,
          "interactive": false,
          "sensitive": false,
          "default": "2"
        },
        "validation_rules": [],
        "aliases": ["r", "scale"],
        "tags": ["scaling"]
      },
      {
        "name": "resources",
        "kind": "Map(String,String,;,=)",
        "description": "Resource limits and requests",
        "hint": "cpu=500m,memory=1Gi format",
        "attributes": {
          "optional": true,
          "multiple": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "aliases": ["res", "limits"],
        "tags": ["resources", "limits"]
      },
      {
        "name": "config_overrides",
        "kind": "JsonString",
        "description": "Configuration overrides as JSON",
        "hint": "JSON configuration data",
        "attributes": {
          "optional": true,
          "multiple": false,
          "interactive": false,
          "sensitive": false
        },
        "validation_rules": [],
        "aliases": ["config", "overrides"],
        "tags": ["configuration"]
      },
      {
        "name": "secrets",
        "kind": "List(String,|)",
        "description": "Secret names to mount",
        "hint": "Pipe-separated secret list",
        "attributes": {
          "optional": true,
          "multiple": false,
          "interactive": false,
          "sensitive": true
        },
        "validation_rules": [],
        "aliases": ["sec"],
        "tags": ["security", "secrets"]
      }
    ],
    "routine_link": "devops.deploy_service_routine"
  }
]"#;

  println!( "📋 Loading comprehensive JSON definitions..." );
  match load_command_definitions_from_json_str( comprehensive_json )
  {
    Ok( commands ) =>
    {
      println!( "✅ Successfully loaded {} commands from JSON", commands.len() );

      for cmd in &commands
      {
        println!( "\n🚀 Command: {}.{}", cmd.namespace, cmd.name );
        println!( "   Description: {}", cmd.description );
        println!( "   Status: {} (v{})", cmd.status, cmd.version );
        println!( "   Arguments: {} defined", cmd.arguments.len() );
        println!( "   Permissions: {:?}", cmd.permissions );

        // Analyze argument complexity
        let mut arg_stats = std::collections::HashMap::new();
        for arg in &cmd.arguments
        {
          let kind_name = match &arg.kind
          {
            unilang::data::Kind::String => "String",
            unilang::data::Kind::Integer => "Integer",
            unilang::data::Kind::Enum(_) => "Enum",
            unilang::data::Kind::Map(_, _, _, _) => "Map",
            unilang::data::Kind::List(_, _) => "List",
            unilang::data::Kind::JsonString => "JsonString",
            _ => "Other",
          };
          *arg_stats.entry( kind_name ).or_insert( 0 ) += 1;
        }

        println!( "   🔢 Argument Types: {:?}", arg_stats );
      }
    },
    Err( error ) =>
    {
      println!( "❌ JSON loading failed: {error}" );
      return Err( error );
    }
  }

  Ok( () )
}

fn demonstrate_error_handling() -> Result< (), unilang::error::Error >
{
  println!( "\n=== Error Handling Scenarios ===\n" );

  let error_test_cases = vec![
    // Invalid YAML syntax
    (
      "Invalid YAML",
      r#"
- name: "test"
  namespace: ".test"
  description: "Test"
  invalid: yaml: syntax: {
"#,
      "YAML"
    ),

    // Invalid JSON syntax
    (
      "Invalid JSON",
      r#"[
  {
    "name": "test",
    "namespace": ".test",
    "description": "Test"
    "invalid": "json syntax"
  }
]"#,
      "JSON"
    ),

    // Empty input handling
    (
      "Empty YAML",
      "",
      "YAML"
    ),

    // Malformed command structure
    (
      "Missing required fields",
      r#"
- name: "incomplete"
  # Missing required fields like namespace, description
"#,
      "YAML"
    ),
  ];

  for ( description, content, format ) in error_test_cases
  {
    println!( "🧪 Testing: {description}" );

    let result = match format
    {
      "YAML" => load_command_definitions_from_yaml_str( content ),
      "JSON" => load_command_definitions_from_json_str( content ),
      _ => unreachable!(),
    };

    match result
    {
      Ok( commands ) =>
      {
        if commands.is_empty()
        {
          println!( "   ✅ Handled gracefully: Empty command list" );
        }
        else
        {
          println!( "   ⚠️ Unexpectedly succeeded with {} commands", commands.len() );
        }
      },
      Err( error ) =>
      {
        println!( "   ✅ Error caught correctly: {}", error );
      }
    }
    println!();
  }

  Ok( () )
}

fn demonstrate_complex_features() -> Result< (), unilang::error::Error >
{
  println!( "=== Complex Features Demonstration ===\n" );

  // Create registry and load complex commands
  let complex_yaml = r#"
- name: "ml_pipeline"
  namespace: ".ai"
  description: "Machine learning pipeline with complex type validation"
  hint: "AI/ML processing pipeline"
  status: "experimental"
  version: "0.8.0"
  tags: ["ai", "ml", "pipeline", "data-science"]
  aliases: ["ml", "train", "pipeline"]
  permissions: ["gpu_access", "large_memory"]
  idempotent: false
  deprecation_message: ""
  http_method_hint: "POST"
  examples:
    - "ai.ml_pipeline dataset::/data/training.csv model_config::'[{\"type\":\"dense\",\"units\":128}]'"
  arguments:
    - name: "dataset"
      kind: "File"
      description: "Training dataset file"
      hint: "CSV or Parquet file"
      attributes:
        optional: false
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["data", "train_data"]
      tags: ["required", "input"]

    - name: "model_config"
      kind: "JsonString"
      description: "Model architecture configuration"
      hint: "JSON array of layer definitions"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: '[{"type":"dense","units":64}]'
      validation_rules: []
      aliases: ["config", "arch"]
      tags: ["model", "architecture"]

    - name: "hyperparams"
      kind: "Map(String,Float,;,=)"
      description: "Hyperparameter values"
      hint: "learning_rate=0.001;batch_size=32"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["params", "hp"]
      tags: ["tuning", "optimization"]

    - name: "feature_columns"
      kind: "List(String,|)"
      description: "Feature columns to use"
      hint: "Pipe-separated column names"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
      validation_rules: []
      aliases: ["features", "cols"]
      tags: ["features"]

    - name: "validation_split"
      kind: "Float"
      description: "Validation data split ratio"
      hint: "Fraction for validation (0.0-1.0)"
      attributes:
        optional: true
        multiple: false
        interactive: false
        sensitive: false
        default: "0.2"
      validation_rules: []
      aliases: ["val_split", "validation"]
      tags: ["validation"]

  routine_link: "ai.ml_pipeline_routine"
"#;

  match load_command_definitions_from_yaml_str( complex_yaml )
  {
    Ok( commands ) =>
    {
      println!( "✅ Complex command loaded successfully" );

      let mut registry = CommandRegistry::new();
      for cmd in commands
      {
        println!( "\n🧠 ML Pipeline Command Analysis:" );
        println!( "   • Name: {}.{}", cmd.namespace, cmd.name );
        println!( "   • Arguments: {}", cmd.arguments.len() );

        // Analyze argument types and complexity
        let mut type_complexity = std::collections::HashMap::new();
        for arg in &cmd.arguments
        {
          let complexity = match &arg.kind
          {
            unilang::data::Kind::String | unilang::data::Kind::Integer | unilang::data::Kind::Float | unilang::data::Kind::Boolean => "Simple",
            unilang::data::Kind::File | unilang::data::Kind::Directory | unilang::data::Kind::Path => "FileSystem",
            unilang::data::Kind::Enum(_) => "Enum",
            unilang::data::Kind::List(_, _) => "Collection",
            unilang::data::Kind::Map(_, _, _, _) => "Collection",
            unilang::data::Kind::JsonString | unilang::data::Kind::Object => "JSON",
            unilang::data::Kind::Url | unilang::data::Kind::DateTime | unilang::data::Kind::Pattern => "Advanced",
          };
          *type_complexity.entry( complexity ).or_insert( 0 ) += 1;
        }

        println!( "   • Type Complexity: {:?}", type_complexity );

        // Create routine for demonstration
        let _demo_routine = Box::new( | cmd : unilang::semantic::VerifiedCommand, _ctx : unilang::interpreter::ExecutionContext | -> Result< unilang::data::OutputData, unilang::error::Error >
        {
          println!( "🚀 Executing ML Pipeline with {} arguments", cmd.arguments.len() );
          for ( name, value ) in &cmd.arguments
          {
            println!( "   Parameter {name}: {}", format_value_for_ml( value ) );
          }

          Ok( unilang::data::OutputData
          {
            content : "ML Pipeline execution completed (demo)".to_string(),
            format : "text".to_string(),
          })
        });

        registry.register( cmd );
        // Note: In a full demo, we'd register the routine too
      }

      let help_generator = HelpGenerator::new( &registry );
      if let Some( help ) = help_generator.command( "ai.ml_pipeline" )
      {
        println!( "\n📖 Generated Help Documentation:" );
        println!( "{}", help );
      }
    },
    Err( error ) =>
    {
      println!( "❌ Complex command loading failed: {error}" );
    }
  }

  Ok( () )
}

fn format_value_for_ml( value : &unilang::types::Value ) -> String
{
  match value
  {
    unilang::types::Value::JsonString( json ) => format!( "JSON({})", json ),
    unilang::types::Value::List( items ) => format!( "List[{}]", items.len() ),
    unilang::types::Value::Map( map ) => format!( "Map{{{}}} ", map.len() ),
    _ => value.to_string(),
  }
}

fn display_best_practices()
{
  println!( "🎯 Command Definition Best Practices:\n" );

  println!( "📋 YAML Recommendations:" );
  println!( "  • Use meaningful command and argument names" );
  println!( "  • Provide comprehensive descriptions and hints" );
  println!( "  • Include practical examples for complex commands" );
  println!( "  • Use appropriate validation rules for arguments" );
  println!( "  • Mark sensitive arguments appropriately" );
  println!( "  • Leverage tags for command categorization" );
  println!( "  • Define useful aliases for common commands" );

  println!( "\n🔒 Security Considerations:" );
  println!( "  • Mark sensitive arguments (passwords, API keys)" );
  println!( "  • Use interactive attributes for secure input" );
  println!( "  • Define appropriate permissions for commands" );
  println!( "  • Validate input patterns for security-critical args" );

  println!( "\n⚡ Performance Optimization:" );
  println!( "  • Use static commands for frequently used operations" );
  println!( "  • Minimize complex validation rules in hot paths" );
  println!( "  • Consider argument default values to reduce input" );
  println!( "  • Use appropriate collection delimiters for readability" );

  println!( "\n🛠️ Development Workflow:" );
  println!( "  • Start with simple YAML definitions" );
  println!( "  • Test loading with various input scenarios" );
  println!( "  • Use JSON for programmatic generation" );
  println!( "  • Validate definitions before deployment" );
  println!( "  • Version control your command definitions" );

  println!( "\n📊 Monitoring and Maintenance:" );
  println!( "  • Use status field to track command maturity" );
  println!( "  • Document deprecations with migration guidance" );
  println!( "  • Track command usage via tags and aliases" );
  println!( "  • Regular review of validation rules effectiveness" );
}